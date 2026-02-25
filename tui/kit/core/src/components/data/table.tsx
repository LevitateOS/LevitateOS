import { Text } from "ink";
import { padRight, truncateLine } from "../../utils/strings";

export type TableProps = {
	headers: ReadonlyArray<string>;
	rows: ReadonlyArray<ReadonlyArray<string>>;
	maxWidth?: number;
};

export function renderTable(
	headers: ReadonlyArray<string>,
	rows: ReadonlyArray<ReadonlyArray<string>>,
	maxWidth = 120,
): string {
	const sourceRows = [Array.from(headers), ...rows.map((row) => Array.from(row))];
	const columnCount = sourceRows.reduce((count, row) => Math.max(count, row.length), 0);
	if (columnCount === 0) {
		return "";
	}

	const widths = Array.from({ length: columnCount }, (_, index) => {
		let width = 1;
		for (const row of sourceRows) {
			width = Math.max(width, String(row[index] ?? "").length);
		}
		return Math.min(40, width);
	});

	const renderRow = (row: ReadonlyArray<string>): string =>
		widths
			.map((width, index) => {
				const value = String(row[index] ?? "");
				return padRight(truncateLine(value, width), width);
			})
			.join(" | ");

	const lines = [renderRow(headers), widths.map((width) => "-".repeat(width)).join("-+-")];
	for (const row of rows) {
		lines.push(renderRow(row));
	}

	return lines.map((line) => truncateLine(line, maxWidth)).join("\n");
}

export function Table({ headers, rows, maxWidth }: TableProps) {
	return <Text>{renderTable(headers, rows, maxWidth)}</Text>;
}
