import { describe, expect, it } from "bun:test";
import {
  createTheme,
  detectColorRuntime,
  footerHint,
  resolveIntentColor,
  type ColorRuntime,
} from "../src/theme";
import { horizontalRule, styleText, wrapText } from "../src/theme/styles";

describe("theme helpers", () => {
  it("normalizeThemeLayout clamps invalid values", () => {
    const theme = createTheme(
      {},
      {
        sidebarWidth: Number.NaN,
        headerHeight: -1,
        footerHeight: -1,
        minColumns: 1,
        minRows: 1,
      },
    );

    expect(theme.layout.sidebarWidth).toBeGreaterThan(0);
    expect(theme.layout.headerHeight).toBeGreaterThan(0);
    expect(theme.layout.footerHeight).toBeGreaterThan(0);
    expect(theme.layout.minRows).toBeGreaterThanOrEqual(
      theme.layout.headerHeight + theme.layout.footerHeight + 1,
    );
  });

  it("createTheme keeps default color values for blank overrides", () => {
    const theme = createTheme({
      accent: {
        truecolor: "",
      },
    });
    expect(theme.colors.accent.truecolor.length).toBeGreaterThan(0);
  });

  it("createTheme supports shorthand literal color overrides", () => {
    const theme = createTheme({
      accent: "#00ff88",
    });

    expect(theme.colors.accent.truecolor).toBe("#00ff88");
    expect(theme.colors.accent.ansi256).toBeGreaterThanOrEqual(0);
    expect(theme.colors.accent.ansi16.length).toBeGreaterThan(0);
  });

  it("footerHint trims scope/extra and uses fallback scope", () => {
    expect(footerHint("   ")).toContain("[tui]");
    expect(footerHint("docs", "  extra  ")).toContain("| extra");
  });

  it("wrapText chunks long words to avoid overflow", () => {
    const lines = wrapText("supercalifragilistic", 5);
    expect(lines.every((line) => line.length <= 5)).toBe(true);
  });

  it("horizontalRule uses a single fill character", () => {
    expect(horizontalRule(4, "==")).toBe("====");
    expect(horizontalRule(3, "")).toBe("---");
  });
});

describe("color runtime", () => {
  it("detectColorRuntime uses FORCE_COLOR when set", () => {
    const runtime = detectColorRuntime({
      env: { FORCE_COLOR: "2" },
      isTTY: true,
    });
    expect(runtime).toEqual({
      mode: "ansi256",
      enabled: true,
    });
  });

  it("detectColorRuntime honors NO_COLOR", () => {
    const runtime = detectColorRuntime({
      env: { FORCE_COLOR: "3", NO_COLOR: "1" },
      isTTY: true,
    });
    expect(runtime).toEqual({
      mode: "mono",
      enabled: false,
    });
  });

  it("detectColorRuntime auto-detects truecolor and tty fallback", () => {
    expect(
      detectColorRuntime({
        env: { COLORTERM: "truecolor" },
        isTTY: true,
      }),
    ).toEqual({
      mode: "truecolor",
      enabled: true,
    });
    expect(
      detectColorRuntime({
        env: {},
        isTTY: false,
      }),
    ).toEqual({
      mode: "mono",
      enabled: false,
    });
  });

  it("resolveIntentColor degrades by mode", () => {
    const theme = createTheme();
    const truecolor: ColorRuntime = { mode: "truecolor", enabled: true };
    const ansi16: ColorRuntime = { mode: "ansi16", enabled: true };
    const mono: ColorRuntime = { mode: "mono", enabled: false };

    expect(resolveIntentColor(theme, "accent", truecolor)?.startsWith("#")).toBe(true);
    expect(resolveIntentColor(theme, "accent", ansi16)).toBe("cyan");
    expect(resolveIntentColor(theme, "accent", mono)).toBeUndefined();
  });
});

describe("styleText", () => {
  const theme = createTheme();

  it("compiles style directives to blessed tags", () => {
    const output = styleText("[[fg=$accent bold]]Hello[[/]]", {
      theme,
      runtime: { mode: "ansi16", enabled: true },
    });
    expect(output).toContain("{cyan-fg}{bold}Hello{/bold}{/cyan-fg}");
  });

  it("keeps invalid directives literal", () => {
    const output = styleText("[[bogus=1]]X[[/]]", {
      theme,
      runtime: { mode: "ansi16", enabled: true },
    });
    expect(output).toContain("[[bogus=1]]X");
  });

  it("supports escaped opening tags", () => {
    const output = styleText("\\[[notatag]]", {
      theme,
      runtime: { mode: "ansi16", enabled: true },
    });
    expect(output).toBe("[[notatag]]");
  });

  it("uses mono emphasis for semantic intents when colors are disabled", () => {
    const output = styleText("[[fg=$accent]]value[[/]]", {
      theme,
      runtime: { mode: "mono", enabled: false },
    });
    expect(output).toContain("{bold}value{/bold}");
  });
});
