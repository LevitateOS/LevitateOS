import {
  createBlessedScreen,
  type BlessedKey,
  type BlessedKeyHandler,
  type BlessedScreen,
} from "../adapters/blessed/screen";
import { createBlessedBox } from "../adapters/blessed/box";
import { TuiKitError } from "./errors";
import { getTerminalSize, normalizeTerminalSize, terminalMeetsMinimum } from "./terminal";
import { toPositiveInt } from "../internal/numbers";

export type TerminalMinimum = {
  columns: number;
  rows: number;
};

export type CreateScreenOptions = {
  title?: string;
  quitKeybinds?: string[];
};

export type ScreenHandle = {
  raw: BlessedScreen;
  readonly width: number;
  readonly height: number;
  render: () => void;
  destroy: () => void;
  key: (keys: string | string[], handler: BlessedKeyHandler) => void;
  unkey: (keys: string | string[], handler: BlessedKeyHandler) => void;
  on: (event: string, handler: (...args: unknown[]) => void) => void;
};

const DEFAULT_MINIMUM: TerminalMinimum = {
  columns: 80,
  rows: 24,
};

class BlessedScreenHandle implements ScreenHandle {
  readonly raw: BlessedScreen;

  constructor(raw: BlessedScreen) {
    this.raw = raw;
  }

  get width(): number {
    const fallback = getTerminalSize().columns;
    return toPositiveInt(this.raw.width, fallback);
  }

  get height(): number {
    const fallback = getTerminalSize().rows;
    return toPositiveInt(this.raw.height, fallback);
  }

  render(): void {
    this.raw.render();
  }

  destroy(): void {
    this.raw.destroy();
  }

  key(keys: string | string[], handler: BlessedKeyHandler): void {
    this.raw.key(keys, handler);
  }

  unkey(keys: string | string[], handler: BlessedKeyHandler): void {
    if (typeof this.raw.unkey === "function") {
      this.raw.unkey(keys, handler);
    }
  }

  on(event: string, handler: (...args: unknown[]) => void): void {
    this.raw.on(event, handler);
  }
}

export function createScreen(options: CreateScreenOptions = {}): ScreenHandle {
  const raw = createBlessedScreen({
    title: options.title ?? "tui-kit",
  });
  const handle = new BlessedScreenHandle(raw);

  const quitKeys = Array.from(
    new Set((options.quitKeybinds ?? ["C-c"]).filter((key) => key.length > 0)),
  );
  if (quitKeys.length > 0) {
    handle.key(quitKeys, () => {
      try {
        handle.destroy();
      } finally {
        process.exit(0);
      }
    });
  }

  return handle;
}

export function ensureTerminalMinimum(
  screen: ScreenHandle,
  minimum: TerminalMinimum = DEFAULT_MINIMUM,
): void {
  const observed = {
    columns: screen.width,
    rows: screen.height,
  };
  const required = normalizeTerminalSize(minimum, DEFAULT_MINIMUM);

  if (terminalMeetsMinimum(observed, required)) {
    return;
  }

  throw new TuiKitError(
    "TERMINAL_TOO_SMALL",
    `Terminal too small: got ${observed.columns}x${observed.rows}, requires at least ${required.columns}x${required.rows}.`,
    {
      observed_columns: observed.columns,
      observed_rows: observed.rows,
      required_columns: required.columns,
      required_rows: required.rows,
    },
  );
}

export function showFatal(screen: ScreenHandle, message: string, title = "Fatal Error"): never {
  try {
    createBlessedBox({
      parent: screen.raw,
      top: "center",
      left: "center",
      width: "80%",
      height: "shrink",
      border: "line",
      tags: true,
      label: ` ${title} `,
      content: `${message}\n\nExiting...`,
      style: {
        border: { fg: "red" },
      },
      padding: {
        left: 1,
        right: 1,
      },
    });
    screen.render();
  } catch {
    // Fallback to stderr only.
  }

  try {
    console.error(`${title}: ${message}`);
    try {
      screen.destroy();
    } catch {
      // Ignore terminal teardown errors during fatal shutdown.
    }
  } finally {
    process.exit(1);
  }

  throw new TuiKitError("INTERNAL", "process.exit returned unexpectedly in showFatal().");
}

export type { BlessedKey };
