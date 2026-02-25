import { Box } from "ink";
import type { TextBlock } from "@levitate/docs-content";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { wrapRichTextPlainLines } from "./shared/rich-text-renderer";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { RichParagraph } from "../primitives/rich-paragraph";

export function TextBlockView({
	block,
	contentWidth,
	indent = 0,
	selectedLinkHref,
}: BlockComponentProps<TextBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const textWidth = Math.max(1, safeWidth - indent);
	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			<RichParagraph
				content={block.content}
				width={textWidth}
				intent="text"
				minimumWidth={1}
				selectedLinkHref={selectedLinkHref}
			/>
		</Box>
	);
}

export const textBlockPlugin: BlockPlugin<"text"> = {
	type: "text",
	rendererKey: defaultDocsBlockRendererKey("text"),
	render: (block, context, indent) => (
		<TextBlockView
			block={block}
			contentWidth={context.contentWidth}
			indent={indent}
			selectedLinkHref={context.selectedLinkHref}
		/>
	),
	measure: (block, context, indent) =>
		wrapRichTextPlainLines(block.content, Math.max(1, context.contentWidth - indent), "text", 1)
			.length,
};
