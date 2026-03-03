# LevitateOS development commands

# QEMU tools environment
# The tooling cache lives under .artifacts/tools/.tools.
tools_prefix := join(justfile_directory(), ".artifacts/tools/.tools")
export PATH := tools_prefix / "usr/bin" + ":" + tools_prefix / "usr/libexec" + ":" + env("PATH")
export LD_LIBRARY_PATH := tools_prefix / "usr/lib64"
export OVMF_PATH := tools_prefix / "usr/share/edk2/ovmf/OVMF_CODE.fd"
export OVMF_VARS_PATH := tools_prefix / "usr/share/edk2/ovmf/OVMF_VARS.fd"

# -----------------------------------------------------------------------------
# xtask wrappers
# Prefer invoking xtask from just, and keep the justfile itself logic-light.

# Print the environment exports that the justfile sets for QEMU/tooling.
#
# Usage:
#   eval "$(just env bash)"
env shell="bash":
    cargo xtask env {{shell}}

# Check that local toolchain/tools match what this repo expects.
doctor:
    cargo xtask doctor

# Install canonical QEMU/OVMF helper tooling into .artifacts/tools/.tools.
tools-install:
    cargo run -p levitate-recipe --bin recipe -- install --build-dir {{join(justfile_directory(), ".artifacts/tools")}} --recipes-path distro-builder/recipes --no-persist-ctx --define TOOLS_PREFIX={{tools_prefix}} qemu-deps

# Short alias for tools-install.
tools:
    just tools-install

# Fail fast on forbidden legacy stage/rootfs bindings.
policy-legacy:
    cargo xtask policy audit-legacy-bindings

# Install/remove the shared pre-commit hook into the workspace + Rust submodules.
hooks-install:
    cargo xtask hooks install

hooks-remove:
    cargo xtask hooks remove

# Kernel helpers (x86_64).
kernels-check:
    cargo xtask kernels check

kernels-check-one distro:
    cargo xtask kernels check {{distro}}

kernels-build-plain distro:
    cargo xtask kernels build {{distro}}

kernels-build distro llm_profile="kernels_nightly" attempts="4" prompt_file="":
    if [ -n "{{prompt_file}}" ]; then cargo xtask kernels build {{distro}} --autofix --autofix-attempts {{attempts}} --llm-profile "{{llm_profile}}" --autofix-prompt-file "{{prompt_file}}"; else cargo xtask kernels build {{distro}} --autofix --autofix-attempts {{attempts}} --llm-profile "{{llm_profile}}"; fi

kernels-build-all-plain:
    cargo xtask kernels build-all

kernels-build-all llm_profile="kernels_nightly" attempts="4" prompt_file="":
    if [ -n "{{prompt_file}}" ]; then cargo xtask kernels build-all --autofix --autofix-attempts {{attempts}} --llm-profile "{{llm_profile}}" --autofix-prompt-file "{{prompt_file}}"; else cargo xtask kernels build-all --autofix --autofix-attempts {{attempts}} --llm-profile "{{llm_profile}}"; fi

kernels-rebuild distro:
    cargo xtask kernels build {{distro}} --rebuild

kernels-rebuild-all:
    cargo xtask kernels build-all --rebuild

# Canonical Stage 01 source preseed helpers (distro-builder artifact commands).
[script, no-exit-message]
preseed distro refresh="true":
    #!/usr/bin/env bash
    set -euo pipefail

    case "{{distro}}" in
      levitate|leviso)
        if [ "{{refresh}}" = "true" ]; then
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rocky-iso levitate --refresh
        else
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rocky-iso levitate
        fi
        ;;
      acorn|acornos)
        if [ "{{refresh}}" = "true" ]; then
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-alpine-stage01-assets acorn --refresh
        else
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-alpine-stage01-assets acorn
        fi
        ;;
      *)
        echo "preseed supports: levitate, acorn (got: {{distro}})" >&2
        exit 2
        ;;
    esac

