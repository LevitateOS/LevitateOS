import { createLifecycleScope } from "../core/lifecycle";
import type { WidgetBaseOptions } from "./types";
import { assertWidgetBaseOptions, createWidgetShell } from "./shared";

export type ShowScrollableOptions = WidgetBaseOptions & {
  content: string;
};

export function showScrollable(options: ShowScrollableOptions): Promise<void> {
  assertWidgetBaseOptions(options, "widgets.scrollable.showScrollable");

  const shell = createWidgetShell(options, "Arrows/PageUp/PageDown scroll | q close", true);

  shell.content.setContent(options.content);
  shell.content.setScroll(0);
  shell.content.focus();
  options.screen.render();

  return new Promise((resolve) => {
    const scope = createLifecycleScope("widgets.scrollable.showScrollable");
    let settled = false;

    const close = () => {
      if (settled) {
        return;
      }
      settled = true;
      scope.dispose();
      resolve();
    };
    scope.onDispose(() => {
      scope.safeRender(options.screen);
    });

    const scrollBy = (delta: number) => {
      shell.content.scroll(delta);
      options.screen.render();
    };

    scope.bindKey(options.screen, ["up", "k"], () => {
      scrollBy(-1);
    });

    scope.bindKey(options.screen, ["down", "j"], () => {
      scrollBy(1);
    });

    scope.bindKey(options.screen, ["pageup", "b"], () => {
      scrollBy(-10);
    });

    scope.bindKey(options.screen, ["pagedown", "space"], () => {
      scrollBy(10);
    });

    scope.bindKey(options.screen, ["g", "home"], () => {
      shell.content.setScroll(0);
      options.screen.render();
    });

    scope.bindKey(options.screen, ["G", "end", "S-g"], () => {
      shell.content.setScrollPerc(100);
      options.screen.render();
    });

    scope.bindKey(options.screen, ["enter", "escape", "q"], () => {
      close();
    });
  });
}
