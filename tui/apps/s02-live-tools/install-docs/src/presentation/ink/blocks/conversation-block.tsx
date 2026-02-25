import { Box } from "ink";
import type { ConversationBlock } from "@levitate/docs-content";
import { prefixWrappedText } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { inlineContentToPlainText } from "./shared/rich-text-renderer";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { PrefixedRichListItem } from "../primitives/prefixed-rich-list-item";

export function ConversationBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<ConversationBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const conversationWidth = Math.max(1, safeWidth - indent);

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			{block.messages.map((message, messageIndex) => {
				const roleLabel = message.role === "ai" ? "AI" : "User";
				const roleIntent = message.role === "ai" ? "info" : "accent";

				return (
					<Box key={`conversation-message-${messageIndex}`} flexDirection="column">
						<PrefixedRichListItem
							lineKeyPrefix={`conversation-text-${messageIndex}`}
							marker={`${roleLabel}:`}
							content={message.text}
							width={conversationWidth}
							markerIntent={roleIntent}
							markerBold
							contentIntent="text"
							backgroundIntent="cardBackground"
						/>
						{(message.list ?? []).map((listItem, listIndex) => (
							<Box
								key={`conversation-list-${messageIndex}-${listIndex}`}
								flexDirection="row"
								paddingLeft={2}
								width={conversationWidth}
							>
								<PrefixedRichListItem
									lineKeyPrefix={`conversation-list-line-${messageIndex}-${listIndex}`}
									marker="•"
									content={listItem}
									width={Math.max(1, conversationWidth - 2)}
									markerIntent="text"
									contentIntent="text"
									backgroundIntent="cardBackground"
								/>
							</Box>
						))}
					</Box>
				);
			})}
		</Box>
	);
}

export const conversationBlockPlugin: BlockPlugin<"conversation"> = {
	type: "conversation",
	rendererKey: defaultDocsBlockRendererKey("conversation"),
	render: (block, context, indent) => (
		<ConversationBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, context, indent) => {
		const conversationWidth = Math.max(1, context.contentWidth - indent);
		let lines = 0;

		for (const message of block.messages) {
			const roleLabel = message.role === "ai" ? "AI" : "User";
			lines += prefixWrappedText(
				`${roleLabel}:`,
				inlineContentToPlainText(message.text),
				conversationWidth,
				1,
			).length;
			for (const listItem of message.list ?? []) {
				lines += prefixWrappedText(
					"•",
					inlineContentToPlainText(listItem),
					Math.max(1, conversationWidth - 2),
					1,
				).length;
			}
		}

		return lines;
	},
};
