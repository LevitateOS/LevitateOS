import { describe, expect, it } from "bun:test";
import { isTuiKitError, TuiKitError } from "../src/core/errors";
import { normalizeKeySpec, onKey } from "../src/core/events";
import { clamp, computeTwoPaneMetrics } from "../src/core/layout";
import { ensureTerminalMinimum } from "../src/core/screen";
import { normalizeTerminalSize, terminalMeetsMinimum } from "../src/core/terminal";
import { createTheme } from "../src/theme";

const TEST_THEME = createTheme();

describe("core utilities", () => {
  it("clamp bounds values", () => {
    expect(clamp(-1, 0, 10)).toBe(0);
    expect(clamp(42, 0, 10)).toBe(10);
    expect(clamp(5, 0, 10)).toBe(5);
  });

  it("clamp handles inverted min/max bounds", () => {
    expect(clamp(50, 10, 0)).toBe(10);
    expect(clamp(-5, 10, 0)).toBe(0);
  });

  it("terminal minimum guard fails with actionable error", () => {
    const fakeScreen = {
      raw: {},
      theme: TEST_THEME,
      colors: { mode: "ansi16", enabled: true },
      width: 79,
      height: 23,
      render: () => {},
      destroy: () => {},
      key: () => {},
      unkey: () => {},
      on: () => {},
    };

    try {
      ensureTerminalMinimum(fakeScreen, { columns: 80, rows: 24 });
      expect.unreachable("expected terminal check to throw");
    } catch (error: unknown) {
      expect(error).toBeInstanceOf(TuiKitError);
      const typed = error as TuiKitError;
      expect(typed.code).toBe("TERMINAL_TOO_SMALL");
    }
  });

  it("normalizes terminal size values to positive integers", () => {
    expect(
      normalizeTerminalSize(
        {
          columns: Number.NaN,
          rows: -10,
        },
        { columns: 120, rows: 40 },
      ),
    ).toEqual({ columns: 120, rows: 40 });
  });

  it("terminal minimum check uses normalized dimensions", () => {
    expect(terminalMeetsMinimum({ columns: 100, rows: 30 }, { columns: 80.9, rows: 24.2 })).toBe(
      true,
    );
  });

  it("two-pane metrics keep sidebar and content within total width", () => {
    const metrics = computeTwoPaneMetrics(24, 10, 30, 1, 1);

    expect(metrics.sidebarColumns + metrics.contentColumns).toBe(metrics.totalColumns);
    expect(metrics.bodyRows + metrics.headerRows + metrics.footerRows).toBe(metrics.totalRows);
    expect(metrics.contentInnerColumns).toBeGreaterThan(0);
    expect(metrics.contentInnerRows).toBeGreaterThan(0);
  });

  it("onKey skips registration when key list is empty", () => {
    const calls = {
      key: 0,
      unkey: 0,
    };

    const fakeScreen = {
      raw: {},
      theme: TEST_THEME,
      colors: { mode: "ansi16", enabled: true },
      width: 80,
      height: 24,
      render: () => {},
      destroy: () => {},
      key: () => {
        calls.key += 1;
      },
      unkey: () => {
        calls.unkey += 1;
      },
      on: () => {},
    };

    const unbind = onKey(fakeScreen as any, [], () => {});
    expect(calls.key).toBe(0);

    unbind();
    unbind();
    expect(calls.unkey).toBe(0);
  });

  it("normalizeKeySpec trims and deduplicates keys", () => {
    expect(normalizeKeySpec("  enter  ")).toBe("enter");
    expect(normalizeKeySpec("   ")).toBeNull();
    expect(normalizeKeySpec([" j", "j ", "k", ""])).toEqual(["j", "k"]);
  });

  it("detects structured TuiKitError values", () => {
    expect(isTuiKitError(new TuiKitError("INTERNAL", "boom"))).toBe(true);
    expect(
      isTuiKitError({
        name: "TuiKitError",
        message: "cancelled",
        code: "CANCELLED",
      }),
    ).toBe(true);
    expect(
      isTuiKitError({
        name: "TuiKitError",
        message: "wrong code",
        code: "NOT_A_CODE",
      }),
    ).toBe(false);
  });
});
