import { toNonNegativeInt, toPositiveInt } from "./clamp";

function normalizeWidth(width: number): number {
	return toNonNegativeInt(width, 0);
}

export function splitLines(value: string | ReadonlyArray<string>): string[] {
	if (typeof value !== "string") {
		return value.map((line) => String(line));
	}

	return value.split("\n");
}

export function padRight(text: string, width: number): string {
	const safeWidth = normalizeWidth(width);
	if (safeWidth <= 0) {
		return "";
	}

	if (text.length >= safeWidth) {
		return text.slice(0, safeWidth);
	}

	return text + " ".repeat(safeWidth - text.length);
}

export function truncateLine(text: string, width: number): string {
	const safeWidth = normalizeWidth(width);
	if (safeWidth <= 0) {
		return "";
	}

	if (text.length <= safeWidth) {
		return text;
	}

	if (safeWidth === 1) {
		return "…";
	}

	return `${text.slice(0, safeWidth - 1)}…`;
}

export function horizontalRule(width: number, char = "-"): string {
	const safeWidth = normalizeWidth(width);
	if (safeWidth <= 0) {
		return "";
	}
	const fill = typeof char === "string" && char.length > 0 ? char[0] : "-";
	return fill.repeat(safeWidth);
}

function chunkWord(word: string, width: number): string[] {
	if (word.length <= width) {
		return [word];
	}

	const chunks: string[] = [];
	for (let index = 0; index < word.length; index += width) {
		chunks.push(word.slice(index, index + width));
	}
	return chunks;
}

export function wrapText(text: string, width: number): string[] {
	const safeWidth = normalizeWidth(width);
	if (safeWidth <= 0) {
		return [text];
	}

	const output: string[] = [];

	for (const line of text.split("\n")) {
		const words = line.split(/\s+/).filter((word) => word.length > 0);

		if (words.length === 0) {
			output.push("");
			continue;
		}

		let current = "";
		for (const word of words) {
			const wordChunks = chunkWord(word, safeWidth);

			for (const chunk of wordChunks) {
				if (current.length === 0) {
					current = chunk;
					continue;
				}

				const candidate = `${current} ${chunk}`;
				if (candidate.length <= safeWidth) {
					current = candidate;
				} else {
					output.push(current);
					current = chunk;
				}
			}
		}

		if (current.length > 0) {
			output.push(current);
		}
	}

	return output.length > 0 ? output : [""];
}

export function normalizeTextWidth(width: number, minimum = 20): number {
	const safeMinimum = Math.max(1, toPositiveInt(minimum, 20));
	return Math.max(safeMinimum, toPositiveInt(width, safeMinimum));
}

export function truncateBoundedLine(text: string, width: number, minimum = 20): string {
	const safeWidth = normalizeTextWidth(width, minimum);
	return truncateLine(text, safeWidth);
}

export function wrapBoundedText(text: string, width: number, minimum = 20): string[] {
	const safeWidth = normalizeTextWidth(width, minimum);
	return wrapText(text, safeWidth).map((line) => truncateLine(line, safeWidth));
}

export function prefixWrappedText(
	prefix: string,
	text: string,
	width: number,
	minimum = 20,
): string[] {
	const safeWidth = normalizeTextWidth(width, minimum);
	const prefixText = prefix.length > 0 ? `${prefix} ` : "";
	const contentWidth = Math.max(1, safeWidth - prefixText.length);
	const wrapped = wrapText(text, contentWidth);

	if (prefixText.length === 0) {
		return wrapped.map((line) => truncateLine(line, safeWidth));
	}

	const indent = " ".repeat(prefixText.length);
	return wrapped.map((line, index) =>
		truncateLine(index === 0 ? `${prefixText}${line}` : `${indent}${line}`, safeWidth),
	);
}
