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

function getDynamicRequire(): (moduleId: string) => unknown {
  const fromGlobal = (globalThis as { require?: unknown }).require;
  if (typeof fromGlobal === "function") {
    return fromGlobal as (moduleId: string) => unknown;
  }

  try {
    return Function("return require")() as (moduleId: string) => unknown;
  } catch {
    throw new TuiKitError(
      "INTERNAL",
      "Unable to resolve CommonJS require for blessed runtime loading.",
      {
        component: "blessed.runtime",
        remediation: "Run in a Bun/Node environment that supports require().",
      },
    );
  }
}

export function assertBlessedModule(value: unknown): asserts value is BlessedModule {
  if (!value || typeof value !== "object") {
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

  const dynamicRequire = getDynamicRequire();

  try {
    const maybeBlessed = dynamicRequire("blessed") as unknown;
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
