import { callBlessedFactory } from "./factory";

export type BlessedKey = {
  name?: string;
  full?: string;
  ctrl?: boolean;
  shift?: boolean;
  sequence?: string;
};

export type BlessedKeyHandler = (ch: string, key: BlessedKey) => void;

export type BlessedScreen = {
  width?: number;
  height?: number;
  key: (keys: string | string[], handler: BlessedKeyHandler) => void;
  unkey?: (keys: string | string[], handler: BlessedKeyHandler) => void;
  on: (event: string, handler: (...args: unknown[]) => void) => void;
  render: () => void;
  destroy: () => void;
};

export type CreateBlessedScreenOptions = {
  title?: string;
  smartCSR?: boolean;
  fullUnicode?: boolean;
  dockBorders?: boolean;
  useBCE?: boolean;
};

export function createBlessedScreen(options: CreateBlessedScreenOptions = {}): BlessedScreen {
  const title =
    typeof options.title === "string" && options.title.trim().length > 0
      ? options.title
      : "tui-kit";

  return callBlessedFactory<BlessedScreen>(
    "screen",
    {
      smartCSR: options.smartCSR ?? true,
      fullUnicode: options.fullUnicode ?? true,
      dockBorders: options.dockBorders ?? true,
      useBCE: options.useBCE ?? true,
      title,
    },
    "blessed.screen",
  );
}
