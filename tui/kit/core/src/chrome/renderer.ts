import { createChromeGrid } from "./grid";
import type { ChromeGlyphSet, ChromeInstruction, ChromeRenderedRow } from "./types";

export function applyChromeInstruction(
	grid: ReturnType<typeof createChromeGrid>,
	instruction: ChromeInstruction,
): void {
	if (instruction.kind === "frame") {
		grid.drawRect(instruction.rect, instruction.intent);
		return;
	}

	if (instruction.kind === "rule") {
		grid.drawHorizontal(instruction.y, instruction.x0, instruction.x1, instruction.intent);
		return;
	}

	if (instruction.kind === "seam") {
		grid.drawSeam(instruction.y, instruction.x0, instruction.x1, instruction.intent);
		return;
	}

	if (instruction.kind === "split") {
		grid.drawSplit(instruction.x, instruction.y0, instruction.y1, instruction.intent);
		return;
	}

	grid.drawText(
		instruction.x,
		instruction.y,
		instruction.text,
		instruction.intent,
		instruction.bold,
	);
}

export function renderChromeRows(options: {
	width: number;
	height: number;
	glyphSet: ChromeGlyphSet;
	instructions: ReadonlyArray<ChromeInstruction>;
}): ChromeRenderedRow[] {
	const grid = createChromeGrid(options.width, options.height);
	for (const instruction of options.instructions) {
		applyChromeInstruction(grid, instruction);
	}
	return grid.renderRows(options.glyphSet);
}

export function renderChromeRowStrings(options: {
	width: number;
	height: number;
	glyphSet: ChromeGlyphSet;
	instructions: ReadonlyArray<ChromeInstruction>;
}): string[] {
	return renderChromeRows(options).map((row) => row.text);
}
