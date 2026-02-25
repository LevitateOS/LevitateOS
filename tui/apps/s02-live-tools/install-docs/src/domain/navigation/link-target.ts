import type { FlatDocsNavItem } from "../content/contracts";
import { docsSlugFromHref } from "./nav-model";

export type LinkTargetResolution =
	| { ok: true; slug: string; index: number }
	| { ok: false; reason: string };

export function resolveDocsLinkTarget(
	href: string,
	currentSlug: string,
	navItems: ReadonlyArray<FlatDocsNavItem>,
): LinkTargetResolution {
	const trimmed = href.trim();
	if (trimmed.length === 0) {
		return { ok: false, reason: "empty href" };
	}

	const targetSlug = trimmed.startsWith("#") ? currentSlug : docsSlugFromHref(trimmed);
	const targetIndex = navItems.findIndex((item) => item.slug === targetSlug);
	if (targetIndex < 0) {
		return {
			ok: false,
			reason: `slug '${targetSlug}' not in docs navigation`,
		};
	}

	return {
		ok: true,
		slug: targetSlug,
		index: targetIndex,
	};
}
