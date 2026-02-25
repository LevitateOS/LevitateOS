import type { DocsContent } from "@levitate/docs-content";
import type { DocsContentLike, DocsSource } from "./contracts";

function missingPage(slug: string, title: string): DocsContent {
	return {
		title,
		intro: `Missing docs page for slug '${slug}'.`,
		sections: [],
	};
}

export function contentForSlug(source: DocsSource, slug: string, title: string): DocsContentLike {
	return source.contentBySlug[slug] ?? missingPage(slug, title);
}
