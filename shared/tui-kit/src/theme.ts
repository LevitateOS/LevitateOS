import {
  defaultThemeLayout,
  defaultThemeTokens,
  type ThemeLayout,
  type ThemeTokens,
  type TuiTheme,
} from "./theme/tokens";
import { toPositiveInt } from "./internal/numbers";

export * from "./theme/tokens";
export * from "./theme/palette";
export * from "./theme/styles";

export const tuiTheme: TuiTheme = {
  tokens: defaultThemeTokens,
  layout: defaultThemeLayout,
};

function stringToken(value: string, fallback: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    return fallback;
  }
  return value;
}

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

export function normalizeThemeTokens(tokens: Partial<ThemeTokens>): ThemeTokens {
  return {
    border: stringToken(tokens.border ?? defaultThemeTokens.border, defaultThemeTokens.border),
    text: stringToken(tokens.text ?? defaultThemeTokens.text, defaultThemeTokens.text),
    dimText: stringToken(tokens.dimText ?? defaultThemeTokens.dimText, defaultThemeTokens.dimText),
    accent: stringToken(tokens.accent ?? defaultThemeTokens.accent, defaultThemeTokens.accent),
    info: stringToken(tokens.info ?? defaultThemeTokens.info, defaultThemeTokens.info),
    warning: stringToken(tokens.warning ?? defaultThemeTokens.warning, defaultThemeTokens.warning),
    error: stringToken(tokens.error ?? defaultThemeTokens.error, defaultThemeTokens.error),
    success: stringToken(tokens.success ?? defaultThemeTokens.success, defaultThemeTokens.success),
    background: stringToken(
      tokens.background ?? defaultThemeTokens.background,
      defaultThemeTokens.background,
    ),
  };
}

export function createTheme(
  tokens: Partial<ThemeTokens> = {},
  layout: Partial<ThemeLayout> = {},
): TuiTheme {
  return {
    tokens: normalizeThemeTokens(tokens),
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
