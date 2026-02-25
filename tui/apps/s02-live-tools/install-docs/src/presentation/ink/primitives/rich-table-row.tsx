import type { RichText } from "@levitate/docs-content";
import { RichTextLine, type ColorIntent, type RichTextRun } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import { wrapRichTextRuns } from "../blocks/shared/rich-text-renderer";
import { padRunsToWidth, truncateRunsToWidth, withBackgroundIntent } from "./rich-text-runs";

export type TableCellContent = string | RichText;

type RichTableRowProps = {
	cells: ReadonlyArray<TableCellContent>;
	columnWidths: ReadonlyArray<number>;
	rowWidth: number;
	fallbackIntent?: ColorIntent;
	backgroundIntent?: ColorIntent;
	bold?: boolean;
};

export const TABLE_COLUMN_SEPARATOR = " │ ";

function tableCellRuns(
	cell: TableCellContent,
	columnWidth: number,
	fallbackIntent: ColorIntent,
	backgroundIntent: ColorIntent,
): RichTextRun[] {
	const wrapped = wrapRichTextRuns(cell, columnWidth, fallbackIntent, 1);
	const firstLine = wrapped[0] ?? [];
	return padRunsToWidth(
		withBackgroundIntent(firstLine, backgroundIntent),
		columnWidth,
		fallbackIntent,
		backgroundIntent,
	);
}

function rowRuns({
	cells,
	columnWidths,
	rowWidth,
	fallbackIntent,
	backgroundIntent,
	bold,
}: {
	cells: ReadonlyArray<TableCellContent>;
	columnWidths: ReadonlyArray<number>;
	rowWidth: number;
	fallbackIntent: ColorIntent;
	backgroundIntent: ColorIntent;
	bold: boolean;
}): RichTextRun[] {
	const runs: RichTextRun[] = [];
	for (const [index, columnWidth] of columnWidths.entries()) {
		const cell = tableCellRuns(
			cells[index] ?? "",
			columnWidth,
			fallbackIntent,
			backgroundIntent,
		).map((run) => (bold ? { ...run, bold: true } : run));
		runs.push(...cell);
		if (index < columnWidths.length - 1) {
			runs.push({
				text: TABLE_COLUMN_SEPARATOR,
				intent: "dimText",
				backgroundIntent,
				bold: false,
			});
		}
	}

	return truncateRunsToWidth(runs, rowWidth, fallbackIntent, backgroundIntent);
}

export function RichTableRow({
	cells,
	columnWidths,
	rowWidth,
	fallbackIntent = "text",
	backgroundIntent = "cardBackground",
	bold = false,
}: RichTableRowProps): ReactNode {
	return (
		<RichTextLine
			runs={rowRuns({
				cells,
				columnWidths,
				rowWidth,
				fallbackIntent,
				backgroundIntent,
				bold,
			})}
			fallbackIntent={fallbackIntent}
		/>
	);
}
