import { clampNumber, toInt } from "../internal/numbers";
import { palette } from "./palette";
import type { ColorIntent, ColorMode, ColorValue, TuiTheme } from "./tokens";

export type ColorRuntime = {
  mode: ColorMode;
  enabled: boolean;
};

export type DetectColorRuntimeOptions = {
  env?: Record<string, string | undefined>;
  isTTY?: boolean;
  mode?: ColorMode;
  enabled?: boolean;
};

type Rgb = {
  r: number;
  g: number;
  b: number;
};

const ANSI16_RGB: Record<string, Rgb> = {
  black: { r: 0, g: 0, b: 0 },
  red: { r: 205, g: 49, b: 49 },
  green: { r: 13, g: 188, b: 121 },
  yellow: { r: 229, g: 229, b: 16 },
  blue: { r: 36, g: 114, b: 200 },
  magenta: { r: 188, g: 63, b: 188 },
  cyan: { r: 17, g: 168, b: 205 },
  white: { r: 229, g: 229, b: 229 },
  gray: { r: 120, g: 120, b: 120 },
  grey: { r: 120, g: 120, b: 120 },
};

const NAMED_COLORS: Record<string, string> = {
  black: palette.black,
  white: palette.white,
  gray: palette.gray500,
  grey: palette.gray500,
  red: palette.red,
  green: palette.green,
  yellow: palette.yellow,
  blue: palette.blue,
  cyan: palette.cyan,
  magenta: "#d946ef",
};

const TRUECOLOR_RUNTIME: ColorRuntime = { mode: "truecolor", enabled: true };
const ANSI256_RUNTIME: ColorRuntime = { mode: "ansi256", enabled: true };
const ANSI16_RUNTIME: ColorRuntime = { mode: "ansi16", enabled: true };

function normalizeHex(input: string): string | null {
  const value = input.trim().toLowerCase();
  if (/^#[0-9a-f]{6}$/.test(value)) {
    return value;
  }
  if (/^#[0-9a-f]{3}$/.test(value)) {
    const [_, r, g, b] = value;
    return `#${r}${r}${g}${g}${b}${b}`;
  }
  return null;
}

function fromHex(hex: string): Rgb {
  const clean = hex.startsWith("#") ? hex.slice(1) : hex;
  return {
    r: Number.parseInt(clean.slice(0, 2), 16),
    g: Number.parseInt(clean.slice(2, 4), 16),
    b: Number.parseInt(clean.slice(4, 6), 16),
  };
}

function toHex(rgb: Rgb): string {
  const toHexPart = (value: number): string => {
    const normalized = clampNumber(Math.round(value), 0, 255);
    return normalized.toString(16).padStart(2, "0");
  };
  return `#${toHexPart(rgb.r)}${toHexPart(rgb.g)}${toHexPart(rgb.b)}`;
}

function ansi256ToRgb(index: number): Rgb {
  const normalized = clampNumber(toInt(index, 0), 0, 255);
  if (normalized < 16) {
    const base: Rgb[] = [
      { r: 0, g: 0, b: 0 },
      { r: 128, g: 0, b: 0 },
      { r: 0, g: 128, b: 0 },
      { r: 128, g: 128, b: 0 },
      { r: 0, g: 0, b: 128 },
      { r: 128, g: 0, b: 128 },
      { r: 0, g: 128, b: 128 },
      { r: 192, g: 192, b: 192 },
      { r: 128, g: 128, b: 128 },
      { r: 255, g: 0, b: 0 },
      { r: 0, g: 255, b: 0 },
      { r: 255, g: 255, b: 0 },
      { r: 0, g: 0, b: 255 },
      { r: 255, g: 0, b: 255 },
      { r: 0, g: 255, b: 255 },
      { r: 255, g: 255, b: 255 },
    ];
    return base[normalized];
  }

  if (normalized <= 231) {
    const offset = normalized - 16;
    const r = Math.floor(offset / 36);
    const g = Math.floor((offset % 36) / 6);
    const b = offset % 6;
    const scale = [0, 95, 135, 175, 215, 255];
    return {
      r: scale[r],
      g: scale[g],
      b: scale[b],
    };
  }

  const level = 8 + (normalized - 232) * 10;
  return { r: level, g: level, b: level };
}

function rgbDistance(a: Rgb, b: Rgb): number {
  const dr = a.r - b.r;
  const dg = a.g - b.g;
  const db = a.b - b.b;
  return dr * dr + dg * dg + db * db;
}

function nearestAnsi16Name(rgb: Rgb): string {
  let best: string = "white";
  let bestDistance = Number.POSITIVE_INFINITY;

  for (const [name, sample] of Object.entries(ANSI16_RGB)) {
    const distance = rgbDistance(rgb, sample);
    if (distance < bestDistance) {
      bestDistance = distance;
      best = name === "grey" ? "gray" : name;
    }
  }

  return best;
}

