# @levitate/tui-kit

Reusable terminal UI primitives for LevitateOS.

This package is intentionally a single flattened package. It does not expose nested sub-packages or a nested monorepo layout.

## Exports

- `@levitate/tui-kit` - top-level re-export surface
- `@levitate/tui-kit/core` - screen handles, terminal sizing, layout math, errors, key events, lifecycle scopes
- `@levitate/tui-kit/widgets` - interactive primitives (prompt, confirm, list, progress, scrollable)
- `@levitate/tui-kit/theme` - palette/tokens/layout helpers and text styling utilities
- `@levitate/tui-kit/wizard` - step flow and sidebar rendering helpers
- `@levitate/tui-kit/docs` - docs nav flattening and docs page/sidebar rendering helpers

## Colors and Themes

Colors are first-class and explicit.

- Build themes with `createTheme(...)`.
- Pass `theme` to `createScreen(...)` (no hidden singleton-based styling path).
- Runtime color capability is auto-detected (`truecolor` / `ansi256` / `ansi16` / `mono`) and can be overridden with `colorMode` / `colorEnabled`.
- Use `styleText(...)` with `[[...]]` directives for styled content (`fg`, `bg`, `bold`, `underline`, `inverse`).

## Runtime Requirements

- Bun or Node runtime with terminal support
- `blessed` available at runtime (`dependencies.blessed`)

## Scripts

- `bun run clean` - remove `dist`
- `bun run build` - clean and compile TypeScript declarations/output
- `bun run typecheck` - strict no-emit typecheck for source files
- `bun run test` - run focused unit tests in `tests/*.test.ts`
- `bun run test:watch` - run tests in watch mode
- `bun run check` - run typecheck and tests
- `bun run lint` - run `oxlint` for `src`, `tests`, and `examples`
- `bun run lint:fix` - auto-fix lint issues where possible
- `bun run format` - format package TypeScript/JSON with `oxfmt`
- `bun run format:check` - verify formatting without writing changes
- `bun run precommit` - run formatter check, lint, typecheck, and tests

## Development

- Keep this package flat: one package, no nested `packages/*` inside `shared/tui-kit`.
- Keep adapters narrow: blessed bindings in `src/adapters/blessed/*`, behavior in core/widgets/wizard/docs/theme modules.
- Add tests for edge cases when changing behavior. Prefer deterministic unit tests over integration-heavy test setup.
- Use `scripts/pre-commit.sh` if you want to run package-local pre-commit checks directly.

## Lifecycle API

Use `createLifecycleScope()` from `@levitate/tui-kit/core` for deterministic cleanup in interactive flows.
- `scope.onDispose(fn)` for teardown handlers
- `scope.bindEvent(node, event, handler)` and `scope.bindKey(node, keys, handler)` for auto-unbind
- `scope.timeout(ms, fn)` and `scope.interval(ms, fn)` for auto-cleared timers
- `scope.child(name?)` for nested scopes
- `scope.dispose()` to teardown idempotently
