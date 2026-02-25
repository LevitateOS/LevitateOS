import type { ContentBlock, RichText } from "@levitate/docs-content";
import { normalizeTextWidth, wrapBoundedText } from "@levitate/tui-kit";
import type { DocRenderItem } from "../../domain/render/types";
import { DEFAULT_INSTALL_BLOCK_PLUGINS } from "../../presentation/ink/blocks/plugins";

function inlineNodeToPlain(node: RichText[number]): string {
	if (typeof node === "string") {
		return node;
	}
	if (node.type === "link") {
		if (node.href.trim().length > 0 && node.href !== node.text) {
			return `${node.text} (${node.href})`;
		}
		return node.text;
	}
	return node.text;
}

function inlineContentToPlainText(content: string | RichText | undefined): string {
	if (typeof content === "string") {
		return content;
	}
	if (!Array.isArray(content)) {
		return "";
	}
	return content.map((node) => inlineNodeToPlain(node)).join("");
}

function countInlineLines(content: string | RichText | undefined, width: number): number {
	return wrapBoundedText(inlineContentToPlainText(content), Math.max(1, width), 1).length;
}

function blockMeasureError(blockType: string): never {
	throw new Error(
		`docs.render missing block measure plugin for '${blockType}'. Remediation: register the block plugin in DEFAULT_INSTALL_BLOCK_PLUGINS.`,
	);
}

export function measureContentBlockLines(
	block: ContentBlock,
	contentWidth: number,
	indent = 0,
): number {
	const safeWidth = Math.max(1, normalizeTextWidth(contentWidth, 1));
	const safeIndent =
		typeof indent === "number" && Number.isFinite(indent) ? Math.max(0, Math.trunc(indent)) : 0;
	const plugin = DEFAULT_INSTALL_BLOCK_PLUGINS[block.type];
	if (!plugin) {
		blockMeasureError(String(block.type));
	}

	return Math.max(
		0,
		plugin.measure(
			block as never,
			{
				contentWidth: safeWidth,
				measureBlock: (nestedBlock, nestedIndent = 0) =>
					measureContentBlockLines(nestedBlock, safeWidth, nestedIndent),
			},
			safeIndent,
		),
	);
}

function countDocItemLines(item: DocRenderItem, contentWidth: number): number {
	if (item.kind === "intro") {
		return countInlineLines(item.content, Math.max(1, contentWidth));
	}
	if (item.kind === "section") {
		return 1;
	}

	const indent =
		typeof item.indent === "number" && Number.isFinite(item.indent) ? Math.max(0, item.indent) : 0;
	return measureContentBlockLines(item.block, contentWidth, indent);
}

export function countRenderPlanLines(
	items: ReadonlyArray<DocRenderItem>,
	contentWidth: number,
): number {
	const safeWidth = Math.max(1, normalizeTextWidth(contentWidth, 1));
	if (items.length === 0) {
		return 0;
	}

	let total = 0;
	for (const [index, item] of items.entries()) {
		total += countDocItemLines(item, safeWidth);
		if (index < items.length - 1) {
			total += 1;
		}
	}
	return total;
}
