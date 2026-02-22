import { palette } from "./palette";

export type ColorMode = "truecolor" | "ansi256" | "ansi16" | "mono";

export type ColorIntent =
  | "border"
  | "text"
  | "dimText"
  | "accent"
  | "info"
  | "warning"
  | "error"
  | "success"
  | "background";

export type ColorValue = {
  truecolor: string;
  ansi256: number;
  ansi16: string;
  mono?: "normal" | "bold" | "dim";
};

export type ThemeColors = Record<ColorIntent, ColorValue>;

export type ThemeColorOverride = Partial<ColorValue> | string;

export type ThemeColorsOverride = Partial<Record<ColorIntent, ThemeColorOverride>>;

export type ThemeLayout = {
  sidebarWidth: number;
  headerHeight: number;
  footerHeight: number;
  minColumns: number;
  minRows: number;
};

export type TuiTheme = {
  colors: ThemeColors;
  layout: ThemeLayout;
};

export const defaultThemeColors: ThemeColors = {
  border: {
    truecolor: palette.gray500,
    ansi256: 243,
    ansi16: "gray",
    mono: "normal",
  },
  text: {
    truecolor: palette.white,
    ansi256: 15,
    ansi16: "white",
    mono: "normal",
  },
  dimText: {
    truecolor: palette.gray300,
    ansi256: 250,
    ansi16: "gray",
    mono: "normal",
  },
  accent: {
    truecolor: palette.cyan,
    ansi256: 51,
    ansi16: "cyan",
    mono: "bold",
  },
  info: {
    truecolor: palette.blue,
    ansi256: 75,
    ansi16: "blue",
    mono: "bold",
  },
  warning: {
    truecolor: palette.yellow,
    ansi256: 220,
    ansi16: "yellow",
    mono: "bold",
  },
  error: {
    truecolor: palette.red,
    ansi256: 203,
    ansi16: "red",
    mono: "bold",
  },
  success: {
    truecolor: palette.green,
    ansi256: 41,
    ansi16: "green",
    mono: "bold",
  },
  background: {
    truecolor: palette.black,
    ansi256: 0,
    ansi16: "black",
    mono: "normal",
  },
};

export const defaultThemeLayout: ThemeLayout = {
  sidebarWidth: 30,
  headerHeight: 1,
  footerHeight: 1,
  minColumns: 80,
  minRows: 24,
};

export const defaultTuiTheme: TuiTheme = {
  colors: defaultThemeColors,
  layout: defaultThemeLayout,
};
