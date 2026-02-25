import type { ContentBlock, RichText } from "@levitate/docs-content";

export type DocIntroItem = {
	kind: "intro";
	key: string;
	content: string | RichText;
};

export type DocSectionHeadingItem = {
	kind: "section";
	key: string;
	title: string;
	level: 2 | 3;
	sectionIndex: number;
};

export type DocBlockItem = {
	kind: "block";
	key: string;
	block: ContentBlock;
	sectionIndex: number;
	sectionTitle: string;
	sectionLevel: 2 | 3;
	blockIndex: number;
	indent?: number;
};

export type DocRenderItem = DocIntroItem | DocSectionHeadingItem | DocBlockItem;

export type DocRenderPlan = {
	items: DocRenderItem[];
};
