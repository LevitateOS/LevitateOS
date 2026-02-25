import { describe, expect, it } from "bun:test";
import { createChromeGrid, resolveChromeGlyphSet } from "../../src/chrome";

function renderRows(
	grid: ReturnType<typeof createChromeGrid>,
	weight: "single" | "bold" | "double" | "round",
) {
	const glyphs = resolveChromeGlyphSet(weight);
	return grid.renderRows(glyphs).map((row) => row.text);
}

describe("chrome grid junction resolution", () => {
	it("draws seam joins against frame edges", () => {
		const grid = createChromeGrid(14, 6);
		grid.drawRect({ x: 0, y: 0, width: 14, height: 6 }, "border");
		grid.drawSeam(2, 0, 13, "border");

		const rows = renderRows(grid, "single");
		expect(rows[2]).toBe(`├${"─".repeat(12)}┤`);
	});

	it("renders split intersections using cross and T-junction glyphs", () => {
		const grid = createChromeGrid(15, 7);
		grid.drawRect({ x: 0, y: 0, width: 15, height: 7 }, "border");
		grid.drawSeam(3, 0, 14, "border");
		grid.drawSplit(7, 0, 6, "border");

		const rows = renderRows(grid, "single");
		expect(rows[0]?.[7]).toBe("┬");
		expect(rows[3]?.[7]).toBe("┼");
		expect(rows[6]?.[7]).toBe("┴");
	});

	it("never emits ASCII hyphen for structural lines", () => {
		for (const weight of ["single", "bold", "double", "round"] as const) {
			const grid = createChromeGrid(18, 8);
			grid.drawRect({ x: 0, y: 0, width: 18, height: 8 }, "border");
			grid.drawSeam(2, 0, 17, "border");
			grid.drawSplit(6, 0, 7, "border");

			const rows = renderRows(grid, weight);
			expect(rows.join("").includes("-")).toBe(false);
		}
	});
});
