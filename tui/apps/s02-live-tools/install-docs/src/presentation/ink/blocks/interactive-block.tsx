import { Box } from "ink";
import type { InteractiveBlock } from "@levitate/docs-content";
import { RichTextLine } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { CommandLineSeries } from "../primitives/command-line-series";
import { withBackgroundIntent } from "../primitives/rich-text-runs";
import { wrapRichTextPlainLines, wrapRichTextRuns } from "./shared/rich-text-renderer";

export function InteractiveBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<InteractiveBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const interactiveWidth = Math.max(1, safeWidth - indent);
	const introLines = block.intro
		? wrapRichTextRuns(block.intro, interactiveWidth, "dimText", 1)
		: [];

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			{introLines.map((lineRuns, lineIndex) => (
				<RichTextLine
					key={`interactive-intro-${lineIndex}`}
					runs={withBackgroundIntent(lineRuns, "cardBackground")}
					fallbackIntent="dimText"
				/>
			))}
			{block.steps.map((step, stepIndex) => (
				<Box key={`interactive-step-${stepIndex}`} flexDirection="column">
					<CommandLineSeries
						lines={[step.command]}
						width={interactiveWidth}
						startRowIndex={stepIndex}
						bold
					/>
					{wrapRichTextRuns(step.description, interactiveWidth, "text", 1).map(
						(lineRuns, lineIndex) => (
							<RichTextLine
								key={`interactive-step-description-${stepIndex}-${lineIndex}`}
								runs={withBackgroundIntent(lineRuns, "cardBackground")}
								fallbackIntent="text"
							/>
						),
					)}
				</Box>
			))}
		</Box>
	);
}

export const interactiveBlockPlugin: BlockPlugin<"interactive"> = {
	type: "interactive",
	rendererKey: defaultDocsBlockRendererKey("interactive"),
	render: (block, context, indent) => (
		<InteractiveBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, context, indent) => {
		const interactiveWidth = Math.max(1, context.contentWidth - indent);
		let lines = 0;
		if (block.intro) {
			lines += wrapRichTextPlainLines(block.intro, interactiveWidth, "dimText", 1).length;
		}
		for (const step of block.steps) {
			lines += 1;
			lines += wrapRichTextPlainLines(step.description, interactiveWidth, "text", 1).length;
		}
		return lines;
	},
};
