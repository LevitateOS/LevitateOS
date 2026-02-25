import { describe, expect, it } from "bun:test";
import { renderSectionList } from "../../src/components/navigation/section-list";

describe("section list", () => {
	it("renders grouped sections with selection marker", () => {
		const rendered = renderSectionList(
			[
				{ section: "Getting Started", label: "Installation" },
				{ section: "Getting Started", label: "Bootloader" },
				{ section: "Tools", label: "recstrap" },
			],
			1,
			{ maxWidth: 40 },
		);

		expect(rendered).toContain("Getting Started");
		expect(rendered).toContain("  > Bootloader");
		expect(rendered).toContain("Tools");
		expect(rendered).toContain("    recstrap");
	});

	it("returns empty label when there are no items", () => {
		expect(renderSectionList([], 0)).toBe("(no items)");
		expect(renderSectionList([], 0, { emptyLabel: "none" })).toBe("none");
	});

	it("honors explicit empty inactive marker for marker-free rows", () => {
		const rendered = renderSectionList(
			[
				{ section: "Getting Started", label: "Installation" },
				{ section: "Getting Started", label: "Bootloader" },
			],
			0,
			{
				marker: "▸",
				inactiveMarker: "",
				maxWidth: 40,
			},
		);

		expect(rendered).toContain("  ▸ Installation");
		expect(rendered).not.toContain("•");
		expect(rendered).toContain("   Bootloader");
	});
});
