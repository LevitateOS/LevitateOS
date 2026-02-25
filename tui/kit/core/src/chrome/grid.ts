import type { ColorIntent } from "../theme";
import type { ChromeGlyphSet, ChromeRect, ChromeRenderedRow, ChromeRenderedSpan } from "./types";

const MASK_NORTH = 1;
const MASK_EAST = 2;
const MASK_SOUTH = 4;
const MASK_WEST = 8;

type ChromeCell = {
	strokeMask: number;
	strokeIntent?: ColorIntent;
	text?: string;
	textIntent?: ColorIntent;
	textBold: boolean;
};

function normalizeCoordinate(value: number): number {
	if (!Number.isFinite(value)) {
		return 0;
	}
	return Math.floor(value);
}

function clamp(value: number, min: number, max: number): number {
	return Math.max(min, Math.min(max, value));
}

function normalizeRange(a: number, b: number, max: number): [number, number] | null {
	const start = clamp(Math.min(normalizeCoordinate(a), normalizeCoordinate(b)), 0, max);
	const end = clamp(Math.max(normalizeCoordinate(a), normalizeCoordinate(b)), 0, max);
	if (start > end) {
		return null;
	}
	return [start, end];
}

function maskToGlyph(mask: number, glyphs: ChromeGlyphSet): string {
	if (mask === 0) {
		return " ";
	}

	const north = (mask & MASK_NORTH) !== 0;
	const east = (mask & MASK_EAST) !== 0;
	const south = (mask & MASK_SOUTH) !== 0;
	const west = (mask & MASK_WEST) !== 0;

	if (north && east && south && west) {
		return glyphs.x;
	}
	if (north && east && south) {
		return glyphs.l;
	}
	if (north && south && west) {
		return glyphs.r;
	}
	if (east && south && west) {
		return glyphs.t;
	}
	if (north && east && west) {
		return glyphs.b;
	}
	if (east && south) {
		return glyphs.tl;
	}
	if (south && west) {
		return glyphs.tr;
	}
	if (east && north) {
		return glyphs.bl;
	}
	if (north && west) {
		return glyphs.br;
	}
	if (east || west) {
		return glyphs.h;
	}
	if (north || south) {
		return glyphs.v;
	}

	return " ";
}

export class ChromeGrid {
	readonly width: number;
	readonly height: number;
	readonly cells: ChromeCell[];

	constructor(width: number, height: number) {
		this.width = Math.max(1, normalizeCoordinate(width));
		this.height = Math.max(1, normalizeCoordinate(height));
		this.cells = Array.from({ length: this.width * this.height }, () => ({
			strokeMask: 0,
			text: undefined,
			textIntent: undefined,
			textBold: false,
		}));
	}

	private inBounds(x: number, y: number): boolean {
		return x >= 0 && x < this.width && y >= 0 && y < this.height;
	}

	private index(x: number, y: number): number {
		return y * this.width + x;
	}

	private cellAt(x: number, y: number): ChromeCell | null {
		if (!this.inBounds(x, y)) {
			return null;
		}
		return this.cells[this.index(x, y)] ?? null;
	}

	private addMask(x: number, y: number, mask: number, intent?: ColorIntent): void {
		if (!this.inBounds(x, y) || mask === 0) {
			return;
		}
		const cell = this.cells[this.index(x, y)]!;
		cell.strokeMask |= mask;
		if (intent) {
			cell.strokeIntent = intent;
		}
	}

	drawHorizontal(y: number, x0: number, x1: number, intent?: ColorIntent): this {
		const row = normalizeCoordinate(y);
		if (row < 0 || row >= this.height) {
			return this;
		}
		const range = normalizeRange(x0, x1, this.width - 1);
		if (!range) {
			return this;
		}
		const [start, end] = range;
		for (let x = start; x <= end; x += 1) {
			let mask = 0;
			if (start === end) {
				mask = MASK_EAST | MASK_WEST;
			} else {
				if (x > start) {
					mask |= MASK_WEST;
				}
				if (x < end) {
					mask |= MASK_EAST;
				}
			}
			this.addMask(x, row, mask, intent);
		}
		return this;
	}

