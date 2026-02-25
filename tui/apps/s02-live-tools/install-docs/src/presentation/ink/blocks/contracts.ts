import type { ContentBlock } from "@levitate/docs-content";
import type { ReactNode } from "react";

export type DocsBlockType = ContentBlock["type"];

export type BlockHotkeyHint = {
	key: string;
	label: string;
};

export type BlockRenderContext = {
	contentWidth: number;
	renderBlock: (block: ContentBlock, indent?: number) => ReactNode;
};

export type BlockMeasureContext = {
	contentWidth: number;
	measureBlock: (block: ContentBlock, indent?: number) => number;
};

export type BlockPlugin<TType extends DocsBlockType = DocsBlockType> = {
	type: TType;
	rendererKey: string;
	render: (
		block: Extract<ContentBlock, { type: TType }>,
		context: BlockRenderContext,
		indent: number,
	) => ReactNode;
	measure: (
		block: Extract<ContentBlock, { type: TType }>,
		context: BlockMeasureContext,
		indent: number,
	) => number;
	hotkeysHelp?: (block: Extract<ContentBlock, { type: TType }>) => ReadonlyArray<BlockHotkeyHint>;
};

export type BlockPluginMap = {
	[K in DocsBlockType]: BlockPlugin<K>;
};

export function defineBlockPlugins<TPlugins extends BlockPluginMap>(plugins: TPlugins): TPlugins {
	return plugins;
}
