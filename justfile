# LevitateOS development commands

# QEMU tools environment
# The tooling cache lives under .artifacts/tools/current.
tools_prefix := join(justfile_directory(), ".artifacts/tools/current")
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

# Install canonical QEMU/OVMF helper tooling into .artifacts/tools/current.
tools-install:
    cargo run -p distro-builder --bin distro-builder -- artifact prepare-qemu-tools

# Short alias for tools-install.
tools:
    just tools-install

# Install/remove the optional whitespace-only pre-commit hook.
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

# Canonical live-source preseed helpers (distro-builder artifact commands).
[script, no-exit-message]
preseed distro refresh="true":
    #!/usr/bin/env bash
    set -euo pipefail

    case "{{distro}}" in
      levitate|leviso)
        if [ "{{refresh}}" = "true" ]; then
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rootfs-source levitate --refresh
        else
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rootfs-source levitate
        fi
        ;;
      acorn|acornos)
        if [ "{{refresh}}" = "true" ]; then
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rootfs-source acorn --refresh
        else
          cargo run -p distro-builder --bin distro-builder -- artifact preseed-rootfs-source acorn
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

# Internal delegate for scenario booting.
# Keep `cargo xtask scenarios boot` as the canonical execution path.
# Boundary rule: boot wrappers consume existing artifacts only.
# Do not add implicit ISO build steps here; freshness is explicit via `just release-build*` or compatibility `just build*`.
[script, no-exit-message]
_boot_scenario target distro="levitate" inject="" inject_file="" ssh="false" no_shell="false" window="false" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey="" ssh_port="2222" inject_append="":
    #!/usr/bin/env bash
    set -euo pipefail
    CARGO_BIN="${CARGO_BIN:-cargo}"
    if ! command -v "$CARGO_BIN" >/dev/null 2>&1; then
      if [ -n "${HOME:-}" ] && [ -x "${HOME}/.cargo/bin/cargo" ]; then
        CARGO_BIN="${HOME}/.cargo/bin/cargo"
      else
        echo "cargo not found in PATH for _boot_scenario." >&2
        echo "Remediation: source \"\$HOME/.cargo/env\" (or install Rust), then rerun." >&2
        exit 127
      fi
    fi
    case "{{target}}" in
      live-boot|live_boot|live-tools|live_tools|installed-boot|installed_boot) ;;
      *)
        echo "scenario boot expects an interactive scenario target: live-boot, live-tools, or installed-boot (got: {{target}})" >&2
        exit 2
        ;;
    esac

    if [ "{{ssh}}" = "true" ]; then
      case "{{target}}" in
        live-boot|live_boot|live-tools|live_tools) ;;
        *)
          echo "SSH boot mode supports only live ISO scenario targets: live-boot or live-tools (got: {{target}})" >&2
          exit 2
          ;;
      esac
    fi

    args=("$CARGO_BIN" xtask scenarios boot {{target}} "{{distro}}")

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
    append_inject_lines() {
      local payload="$1"
      [ -z "$payload" ] && return 0
      local old_ifs="$IFS"
      IFS=','
      for item in $payload; do
        [ -n "$item" ] && printf '%s\n' "$item"
      done
      IFS="$old_ifs"
    }
    tmp=""
    cleanup_tmp() {
      [ -n "$tmp" ] && [ -f "$tmp" ] && rm -f "$tmp"
    }
    trap cleanup_tmp EXIT

    # Boot injection is read by the live init only: installed-boot gets no
    # default injection, and xtask rejects an explicit one.
    case "{{target}}" in
      installed-boot|installed_boot) live=false ;;
      *) live=true ;;
    esac
    if [ -n "{{inject_file}}" ]; then
      if [ -n "$inject_append" ]; then
        tmp=$(mktemp)
        cat "{{inject_file}}" > "$tmp"
        append_inject_lines "$inject_append" >> "$tmp"
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
    elif [ "$live" = true ] && [ -f "{{ssh_pubkey}}" ]; then
      tmp=$(mktemp)
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"
      [ -n "$inject_append" ] && append_inject_lines "$inject_append" >> "$tmp"
      args+=(--inject-file "$tmp")
    elif [ "$live" = true ] && [ -n "$inject_append" ]; then
      args+=(--inject "$inject_append")
    fi

    if [ "{{ssh}}" = "true" ] && [ -n "{{ssh_privkey}}" ]; then
      args+=(--ssh-private-key "{{ssh_privkey}}")
    fi

    "${args[@]}"