function normalizeAnsi16Name(value: string, fallback: string): string {
  const normalized = value.trim().toLowerCase();
  if (normalized in ANSI16_RGB) {
    return normalized === "grey" ? "gray" : normalized;
  }
  return fallback;
}

function clampAnsi256(value: number, fallback: number): number {
  if (!Number.isFinite(value)) {
    return fallback;
  }
  return clampNumber(Math.floor(value), 0, 255);
}

function parseRgb(value: string): Rgb | null {
  const named = NAMED_COLORS[value.trim().toLowerCase()];
  if (named) {
    return fromHex(named);
  }

  const hex = normalizeHex(value);
  if (!hex) {
    return null;
  }

  return fromHex(hex);
}

function rgbToAnsi256(rgb: Rgb): number {
  if (rgb.r === rgb.g && rgb.g === rgb.b) {
    if (rgb.r < 8) {
      return 16;
    }
    if (rgb.r > 248) {
      return 231;
    }
    return 232 + Math.round(((rgb.r - 8) / 247) * 24);
  }

  const toCube = (value: number): number => clampNumber(Math.round((value / 255) * 5), 0, 5);
  const r = toCube(rgb.r);
  const g = toCube(rgb.g);
  const b = toCube(rgb.b);
  return 16 + 36 * r + 6 * g + b;
}

function parseForcedColor(forceColor: string): ColorRuntime | null {
  const normalized = forceColor.trim();
  if (normalized.length === 0) {
    return { mode: "ansi16", enabled: true };
  }

  const value = Number.parseInt(normalized, 10);
  if (Number.isNaN(value)) {
    return { mode: "ansi16", enabled: true };
  }
  if (value <= 0) {
    return { mode: "mono", enabled: false };
  }
  if (value === 1) {
    return { mode: "ansi16", enabled: true };
  }
  if (value === 2) {
    return { mode: "ansi256", enabled: true };
  }
  return { mode: "truecolor", enabled: true };
}

function runtimeModeEnabled(mode: ColorMode): boolean {
  return mode !== "mono";
}

export function detectColorRuntime(options: DetectColorRuntimeOptions = {}): ColorRuntime {
  if (typeof options.enabled === "boolean" && !options.enabled) {
    return { mode: "mono", enabled: false };
  }

  if (options.mode) {
    return {
      mode: options.mode,
      enabled: runtimeModeEnabled(options.mode),
    };
  }

  const env = options.env ?? process.env;
  const isTTY = options.isTTY ?? process.stdout?.isTTY === true;

  if (typeof options.enabled === "boolean" && options.enabled) {
    return {
      mode: isTTY ? "ansi16" : "mono",
      enabled: isTTY,
    };
  }

  if (typeof env.NO_COLOR === "string" && env.NO_COLOR.length > 0) {
    return { mode: "mono", enabled: false };
  }

  if (typeof env.FORCE_COLOR === "string") {
    const forced = parseForcedColor(env.FORCE_COLOR);
    if (forced) {
      return forced;
    }
  }

  if (!isTTY) {
    return { mode: "mono", enabled: false };
  }

  const colorTerm = (env.COLORTERM ?? "").toLowerCase();
  if (colorTerm.includes("truecolor") || colorTerm.includes("24bit")) {
    return { mode: "truecolor", enabled: true };
  }

  const term = (env.TERM ?? "").toLowerCase();
  if (term.includes("256color")) {
    return { mode: "ansi256", enabled: true };
  }

  return { mode: "ansi16", enabled: true };
}

export function resolveLiteralColor(color: string, runtime: ColorRuntime): string | undefined {
  if (!runtime.enabled || runtime.mode === "mono") {
    return undefined;
  }

  const trimmed = color.trim();
  if (trimmed.length === 0) {
    return undefined;
  }

  if (runtime.mode === "truecolor") {
    const normalizedHex = normalizeHex(trimmed);
    if (normalizedHex) {
      return normalizedHex;
    }
    const lower = trimmed.toLowerCase();
    if (lower in NAMED_COLORS || lower in ANSI16_RGB) {
      return lower === "grey" ? "gray" : lower;
    }
    const maybeIndex = Number.parseInt(trimmed, 10);
    if (!Number.isNaN(maybeIndex)) {
      return String(clampAnsi256(maybeIndex, 0));
    }
    return trimmed;
  }

  if (runtime.mode === "ansi256") {
    const maybeIndex = Number.parseInt(trimmed, 10);
    if (!Number.isNaN(maybeIndex) && String(maybeIndex) === trimmed) {
      return String(clampAnsi256(maybeIndex, 0));
    }

    const rgb = parseRgb(trimmed);
    if (!rgb) {
      return undefined;
    }
    return String(rgbToAnsi256(rgb));
  }

  const normalizedAnsi16 = normalizeAnsi16Name(trimmed, "");
  if (normalizedAnsi16.length > 0) {
    return normalizedAnsi16;
  }

  const maybeIndex = Number.parseInt(trimmed, 10);
  if (!Number.isNaN(maybeIndex) && String(maybeIndex) === trimmed) {
    const rgb = ansi256ToRgb(clampAnsi256(maybeIndex, 0));
    return nearestAnsi16Name(rgb);
  }

  const rgb = parseRgb(trimmed);
  if (!rgb) {
    return undefined;
  }
  return nearestAnsi16Name(rgb);
}

