#!/bin/sh
#
# Install the shared pre-commit hook into all Rust submodules.
#
# Usage:
#   tools/install-hooks.sh          # Install to all Rust submodules + parent repo
#   tools/install-hooks.sh --remove # Remove hooks from all repos
#
# The hook is symlinked (not copied), so updating tools/pre-commit-hook.sh
# updates all submodules automatically.
#

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOK_SOURCE="$SCRIPT_DIR/pre-commit-hook.sh"

# All Rust submodules (skip: linux, docs/*, llm-toolkit)
RUST_SUBMODULES="
AcornOS
IuppiterOS
distro-builder
distro-spec
leviso
leviso-elf
testing/cheat-guard
testing/cheat-test
testing/fsdbg
testing/hardware-compat
testing/install-tests
testing/rootfs-tests
tools/recchroot
tools/recfstab
tools/recinit
tools/recipe
tools/reciso
tools/recqemu
tools/recstrap
tools/recuki
"

# Colors
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[0;33m'
    BOLD='\033[1m'
    RESET='\033[0m'
else
    GREEN='' RED='' YELLOW='' BOLD='' RESET=''
fi

resolve_hooks_dir() {
    submodule_path="$1"
    full_path="$REPO_ROOT/$submodule_path"

    if [ ! -f "$full_path/.git" ]; then
        echo ""
        return
    fi

    # Read gitdir from the .git file
    gitdir=$(sed 's/gitdir: //' "$full_path/.git")

    # Resolve relative path
    hooks_dir="$full_path/$gitdir/hooks"

    if [ -d "$hooks_dir" ]; then
        echo "$hooks_dir"
    else
        echo ""
    fi
}

install_hook() {
    hooks_dir="$1"
    label="$2"
    target="$hooks_dir/pre-commit"

    if [ -L "$target" ] && [ "$(readlink -f "$target")" = "$(readlink -f "$HOOK_SOURCE")" ]; then
        printf "  ${YELLOW}skip${RESET}  %-30s (already installed)\n" "$label"
        return
    fi

    # Remove existing hook if present (backup non-symlink hooks)
    if [ -f "$target" ] && [ ! -L "$target" ]; then
        mv "$target" "${target}.backup"
        printf "  ${YELLOW}back${RESET}  %-30s (existing hook backed up to pre-commit.backup)\n" "$label"
    fi

    ln -sf "$HOOK_SOURCE" "$target"
    printf "  ${GREEN}done${RESET}  %-30s\n" "$label"
}

remove_hook() {
    hooks_dir="$1"
    label="$2"
    target="$hooks_dir/pre-commit"

    if [ -L "$target" ] && [ "$(readlink -f "$target")" = "$(readlink -f "$HOOK_SOURCE")" ]; then
        rm "$target"
        # Restore backup if exists
        if [ -f "${target}.backup" ]; then
            mv "${target}.backup" "$target"
            printf "  ${YELLOW}rest${RESET}  %-30s (restored backup)\n" "$label"
        else
            printf "  ${GREEN}done${RESET}  %-30s\n" "$label"
        fi
    else
        printf "  ${YELLOW}skip${RESET}  %-30s (not our hook)\n" "$label"
    fi
}

# ── Main ───────────────────────────────────────────────────────────

if [ "$1" = "--remove" ]; then
    echo "Removing pre-commit hooks..."
    echo ""

    # Parent repo
    remove_hook "$REPO_ROOT/.git/hooks" "(parent repo)"

    # Submodules
    for sub in $RUST_SUBMODULES; do
        hooks_dir=$(resolve_hooks_dir "$sub")
        if [ -n "$hooks_dir" ]; then
            remove_hook "$hooks_dir" "$sub"
        fi
    done

    echo ""
    echo "Done."
    exit 0
fi

# Install mode
echo "Installing pre-commit hooks..."
echo ""

# Ensure hook is executable
chmod +x "$HOOK_SOURCE"

installed=0
skipped=0

# Parent repo
parent_hooks="$REPO_ROOT/.git/hooks"
if [ -d "$parent_hooks" ]; then
    install_hook "$parent_hooks" "(parent repo)"
    installed=$((installed + 1))
fi

# Submodules
for sub in $RUST_SUBMODULES; do
    hooks_dir=$(resolve_hooks_dir "$sub")
    if [ -n "$hooks_dir" ]; then
        install_hook "$hooks_dir" "$sub"
        installed=$((installed + 1))
    else
        printf "  ${RED}miss${RESET}  %-30s (hooks dir not found)\n" "$sub"
        skipped=$((skipped + 1))
    fi
done

echo ""
printf "${BOLD}Installed: %d  Skipped: %d${RESET}\n" "$installed" "$skipped"
echo ""
echo "Hook runs: cargo fmt (auto-fix) + clippy + unit tests"
echo "Skip with: git commit --no-verify"
