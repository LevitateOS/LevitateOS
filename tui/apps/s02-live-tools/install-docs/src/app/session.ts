import type { DocsSource, FlatDocsNavItem } from "../domain/content/contracts";
import type { DistroProfile } from "../domain/distro/profile";
import { buildInstallNavItems } from "../domain/navigation/nav-model";
import { guardRequestedSlug } from "../domain/navigation/slug-guard";
import { resolveAllowedSlugs, resolveAllowedSlugSet } from "../domain/scope/allowed-slugs";

type InstallSession = {
	profile: DistroProfile;
	source: DocsSource;
	allowedSlugs: string[];
	navItems: FlatDocsNavItem[];
	initialSlug?: string;
};

function resolveDefaultSlug(
	allowedSlugs: ReadonlyArray<string>,
	navItems: ReadonlyArray<FlatDocsNavItem>,
	profile: DistroProfile,
): string {
	if (allowedSlugs.includes(profile.defaultSlug)) {
		return profile.defaultSlug;
	}

	const navFallback = navItems[0]?.slug;
	if (navFallback) {
		return navFallback;
	}

	const fallback = allowedSlugs[0];
	if (!fallback) {
		throw new Error(`No docs pages are available for profile '${profile.id}'.`);
	}

	return fallback;
}

export function createInstallSession(
	source: DocsSource,
	profile: DistroProfile,
	requestedSlug?: string,
): InstallSession {
	const allowedSlugs = resolveAllowedSlugs(source);
	if (allowedSlugs.length === 0) {
		throw new Error(`No docs pages are available for profile '${profile.id}'.`);
	}

	const allowedSlugSet = resolveAllowedSlugSet(source);
	const navItems = buildInstallNavItems(source.docsNav, allowedSlugSet);
	if (navItems.length === 0) {
		throw new Error(`Docs navigation is empty for profile '${profile.id}'.`);
	}

	const validatedSlug = guardRequestedSlug(requestedSlug, allowedSlugs);
	const initialSlug = validatedSlug ?? resolveDefaultSlug(allowedSlugs, navItems, profile);

	return {
		profile,
		source,
		allowedSlugs,
		navItems,
		initialSlug,
	};
}