preseed-levitate:
    just preseed levitate true

preseed-acorn:
    just preseed acorn true

# Internal delegate for stage booting.
# Keep `cargo xtask stages boot` as the only execution path for stage wrappers.
# Boundary rule: stage wrappers consume existing artifacts only.
# Do not add implicit ISO build steps here; freshness is explicit via `just build*`.
[script, no-exit-message]
_boot_stage n distro="levitate" inject="" inject_file="" ssh="false" no_shell="false" window="false" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey="" ssh_port="2222" inject_append="":
    #!/usr/bin/env bash
    set -euo pipefail
    CARGO_BIN="${CARGO_BIN:-cargo}"
    if ! command -v "$CARGO_BIN" >/dev/null 2>&1; then
      if [ -n "${HOME:-}" ] && [ -x "${HOME}/.cargo/bin/cargo" ]; then
        CARGO_BIN="${HOME}/.cargo/bin/cargo"
      else
        echo "cargo not found in PATH for _boot_stage." >&2
        echo "Remediation: source \"\$HOME/.cargo/env\" (or install Rust), then rerun." >&2
        exit 127
      fi
    fi
    if [ "{{ssh}}" = "true" ] && [ "{{n}}" != "1" ] && [ "{{n}}" != "01" ] && [ "{{n}}" != "2" ] && [ "{{n}}" != "02" ]; then
      echo "SSH boot mode supports only live stages 1/2 (got: {{n}})" >&2
      exit 2
    fi

    args=("$CARGO_BIN" xtask stages boot {{n}} "{{distro}}")

    if [ "{{ssh}}" = "true" ]; then
      args+=(--ssh --ssh-port "{{ssh_port}}")
    fi

    if [ "{{no_shell}}" = "true" ]; then
      args+=(--no-shell)
    fi

    if [ "{{window}}" = "true" ]; then
      args+=(--window)
    fi

    inject_append="{{inject_append}}"
    tmp=""
    cleanup_tmp() {
      [ -n "$tmp" ] && [ -f "$tmp" ] && rm -f "$tmp"
    }
    trap cleanup_tmp EXIT

    if [ -n "{{inject_file}}" ]; then
      if [ -n "$inject_append" ]; then
        tmp=$(mktemp)
        cat "{{inject_file}}" > "$tmp"
        printf '%s\n' "$inject_append" >> "$tmp"
        args+=(--inject-file "$tmp")
      else
        args+=(--inject-file "{{inject_file}}")
      fi
    elif [ -n "{{inject}}" ]; then
      if [ -n "$inject_append" ]; then
        args+=(--inject "{{inject}},$inject_append")
      else
        args+=(--inject "{{inject}}")
      fi
    elif [ -f "{{ssh_pubkey}}" ]; then
      tmp=$(mktemp)
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"
      [ -n "$inject_append" ] && printf '%s\n' "$inject_append" >> "$tmp"
      args+=(--inject-file "$tmp")
    elif [ -n "$inject_append" ]; then
      args+=(--inject "$inject_append")
    fi

    if [ "{{ssh}}" = "true" ] && [ -n "{{ssh_privkey}}" ]; then
      args+=(--ssh-private-key "{{ssh_privkey}}")
    fi

    "${args[@]}"

# Boot into a stage (interactive serial, Ctrl-A X to exit)
[no-exit-message]
stage n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    just _boot_stage {{n}} {{distro}} "{{inject}}" "{{inject_file}}" false false false "{{ssh_pubkey}}" "" 2222 STAGE02_SERIAL_UX=1

# Boot a live stage in background and SSH into it (no serial wrapper harness).
[no-exit-message]
stage-ssh n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey=(env("HOME") + "/.ssh/id_ed25519") ssh_port="2222":
    just _boot_stage {{n}} {{distro}} "{{inject}}" "{{inject_file}}" true false false "{{ssh_pubkey}}" "{{ssh_privkey}}" "{{ssh_port}}" ""

