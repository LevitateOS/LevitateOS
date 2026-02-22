import { askBlessedInput } from "../adapters/blessed/input";
import { TuiKitError } from "../core/errors";
import type { WidgetBaseOptions } from "./types";
import { assertWidgetBaseOptions, createWidgetShell } from "./shared";

export type AskTextOptions = WidgetBaseOptions & {
  prompt: string;
  initial?: string;
  allowEmpty?: boolean;
};

export async function askText(options: AskTextOptions): Promise<string> {
  assertWidgetBaseOptions(options, "widgets.prompt.askText");

  if (typeof options.prompt !== "string" || options.prompt.trim().length === 0) {
    throw new TuiKitError("INTERNAL", "askText requires a non-empty prompt.", {
      component: "widgets.prompt.askText",
    });
  }

  createWidgetShell(options, "Enter confirm | Esc cancel");

  const value = await askBlessedInput({
    screen: options.screen.raw,
    theme: options.screen.theme,
    colors: options.screen.colors,
    title: options.title.trim(),
    prompt: options.prompt,
    initial: options.initial,
  });

  if ((options.allowEmpty ?? true) || value.trim().length > 0) {
    return value;
  }

  if (options.initial !== undefined) {
    return options.initial;
  }

  throw new TuiKitError("INTERNAL", "Empty input is not allowed.", {
    component: "widgets.prompt.askText",
    prompt: options.prompt,
  });
}
