import { describe, expect, it } from "bun:test";
import type { ContentBlock } from "@levitate/docs-content";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";
import { DEFAULT_INSTALL_BLOCK_PLUGINS } from "./plugins";

type DocsBlockType = ContentBlock["type"];

const SAMPLE_BLOCKS: Record<DocsBlockType, ContentBlock> = {
	text: {
		type: "text",
		content: "alpha",
	},
	code: {
		type: "code",
		language: "bash",
		content: "echo alpha",
		highlightedLines: ["echo alpha"],
	},
	table: {
		type: "table",
		headers: ["name"],
		rows: [["alpha"]],
	},
	list: {
		type: "list",
		items: ["alpha"],
	},
	conversation: {
		type: "conversation",
		messages: [
			{
				role: "user",
				text: "alpha",
			},
		],
	},
	interactive: {
		type: "interactive",
		steps: [
			{
				command: "echo alpha",
				description: "Print alpha",
			},
		],
	},
	command: {
		type: "command",
		language: "bash",
		description: "Run alpha",
		command: "echo alpha",
		highlightedCommandLines: ["echo alpha"],
	},
	qa: {
		type: "qa",
		items: [
			{
				question: "What is alpha?",
				answer: [
					{
						type: "text",
						content: "Alpha is first.",
					},
				],
			},
		],
	},
	note: {
		type: "note",
		variant: "info",
		content: "Alpha note",
	},
};

describe("block plugins", () => {
	it("registers every ContentBlock type exactly once", () => {
		const expected = Object.keys(SAMPLE_BLOCKS).sort();
		const actual = Object.keys(DEFAULT_INSTALL_BLOCK_PLUGINS).sort();
		expect(actual).toEqual(expected);
	});

	it("covers every docs block type with renderer key + measure contract", () => {
		const blockTypes = Object.keys(DEFAULT_INSTALL_BLOCK_PLUGINS) as DocsBlockType[];
		for (const blockType of blockTypes) {
			const plugin = DEFAULT_INSTALL_BLOCK_PLUGINS[blockType];
			expect(plugin.type).toBe(blockType);
			expect(plugin.rendererKey).toBe(defaultDocsBlockRendererKey(blockType));
			const measured = plugin.measure(
				SAMPLE_BLOCKS[blockType] as never,
				{
					contentWidth: 80,
					measureBlock: (block, indent = 0) => {
						const nestedPlugin = DEFAULT_INSTALL_BLOCK_PLUGINS[block.type];
						return nestedPlugin.measure(
							block as never,
							{ contentWidth: 80, measureBlock: () => 1 },
							indent,
						);
					},
				},
				0,
			);
			expect(measured).toBeGreaterThan(0);
		}
	});
});
