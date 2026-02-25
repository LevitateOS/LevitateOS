import { Box, Text } from "ink";
import type { RichText } from "@levitate/docs-content";
import { RichTextLine, type ColorIntent } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import { wrapRichTextRuns } from "../blocks/shared/rich-text-renderer";
import { withBackgroundIntent } from "./rich-text-runs";

type PrefixedRichListItemProps = {
	marker: string;
	content: string | RichText;
	width: number;
	markerIntent?: ColorIntent;
	backgroundIntent?: ColorIntent;
	markerBold?: boolean;
	contentIntent?: ColorIntent;
	lineKeyPrefix?: string;
};

export function PrefixedRichListItem({
	marker,
	content,
	width,
	markerIntent = "text",
	backgroundIntent,
	markerBold = false,
	contentIntent = "text",
	lineKeyPrefix = "list-item",
}: PrefixedRichListItemProps): ReactNode {
	const safeWidth = Math.max(1, width);
	const prefix = `${marker} `;
	const textWidth = Math.max(1, safeWidth - prefix.length);
	const wrappedLines = wrapRichTextRuns(content, textWidth, contentIntent, 1).map((lineRuns) =>
		backgroundIntent ? withBackgroundIntent(lineRuns, backgroundIntent) : lineRuns,
	);

	return (
		<Box flexDirection="row" width={safeWidth}>
			<Text color={markerIntent} bold={markerBold}>
				{prefix}
			</Text>
			<Box flexDirection="column" width={textWidth}>
				{wrappedLines.map((lineRuns, lineIndex) => (
					<RichTextLine
						key={`${lineKeyPrefix}-${lineIndex}`}
						runs={lineRuns}
						fallbackIntent={contentIntent}
					/>
				))}
			</Box>
		</Box>
	);
}
