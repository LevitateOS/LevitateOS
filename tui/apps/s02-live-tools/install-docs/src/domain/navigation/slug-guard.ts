export function guardRequestedSlug(
	requestedSlug: string | undefined,
	allowedSlugs: ReadonlyArray<string>,
): string | undefined {
	if (!requestedSlug) {
		return undefined;
	}

	const normalized = requestedSlug.trim();
	if (normalized.length === 0) {
		throw new Error("Flag '--slug' requires a non-empty page slug.");
	}

	if (allowedSlugs.includes(normalized)) {
		return normalized;
	}

	throw new Error(`Slug '${normalized}' is not available in docs navigation.`);
}
