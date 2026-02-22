import { toPositiveInt } from "./internal/numbers";
import {
  detectColorRuntime,
  normalizeColorValue,
  normalizeThemeColorMode,
  resolveColorReference,
  resolveIntentColor,
  resolveIntentMono,
  resolveLiteralColor,
  resolveRgbLiteral,
  type ColorRuntime,
  type DetectColorRuntimeOptions,
} from "./theme/colors";
import {
  defaultThemeColors,
  defaultThemeLayout,
  defaultTuiTheme,
  type ColorIntent,
  type ColorMode,
  type ThemeColorOverride,
  type ThemeColors,
  type ThemeColorsOverride,
  type ThemeLayout,
  type TuiTheme,
} from "./theme/tokens";

export * from "./theme/tokens";
export * from "./theme/palette";
export * from "./theme/styles";
export * from "./theme/colors";

export function normalizeThemeLayout(layout: Partial<ThemeLayout>): ThemeLayout {
  const candidate: ThemeLayout = {
    sidebarWidth: toPositiveInt(
      layout.sidebarWidth ?? defaultThemeLayout.sidebarWidth,
      defaultThemeLayout.sidebarWidth,
    ),
    headerHeight: toPositiveInt(
      layout.headerHeight ?? defaultThemeLayout.headerHeight,
      defaultThemeLayout.headerHeight,
    ),
    footerHeight: toPositiveInt(
      layout.footerHeight ?? defaultThemeLayout.footerHeight,
      defaultThemeLayout.footerHeight,
    ),
    minColumns: toPositiveInt(
      layout.minColumns ?? defaultThemeLayout.minColumns,
      defaultThemeLayout.minColumns,
    ),
    minRows: toPositiveInt(
      layout.minRows ?? defaultThemeLayout.minRows,
      defaultThemeLayout.minRows,
    ),
  };

  const minRowsNeeded = candidate.headerHeight + candidate.footerHeight + 1;
  if (candidate.minRows < minRowsNeeded) {
    candidate.minRows = minRowsNeeded;
  }

  const minColumnsNeeded = candidate.sidebarWidth + 1;
  if (candidate.minColumns < minColumnsNeeded) {
    candidate.minColumns = minColumnsNeeded;
  }

  return candidate;
}

function normalizeColorOverride(
  intent: ColorIntent,
  overrides: ThemeColorsOverride,
  fallback: ThemeColors,
): ThemeColors[ColorIntent] {
  const base = fallback[intent];
  const override = overrides[intent] as ThemeColorOverride | undefined;
  return normalizeColorValue(override, base);
}

export function normalizeThemeColors(
  colors: ThemeColorsOverride = {},
  fallback: ThemeColors = defaultThemeColors,
): ThemeColors {
  return {
    border: normalizeColorOverride("border", colors, fallback),
    text: normalizeColorOverride("text", colors, fallback),
    dimText: normalizeColorOverride("dimText", colors, fallback),
    accent: normalizeColorOverride("accent", colors, fallback),
    info: normalizeColorOverride("info", colors, fallback),
    warning: normalizeColorOverride("warning", colors, fallback),
    error: normalizeColorOverride("error", colors, fallback),
    success: normalizeColorOverride("success", colors, fallback),
    background: normalizeColorOverride("background", colors, fallback),
  };
}

export function createTheme(
  colors: ThemeColorsOverride = {},
  layout: Partial<ThemeLayout> = {},
): TuiTheme {
  return {
    colors: normalizeThemeColors(colors),
    layout: normalizeThemeLayout(layout),
  };
}

export function footerHint(scope: string, extra?: string): string {
  const safeScope = scope.trim().length > 0 ? scope.trim() : "tui";
  const base = `[${safeScope}] q quit | arrows navigate`;
  if (!extra || extra.trim().length === 0) {
    return base;
  }
  return `${base} | ${extra.trim()}`;
}

export {
  defaultThemeColors,
  defaultThemeLayout,
  defaultTuiTheme,
  detectColorRuntime,
  normalizeThemeColorMode,
  resolveColorReference,
  resolveIntentColor,
  resolveIntentMono,
  resolveLiteralColor,
  resolveRgbLiteral,
};

export type {
  ColorIntent,
  ColorMode,
  ColorRuntime,
  DetectColorRuntimeOptions,
  ThemeColorsOverride,
};
