import { Text } from "ink";
import { clampNumber, toPositiveInt } from "../../utils/clamp";

export type PagerWindow = {
	lines: string[];
	page: number;
	totalPages: number;
};

export function computePagerWindow(
	lines: ReadonlyArray<string>,
	page: number,
	pageSize: number,
): PagerWindow {
	const safePageSize = Math.max(1, toPositiveInt(pageSize, 1));
	const totalPages = Math.max(1, Math.ceil(lines.length / safePageSize));
	const safePage = clampNumber(page, 0, totalPages - 1);
	const start = safePage * safePageSize;

	return {
		lines: lines.slice(start, start + safePageSize),
		page: safePage,
		totalPages,
	};
}

export type PagerProps = {
	lines: ReadonlyArray<string>;
	page: number;
	pageSize: number;
};

export function Pager({ lines, page, pageSize }: PagerProps) {
	const windowed = computePagerWindow(lines, page, pageSize);
	return <Text>{windowed.lines.join("\n")}</Text>;
}
