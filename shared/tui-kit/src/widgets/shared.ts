import { createTwoPaneShell } from "../core/layout";
import { TuiKitError } from "../core/errors";
import { renderWizardSidebar } from "../wizard/sidebar";
import type { WidgetBaseOptions } from "./types";

export function assertWidgetBaseOptions(options: WidgetBaseOptions, component: string): void {
  if (!options || typeof options !== "object") {
    throw new TuiKitError("INTERNAL", "Widget options are missing.", {
      component,
    });
  }

  if (!options.screen) {
    throw new TuiKitError("INTERNAL", "Widget options.screen is required.", {
      component,
    });
  }

  if (typeof options.title !== "string" || options.title.trim().length === 0) {
    throw new TuiKitError("INTERNAL", "Widget options.title must be non-empty.", {
      component,
    });
  }
}

export function widgetSidebarContent(options: WidgetBaseOptions): string {
  const steps = options.steps ?? [];
  const safeSidebarTitle =
    typeof options.sidebarTitle === "string" ? options.sidebarTitle.trim() : "";
  const safeSidebarSubtitle =
    typeof options.sidebarSubtitle === "string" ? options.sidebarSubtitle.trim() : "";

  if (steps.length > 0) {
    const fallbackStepId = steps[0].id;
    const requestedStepId = options.currentStep ?? fallbackStepId;
    const current = steps.some((step) => step.id === requestedStepId)
      ? requestedStepId
      : fallbackStepId;
    return renderWizardSidebar(steps, current, {
      title: safeSidebarTitle,
      subtitle: safeSidebarSubtitle,
    });
  }

  const lines: string[] = [];
  if (safeSidebarTitle.length > 0) {
    lines.push(safeSidebarTitle);
  }
  if (safeSidebarSubtitle.length > 0) {
    lines.push(safeSidebarSubtitle);
  }

  if (lines.length === 0) {
    lines.push("tui-kit");
  }

  return lines.join("\n");
}

export function createWidgetShell(
  options: WidgetBaseOptions,
  footerText: string,
  contentScrollable = false,
) {
  assertWidgetBaseOptions(options, "widgets.shared.createWidgetShell");

  return createTwoPaneShell(options.screen, {
    title: options.title.trim(),
    sidebarContent: widgetSidebarContent(options),
    footerText,
    contentScrollable,
  });
}

export function cancelled(message: string): TuiKitError {
  return new TuiKitError("CANCELLED", message, {
    component: "widgets",
  });
}
