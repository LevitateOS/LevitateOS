import { describe, expect, it } from "bun:test";
import { toggleChoice } from "../src/widgets/confirm";
import { nextListIndex } from "../src/widgets/list";
import { progressRatio, renderProgress, renderProgressBar } from "../src/widgets/progress";
import { assertWidgetBaseOptions, cancelled, widgetSidebarContent } from "../src/widgets/shared";

describe("widgets helpers", () => {
  it("toggleChoice flips boolean state", () => {
    expect(toggleChoice(true)).toBe(false);
    expect(toggleChoice(false)).toBe(true);
  });

  it("nextListIndex clamps movement", () => {
    expect(nextListIndex(0, -1, 3)).toBe(0);
    expect(nextListIndex(1, 1, 3)).toBe(2);
    expect(nextListIndex(2, 1, 3)).toBe(2);
  });

  it("nextListIndex handles non-finite inputs", () => {
    expect(nextListIndex(Number.NaN, Number.NaN, 3)).toBe(0);
    expect(nextListIndex(1, 1, Number.NaN)).toBe(0);
  });

  it("progress rendering clamps ratios", () => {
    expect(renderProgress({ current: 5, total: 10 })).toBe("50%");
    expect(renderProgress({ current: 50, total: 10 })).toBe("100%");
    expect(renderProgressBar({ current: 5, total: 10 }, 10)).toContain("50%");
  });

  it("progress helpers are stable with invalid numbers", () => {
    expect(progressRatio({ current: Number.NaN, total: 10 })).toBe(0);
    expect(progressRatio({ current: 10, total: Number.NaN })).toBe(0);
    expect(renderProgress({ current: Number.NaN, total: 10 })).toBe("0%");
    expect(renderProgressBar({ current: 5, total: 10 }, Number.NaN)).toContain("50%");
  });

  it("sidebar falls back to first step when current step is unknown", () => {
    const sidebar = widgetSidebarContent({
      screen: {} as any,
      title: "Demo",
      steps: [
        { id: "first", label: "First" },
        { id: "second", label: "Second" },
      ],
      currentStep: "missing",
    });

    expect(sidebar.includes("> 1. First")).toBe(true);
  });

  it("widget option validation rejects empty titles", () => {
    expect(() =>
      assertWidgetBaseOptions(
        {
          screen: {} as any,
          title: "   ",
        },
        "tests",
      ),
    ).toThrow();
  });

  it("cancelled helper sets code and component context", () => {
    const error = cancelled("stop");
    expect(error.code).toBe("CANCELLED");
    expect(error.context.component).toBe("widgets");
  });
});
