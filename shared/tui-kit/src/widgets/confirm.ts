import { createLifecycleScope } from "../core/lifecycle";
import type { WidgetBaseOptions } from "./types";
import { assertWidgetBaseOptions, cancelled, createWidgetShell } from "./shared";

export type AskYesNoOptions = WidgetBaseOptions & {
  prompt: string;
  defaultYes?: boolean;
};

export function toggleChoice(current: boolean): boolean {
  return !current;
}

export function askYesNo(options: AskYesNoOptions): Promise<boolean> {
  assertWidgetBaseOptions(options, "widgets.confirm.askYesNo");

  const shell = createWidgetShell(options, "Y/N or arrows | Enter confirm | Esc cancel");
  let selected = options.defaultYes ?? true;

  const render = () => {
    const yesMarker = selected ? "[x]" : "[ ]";
    const noMarker = selected ? "[ ]" : "[x]";
    shell.content.setContent([options.prompt, "", `${yesMarker} Yes`, `${noMarker} No`].join("\n"));
    options.screen.render();
  };

  return new Promise((resolve, reject) => {
    const scope = createLifecycleScope("widgets.confirm.askYesNo");
    let settled = false;

    const settleResolve = (value: boolean) => {
      if (settled) {
        return;
      }
      settled = true;
      scope.dispose();
      resolve(value);
    };

    const settleReject = () => {
      if (settled) {
        return;
      }
      settled = true;
      scope.dispose();
      reject(cancelled("Confirmation cancelled by user."));
    };
    scope.onDispose(() => {
      scope.safeRender(options.screen);
    });

    scope.bindKey(options.screen, ["left", "right", "tab", "S-tab", "h", "l"], () => {
      selected = toggleChoice(selected);
      render();
    });

    scope.bindKey(options.screen, ["y", "Y"], () => {
      selected = true;
      render();
    });

    scope.bindKey(options.screen, ["n", "N"], () => {
      selected = false;
      render();
    });

    scope.bindKey(options.screen, ["enter"], () => {
      settleResolve(selected);
    });

    scope.bindKey(options.screen, ["escape", "q"], () => {
      settleReject();
    });

    render();
  });
}
