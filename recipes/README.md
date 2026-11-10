# User-managed software recipes

This is the catalog for software users choose and manage independently of an OS
image. [ripgrep](ripgrep.rhai) is the first package: its pinned static Linux musl
release runs on both variants. The builder ships this catalog in
`/usr/share/recipe/recipes` with the Ginux launcher and verified runtime image.

OS image producers live in [`distro/builder/os-recipes/`](../distro/builder/os-recipes/):
base-package acquisition, kernel and tool builds, system composition and live
overlays. Applications included in those images are managed with the image too.

Both collections use the same [Recipe engine](../tools/recipe/), Rhai API 2,
`NEEDS` declarations and acquire/build/install/remove hooks. Put future public
package recipes here as `<name>.rhai`, following the existing
[authoring contract](../tools/recipe/WRITING_RECIPES.md). Engine examples and test
fixtures remain under `tools/recipe`. A public recipe runs without required
builder-specific defines and publishes a directory output named `"package"`.
That tree contains regular `bin/COMMAND` and `share/PATH` files for activation.

From the workspace root, `recipe list --recipes-path ./recipes` selects this
catalog. The OS builder selects its own collection and passes `--no-activate`.
An explicit `--recipes-path` or `$RECIPE_PATH` selects only that collection.
Otherwise, the CLI searches `$XDG_DATA_HOME/recipe/recipes` (normally
`~/.local/share/recipe/recipes`) before the shipped catalog. Root's override
directory on the installed system is `/var/lib/recipe/recipes`.

## Install, update and remove

On an image built with this runtime, use the distro's administrator command
(`sudo` on LevitateOS, `doas` on AcornOS):

```sh
sudo recipe install ripgrep
rg --version
sudo recipe installed
sudo recipe update ripgrep
sudo recipe remove ripgrep
```

`install` runs the recipe in Ginux, then activates its verified package tree.
`update` uses the version pinned by the current selected recipe; it does not
discover upstream releases or rewrite pins. To upgrade independently of an OS
image, put a reviewed newer `ripgrep.rhai` in root's override directory, or select
another catalog with `--recipes-path`. A failed update leaves the active version
unchanged. `remove` uses the installed receipt, even if its recipe or Ginux is
unavailable. `installed` lists active packages; `list` lists available recipes.

The active profile is `/var/lib/recipe/profile` for root and
`$XDG_DATA_HOME/recipe/profile` for an ordinary user. The installed login profile
adds their `bin` directories to PATH. `--package-root PATH` selects an isolated
store with its own `profile/bin`; add that path explicitly when testing outside
the installed OS. Native Ginux currently needs whole-coordinator bootstrap
privileges; see [runtime setup](../tools/recipe/README.md).

Activation switches one profile symlink after the files and receipt are durable.
Packages cannot overwrite each other's exported paths. Removing a package
deactivates its files; old generations, published products and build caches are
retained. Recipe does not run installation scripts on the host. Package payload
symlinks, special files and setuid/setgid files are rejected.

Root's active software, override catalog, retained state and image cache live on
shared `/var`, independently of the OS image. `recab` owns image updates; Recipe
owns this software profile. See [the format and ownership guide](../docs/package-recipe-format.md).
