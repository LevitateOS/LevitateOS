import type { ChromeGlyphSet, ChromeLineWeight } from "./types";

const SINGLE_GLYPHS: ChromeGlyphSet = {
	h: "─",
	v: "│",
	tl: "┌",
	tr: "┐",
	bl: "└",
	br: "┘",
	t: "┬",
	b: "┴",
	l: "├",
	r: "┤",
	x: "┼",
};

const BOLD_GLYPHS: ChromeGlyphSet = {
	h: "━",
	v: "┃",
	tl: "┏",
	tr: "┓",
	bl: "┗",
	br: "┛",
	t: "┳",
	b: "┻",
	l: "┣",
	r: "┫",
	x: "╋",
};

const DOUBLE_GLYPHS: ChromeGlyphSet = {
	h: "═",
	v: "║",
	tl: "╔",
	tr: "╗",
	bl: "╚",
	br: "╝",
	t: "╦",
	b: "╩",
	l: "╠",
	r: "╣",
	x: "╬",
};

const ROUND_GLYPHS: ChromeGlyphSet = {
	h: "─",
	v: "│",
	tl: "╭",
	tr: "╮",
	bl: "╰",
	br: "╯",
	t: "┬",
	b: "┴",
	l: "├",
	r: "┤",
	x: "┼",
};

export function resolveChromeGlyphSet(weight: ChromeLineWeight): ChromeGlyphSet {
	if (weight === "double") {
		return DOUBLE_GLYPHS;
	}
	if (weight === "bold") {
		return BOLD_GLYPHS;
	}
	if (weight === "round") {
		return ROUND_GLYPHS;
	}
	return SINGLE_GLYPHS;
}
