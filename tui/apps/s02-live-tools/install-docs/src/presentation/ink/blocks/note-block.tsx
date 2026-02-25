import { Box, Text } from "ink";
import type { NoteBlock } from "@levitate/docs-content";
import { RichTextLine, type ColorIntent } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { useIntentColor } from "./shared/intent-color";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import {
	RichTextRenderer,
	wrapRichTextPlainLines,
	wrapRichTextRuns,
} from "./shared/rich-text-renderer";
import { AdmonitionFrame } from "../primitives/admonition-frame";
import { padRunsToWidth } from "../primitives/rich-text-runs";

type NotePalette = {
	label: string;
	labelColor: string;
	backgroundColor: string;
	borderColor: string;
};

function notePalette(variant: NoteBlock["variant"]): NotePalette {
	if (variant === "danger") {
		return {
			label: "DANGER",
			labelColor: "#ffd0d0",
			backgroundColor: "#371010",
			borderColor: "#ff7d7d",
		};
	}
	if (variant === "warning") {
		return {
			label: "WARNING",
			labelColor: "#ffe8a6",
			backgroundColor: "#47310b",
			borderColor: "#ffd36e",
		};
	}
	return {
		label: "INFO",
		labelColor: "#b6ffe7",
		backgroundColor: "#0d2b22",
		borderColor: "#6decc9",
	};
}

function noteContentIntent(variant: NoteBlock["variant"]): ColorIntent {
	if (variant === "warning" || variant === "danger") {
		return "warning";
	}
	return "info";
}

export function NoteBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<NoteBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const noteWidth = Math.max(1, safeWidth - indent);
	const palette = notePalette(block.variant);
	const contentIntent = noteContentIntent(block.variant);
	const fallbackCardBackground = useIntentColor("cardBackground");
	const backgroundColor = palette.backgroundColor || fallbackCardBackground;

	if (noteWidth < 4) {
		return (
			<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
				<Text backgroundColor={backgroundColor}>
					<Text color={palette.labelColor} backgroundColor={backgroundColor} bold>
						{`${palette.label}: `}
					</Text>
					<RichTextRenderer content={block.content} defaultIntent={contentIntent} />
				</Text>
			</Box>
		);
	}
	const noteTextWidth = Math.max(1, noteWidth - 4);
	const noteLines = wrapRichTextRuns(block.content, noteTextWidth, contentIntent, 1);

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			<AdmonitionFrame
				width={noteWidth}
				label={palette.label}
				borderColor={palette.borderColor}
				labelColor={palette.labelColor}
				backgroundColor={backgroundColor}
				rows={noteLines.map((lineRuns, index) => (
					<RichTextLine
						key={`note-line-${index}`}
						runs={padRunsToWidth(lineRuns, noteTextWidth, contentIntent)}
						fallbackIntent={contentIntent}
					/>
				))}
			/>
		</Box>
	);
}

export const noteBlockPlugin: BlockPlugin<"note"> = {
	type: "note",
	rendererKey: defaultDocsBlockRendererKey("note"),
	render: (block, context, indent) => (
		<NoteBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, context, indent) => {
		const noteWidth = Math.max(1, context.contentWidth - indent);
		if (noteWidth < 4) {
			return 1;
		}
		const noteTextWidth = Math.max(1, noteWidth - 4);
		const noteLines = wrapRichTextPlainLines(block.content, noteTextWidth, "text", 1).length;
		return noteLines + 3;
	},
};
