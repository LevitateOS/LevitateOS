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
	| "background"
	| "sidebarBackground"
	| "contentBackground"
	| "headerBackground"
	| "headerAccentBackground"
	| "footerBackground"
	| "sidebarSectionText"
	| "sidebarItemText"
	| "sidebarItemActiveText"
	| "sidebarItemActiveBackground"
	| "sidebarBorder"
	| "footerSeparator"
	| "commandBarBackground"
	| "commandPrompt"
	| "warningBackground"
	| "sectionHeading"
	| "sectionSubheading"
	| "cardBorder"
	| "cardBackground";

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

export type ThemeChrome = {
	borderGlyphSet: "single" | "double" | "round" | "bold";
	titleStyle: "slot" | "notched" | "plain";
	panePaddingX: number;
	panePaddingY: number;
	framePaneGap: number;
	sidebarHeaderMode: "current-section-title" | "all-section-headers";
};

export type TuiTheme = {
	colors: ThemeColors;
	layout: ThemeLayout;
	chrome: ThemeChrome;
};

export const defaultThemeColors: ThemeColors = {
	border: {
		truecolor: palette.white,
		ansi256: 255,
		ansi16: "white",
		mono: "normal",
	},
	text: {
		truecolor: "#e5e7eb",
		ansi256: 15,
		ansi16: "white",
		mono: "normal",
	},
	dimText: {
		truecolor: "#94a3b8",
		ansi256: 248,
		ansi16: "gray",
		mono: "normal",
	},
	accent: {
		truecolor: "#7dd3fc",
		ansi256: 117,
		ansi16: "cyan",
		mono: "bold",
	},
	info: {
		truecolor: "#93c5fd",
		ansi256: 111,
		ansi16: "blue",
		mono: "bold",
	},
	warning: {
		truecolor: "#facc15",
		ansi256: 220,
		ansi16: "yellow",
		mono: "bold",
	},
	error: {
		truecolor: "#f87171",
		ansi256: 203,
		ansi16: "red",
		mono: "bold",
	},
	success: {
		truecolor: "#86efac",
		ansi256: 114,
		ansi16: "green",
		mono: "bold",
	},
	background: {
		truecolor: palette.zinc950,
		ansi256: 232,
		ansi16: "black",
		mono: "normal",
	},
	sidebarBackground: {
		truecolor: palette.blue900,
		ansi256: 18,
		ansi16: "blue",
		mono: "normal",
	},
	contentBackground: {
		truecolor: palette.zinc900,
		ansi256: 234,
		ansi16: "black",
		mono: "normal",
	},
	headerBackground: {
		truecolor: palette.slate800,
		ansi256: 237,
		ansi16: "gray",
		mono: "normal",
	},
	headerAccentBackground: {
		truecolor: palette.slate600,
		ansi256: 60,
		ansi16: "blue",
		mono: "bold",
	},
	footerBackground: {
		truecolor: palette.slate900,
		ansi256: 233,
		ansi16: "black",
		mono: "normal",
	},
	sidebarSectionText: {
		truecolor: "#dbeafe",
		ansi256: 189,
		ansi16: "white",
		mono: "bold",
	},
	sidebarItemText: {
		truecolor: "#bfdbfe",
		ansi256: 153,
		ansi16: "white",
		mono: "normal",
	},
	sidebarItemActiveText: {
		truecolor: "#0f172a",
		ansi256: 17,
		ansi16: "black",
		mono: "bold",
	},
	sidebarItemActiveBackground: {
		truecolor: "#cbd5e1",
		ansi256: 251,
		ansi16: "white",
		mono: "bold",
	},
	sidebarBorder: {
		truecolor: palette.white,
		ansi256: 255,
		ansi16: "white",
		mono: "normal",
	},
	footerSeparator: {
		truecolor: palette.gray500,
		ansi256: 242,
		ansi16: "gray",
		mono: "dim",
	},
	commandBarBackground: {
		truecolor: palette.zinc800,
		ansi256: 238,
		ansi16: "gray",
		mono: "normal",
	},
	commandPrompt: {
		truecolor: "#fef08a",
		ansi256: 229,
		ansi16: "yellow",
		mono: "bold",
	},
	warningBackground: {
		truecolor: palette.amber800,
		ansi256: 94,
		ansi16: "yellow",
		mono: "normal",
	},
	sectionHeading: {
		truecolor: "#e2e8f0",
		ansi256: 255,
		ansi16: "white",
		mono: "bold",
	},
	sectionSubheading: {
		truecolor: "#7dd3fc",
		ansi256: 117,
		ansi16: "cyan",
		mono: "bold",
	},
	cardBorder: {
		truecolor: "#64748b",
		ansi256: 244,
		ansi16: "gray",
		mono: "normal",
	},
	cardBackground: {
		truecolor: palette.zinc800,
		ansi256: 238,
		ansi16: "gray",
		mono: "normal",
	},
};

export const defaultThemeLayout: ThemeLayout = {
	sidebarWidth: 30,
	headerHeight: 2,
	footerHeight: 2,
	minColumns: 80,
	minRows: 24,
};

export const defaultThemeChrome: ThemeChrome = {
	borderGlyphSet: "single",
	titleStyle: "notched",
	panePaddingX: 1,
	panePaddingY: 0,
	framePaneGap: 1,
	sidebarHeaderMode: "current-section-title",
};

export const defaultTuiTheme: TuiTheme = {
	colors: defaultThemeColors,
	layout: defaultThemeLayout,
	chrome: defaultThemeChrome,
};
