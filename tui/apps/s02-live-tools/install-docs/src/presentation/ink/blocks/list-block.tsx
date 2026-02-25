import { Box } from "ink";
import { type ListBlock, type RichText } from "@levitate/docs-content";
import { prefixWrappedText } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { inlineContentToPlainText } from "./shared/rich-text-renderer";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { PrefixedRichListItem } from "../primitives/prefixed-rich-list-item";

export function ListBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<ListBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const listWidth = Math.max(1, safeWidth - indent);

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			{block.items.map((item, index) => {
				const marker = block.ordered ? `${index + 1}.` : "•";
				if (typeof item === "string" || Array.isArray(item)) {
					return (
						<Box key={`list-item-${index}`} flexDirection="column">
							<PrefixedRichListItem
								lineKeyPrefix={`list-line-${index}`}
								marker={marker}
								content={item as string | RichText}
								width={listWidth}
								markerIntent="sectionSubheading"
								contentIntent="sectionSubheading"
							/>
						</Box>
					);
				}

				return (
					<Box key={`list-item-${index}`} flexDirection="column">
						<PrefixedRichListItem
							lineKeyPrefix={`list-item-line-${index}`}
							marker={marker}
							content={item.text}
							width={listWidth}
							markerIntent="sectionSubheading"
							contentIntent="sectionSubheading"
						/>
						{(item.children ?? []).map((child, childIndex) => (
							<Box key={`list-child-${index}-${childIndex}`} paddingLeft={2}>
								<PrefixedRichListItem
									lineKeyPrefix={`list-child-line-${index}-${childIndex}`}
									marker="•"
									content={child}
									width={Math.max(1, listWidth - 2)}
									markerIntent="dimText"
									contentIntent="dimText"
								/>
							</Box>
						))}
					</Box>
				);
			})}
		</Box>
	);
}

export const listBlockPlugin: BlockPlugin<"list"> = {
	type: "list",
	rendererKey: defaultDocsBlockRendererKey("list"),
	render: (block, context, indent) => (
		<ListBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, context, indent) => {
		const listWidth = Math.max(1, context.contentWidth - indent);
		let lines = 0;

		for (const [index, item] of block.items.entries()) {
			const marker = block.ordered ? `${index + 1}.` : "•";
			if (typeof item === "string" || Array.isArray(item)) {
				lines += prefixWrappedText(
					marker,
					inlineContentToPlainText(item as string | RichText),
					listWidth,
					1,
				).length;
				continue;
			}

			lines += prefixWrappedText(marker, inlineContentToPlainText(item.text), listWidth, 1).length;
			for (const child of item.children ?? []) {
				lines += prefixWrappedText(
					"•",
					inlineContentToPlainText(child),
					Math.max(1, listWidth - 2),
					1,
				).length;
			}
		}

		return lines;
	},
};
