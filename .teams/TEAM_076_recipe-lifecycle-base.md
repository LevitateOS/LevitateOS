# TEAM_076: Recipe Lifecycle for Base Binaries

## Mission
Implement recipe lifecycle for base system packages, starting with `less` as the prototype.

## Status: Complete

## Approach
Focus on ONE package (`less`) to understand the full lifecycle before scaling to ~209 packages.

### Key Files
- `leviso/src/rootfs/parts/recipe_gen.rs` - NEW: RPM metadata extraction + .rhai generation
- `leviso/src/rootfs/parts/mod.rs` - Export recipe_gen module
- `leviso/src/rootfs/builder.rs` - Call recipe generator after copying binaries
- `recipe/SYSTEM_VS_USER_RECIPES.md` - NEW: Document system vs user recipe concepts

### Philosophy
**Recipe is NOT an RPM shim** - except for base system packages where it makes sense:
- Rocky = slow-moving enterprise distro
- For BASE packages: proper RPM shim recipes that download from Rocky mirrors
- For USER packages: real upstream recipes

### Directory Structure on Installed System
```
/etc/recipe/repos/rocky10/     # Base system (RPM shims, generated at build time)
~/.local/share/recipe/recipes/ # USER recipes (per-user)
```

## Progress
- [x] Create team file
- [x] Create recipe_gen.rs module (RPM metadata extraction + .rhai generation)
- [x] Modify parts/mod.rs to export recipe_gen
- [x] Modify builder.rs to generate recipes after build_rootfs()
- [x] Modify recipe.rs to create repos/rocky10 directory and profile.d script
- [x] Create SYSTEM_VS_USER_RECIPES.md documentation
- [x] Test with `less` package - verified 100 recipes generated

## Log
- Started implementation of recipe lifecycle for base binaries
- Created recipe_gen.rs with RpmInfo struct, extract_rpm_info(), and generate_recipe()
- Added RecipeGenerator struct to generate recipes for all REQUIRED_PACKAGES
- Fixed dependency filtering to exclude rtld(), config(), rpmlib(), .so requirements
- Verified less.rhai and bash.rhai generated correctly with proper metadata
- All 100 base packages now have .rhai recipes in /etc/recipe/repos/rocky10/
