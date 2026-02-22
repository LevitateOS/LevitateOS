import { TuiKitError } from "../../core/errors";

export type BlessedFactory = (options: Record<string, unknown>) => unknown;

export type BlessedModule = {
  screen: BlessedFactory;
  box: BlessedFactory;
  list: BlessedFactory;
  textbox: BlessedFactory;
};

const REQUIRED_EXPORTS: ReadonlyArray<keyof BlessedModule> = ["screen", "box", "list", "textbox"];

let cachedBlessed: BlessedModule | null = null;
const blessedModuleId = "blessed";
const blessedNamespace = await import(blessedModuleId);

function resolveBlessedModule(): unknown {
  const maybeDefault = (blessedNamespace as { default?: unknown }).default;
  return maybeDefault ?? (blessedNamespace as unknown);
}

export function assertBlessedModule(value: unknown): asserts value is BlessedModule {
  if (!value || (typeof value !== "object" && typeof value !== "function")) {
    throw new TuiKitError("INTERNAL", "Blessed module did not load as an object.", {
      component: "blessed.runtime",
      observed: typeof value,
    });
  }

  for (const key of REQUIRED_EXPORTS) {
    const factory = (value as Record<string, unknown>)[key];
    if (typeof factory !== "function") {
      throw new TuiKitError("INTERNAL", `Blessed module is missing required factory '${key}'.`, {
        component: "blessed.runtime",
        missing_export: key,
      });
    }
  }
}

export function loadBlessed(): BlessedModule {
  if (cachedBlessed) {
    return cachedBlessed;
  }

  try {
    const maybeBlessed = resolveBlessedModule();
    assertBlessedModule(maybeBlessed);
    cachedBlessed = maybeBlessed;
    return maybeBlessed;
  } catch (error: unknown) {
    if (error instanceof TuiKitError) {
      throw error;
    }

    const message = error instanceof Error ? error.message : String(error);
    throw new TuiKitError("INTERNAL", "Blessed runtime is unavailable.", {
      component: "blessed.runtime",
      observed: message,
      remediation: "Install blessed in the runtime package.",
    });
  }
}

export function setBlessedForTesting(mock: BlessedModule | null): void {
  if (mock === null) {
    cachedBlessed = null;
    return;
  }

  assertBlessedModule(mock);
  cachedBlessed = mock;
}
