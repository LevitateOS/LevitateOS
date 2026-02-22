import { createBlessedList } from "../adapters/blessed/list";
import { TuiKitError } from "../core/errors";
import { clamp } from "../core/layout";
import { createLifecycleScope } from "../core/lifecycle";
import { resolveIntentColor } from "../theme";
import { toInt } from "../internal/numbers";
import type { WidgetBaseOptions } from "./types";
import { assertWidgetBaseOptions, cancelled, createWidgetShell } from "./shared";

export type PickFromListOptions = WidgetBaseOptions & {
  items: string[];
  defaultIndex?: number;
};

export function nextListIndex(currentIndex: number, delta: number, itemCount: number): number {
  if (!Number.isFinite(itemCount) || itemCount <= 0) {
    return 0;
  }

  const base = toInt(currentIndex, 0);
  const step = toInt(delta, 0);

  return clamp(base + step, 0, Math.floor(itemCount) - 1);
}

export function pickFromList(options: PickFromListOptions): Promise<number> {
  assertWidgetBaseOptions(options, "widgets.list.pickFromList");

  if (options.items.length === 0) {
    return Promise.reject(
      new TuiKitError("INTERNAL", "pickFromList requires at least one item.", {
        component: "widgets.list.pickFromList",
      }),
    );
  }

  const shell = createWidgetShell(options, "Arrows move | Enter select | Esc cancel");
  const selectedFg = resolveIntentColor(options.screen.theme, "background", options.screen.colors);
  const selectedBg = resolveIntentColor(options.screen.theme, "accent", options.screen.colors);
  const itemFg = resolveIntentColor(options.screen.theme, "text", options.screen.colors);

  const list = createBlessedList({
    parent: shell.content,
    top: 0,
    left: 0,
    width: "100%-2",
    height: "100%-2",
    items: options.items,
    keys: true,
    vi: true,
    mouse: true,
    style: {
      selected: {
        fg: selectedFg,
        bg: selectedBg,
      },
      item: {
        fg: itemFg,
      },
    },
  });

  const startIndex = clamp(toInt(options.defaultIndex, 0), 0, options.items.length - 1);

  list.select(startIndex);
  list.focus();
  options.screen.render();

  return new Promise((resolve, reject) => {
    const scope = createLifecycleScope("widgets.list.pickFromList");
    let settled = false;
    scope.onDispose(() => {
      scope.safeDestroy(list);
    });
    scope.onDispose(() => {
      scope.safeRender(options.screen);
    });

    const selectedIndex = (): number => {
      if (typeof list.selected === "number") {
        return clamp(list.selected, 0, options.items.length - 1);
      }
      return startIndex;
    };

    const settleResolve = (index: number) => {
      if (settled) {
        return;
      }
      settled = true;
      scope.dispose();
      resolve(index);
    };

    const settleReject = () => {
      if (settled) {
        return;
      }
      settled = true;
      scope.dispose();
      reject(cancelled("List selection cancelled by user."));
    };

    const onSelect = (_item: unknown, index?: unknown) => {
      if (typeof index === "number") {
        settleResolve(clamp(index, 0, options.items.length - 1));
        return;
      }
      settleResolve(selectedIndex());
    };
    scope.bindEvent(list, "select", onSelect);

    scope.bindKey(options.screen, ["enter"], () => {
      settleResolve(selectedIndex());
    });

    scope.bindKey(options.screen, ["escape", "q"], () => {
      settleReject();
    });
  });
}
