# User-managed software recipes

This is the catalog for software users choose and manage independently of an OS
image. It currently contains no package recipes, and no catalog is shipped into
installed systems yet.

OS image producers live in [`distro/builder/os-recipes/`](../distro/builder/os-recipes/):
base-package acquisition, kernel and tool builds, system composition and live
overlays. Applications included in those images are managed with the image too.

Both collections use the same [Recipe engine](../tools/recipe/), Rhai API 2,
`NEEDS` declarations and acquire/build/install/remove hooks. Put future public
package recipes here as `<name>.rhai`, following the existing
[authoring contract](../tools/recipe/WRITING_RECIPES.md). Engine examples and test
fixtures remain under `tools/recipe`. A public recipe should run without required
builder-specific defines.

From the workspace root, `recipe list --recipes-path ./recipes` selects this
catalog; it currently lists no recipes. The OS builder selects its own collection
explicitly. The CLI's default lookup is unchanged: `$RECIPE_PATH`, otherwise
`$XDG_DATA_HOME/recipe/recipes` (normally `~/.local/share/recipe/recipes`). On the
installed system, root's lookup points to `/var/lib/recipe/recipes`.

## Current lifecycle boundary

Recipe runs in Ginux and publishes named files/directories. `install` populates
an owned output tree; `remove` changes that tree and preserves previous published
generations. These operations do not change the running system. Recipe still
needs catalog delivery and user-software activation, update and removal. The
installed system also needs the Ginux launcher and root image to execute recipes.

Root's recipe catalog, retained state and image cache live on shared `/var` so
they can survive A/B system updates. This is storage preparation, not an active
application installation. `recab` owns OS image updates; the independent software
lifecycle belongs to Recipe. See [the format and ownership guide](../docs/package-recipe-format.md).
