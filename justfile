# Short aliases for `cargo xtask`, the workspace's one front door.
# Everything is a `cargo xtask <command>`; `cargo xtask --help` lists them all.

default:
    @cargo xtask --help

setup:
    cargo xtask setup

doctor *args:
    cargo xtask doctor {{args}}

# e.g. `just build live-tools levitate`, `just build kernel levitate --prebuilt`
build *args:
    cargo xtask build {{args}}

# e.g. `just run live-tools levitate`, `just run live-boot levitate --ssh`
run *args:
    cargo xtask run {{args}}

# e.g. `just test all levitate`, `just test install levitate --force`
test *args:
    cargo xtask test {{args}}

status *args:
    cargo xtask status {{args}}

gen:
    cargo xtask gen

check:
    cargo xtask check

# e.g. `just tui install-docs`
tui *args:
    cargo xtask tui {{args}}

# Remove `.artifacts/out` (release products and scenario runtimes).
clean-out:
    rm -rf .artifacts/out

website-dev:
    cd docs/website && bun run dev

website-build:
    cd docs/website && bun run build
