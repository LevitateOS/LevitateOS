import type { FlatDocsNavItem, NavSection } from "../content/contracts";

function slugFromHref(href: string): string {
	const normalizedHref = href.trim();
	const withoutQuery = normalizedHref.split("?")[0]?.split("#")[0] ?? normalizedHref;
	const noTrailingSlash = withoutQuery.replace(/\/+$/, "");

	if (noTrailingSlash.startsWith("/docs/")) {
		return noTrailingSlash.slice("/docs/".length);
	}

	if (noTrailingSlash.startsWith("docs/")) {
		return noTrailingSlash.slice("docs/".length);
	}

	if (noTrailingSlash.startsWith("/")) {
		return noTrailingSlash.slice(1);
	}

	return noTrailingSlash.length > 0 ? noTrailingSlash : "index";
}

export function flattenDocsNav(sections: ReadonlyArray<NavSection>): FlatDocsNavItem[] {
	const output: FlatDocsNavItem[] = [];

	for (const section of sections) {
		for (const item of section.items) {
			output.push({
				sectionTitle: section.title,
				title: item.title,
				href: item.href,
				slug: slugFromHref(item.href),
			});
		}
	}

	return output;
}

function filterNavItemsByAllowedSlugs(
	items: ReadonlyArray<FlatDocsNavItem>,
	allowedSlugs: ReadonlySet<string>,
): FlatDocsNavItem[] {
	return items.filter((item) => allowedSlugs.has(item.slug));
}

export function buildInstallNavItems(
	sections: ReadonlyArray<NavSection>,
	allowedSlugs: ReadonlySet<string>,
): FlatDocsNavItem[] {
	return filterNavItemsByAllowedSlugs(flattenDocsNav(sections), allowedSlugs);
}

export type NavSectionSpan = {
	sectionTitle: string;
	startIndex: number;
	endIndex: number;
};

export function buildNavSectionSpans(
	items: ReadonlyArray<FlatDocsNavItem>,
): ReadonlyArray<NavSectionSpan> {
	const spans: NavSectionSpan[] = [];

	for (const [index, item] of items.entries()) {
		const current = spans[spans.length - 1];
		if (!current || current.sectionTitle !== item.sectionTitle) {
			spans.push({
				sectionTitle: item.sectionTitle,
				startIndex: index,
				endIndex: index,
			});
			continue;
		}
		current.endIndex = index;
	}

	return spans;
}

export function findSectionIndexForPageIndex(
	spans: ReadonlyArray<NavSectionSpan>,
	pageIndex: number,
): number {
	let resolved = 0;

	for (const [index, span] of spans.entries()) {
		if (pageIndex < span.startIndex) {
			break;
		}
		if (pageIndex <= span.endIndex) {
			return index;
		}
		resolved = index;
	}

	return resolved;
}

export function jumpToSectionStart(
	spans: ReadonlyArray<NavSectionSpan>,
	currentPageIndex: number,
	delta: number,
): number {
	if (spans.length === 0) {
		return 0;
	}

	const currentSection = findSectionIndexForPageIndex(spans, currentPageIndex);
	const targetSection = Math.max(0, Math.min(spans.length - 1, currentSection + delta));
	return spans[targetSection]?.startIndex ?? 0;
}
