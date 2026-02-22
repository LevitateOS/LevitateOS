import { callBlessedFactory } from "./factory";

export type BlessedBox = {
  destroy: () => void;
  detach: () => void;
  setContent: (value: string) => void;
  setScroll: (value: number) => void;
  setScrollPerc: (value: number) => void;
  focus: () => void;
  render: () => void;
  scroll: (delta: number) => void;
  [key: string]: unknown;
};

export function createBlessedBox(options: Record<string, unknown> = {}): BlessedBox {
  return callBlessedFactory<BlessedBox>("box", options, "blessed.box");
}
