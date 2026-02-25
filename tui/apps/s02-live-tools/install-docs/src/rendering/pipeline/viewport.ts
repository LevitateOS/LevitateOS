import { clampNumber } from "@levitate/tui-kit";
import type {
	DocsContentLike,
	DocsRenderStyleContext,
	FlatDocsNavItem,
} from "../../domain/content/contracts";
import type { DocRenderItem } from "../../domain/render/types";
import { buildDocRenderPlan } from "../plan/build-doc-plan";
import { countRenderPlanLines } from "../plan/line-metrics";

type InitialDocSelection = {
	index: number;
	unknownSlug?: string;
};

export type DocsViewport = {
	visibleItems: DocRenderItem[];
	totalItems: number;
	visibleCount: number;
	maxScroll: number;
	scrollOffset: number;
	startItem: number;
	endItem: number;
	contentWidth: number;
};

export function resolveInitialDocSelection(
	navItems: ReadonlyArray<FlatDocsNavItem>,
	slug?: string,
): InitialDocSelection {
	if (navItems.length === 0) {
		return { index: 0 };
	}

	const normalized = slug?.trim() ?? "";
	if (normalized.length === 0) {
		return { index: 0 };
	}

	const index = navItems.findIndex((item) => item.slug === normalized);
	if (index >= 0) {
		return { index };
	}

	return {
		index: 0,
		unknownSlug: normalized,
	};
}

export function resolveInitialDocIndex(
	navItems: ReadonlyArray<FlatDocsNavItem>,
	slug?: string,
): number {
	return resolveInitialDocSelection(navItems, slug).index;
}

export function computeDocsViewport(
	content: DocsContentLike,
	_slug: string,
	requestedScrollOffset: number,
	contentInnerRows: number,
	contentWidth: number,
	_styleContext?: DocsRenderStyleContext,
): DocsViewport {
	const safeVisibleCount = Math.max(1, contentInnerRows);
	const safeWidth = Math.max(1, contentWidth);
	const plan = buildDocRenderPlan(content);
	const allItems = plan.items;
	const totalLines = countRenderPlanLines(allItems, safeWidth);

	const maxScroll = Math.max(0, totalLines - safeVisibleCount);
	const scrollOffset = clampNumber(requestedScrollOffset, 0, maxScroll);
	const startItem = totalLines === 0 ? 0 : scrollOffset + 1;
	const endItem = totalLines === 0 ? 0 : Math.min(totalLines, scrollOffset + safeVisibleCount);

	return {
		visibleItems: allItems,
		totalItems: totalLines,
		visibleCount: safeVisibleCount,
		maxScroll,
		scrollOffset,
		startItem,
		endItem,
		contentWidth: safeWidth,
	};
}
