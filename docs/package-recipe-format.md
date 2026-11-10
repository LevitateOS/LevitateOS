# Package recipe format

Recipes are Rhai scripts executed inside a configured Ginux domain. Every
executable recipe declares `const RECIPE_API = 2;` and defines its package facts,
custom readiness and acquire/build/install behavior.

## Two consumers, one format

| Collection | Consumer | Output lifecycle |
| --- | --- | --- |
| [`distro/builder/os-recipes/`](../distro/builder/os-recipes/) | OS builder | Produces system trees, kernels, tools and live overlays. The builder assembles images; `recab` stages and activates system image updates. |
| [`recipes/`](../recipes/) | User-managed software | The application catalog, currently empty. Making published software available on a running system and managing its updates/removal remain to be implemented in Recipe. |

Both use the same engine in `tools/recipe`, the same API 2 declarations and the
same Ginux execution rules. The directories express ownership and discovery;
they introduce no recipe type, alternate interpreter or additional permissions.
The builder passes its OS collection as `--recipes-path`; the CLI otherwise uses
`$RECIPE_PATH` or the user's XDG recipe directory. A path can select either kind
of producer explicitly. Engine examples and test fixtures stay with the engine.

An app included in the desktop image belongs to the OS lifecycle. Being a
graphical app does not automatically make it independently user-managed.
Likewise, `deps` and `build_deps` describe dependency phases, not OS/user ownership.

Today, `install` means installing into Recipe's owned output tree and publishing
its named products. `remove` changes that tree, leaving previous publications
intact. Neither command activates or removes software on the running system.
`list` lists recipe declarations, not an inventory of installed applications.

## Shared authoring contract

The current authoring contract and runnable example live in
[WRITING_RECIPES.md](../tools/recipe/WRITING_RECIPES.md). Use the
[Recipe README](../tools/recipe/README.md) for environment preparation, CLI
options, diagnostics and recovery, and the
[helper reference](../tools/recipe/HELPERS_AUDIT.md) for registered signatures.

- Setup reads immutable `RUNTIME`; hooks receive the same `ctx.runtime`.
- Checks return `ready(ctx)` or `needs_work(ctx, reason)`. Errors fail execution.
  Phase hooks return updated context, and custom readiness checks the real files.
- Only `ctx.state` is restored, paired with its durable filesystem checkpoint.
  Recipe source files stay unchanged; current package facts are initialized again.
- Callers import explicit immutable inputs at `/inputs/NAME`. Owned build work is
  `/work/build`, shared dependency tools `/work/tools`, and selected target/output
  `/out`. A named `target` input seeds `/out` on fresh state.
- Recipes select real files/directories in `ctx.outputs`. After successful
  execution and archive validation, final JSON contains published host paths.
  Consumers use those names and kinds instead of guessing build paths. Published
  products retain coordinator ownership/private export permissions; privileged
  producers need matching consumer privileges or explicit product installation.
- Payload downloads require SHA256; Git acquisition requires a full commit and
  exact destination. Recipes declare endpoint permissions and resource bounds in
  `NEEDS`; callers may narrow them.
- Removal changes the owned target; imported inputs and earlier publications
  remain immutable. Current scope does not include live-system activation.

The [coordinator/launcher interface](ginux-recipe-interfaces.md) describes the
execution boundary. Exact demonstrated workloads and remaining production/VM
limitations are recorded in the [implementation checkpoint](recipe-simplification-implementation-plan.md).
