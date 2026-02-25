import { Box, Text } from "ink";
import type { ReactNode } from "react";
import type { FrameCanvasProps } from "./types";

function renderNode(value: ReactNode, color?: string, bold = false): ReactNode {
	if (typeof value === "string") {
		return (
			<Text color={color} bold={bold}>
				{value}
			</Text>
		);
	}
	return value;
}

export function FrameCanvas({
	frameColumns,
	frameRows,
	headerRows,
	footerRows,
	bodyRows,
	leftWidth,
	rightWidth,
	gutterColumns,
	showHeader,
	header,
	footer,
	headerBackground,
	footerBackground,
	footerTextColor,
	leftPane,
	rightPane,
}: FrameCanvasProps) {
	return (
		<Box width={frameColumns} height={frameRows} flexDirection="column">
			{showHeader ? (
				<Box paddingX={1} height={headerRows} flexShrink={0} backgroundColor={headerBackground}>
					{header}
				</Box>
			) : null}

			<Box flexDirection="row" height={bodyRows} flexShrink={0}>
				<Box width={leftWidth} flexShrink={0} flexGrow={0}>
					{leftPane}
				</Box>
				{gutterColumns > 0 ? <Box width={gutterColumns} flexShrink={0} /> : null}
				<Box width={rightWidth} flexGrow={1}>
					{rightPane}
				</Box>
			</Box>

			{footer ? (
				<Box paddingX={1} height={footerRows} flexShrink={0} backgroundColor={footerBackground}>
					{renderNode(footer, footerTextColor)}
				</Box>
			) : null}
		</Box>
	);
}
