# recguest Implementation Plan

Last updated: 2026-11-15

Status: **planned, not started.** The crate scaffold exists (`tools/recguest`, repo `LevitateOS/recguest`, commit `3c606de`) with only `--help`/`--version`. This document is the task list for implementing the agent. Work task by task, in order, and **commit after each task** — in the recguest repo for crate work, and submodule-first for anything that lands in the superproject (commit inside the submodule, push, then commit the submodule pointer in the superproject).

Design bounds (from `docs/rec-runtime-ports.md`, do not widen without a new decision):

- Speak the QEMU guest-agent JSON protocol over `/dev/virtio-ports/org.qemu.guest_agent.0`.
- Version 1 commands only: `guest-ping`, `guest-sync`, `guest-sync-delimited`, `guest-info`, `guest-shutdown`.
- No `guest-exec`, no filesystem freeze/thaw, no network/disk info commands.
- No virtio port present = clean exit 0, no error.
- Protocol reference: QEMU Guest Agent Protocol Reference (`guest-sync-delimited` semantics: leading `0xFF` sentinel in the reply, clients may prepend `0xFF` to flush stale input; responses are QMP-style `{"return": …}` / `{"error": {...}}`).

## Task 1 — Port transport and framing

**Do:**

- Add `serde`/`serde_json` (workspace versions) to `tools/recguest/Cargo.toml`.
- Implement port opening: default path `/dev/virtio-ports/org.qemu.guest_agent.0`, `--port <path>` override (needed for host-side testing), `--version`/`--help` kept.
- Missing/unopenable default port: print one line to stderr and exit 0. An explicitly passed `--port` that cannot be opened is an error exit.
- Blocking read loop over the port: buffer bytes, split requests on newlines, tolerate a leading `0xFF` byte before a request by discarding buffered partial input (protocol flush).
- Request/response types: parse `{"execute": string, "arguments": object?}`; serialize `{"return": value}` on success and `{"error": {"class": string, "desc": string}}` on failure. Malformed JSON gets an error reply, never a panic, and the loop continues.

**Done when:** the binary compiles warning-free, starts against a nonexistent default port and exits 0, and a manual echo test (Task 3 harness or `socat`-style PTY) shows one request line producing one response line.

**Commit:** `recguest: port transport and JSON framing` (crate repo).

## Task 2 — The five commands

**Do:**

- `guest-ping` → `{"return": {}}`.
- `guest-sync` → echo the integer `arguments.id` as `{"return": id}`.
- `guest-sync-delimited` → write byte `0xFF`, then `{"return": id}`.
- `guest-info` → `{"return": {"version": <crate version>, "supported_commands": [...]}}` listing exactly the implemented commands with `enabled: true`, and anything else either omitted or `enabled: false` (match upstream's shape).
- `guest-shutdown` → flush `{"return": {}}` to the port first, then power off via the `reboot` syscall (`LINUX_REBOOT_CMD_POWER_OFF`) so behavior is identical under systemd and OpenRC/busybox. If the syscall fails, log to stderr; the process then exits nonzero.
- Any other `execute` value → error reply (`CommandNotFound` class). Wrong argument types → error reply (`InvalidParameter`), never a panic.

**Done when:** each command's response matches the QEMU reference examples, and `guest-shutdown` issued against a test double (see Task 3) is observable as “response flushed before poweroff was attempted” (in tests, intercept: run with a flag/env that substitutes a marker print for the syscall — name it `--dry-run-shutdown` and document it as test-only).

**Commit:** `recguest: implement ping, sync, info and shutdown commands` (crate repo).

## Task 3 — Host-side protocol proof (no VM)

**Do:**

- Add a committed check in the repo's scenario style (the repo has no unit tests): a small script or Rust example under `tools/recguest/` that spawns recguest on a PTY/socketpair via `--port`, then asserts:
  - `guest-ping` returns `{}`;
  - `guest-sync` and `guest-sync-delimited` echo the id (and the delimited reply starts with byte `0xFF`);
  - `guest-info` lists the five commands;
  - an unknown command and a malformed JSON line each produce an `{"error": ...}` reply and the agent keeps serving;
  - `guest-shutdown --dry-run-shutdown` flushes its reply and prints the marker instead of powering off.
- Document how to run the check in `tools/recguest/README.md`.

**Done when:** the check passes from a clean checkout with `cargo build -p recguest` and shows the full transcript.

**Commit:** `recguest: host-side protocol check` (crate repo).

## Task 4 — Ship it in both distros

**Do:**

- LevitateOS: in `distro/variants/levitate/distro.toml` and the builder's tool selection, select the recguest binary where `qemu-guest-agent` is currently packaged (see `distro/variants/levitate/distro.toml:223`, `distro/builder/os-recipes/installed-system.rhai` `desktop_services()`), replacing the upstream package in the tool set while keeping the `vm-guest-agents` wrapper behavior: start only when `/dev/virtio-ports/org.qemu.guest_agent.0` exists.
- AcornOS: same replacement in `distro/variants/acorn/distro.toml:228` and the OpenRC path of the wrapper (`rc-service` start in `installed-system.rhai:514`), keeping service entry names/behavior host tooling expects.
- Rebuild the affected image parts for both distros; confirm the upstream agent binary is gone from the rootfs and recguest is present at the path the wrapper invokes.

**Done when:** both rootfs trees contain recguest (and not the upstream agent), and the wrapper starts it only when the virtio port exists.

**Commit:** inside each touched submodule first (builder/variants live in the superproject tree: commit there as `Ship recguest in LevitateOS and AcornOS`), push submodules before the superproject pointer commit, per repo convention.

## Task 5 — End-to-end proof in real guests

**Do:**

- Boot LevitateOS and AcornOS (recqemu scenarios or the manual libvirt VMs already on this machine).
- From the host, against each guest: `virsh -c qemu:///session qemu-agent-command <vm> '{"execute":"guest-ping"}'`, `guest-info`, and `guest-sync-delimited` with a random id — verify exact responses.
- Issue `guest-shutdown` last and verify the VM actually powers off.
- Confirm a non-VM boot is unaffected (recguest exited cleanly, no failed service) and that spice-vdagent (untouched) still works in a SPICE session if one is available.

**Done when:** ping/info/sync succeed on **both** distros via the real libvirt client path (not only the Task 3 harness), shutdown powers each VM off, and the results are recorded in the commit message or a short note in `tools/recguest/README.md`.

**Commit:** `recguest: record dual-distro guest proof` (crate repo) if the note lands there; any scenario/wiring fixes discovered get their own commit in the owning repo first.

## Out of scope (separate decisions, do not add)

- `guest-exec`/`guest-exec-status` (host command execution in the guest).
- `guest-fsfreeze-*`, `guest-network-get-interfaces`, disk/FS info commands.
- Replacing spice-vdagent; this plan only replaces the QEMU guest agent.
