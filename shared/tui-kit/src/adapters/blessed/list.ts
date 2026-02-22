import { callBlessedFactory } from "./factory";

export type BlessedList = {
  selected?: number;
  on: (event: string, handler: (...args: unknown[]) => void) => void;
  off?: (event: string, handler: (...args: unknown[]) => void) => void;
  removeListener?: (event: string, handler: (...args: unknown[]) => void) => void;
  key?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
  unkey?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
  focus: () => void;
  select: (index: number) => void;
  destroy: () => void;
  detach: () => void;
  [key: string]: unknown;
};

export function createBlessedList(options: Record<string, unknown> = {}): BlessedList {
  return callBlessedFactory<BlessedList>("list", options, "blessed.list");
}