# Boot a scenario by canonical scenario name (interactive serial, Ctrl-A X to exit).
[no-exit-message]
scenario target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    just _boot_scenario {{target}} {{distro}} "{{inject}}" "{{inject_file}}" false false false "{{ssh_pubkey}}" "" 2222 LEVITATE_INSTALL_SERIAL_UX=1

# Boot a scenario by canonical scenario name with verbose serial logging.
[no-exit-message]
scenario-verbose target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    just _boot_scenario {{target}} {{distro}} "{{inject}}" "{{inject_file}}" false false false "{{ssh_pubkey}}" "" 2222 LEVITATE_INSTALL_SERIAL_UX=1,LEVITATE_INSTALL_SERIAL_VERBOSE=1

# Boot a live ISO scenario in background and SSH into it.
[no-exit-message]
scenario-ssh target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey=(env("HOME") + "/.ssh/id_ed25519") ssh_port="2222":
    just _boot_scenario {{target}} {{distro}} "{{inject}}" "{{inject_file}}" true false false "{{ssh_pubkey}}" "{{ssh_privkey}}" "{{ssh_port}}" ""

# Boot a scenario with a local QEMU GUI window in foreground mode (Ctrl-C to stop).
[no-exit-message]
scenario-window target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    LEVITATE_SCENARIO_WINDOW_MODE=local just _boot_scenario {{target}} {{distro}} "{{inject}}" "{{inject_file}}" false false true "{{ssh_pubkey}}" "" 2222 ""

# Boot a scenario with a remote VNC window endpoint in foreground mode (Ctrl-C to stop).
[no-exit-message]
scenario-window-remote target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    just _boot_scenario {{target}} {{distro}} "{{inject}}" "{{inject_file}}" false false true "{{ssh_pubkey}}" "" 2222 ""

# Single-path live-boot parity gate (serial boot + SSH boot).
[script, no-exit-message]
live-boot-parity distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub") ssh_privkey=(env("HOME") + "/.ssh/id_ed25519") ssh_port="2222":
    #!/usr/bin/env bash
    set -euo pipefail

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

    cargo xtask scenarios boot live-boot "{{distro}}" --no-shell "${serial_args[@]}"
    cargo xtask scenarios boot live-boot "{{distro}}" "${ssh_args[@]}"

# Run one automated scenario by canonical scenario name.
scenario-test target distro="levitate" inject="" inject_file="" ssh_pubkey=(env("HOME") + "/.ssh/id_ed25519.pub"):
    if [ -n "{{inject_file}}" ]; then \
      cargo xtask scenarios test {{target}} {{distro}} --inject-file "{{inject_file}}"; \
    elif [ -n "{{inject}}" ]; then \
      cargo xtask scenarios test {{target}} {{distro}} --inject "{{inject}}"; \
    elif [ -f "{{ssh_pubkey}}" ]; then \
      tmp=$(mktemp); \
      trap 'rm -f "$tmp"' EXIT; \
      key="$(tr -d '\n' < "{{ssh_pubkey}}")"; \
      printf 'SSH_AUTHORIZED_KEY=%s\n' "$key" > "$tmp"; \
      cargo xtask scenarios test {{target}} {{distro}} --inject-file "$tmp"; \
    else \
      cargo xtask scenarios test {{target}} {{distro}}; \
    fi

