# Package recipe format

Recipes are Rhai files. Every recipe declares `const RECIPE_API = 1;` and a
literal `NEEDS`, states its package facts, sources and files as declarations,
and writes code only for what is unusual. Recipe downloads, unpacks, places
files, records what is done, installs, removes and cleans up. The full
specification is [recipe-format.md](../tools/recipe/docs/recipe-format.md).

## Two consumers, one format

| Collection | Consumer | Output lifecycle |
| --- | --- | --- |
| [`distro/builder/os-recipes/`](../distro/builder/os-recipes/) | OS builder | Produces system trees, kernels, tools and live overlays. The builder assembles images; `recab` owns system updates and A/B continuation, coordinating Recipe software operations. |
| [`recipes/`](../recipes/) (its own repository, the official [source](recipe-sources.md)) | User-managed software | Publishes a `package` directory for profile activation. Recipe owns install, update and removal; ripgrep is the first package. |

Both use the same engine in `tools/recipe`, the same API 1 declarations and the
same execution rules (Ginux for a recipe that starts programs). The directories express ownership and discovery;
they introduce no recipe type, alternate interpreter or additional permissions.
The builder passes its OS collection as `--recipes-path` with `--no-activate`.
The CLI uses an explicitly selected catalog, or the user's XDG recipe directory
followed by the sources, of which the image ships the official one under
`/usr/share/recipe/sources`. A path can select either kind
of producer explicitly. Engine examples and test fixtures stay with the engine.

An app included in the desktop image belongs to the OS lifecycle. Being a
graphical app does not automatically make it independently user-managed.
Likewise, `deps` and `build_deps` describe dependency phases, not OS/user ownership.
System changes must target recab's prepared inactive system and survive an update
applied to that same preparation. The current shared Recipe profile activates
independent software; it does not yet implement that system-targeting contract.
See [recab's lifecycle and implementation status](../tools/recab/README.md).

A directory output named `"package"` tells the CLI to activate its regular
`bin/COMMAND` and `share/PATH` files, unless `--no-activate` is set. The
package needs `PACKAGE.name` and `PACKAGE.version`. This uses the typed output
contract, not a second format.

`update` installs the version the recipe, as fetched, names; it never edits a
recipe and never installs a version the recipe does not name (`updates
--upstream` only tells that upstream has a newer one). `remove` removes an
installed package from its receipt without evaluating the recipe, and frees its
space at once: there are no earlier versions. After a successful install Recipe
deletes the downloads, unpacked sources, build material and saved state, unless
`--keep` was given. `list` lists available recipes; `installed` lists active
packages. See the [catalog guide](../recipes/README.md).

## Shared authoring contract

The authoring guide is [WRITING_RECIPES.md](../tools/recipe/WRITING_RECIPES.md).
Use the [Recipe README](../tools/recipe/README.md) for environment preparation,
CLI options, diagnostics and recovery. In short:

- `SOURCES` lists what Recipe downloads (each `url` with its SHA-256, or a `git`
  repository with a full commit); `FILES` says which files go where in the
  output; `PACKAGES` lists the distribution packages of a group, pinned in a
  `NAME.pins` file beside the recipe. `build(src, out)`, `configure(tree)`,
  `verify(out)`, `latest()` and `cleanup(reason)` are optional functions.
- Engine constants replace the old runtime context: `PREFIX`, `JOBS`,
  `HOST_LIBC`, `PACKAGE_FORMAT`, `INPUTS`, `DEFINES`, `DEPS` and `WORK`.
- Callers import explicit read-only inputs, reached through `INPUTS`. A named
  `target` input seeds the output.
- A normal recipe has one output, never named in its body. After successful
  execution and archive validation, final JSON contains published host paths.
  Consumers use those names and kinds instead of guessing build paths.
  Published products retain coordinator ownership/private export permissions;
  privileged producers need matching consumer privileges or explicit product
  installation.
- Recipe decides a program's state (`none`, `downloaded`, `built`, `installed`)
  from its own records, not by running recipe code. An interrupted build
  resumes from one checkpoint, kept with `save` and `checkpoint`.
- Payload downloads require SHA256; Git sources require a full commit. Recipes
  declare endpoint permissions and resource bounds in `NEEDS`; callers may
  narrow them.
- Imported inputs and earlier publications remain immutable. The CLI activates
  verified package publications separately, without running host installation
  hooks.
- When a step fails, Recipe reports the failure. It never asks a language model
  or starts an agent.

The [coordinator/launcher interface](ginux-recipe-interfaces.md) describes the
execution boundary. Exact demonstrated workloads and remaining production/VM
limitations are recorded in the [implementation checkpoint](recipe-simplification-implementation-plan.md).
