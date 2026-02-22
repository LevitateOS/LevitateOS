import { TuiKitError } from "../../core/errors";
import { loadBlessed, type BlessedModule } from "./runtime";

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function callBlessedFactory<TNode>(
  factory: keyof BlessedModule,
  options: Record<string, unknown>,
  component: string,
): TNode {
  const blessed = loadBlessed();

  try {
    return blessed[factory](options) as TNode;
  } catch (error: unknown) {
    throw new TuiKitError("INTERNAL", `Failed to create blessed ${factory}.`, {
      component,
      observed: errorMessage(error),
    });
  }
}