# Boot into a stage with a local QEMU GUI window in foreground mode (Ctrl-C to stop).
[script, no-exit-message]
stage-window n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    #!/usr/bin/env bash
    set -euo pipefail
    CARGO_BIN="${CARGO_BIN:-cargo}"
    if ! command -v "$CARGO_BIN" >/dev/null 2>&1; then
      if [ -n "${HOME:-}" ] && [ -x "${HOME}/.cargo/bin/cargo" ]; then
        CARGO_BIN="${HOME}/.cargo/bin/cargo"
      else
        echo "cargo not found in PATH for stage-window." >&2
        echo "Remediation: source \"\$HOME/.cargo/env\" (or install Rust), then rerun." >&2
        exit 127
      fi
    fi

    export LEVITATE_STAGE_WINDOW_MODE=local
    args=("$CARGO_BIN" xtask stages boot {{n}} "{{distro}}" --window)

    if [ -n "{{inject_file}}" ]; then
      args+=(--inject-file "{{inject_file}}")
    elif [ -n "{{inject}}" ]; then
      args+=(--inject "{{inject}}")
    elif [ -f "{{ssh_pubkey}}" ]; then
      tmp=$(mktemp)
      trap 'rm -f "$tmp"' EXIT
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"
      args+=(--inject-file "$tmp")
    fi

    "${args[@]}"

# Boot into a stage with a remote VNC window endpoint in foreground mode (Ctrl-C to stop).
[no-exit-message]
stage-window-remote n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    just _boot_stage {{n}} {{distro}} "{{inject}}" "{{inject_file}}" false false true "{{ssh_pubkey}}" "" 2222 ""

# Single-path Stage 01 parity gate (serial boot + SSH boot).
[script, no-exit-message]
s01-parity distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey=(env("HOME") + "/.ssh/id_ed25519") ssh_port="2222":
    #!/usr/bin/env bash
    set -euo pipefail
    just build 01Boot {{distro}}

    tmp_serial=""
    tmp_ssh=""
    cleanup() {
      [ -n "${tmp_serial}" ] && [ -f "${tmp_serial}" ] && rm -f "${tmp_serial}"
      [ -n "${tmp_ssh}" ] && [ -f "${tmp_ssh}" ] && rm -f "${tmp_ssh}"
    }
    trap cleanup EXIT INT TERM

    serial_args=()
    if [ -n "{{inject_file}}" ]; then
      serial_args=(--inject-file "{{inject_file}}")
    elif [ -n "{{inject}}" ]; then
      serial_args=(--inject "{{inject}}")
    elif [ -f "{{ssh_pubkey}}" ]; then
      tmp_serial="$(mktemp)"
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp_serial"
      serial_args=(--inject-file "$tmp_serial")
    fi

    ssh_args=(--ssh --ssh-port "{{ssh_port}}" --no-shell)
    if [ -n "{{ssh_privkey}}" ]; then
      ssh_args+=(--ssh-private-key "{{ssh_privkey}}")
    fi
    if [ -n "{{inject_file}}" ]; then
      ssh_args+=(--inject-file "{{inject_file}}")
    elif [ -n "{{inject}}" ]; then
      ssh_args+=(--inject "{{inject}}")
    elif [ -f "{{ssh_pubkey}}" ]; then
      tmp_ssh="$(mktemp)"
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp_ssh"
      ssh_args+=(--inject-file "$tmp_ssh")
    fi

    cargo xtask stages boot 1 "{{distro}}" --no-shell "${serial_args[@]}"
    cargo xtask stages boot 1 "{{distro}}" "${ssh_args[@]}"

