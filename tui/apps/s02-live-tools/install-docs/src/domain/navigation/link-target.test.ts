import { describe, expect, it } from "bun:test";
import { resolveDocsLinkTarget } from "./link-target";
import type { FlatDocsNavItem } from "../content/contracts";

const NAV_ITEMS: FlatDocsNavItem[] = [
	{
		sectionTitle: "Install",
		title: "Installation",
		href: "/docs/installation",
		slug: "installation",
	},
	{
		sectionTitle: "Install",
		title: "Recstrap",
		href: "/docs/recstrap",
		slug: "recstrap",
	},
];

describe("resolveDocsLinkTarget", () => {
	it("resolves /docs path href", () => {
		const target = resolveDocsLinkTarget("/docs/recstrap", "installation", NAV_ITEMS);
		expect(target).toEqual({ ok: true, slug: "recstrap", index: 1 });
	});

	it("resolves fragment href to current page", () => {
		const target = resolveDocsLinkTarget("#partitions", "installation", NAV_ITEMS);
		expect(target).toEqual({ ok: true, slug: "installation", index: 0 });
	});

	it("fails for unknown/external href", () => {
		const target = resolveDocsLinkTarget("https://example.com/docs", "installation", NAV_ITEMS);
		expect(target.ok).toBe(false);
		if (target.ok) {
			return;
		}
		expect(target.reason.includes("not in docs navigation")).toBe(true);
	});
});
