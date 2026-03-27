# Levitate Variant (Build-Host Model Scaffold)

This directory is the canonical build-host scaffold for all distro variants.

## Required Variant Files

- `build-host/build-host.toml`
- `build-host/kernel/kconfig`
- `build-host/recipes/kernel.rhai`
- `build-host/evidence/build-capability.sh`
- `ring0/hooks/build-release.sh`

## Build-Host Invariants Enforced

- Kernel configuration file must be declared as `kernel_kconfig_path = "kernel/kconfig"`.
- Kernel build must be orchestrated through Recipe Rhai:
  - `recipe_kernel_script = "distro-builder/recipes/linux-prebuilt.rhai"`
  - `recipe_kernel_invocation = "recipe install"`
- Kernel outputs and provenance fields are mandatory and validated.
- Modules installation path must be `/usr/lib/modules` for cross-distro consistency.

## Source Of Truth

`build-host/build-host.toml` is the authoritative build-host contract for this variant.
`distro-contract` loads and validates the canonical ring manifest family directly from `distro-variants`.
