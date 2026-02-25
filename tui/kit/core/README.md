# @levitate/tui-kit

Reusable React + Ink primitives for LevitateOS terminal applications.

## API Model

`tui-kit` is app/runtime/layout first:

- `app/*`: create and mount TUI apps
- `runtime/*`: input, focus, terminal capability helpers
- `primitives/*`: low-level layout/display atoms
- `components/*`: reusable form/navigation/feedback/data widgets
- `patterns/*`: higher-level screen composition patterns
- `theme/*`: tokens, color runtime, spacing, typography

Docs-domain rendering/navigation no longer lives in `tui-kit`; it belongs in the consuming app.

## Exports

- `@levitate/tui-kit` - stable top-level API (app, hooks, primitives, components, patterns, theme)
- `@levitate/tui-kit/theme` - theme-focused API

## Quick Start

```ts
import { createTuiApp, renderApp, TwoPane } from "@levitate/tui-kit";

const app = createTuiApp({ title: "demo" });

const mounted = renderApp(
  <TwoPane title="Demo" sidebar="menu" footer="q quit">
    Hello from tui-kit
  </TwoPane>,
  { app },
);

await mounted.waitUntilExit();
```

## Scripts

- `bun run clean`
- `bun run build`
- `bun run typecheck`
- `bun run test`
- `bun run lint`
- `bun run format:check`
- `bun run precommit`
