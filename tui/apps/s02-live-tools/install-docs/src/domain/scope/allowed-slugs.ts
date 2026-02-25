import type { DocsSource } from "../content/contracts";
import { flattenDocsNav } from "../navigation/nav-model";

export function resolveAllowedSlugs(source: DocsSource): string[] {
	const slugs: string[] = [];
	const seen = new Set<string>();

	for (const item of flattenDocsNav(source.docsNav)) {
		if (source.contentBySlug[item.slug] === undefined) {
			continue;
		}

		if (seen.has(item.slug)) {
			continue;
		}

		seen.add(item.slug);
		slugs.push(item.slug);
	}

	return slugs;
}

export function resolveAllowedSlugSet(source: DocsSource): Set<string> {
	return new Set(resolveAllowedSlugs(source));
}
