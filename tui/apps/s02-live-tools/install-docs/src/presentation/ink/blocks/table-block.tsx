import { horizontalRule, normalizeTextWidth } from "@levitate/tui-kit";
import { Box, Text } from "ink";
import { type RichText, type TableBlock } from "@levitate/docs-content";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { useIntentColor } from "./shared/intent-color";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { inlineContentToPlainText } from "./shared/rich-text-renderer";
import {
	RichTableRow,
	TABLE_COLUMN_SEPARATOR,
	type TableCellContent,
} from "../primitives/rich-table-row";

function tableDividerLine(columnWidths: ReadonlyArray<number>, rowWidth: number): string {
	const junction = "─┼─";
	const natural = columnWidths
		.map((width) => horizontalRule(Math.max(1, width), "─"))
		.join(junction);
	if (natural.length >= rowWidth) {
		return natural.slice(0, rowWidth);
	}
	return `${natural}${horizontalRule(rowWidth - natural.length, "─")}`;
}

function computeColumnWidths(matrix: ReadonlyArray<ReadonlyArray<string>>): number[] {
	const columnCount = matrix.reduce((max, row) => Math.max(max, row.length), 0);
	return Array.from({ length: columnCount }, (_, index) => {
		let max = 1;
		for (const row of matrix) {
			max = Math.max(max, String(row[index] ?? "").length);
		}
		return Math.min(32, max);
	});
}

export function TableBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<TableBlock>): ReactNode {
	const headers = block.headers;
	const rows = block.rows;
	const plainHeaders = headers.map((cell) => inlineContentToPlainText(cell as string | RichText));
	const plainRows = rows.map((row) =>
		row.map((cell) => inlineContentToPlainText(cell as string | RichText)),
	);
	const matrix = [plainHeaders, ...plainRows];
	const widths = computeColumnWidths(matrix);
	if (widths.length === 0) {
		return null;
	}

	const safeWidth = Math.max(1, normalizeTextWidth(contentWidth, 1));
	const tableWidth = Math.max(1, safeWidth - indent);
	const backgroundColor = useIntentColor("cardBackground");
	const dividerColor = useIntentColor("dimText");
	const separatorWidth = (widths.length - 1) * TABLE_COLUMN_SEPARATOR.length;
	const maxContentWidth = Math.max(1, tableWidth - separatorWidth);
	const normalizedWidths = (() => {
		const current = widths.reduce((sum, width) => sum + width, 0);
		if (current <= maxContentWidth) {
			return widths;
		}
		const scale = maxContentWidth / current;
		const scaled = widths.map((width) => Math.max(1, Math.floor(width * scale)));
		let remaining = maxContentWidth - scaled.reduce((sum, width) => sum + width, 0);
		let index = 0;
		while (remaining > 0) {
			scaled[index % scaled.length] += 1;
			remaining -= 1;
			index += 1;
		}
		return scaled;
	})();

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			<RichTableRow
				cells={headers as ReadonlyArray<TableCellContent>}
				columnWidths={normalizedWidths}
				rowWidth={tableWidth}
				fallbackIntent="sectionHeading"
				backgroundIntent="cardBackground"
				bold
			/>
			<Text color={dividerColor} backgroundColor={backgroundColor}>
				{tableDividerLine(normalizedWidths, tableWidth)}
			</Text>
			{rows.map((row, rowIndex) => (
				<RichTableRow
					key={`table-row-${rowIndex}`}
					cells={row as ReadonlyArray<TableCellContent>}
					columnWidths={normalizedWidths}
					rowWidth={tableWidth}
					fallbackIntent="text"
					backgroundIntent="cardBackground"
				/>
			))}
		</Box>
	);
}

export const tableBlockPlugin: BlockPlugin<"table"> = {
	type: "table",
	rendererKey: defaultDocsBlockRendererKey("table"),
	render: (block, context, indent) => (
		<TableBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, _context, _indent) => {
		const headers = block.headers.map((cell) =>
			inlineContentToPlainText(cell as string | RichText),
		);
		const rows = block.rows.map((row) =>
			row.map((cell) => inlineContentToPlainText(cell as string | RichText)),
		);
		const matrix = [headers, ...rows];
		const widths = computeColumnWidths(matrix);
		if (widths.length === 0) {
			return 0;
		}
		return 2 + rows.length;
	},
};