# Run automated stage test (pass/fail)
test n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    if [ -n "{{inject_file}}" ]; then \
      cargo xtask stages test {{n}} {{distro}} --inject-file "{{inject_file}}"; \
    elif [ -n "{{inject}}" ]; then \
      cargo xtask stages test {{n}} {{distro}} --inject "{{inject}}"; \
    elif [ -f "{{ssh_pubkey}}" ]; then \
      tmp=$(mktemp); \
      trap 'rm -f "$tmp"' EXIT; \
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"; \
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"; \
      cargo xtask stages test {{n}} {{distro}} --inject-file "$tmp"; \
    else \
      cargo xtask stages test {{n}} {{distro}}; \
    fi

# Run all stage tests up to N
test-up-to n distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    if [ -n "{{inject_file}}" ]; then \
      cargo xtask stages test-up-to {{n}} {{distro}} --inject-file "{{inject_file}}"; \
    elif [ -n "{{inject}}" ]; then \
      cargo xtask stages test-up-to {{n}} {{distro}} --inject "{{inject}}"; \
    elif [ -f "{{ssh_pubkey}}" ]; then \
      tmp=$(mktemp); \
      trap 'rm -f "$tmp"' EXIT; \
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"; \
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"; \
      cargo xtask stages test-up-to {{n}} {{distro}} --inject-file "$tmp"; \
    else \
      cargo xtask stages test-up-to {{n}} {{distro}}; \
    fi

# Show stage test status
test-status distro="levitate":
    cargo xtask stages status {{distro}}

# Reset stage test state
test-reset distro="levitate":
    cargo xtask stages reset {{distro}}

# Build ISO via new distro-builder endpoint (`distro-variants` Stage flow).
# Human-friendly: use `<stage-or-distro> [<stage-or-distro>]`.
# `distro-builder` canonicalizes missing/default values and aliases.
[script, no-exit-message]
build *args:
    #!/usr/bin/env bash
    set -euo pipefail

    stage03_build() {
      local distro="$1"
      local ssh_pubkey="${HOME}/.ssh/id_ed25519.pub"

      if [ -f "$ssh_pubkey" ]; then
        local tmp
        tmp="$(mktemp)"
        trap "rm -f '$tmp'" EXIT
        local key
        key="$(tr -d '\n' < "$ssh_pubkey")"
        printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"
        cargo xtask stages test 3 "$distro" --force --inject-file "$tmp"
      else
        cargo xtask stages test 3 "$distro" --force
      fi
    }

    is_stage03() {
      case "$1" in
        3|03|03Install) return 0 ;;
        *) return 1 ;;
      esac
    }

    set -- {{args}}

    if [ "$#" -eq 1 ] && is_stage03 "$1"; then
      stage03_build levitate
      exit 0
    fi

    if [ "$#" -eq 2 ]; then
      if is_stage03 "$1"; then
        stage03_build "$2"
        exit 0
      fi
      if is_stage03 "$2"; then
        stage03_build "$1"
        exit 0
      fi
    fi

    cargo run -p distro-builder --bin distro-builder -- iso build "$@"

# Build stage ISOs from 00 up to N (inclusive) for a distro.
# Usage: just build-up-to 3 levitate
[script, no-exit-message]
build-up-to n distro="levitate":
    #!/usr/bin/env bash
    set -euo pipefail

    case "{{n}}" in
      0|00) target=0 ;;
      1|01) target=1 ;;
      2|02) target=2 ;;
      3|03) target=3 ;;
      *)
        echo "build-up-to supports stages 0..3 (got: {{n}})" >&2
        exit 2
        ;;
    esac

    stages=(00Build 01Boot 02LiveTools 03Install)
    for i in $(seq 0 "$target"); do
      stage="${stages[$i]}"
      echo "==> Building ${stage} for {{distro}}"
      just build "{{distro}}" "${stage}"
    done

# Build ISOs for all variants via new endpoint
build-all *args:
    cargo run -p distro-builder --bin distro-builder -- iso build-all {{args}}

