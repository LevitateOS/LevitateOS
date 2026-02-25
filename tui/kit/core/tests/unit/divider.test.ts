import { describe, expect, it } from "bun:test";
import { resolveDividerGlyph } from "../../src/primitives/display/divider";

describe("divider glyph", () => {
	it("uses box-drawing horizontal glyphs for every border style", () => {
		expect(resolveDividerGlyph("single")).toBe("─");
		expect(resolveDividerGlyph("bold")).toBe("━");
		expect(resolveDividerGlyph("double")).toBe("═");
		expect(resolveDividerGlyph("round")).toBe("─");
	});

	it("never resolves to ASCII hyphen", () => {
		for (const style of ["single", "bold", "double", "round"] as const) {
			expect(resolveDividerGlyph(style)).not.toBe("-");
		}
	});
});
