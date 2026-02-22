import { palette } from "./palette";

export type ThemeTokens = {
  border: string;
  text: string;
  dimText: string;
  accent: string;
  info: string;
  warning: string;
  error: string;
  success: string;
  background: string;
};

export type ThemeLayout = {
  sidebarWidth: number;
  headerHeight: number;
  footerHeight: number;
  minColumns: number;
  minRows: number;
};

export type TuiTheme = {
  tokens: ThemeTokens;
  layout: ThemeLayout;
};

export const defaultThemeTokens: ThemeTokens = {
  border: palette.gray500,
  text: palette.white,
  dimText: palette.gray300,
  accent: palette.cyan,
  info: palette.blue,
  warning: palette.yellow,
  error: palette.red,
  success: palette.green,
  background: palette.black,
};

export const defaultThemeLayout: ThemeLayout = {
  sidebarWidth: 30,
  headerHeight: 1,
  footerHeight: 1,
  minColumns: 80,
  minRows: 24,
};
