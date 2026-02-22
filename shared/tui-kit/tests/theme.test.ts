import { describe, expect, it } from "bun:test";
import { createTheme, footerHint, normalizeThemeLayout } from "../src/theme";
import { horizontalRule, wrapText } from "../src/theme/styles";

describe("theme helpers", () => {
  it("normalizeThemeLayout clamps invalid values", () => {
    const layout = normalizeThemeLayout({
      sidebarWidth: Number.NaN,
      headerHeight: -1,
      footerHeight: -1,
      minColumns: 1,
      minRows: 1,
    });

    expect(layout.sidebarWidth).toBeGreaterThan(0);
    expect(layout.headerHeight).toBeGreaterThan(0);
    expect(layout.footerHeight).toBeGreaterThan(0);
    expect(layout.minRows).toBeGreaterThanOrEqual(layout.headerHeight + layout.footerHeight + 1);
  });

  it("createTheme keeps default token for blank overrides", () => {
    const theme = createTheme({ accent: "" });
    expect(theme.tokens.accent.length).toBeGreaterThan(0);
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
