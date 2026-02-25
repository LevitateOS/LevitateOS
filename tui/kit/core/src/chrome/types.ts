import type { ReactNode } from "react";
import type { ColorIntent } from "../theme";

export type ChromeLineWeight = "single" | "double" | "round" | "bold";

export type ChromeGlyphSet = {
	h: string;
	v: string;
	tl: string;
	tr: string;
	bl: string;
	br: string;
	t: string;
	b: string;
	l: string;
	r: string;
	x: string;
};

export type ChromeRect = {
	x: number;
	y: number;
	width: number;
	height: number;
};

export type ChromeInstruction =
	| {
			kind: "frame";
			rect: ChromeRect;
			intent?: ColorIntent;
	  }
	| {
			kind: "rule";
			y: number;
			x0: number;
			x1: number;
			intent?: ColorIntent;
	  }
	| {
			kind: "seam";
			y: number;
			x0: number;
			x1: number;
			intent?: ColorIntent;
	  }
	| {
			kind: "split";
			x: number;
			y0: number;
			y1: number;
			intent?: ColorIntent;
	  }
	| {
			kind: "text";
			x: number;
			y: number;
			text: string;
			intent?: ColorIntent;
			bold?: boolean;
	  };

export type ChromeRenderedSpan = {
	text: string;
	intent?: ColorIntent;
	bold?: boolean;
};

export type ChromeRenderedRow = {
	text: string;
	spans: ChromeRenderedSpan[];
};

export type ChromeLayerProps = {
	width: number;
	height: number;
	lineWeight?: ChromeLineWeight;
	glyphSet?: ChromeGlyphSet;
	instructions?: ReadonlyArray<ChromeInstruction>;
	children?: ReactNode;
	rowOffset?: number;
	rowCount?: number;
};

export type ChromeFrameProps = {
	rect: ChromeRect;
	intent?: ColorIntent;
};

export type ChromeSeamProps = {
	y: number;
	x0: number;
	x1: number;
	intent?: ColorIntent;
};

export type ChromeSplitProps = {
	x: number;
	y0: number;
	y1: number;
	intent?: ColorIntent;
};

export type ChromeRuleProps = {
	y: number;
	x0: number;
	x1: number;
	intent?: ColorIntent;
};

export type ChromeTextProps = {
	x: number;
	y: number;
	text: string;
	intent?: ColorIntent;
	bold?: boolean;
};
