import type { DocsPageMeta } from "../content/contracts";
import type { DistroProfile } from "./profile";

const ALLOWED_PRODUCTS = ["levitate", "shared"] as const;
const ALLOWED_SCOPES = ["install"] as const;

function isAllowedProduct(
	product: DocsPageMeta["product"],
): product is (typeof ALLOWED_PRODUCTS)[number] {
	return (ALLOWED_PRODUCTS as readonly string[]).includes(product);
}

function isAllowedScope(
	scope: DocsPageMeta["scopes"][number],
): scope is (typeof ALLOWED_SCOPES)[number] {
	return (ALLOWED_SCOPES as readonly string[]).includes(scope);
}

export const levitateProfile: DistroProfile = {
	id: "levitate",
	title: "LevitateOS Field Manual",
	allowedProducts: ALLOWED_PRODUCTS,
	allowedScopes: ALLOWED_SCOPES,
	defaultSlug: "installation",
	allowsMeta: (meta: DocsPageMeta): boolean => {
		const productAllowed = isAllowedProduct(meta.product);
		const scopeAllowed = meta.scopes.some((scope) => isAllowedScope(scope));
		return productAllowed && scopeAllowed;
	},
};