function resolveColorValue(value: ColorValue, runtime: ColorRuntime): string | undefined {
  if (!runtime.enabled || runtime.mode === "mono") {
    return undefined;
  }

  if (runtime.mode === "truecolor") {
    return resolveLiteralColor(value.truecolor, TRUECOLOR_RUNTIME);
  }

  if (runtime.mode === "ansi256") {
    return String(clampAnsi256(value.ansi256, 0));
  }

  return resolveLiteralColor(value.ansi16, ANSI16_RUNTIME);
}

export function resolveIntentColor(
  theme: TuiTheme,
  intent: ColorIntent,
  runtime: ColorRuntime,
): string | undefined {
  return resolveColorValue(theme.colors[intent], runtime);
}

export function resolveIntentMono(theme: TuiTheme, intent: ColorIntent): ColorValue["mono"] {
  return theme.colors[intent].mono;
}

export function resolveColorReference(
  reference: string,
  theme: TuiTheme,
  runtime: ColorRuntime,
): string | undefined {
  const trimmed = reference.trim();
  if (trimmed.startsWith("$")) {
    const intent = trimmed.slice(1) as ColorIntent;
    if (!(intent in theme.colors)) {
      return undefined;
    }
    return resolveIntentColor(theme, intent, runtime);
  }
  return resolveLiteralColor(trimmed, runtime);
}

export function colorOverrideToDefaults(literal: string, fallback: ColorValue): ColorValue {
  const truecolor = resolveLiteralColor(literal, TRUECOLOR_RUNTIME) ?? fallback.truecolor;
  const ansi256Resolved = resolveLiteralColor(literal, ANSI256_RUNTIME);
  const ansi256 = clampAnsi256(
    ansi256Resolved ? Number.parseInt(ansi256Resolved, 10) : Number.NaN,
    fallback.ansi256,
  );
  const ansi16 = resolveLiteralColor(literal, ANSI16_RUNTIME) ?? fallback.ansi16;

  return {
    truecolor,
    ansi256,
    ansi16: normalizeAnsi16Name(ansi16, fallback.ansi16),
    mono: fallback.mono,
  };
}

export function normalizeColorValue(
  value: Partial<ColorValue> | string | undefined,
  fallback: ColorValue,
): ColorValue {
  if (typeof value === "string") {
    return colorOverrideToDefaults(value, fallback);
  }

  const normalizedTruecolor =
    typeof value?.truecolor === "string" && value.truecolor.trim().length > 0
      ? (resolveLiteralColor(value.truecolor, TRUECOLOR_RUNTIME) ?? fallback.truecolor)
      : fallback.truecolor;

  const normalizedAnsi16 =
    typeof value?.ansi16 === "string" && value.ansi16.trim().length > 0
      ? normalizeAnsi16Name(value.ansi16, fallback.ansi16)
      : fallback.ansi16;

  const normalizedMono =
    value?.mono === "bold" || value?.mono === "dim" || value?.mono === "normal"
      ? value.mono
      : fallback.mono;

  return {
    truecolor: normalizedTruecolor,
    ansi256: clampAnsi256(value?.ansi256 ?? Number.NaN, fallback.ansi256),
    ansi16: normalizedAnsi16,
    mono: normalizedMono,
  };
}

export function normalizeThemeColorMode(value: string | undefined): ColorMode | null {
  if (!value) {
    return null;
  }
  const normalized = value.trim().toLowerCase();
  if (normalized === "truecolor" || normalized === "ansi256" || normalized === "ansi16") {
    return normalized;
  }
  if (normalized === "mono" || normalized === "none") {
    return "mono";
  }
  return null;
}

export function resolveRgbLiteral(value: string): string | null {
  const rgb = parseRgb(value);
  if (!rgb) {
    return null;
  }
  return toHex(rgb);
}
