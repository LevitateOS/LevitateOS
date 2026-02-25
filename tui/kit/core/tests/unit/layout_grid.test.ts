import { describe, expect, it } from "bun:test";
import { groupGridRows, resolveGridColumns } from "../../src/primitives/layout/grid";

describe("layout grid", () => {
	it("resolves fixed columns", () => {
		expect(resolveGridColumns(2, 80, 1)).toBe(2);
		expect(resolveGridColumns(0, 80, 1)).toBe(1);
	});

	it("resolves responsive breakpoint columns", () => {
		const spec = {
			base: 1,
			breakpoints: [
				{ minWidth: 110, columns: 2 },
				{ minWidth: 180, columns: 4 },
			],
		};
		expect(resolveGridColumns(spec, 100, 1)).toBe(1);
		expect(resolveGridColumns(spec, 120, 1)).toBe(2);
		expect(resolveGridColumns(spec, 200, 1)).toBe(4);
	});

	it("normalizes unsorted/invalid breakpoints safely", () => {
		const spec = {
			base: 0,
			breakpoints: [
				{ minWidth: 180, columns: 4 },
				{ minWidth: 110, columns: 2 },
				{ minWidth: -50, columns: 0 },
			],
		};
		expect(resolveGridColumns(spec, 100, 1)).toBe(1);
		expect(resolveGridColumns(spec, 150, 1)).toBe(2);
		expect(resolveGridColumns(spec, 200, 1)).toBe(4);
	});

	it("groups rows with partial final row", () => {
		expect(groupGridRows(["a", "b", "c", "d", "e"], 2)).toEqual([["a", "b"], ["c", "d"], ["e"]]);
	});

	it("returns an empty row set when there are no items", () => {
		expect(groupGridRows([], 2)).toEqual([]);
	});
});
