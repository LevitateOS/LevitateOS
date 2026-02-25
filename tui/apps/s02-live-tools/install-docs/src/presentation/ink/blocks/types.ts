import type { ContentBlock } from "@levitate/docs-content";
import type { ReactNode } from "react";

export type BlockComponentProps<TBlock extends ContentBlock> = {
	block: TBlock;
	contentWidth: number;
	indent?: number;
};

export type RenderNestedBlock = (block: ContentBlock, indent: number) => ReactNode;