	drawVertical(x: number, y0: number, y1: number, intent?: ColorIntent): this {
		const column = normalizeCoordinate(x);
		if (column < 0 || column >= this.width) {
			return this;
		}
		const range = normalizeRange(y0, y1, this.height - 1);
		if (!range) {
			return this;
		}
		const [start, end] = range;
		for (let y = start; y <= end; y += 1) {
			let mask = 0;
			if (start === end) {
				mask = MASK_NORTH | MASK_SOUTH;
			} else {
				if (y > start) {
					mask |= MASK_NORTH;
				}
				if (y < end) {
					mask |= MASK_SOUTH;
				}
			}
			this.addMask(column, y, mask, intent);
		}
		return this;
	}

	drawRect(rect: ChromeRect, intent?: ColorIntent): this {
		const width = Math.max(1, normalizeCoordinate(rect.width));
		const height = Math.max(1, normalizeCoordinate(rect.height));
		const x0 = normalizeCoordinate(rect.x);
		const y0 = normalizeCoordinate(rect.y);
		const x1 = x0 + width - 1;
		const y1 = y0 + height - 1;

		this.drawHorizontal(y0, x0, x1, intent);
		this.drawHorizontal(y1, x0, x1, intent);
		this.drawVertical(x0, y0, y1, intent);
		this.drawVertical(x1, y0, y1, intent);
		return this;
	}

	drawSeam(y: number, x0: number, x1: number, intent?: ColorIntent): this {
		return this.drawHorizontal(y, x0, x1, intent);
	}

	drawSplit(x: number, y0: number, y1: number, intent?: ColorIntent): this {
		return this.drawVertical(x, y0, y1, intent);
	}

	drawText(x: number, y: number, text: string, intent?: ColorIntent, bold = false): this {
		const row = normalizeCoordinate(y);
		if (row < 0 || row >= this.height) {
			return this;
		}
		let cursor = normalizeCoordinate(x);
		for (const char of Array.from(text)) {
			if (cursor >= this.width) {
				break;
			}
			if (cursor >= 0) {
				const cell = this.cellAt(cursor, row);
				if (cell) {
					cell.text = char;
					cell.textIntent = intent;
					cell.textBold = Boolean(bold);
				}
			}
			cursor += 1;
		}
		return this;
	}

	renderRows(glyphs: ChromeGlyphSet): ChromeRenderedRow[] {
		const rows: ChromeRenderedRow[] = [];

		for (let y = 0; y < this.height; y += 1) {
			let rowText = "";
			const spans: ChromeRenderedSpan[] = [];
			let currentText = "";
			let currentIntent: ColorIntent | undefined;
			let currentBold = false;
			let hasCurrent = false;

			for (let x = 0; x < this.width; x += 1) {
				const cell = this.cells[this.index(x, y)]!;
				const char =
					typeof cell.text === "string" ? cell.text : maskToGlyph(cell.strokeMask, glyphs);
				const intent = typeof cell.text === "string" ? cell.textIntent : cell.strokeIntent;
				const bold = typeof cell.text === "string" ? cell.textBold : false;

				rowText += char;

				if (!hasCurrent) {
					currentText = char;
					currentIntent = intent;
					currentBold = bold;
					hasCurrent = true;
					continue;
				}

				if (currentIntent === intent && currentBold === bold) {
					currentText += char;
					continue;
				}

				spans.push({ text: currentText, intent: currentIntent, bold: currentBold });
				currentText = char;
				currentIntent = intent;
				currentBold = bold;
			}

			if (hasCurrent) {
				spans.push({ text: currentText, intent: currentIntent, bold: currentBold });
			}

			rows.push({ text: rowText, spans });
		}

		return rows;
	}
}

export function createChromeGrid(width: number, height: number): ChromeGrid {
	return new ChromeGrid(width, height);
}
