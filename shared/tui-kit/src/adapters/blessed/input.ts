import { TuiKitError } from "../../core/errors";
import { createLifecycleScope } from "../../core/lifecycle";
import { resolveIntentColor, type ColorRuntime, type TuiTheme } from "../../theme";
import type { BlessedBox } from "./box";
import { createBlessedBox } from "./box";
import { callBlessedFactory } from "./factory";
import type { BlessedScreen } from "./screen";

export type BlessedTextbox = {
  on: (event: string, handler: (...args: unknown[]) => void) => void;
  removeListener?: (event: string, handler: (...args: unknown[]) => void) => void;
  key?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
  unkey?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
  focus?: () => void;
  readInput?: () => void;
  getValue?: () => unknown;
  destroy?: () => void;
  detach?: () => void;
  [key: string]: unknown;
};

export type AskBlessedInputOptions = {
  screen: BlessedScreen;
  theme: TuiTheme;
  colors: ColorRuntime;
  title: string;
  prompt: string;
  initial?: string;
};

function normalizeInputValue(value: unknown): string {
  if (typeof value === "string") {
    return value;
  }

  if (value === null || value === undefined) {
    return "";
  }

  return String(value);
}

export function createBlessedTextbox(options: Record<string, unknown> = {}): BlessedTextbox {
  return callBlessedFactory<BlessedTextbox>("textbox", options, "blessed.input");
}

export function askBlessedInput(options: AskBlessedInputOptions): Promise<string> {
  const title = options.title.trim().length > 0 ? options.title : "Input";
  const borderColor = resolveIntentColor(options.theme, "accent", options.colors);
  const hintColor = resolveIntentColor(options.theme, "dimText", options.colors);

  return new Promise((resolve, reject) => {
    if (!options.screen || typeof options.screen.render !== "function") {
      reject(
        new TuiKitError("INTERNAL", "askBlessedInput requires a valid screen.", {
          component: "blessed.input",
        }),
      );
      return;
    }

    const scope = createLifecycleScope("blessed.input.ask");
    let settled = false;

    let modal: BlessedBox | undefined;
    let input: BlessedTextbox | undefined;

    const cleanup = () => {
      scope.dispose();
    };

    const settleResolve = (value: unknown) => {
      if (settled) {
        return;
      }
      settled = true;
      cleanup();
      resolve(normalizeInputValue(value));
    };

    const settleRejectCancelled = () => {
      if (settled) {
        return;
      }
      settled = true;
      cleanup();
      reject(
        new TuiKitError("CANCELLED", "Input cancelled by user.", {
          component: "blessed.input",
        }),
      );
    };

    const settleRejectInternal = (error: unknown) => {
      if (settled) {
        return;
      }
      settled = true;
      cleanup();
      const message = error instanceof Error ? error.message : String(error);
      reject(
        new TuiKitError("INTERNAL", "Input dialog failed.", {
          component: "blessed.input",
          observed: message,
        }),
      );
    };

    try {
      modal = createBlessedBox({
        parent: options.screen,
        top: "center",
        left: "center",
        width: "70%",
        height: 9,
        border: "line",
        label: ` ${title} `,
        tags: true,
        style: {
          border: borderColor ? { fg: borderColor } : undefined,
        },
      });
      scope.onDispose(() => {
        scope.safeDestroy(modal);
      });
      scope.onDispose(() => {
        scope.safeRender(options.screen);
      });

      createBlessedBox({
        parent: modal,
        top: 1,
        left: 1,
        width: "100%-2",
        height: 1,
        content: options.prompt,
        tags: true,
      });

      input = createBlessedTextbox({
        parent: modal,
        top: 3,
        left: 1,
        width: "100%-2",
        height: 3,
        border: "line",
        inputOnFocus: true,
        keys: true,
        mouse: true,
        value: options.initial ?? "",
      });
      scope.onDispose(() => {
        scope.safeDestroy(input);
      });

      createBlessedBox({
        parent: modal,
        bottom: 0,
        left: 1,
        width: "100%-2",
        height: 1,
        content: "Enter confirm | Esc cancel",
        style: hintColor ? { fg: hintColor } : undefined,
      });

      scope.bindEvent(input, "submit", (value: unknown) => {
        if (value !== undefined) {
          settleResolve(value);
          return;
        }

        if (typeof input?.getValue === "function") {
          settleResolve(input.getValue());
          return;
        }

        settleResolve("");
      });

      scope.bindEvent(input, "error", (error: unknown) => {
        settleRejectInternal(error);
      });

      scope.bindKey(input, ["escape", "C-c"], () => {
        settleRejectCancelled();
      });

      // Fallback cancellation in case focus leaves the input widget.
      scope.bindKey(options.screen, ["escape", "C-c"], () => {
        settleRejectCancelled();
      });

      if (typeof input.focus === "function") {
        input.focus();
      }
      scope.safeRender(options.screen);

      if (typeof input.readInput === "function") {
        input.readInput();
      }
    } catch (error: unknown) {
      settleRejectInternal(error);
    }
  });
}