# Prepare stage inputs and build both rootfs/overlay EROFS artifacts.
# Example: just stage-erofs 02LiveTools levitate
stage-erofs stage="02LiveTools" distro="levitate":
    cargo run -p distro-builder --bin distro-builder -- artifact build-stage-erofs {{stage}} {{distro}}

# Remove stage artifacts output tree (all stage run directories and manifests).
clean-out:
    rm -rf .artifacts/out

# Docs content (shared by website + tui)
docs-content-build:
    cd docs/content && bun run build

docs-content-check:
    cd docs/content && bun run check

# Docs TUI (installation-focused)
docs-tui-check:
    cd tui/apps/s02-live-tools/install-docs && bun run typecheck && bun run test

docs-tui-inspect-check:
    cd tui/apps/s02-live-tools/install-docs && bun run inspect:check

[script, no-exit-message]
docs-tui *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui command is now routed to tui/apps/s02-live-tools/install-docs." >&2

    if [ ! -t 0 ] || [ ! -t 1 ]; then
      if [ -r /dev/tty ] && [ -w /dev/tty ]; then
        exec </dev/tty >/dev/tty 2>&1
      else
        echo "docs-tui requires interactive TTY stdin/stdout. Run from a terminal." >&2
        exit 2
      fi
    fi

    cd tui/apps/s02-live-tools/install-docs
    if [ ! -f node_modules/@levitate/tui-kit/package.json ] || \
       [ ../../../kit/core/package.json -nt node_modules/@levitate/tui-kit/package.json ] || \
       [ "$(find ../../../kit/core/src -type f -newer node_modules/@levitate/tui-kit/package.json | head -n 1)" != "" ]; then
      bun install
    fi
    unset NO_COLOR
    export FORCE_COLOR="${FORCE_COLOR:-3}"
    exec bun src/main.ts {{args}}

[script, no-exit-message]
docs-tui-inspect *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui-inspect command is now routed to tui/apps/s02-live-tools/install-docs." >&2

    cd tui/apps/s02-live-tools/install-docs
    if [ ! -f node_modules/@levitate/tui-kit/package.json ] || \
       [ ../../../kit/core/package.json -nt node_modules/@levitate/tui-kit/package.json ] || \
       [ "$(find ../../../kit/core/src -type f -newer node_modules/@levitate/tui-kit/package.json | head -n 1)" != "" ]; then
      bun install
    fi
    unset NO_COLOR
    export FORCE_COLOR="${FORCE_COLOR:-3}"
    bun run typecheck
    bun run test
    exec bun src/main.ts {{args}}

[script, no-exit-message]
docs-tui-refresh *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui-refresh command is now routed to tui/apps/s02-live-tools/install-docs." >&2

    cd tui/apps/s02-live-tools/install-docs
    bun install
    exec bun src/main.ts {{args}}

[script, no-exit-message]
docs-tui-split *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui-split command is now routed to tui/apps/s02-live-tools/install-docs." >&2

    cd tui/apps/s02-live-tools/install-docs
    exec bash bin/levitate-install-docs-split {{args}}

# recpart TUI
[script, no-exit-message]
tui-s03-disk-plan *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: running Stage 03 disk-plan TUI (tui/apps/s03-install/disk-plan)." >&2

    if [ ! -t 0 ] || [ ! -t 1 ]; then
      echo "tui-s03-disk-plan requires interactive TTY stdin/stdout. Run from a terminal." >&2
      exit 2
    fi

    cd tui/apps/s03-install/disk-plan
    exec bun run start -- {{args}}

# Website (Astro)
website-dev:
    cd docs/website && bun run dev

website-build:
    cd docs/website && bun run build

website-typecheck:
    cd docs/website && bun run typecheck

# Launch Codex CLI from the workspace root.
[script, no-exit-message]
codex *args:
    #!/usr/bin/env bash
    set -euo pipefail

    exec codex {{args}}
