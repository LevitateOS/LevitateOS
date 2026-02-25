import { Box } from "ink";
import { UiText } from "@levitate/tui-kit";
import type { QABlock } from "@levitate/docs-content";
import type { ReactNode } from "react";
import type { BlockComponentProps, RenderNestedBlock } from "./types";
import type { BlockPlugin } from "./contracts";
import { RichTextRenderer } from "./shared/rich-text-renderer";
import { wrapRichTextPlainLines } from "./shared/rich-text-renderer";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";

type QABlockProps = BlockComponentProps<QABlock> & {
	renderNestedBlock: RenderNestedBlock;
};

export function QABlockView({
	block,
	contentWidth,
	indent = 0,
	renderNestedBlock,
}: QABlockProps): ReactNode {
	const safeWidth = Math.max(1, contentWidth);

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			{block.items.map((item, itemIndex) => {
				return (
					<Box key={`qa-item-${itemIndex}`} flexDirection="column">
						<UiText intent="sectionHeading" bold>
							{"Q >"}
						</UiText>
						<Box paddingLeft={2}>
							<RichTextRenderer
								content={item.question}
								defaultIntent="sectionHeading"
								width={Math.max(1, safeWidth - 2)}
								minimumWidth={1}
							/>
						</Box>
						<UiText intent="sectionSubheading" bold>
							{"A >"}
						</UiText>
						{item.answer.map((answerBlock, answerIndex) => (
							<Box key={`qa-answer-${itemIndex}-${answerIndex}`} flexDirection="column">
								{renderNestedBlock(answerBlock, 2)}
							</Box>
						))}
						{item.answer.length === 0 ? (
							<Box paddingLeft={2}>
								<UiText intent="dimText">(no answer provided)</UiText>
							</Box>
						) : null}
						{itemIndex < block.items.length - 1 ? <UiText intent="text"> </UiText> : null}
					</Box>
				);
			})}
		</Box>
	);
}

export const qaBlockPlugin: BlockPlugin<"qa"> = {
	type: "qa",
	rendererKey: defaultDocsBlockRendererKey("qa"),
	render: (block, context, indent) => (
		<QABlockView
			block={block}
			contentWidth={context.contentWidth}
			indent={indent}
			renderNestedBlock={(nestedBlock, nestedIndent) =>
				context.renderBlock(nestedBlock, nestedIndent)
			}
		/>
	),
	measure: (block, context, _indent) => {
		const safeWidth = Math.max(1, context.contentWidth);
		let lines = 0;

		for (const [itemIndex, item] of block.items.entries()) {
			lines += 1;
			lines += wrapRichTextPlainLines(item.question, Math.max(1, safeWidth - 2), "text", 1).length;
			lines += 1;

			if (item.answer.length === 0) {
				lines += 1;
			} else {
				for (const answerBlock of item.answer) {
					lines += context.measureBlock(answerBlock, 2);
				}
			}

			if (itemIndex < block.items.length - 1) {
				lines += 1;
			}
		}

		return lines;
	},
};
