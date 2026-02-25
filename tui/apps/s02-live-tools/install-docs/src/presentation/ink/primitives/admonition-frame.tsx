import { Box, Text } from "ink";
import { padRight, resolveChromeGlyphSet, truncateLine } from "@levitate/tui-kit";
import type { ReactNode } from "react";

type AdmonitionFrameProps = {
	width: number;
	label: string;
	borderColor?: string;
	labelColor?: string;
	backgroundColor?: string;
	rows: ReadonlyArray<ReactNode>;
};

export function AdmonitionFrame({
	width,
	label,
	borderColor,
	labelColor,
	backgroundColor,
	rows,
}: AdmonitionFrameProps): ReactNode {
	const safeWidth = Math.max(1, width);
	if (safeWidth < 4) {
		return (
			<Text color={labelColor} backgroundColor={backgroundColor} bold>
				{label}
			</Text>
		);
	}

	const chrome = resolveChromeGlyphSet("round");
	const horizontalWidth = Math.max(0, safeWidth - 2);
	const innerWidth = safeWidth - 2;
	const contentWidth = Math.max(1, innerWidth - 2);
	const topLine = `${chrome.tl}${chrome.h.repeat(horizontalWidth)}${chrome.tr}`;
	const bottomLine = `${chrome.bl}${chrome.h.repeat(horizontalWidth)}${chrome.br}`;
	const rowLeft = `${chrome.v} `;
	const rowRight = ` ${chrome.v}`;
	const labelLine = padRight(truncateLine(label, contentWidth), contentWidth);

	return (
		<Box flexDirection="column" width={safeWidth}>
			<Text color={borderColor} backgroundColor={backgroundColor}>
				{topLine}
			</Text>
			<Text backgroundColor={backgroundColor}>
				<Text color={borderColor} backgroundColor={backgroundColor}>
					{rowLeft}
				</Text>
				<Text color={labelColor} backgroundColor={backgroundColor} bold>
					{labelLine}
				</Text>
				<Text color={borderColor} backgroundColor={backgroundColor}>
					{rowRight}
				</Text>
			</Text>
			{rows.map((row, index) => (
				<Text key={`admonition-row-${index}`} backgroundColor={backgroundColor}>
					<Text color={borderColor} backgroundColor={backgroundColor}>
						{rowLeft}
					</Text>
					{row}
					<Text color={borderColor} backgroundColor={backgroundColor}>
						{rowRight}
					</Text>
				</Text>
			))}
			<Text color={borderColor} backgroundColor={backgroundColor}>
				{bottomLine}
			</Text>
		</Box>
	);
}