# Show scenario test status.
scenario-status distro="levitate":
    cargo xtask scenarios status {{distro}}

# Reset scenario test state.
scenario-reset distro="levitate":
    cargo xtask scenarios reset {{distro}}

# Build release ISOs via the canonical product-first distro-builder endpoint.
# Human-friendly: use `<product-or-distro> [<product-or-distro>]`.
[no-exit-message]
release-build *args:
    cargo run -p distro-builder --bin distro-builder -- release build iso {{args}}

# Build release ISOs for all variants via the canonical endpoint.
release-build-all *args:
    cargo run -p distro-builder --bin distro-builder -- release build-all iso {{args}}

# Prepare a canonical product and build both rootfs/overlay EROFS artifacts.
# Example: just product-erofs live-tools levitate
[script]
product-erofs product="live-tools" distro="levitate":
    #!/usr/bin/env bash
    set -euo pipefail
    prepared_dir=".artifacts/prepared/{{distro}}/{{product}}"
    rm -rf "${prepared_dir}"
    mkdir -p "${prepared_dir}"
    cargo run -p distro-builder --bin distro-builder -- product prepare {{product}} {{distro}} "${prepared_dir}"
    cargo run -p distro-builder --bin distro-builder -- artifact build product-erofs "${prepared_dir}"

# Remove `.artifacts/out` (release products, scenario runtimes, and compatibility leftovers).
clean-out:
    rm -rf .artifacts/out

# Docs content (shared by website + tui)
docs-content-build:
    cd docs/content && bun run build

docs-content-check:
    cd docs/content && bun run check

# Docs TUI (installation-focused)
docs-tui-check:
    cd docs/content && bun run build
    cd tui/apps/live-tools/install-docs && bun run typecheck

docs-tui-inspect-check:
    cd docs/content && bun run build
    cd tui/apps/live-tools/install-docs && bun run inspect:check

[script, no-exit-message]
docs-tui *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui command is now routed to tui/apps/live-tools/install-docs." >&2

    if [ ! -t 0 ] || [ ! -t 1 ]; then
      if [ -r /dev/tty ] && [ -w /dev/tty ]; then
        exec </dev/tty >/dev/tty 2>&1
      else
        echo "docs-tui requires interactive TTY stdin/stdout. Run from a terminal." >&2
        exit 2
      fi
    fi

    cd tui/apps/live-tools/install-docs
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

    echo "NOTICE: docs-tui-inspect command is now routed to tui/apps/live-tools/install-docs." >&2

    cd tui/apps/live-tools/install-docs
    if [ ! -f node_modules/@levitate/tui-kit/package.json ] || \
       [ ../../../kit/core/package.json -nt node_modules/@levitate/tui-kit/package.json ] || \
       [ "$(find ../../../kit/core/src -type f -newer node_modules/@levitate/tui-kit/package.json | head -n 1)" != "" ]; then
      bun install
    fi
    unset NO_COLOR
    export FORCE_COLOR="${FORCE_COLOR:-3}"
    bun run typecheck
    exec bun src/main.ts {{args}}

[script, no-exit-message]
docs-tui-refresh *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui-refresh command is now routed to tui/apps/live-tools/install-docs." >&2

    cd tui/apps/live-tools/install-docs
    bun install
    exec bun src/main.ts {{args}}

[script, no-exit-message]
docs-tui-split *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: docs-tui-split command is now routed to tui/apps/live-tools/install-docs." >&2

    cd tui/apps/live-tools/install-docs
    exec bash bin/levitate-install-docs-split {{args}}

# recpart TUI
[script, no-exit-message]
tui-install-disk-plan *args:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "NOTICE: running install disk-plan TUI (tui/apps/install/disk-plan) with this checkout's recpart." >&2

    cargo build -q -p recpart
    export PATH="$PWD/target/debug:$PATH"
    cd tui/apps/install/disk-plan
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
