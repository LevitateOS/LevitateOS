import { contentBySlug, docsNav, metaBySlug } from "@levitate/docs-content";
import type { DocsSource } from "../../domain/content/contracts";

export function loadDocsSource(): DocsSource {
	return {
		docsNav,
		contentBySlug,
		metaBySlug,
	};
}
