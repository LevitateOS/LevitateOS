# Team 097: Wire distro-spec SSOT to leviso

**Team ID:** TEAM_097
**Task:** Wire `distro-spec` as the Single Source of Truth (SSOT) for `leviso` and `distro-spec` itself, replacing hardcoded values and implementing a template system for the initramfs init script.

## Objectives
1. Add shared and distro-specific constants to `distro-spec`.
2. Update `distro-spec` module structure and re-exports.
3. Wire `distro-spec` constants into `leviso` (various modules).
4. Convert `init_tiny` to a template and implement its generation in `leviso`.
5. Verify parity and ensure no hardcoded values remain.

## Progress Log
- [x] Part 1: Add Constants to distro-spec
- [x] Part 2: Wire distro-spec into leviso
- [x] Part 3: Convert init_tiny to Template
- [x] Part 4: Verification

## Handoff Notes
- `distro-spec` is now the SSOT for all ISO and build constants.
- `leviso` no longer contains hardcoded values for ISO labels, filenames, or boot parameters.
- The initramfs `init` script is now dynamically generated from `profile/init_tiny.template`.
- Functional verification (full build and run) recommended to ensure template substitution produces a working boot script.
