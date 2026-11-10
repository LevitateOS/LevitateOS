# Package recipe format

Recipes are Rhai scripts executed inside a configured Ginux domain. Every
executable recipe declares `const RECIPE_API = 2;` and defines its package facts,
custom readiness and acquire/build/install behavior.

## Two consumers, one format

| Collection | Consumer | Output lifecycle |
| --- | --- | --- |
| [`distro/builder/os-recipes/`](../distro/builder/os-recipes/) | OS builder | Produces system trees, kernels, tools and live overlays. The builder assembles images; `recab` owns system updates and A/B continuation, coordinating Recipe software operations. |
| [`recipes/`](../recipes/) (its own repository, the official [source](recipe-sources.md)) | User-managed software | Publishes a `package` directory for atomic profile activation. Recipe owns install, update and removal; ripgrep is the first package. |

Both use the same engine in `tools/recipe`, the same API 2 declarations and the
same Ginux execution rules. The directories express ownership and discovery;
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

The engine's `install` hook populates an owned output tree. A directory output
named `"package"` tells the CLI to activate its regular `bin/COMMAND` and
`share/PATH` files, unless `--no-activate` is set. The package needs `ctx.name` and
`ctx.version`. This uses the existing typed output contract, not a second format.

`update` installs the selected recipe's pinned version before switching the
profile; it does not fetch unpinned upstream releases. `remove` deactivates an
installed package from its receipt without evaluating the recipe. Old inactive
generations are retained. For output-only producers, `remove --no-activate`
still runs the recipe hook on its isolated tree. `list` lists available recipes;
`installed` lists active packages. See the [catalog guide](../recipes/README.md).

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
- Recipe hooks change only their owned target; imported inputs and earlier
  publications remain immutable. The CLI activates verified package publications
  separately, without running host installation hooks.

The [coordinator/launcher interface](ginux-recipe-interfaces.md) describes the
execution boundary. Exact demonstrated workloads and remaining production/VM
limitations are recorded in the [implementation checkpoint](recipe-simplification-implementation-plan.md).
