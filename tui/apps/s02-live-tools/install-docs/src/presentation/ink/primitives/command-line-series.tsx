import { Box } from "ink";
import type { ReactNode } from "react";
import { CommandLineRow } from "./command-line-row";

type CommandLineSeriesProps = {
	lines: ReadonlyArray<string>;
	width: number;
	startRowIndex?: number;
	firstPrefix?: string;
	continuationPrefix?: string;
	bold?: boolean;
};

export function CommandLineSeries({
	lines,
	width,
	startRowIndex = 0,
	firstPrefix = "$ ",
	continuationPrefix = "  ",
	bold = true,
}: CommandLineSeriesProps): ReactNode {
	return (
		<Box flexDirection="column">
			{lines.map((line, index) => (
				<CommandLineRow
					key={`command-series-${startRowIndex + index}`}
					line={line}
					rowIndex={startRowIndex + index}
					prefix={index === 0 ? firstPrefix : continuationPrefix}
					width={width}
					bold={bold}
				/>
			))}
		</Box>
	);
}
