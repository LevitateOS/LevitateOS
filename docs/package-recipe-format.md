# Package recipe format

Recipes are Rhai scripts executed inside a configured Ginux domain. Every
executable recipe declares `const RECIPE_API = 2;` and defines its package facts,
custom readiness and acquire/build/install behavior.

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
  exact destination. Endpoint permissions and resource bounds belong to callers.
- Removal changes the owned target; imported inputs and earlier publications
  remain immutable. Current scope does not include live-system activation.

The [coordinator/launcher interface](ginux-recipe-interfaces.md) describes the
execution boundary. Exact demonstrated workloads and remaining production/VM
limitations are recorded in the [implementation checkpoint](recipe-simplification-implementation-plan.md).
