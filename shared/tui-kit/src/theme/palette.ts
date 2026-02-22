export const palette = {
  black: "#000000",
  white: "#ffffff",
  gray100: "#f5f5f5",
  gray300: "#d4d4d4",
  gray500: "#737373",
  gray700: "#404040",
  cyan: "#00bcd4",
  blue: "#60a5fa",
  yellow: "#facc15",
  red: "#ef4444",
  green: "#22c55e",
} as const;

export type PaletteKey = keyof typeof palette;
