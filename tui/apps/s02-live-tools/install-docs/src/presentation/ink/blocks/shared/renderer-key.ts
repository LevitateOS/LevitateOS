import type { ContentBlock } from "@levitate/docs-content";

export type DocsBlockType = ContentBlock["type"];

export function defaultDocsBlockRendererKey(type: DocsBlockType): string {
	return `docs:block:${type}`;
}
