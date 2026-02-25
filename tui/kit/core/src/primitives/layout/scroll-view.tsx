import { Text } from "ink";
import { useMemo } from "react";
import { clampNumber, toPositiveInt } from "../../utils/clamp";
import { splitLines } from "../../utils/strings";

export type ScrollViewProps = {
	content: string | ReadonlyArray<string>;
	offset: number;
	height: number;
};

export type ScrollWindow = {
	lines: string[];
	start: number;
	end: number;
	maxOffset: number;
};

export function computeScrollWindow(
	content: string | ReadonlyArray<string>,
	offset: number,
	height: number,
): ScrollWindow {
	const lines = splitLines(content);
	const visibleRows = Math.max(1, toPositiveInt(height, 1));
	const maxOffset = Math.max(0, lines.length - visibleRows);
	const clampedOffset = clampNumber(offset, 0, maxOffset);

	const start = lines.length === 0 ? 0 : clampedOffset + 1;
	const end = lines.length === 0 ? 0 : Math.min(lines.length, clampedOffset + visibleRows);

	return {
		lines: lines.slice(clampedOffset, clampedOffset + visibleRows),
		start,
		end,
		maxOffset,
	};
}

export function ScrollView({ content, offset, height }: ScrollViewProps) {
	const windowed = useMemo(
		() => computeScrollWindow(content, offset, height),
		[content, height, offset],
	);

	return <Text>{windowed.lines.join("\n")}</Text>;
}
