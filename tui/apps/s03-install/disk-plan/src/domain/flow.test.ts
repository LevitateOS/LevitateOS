import { describe, expect, it } from "bun:test";
import { canAdvance, clampPageIndex, currentPage, nextPage, prevPage, setMode } from "./flow";

describe("flow state", () => {
	it("clamps page index bounds", () => {
		expect(clampPageIndex(-10)).toBe(0);
		expect(clampPageIndex(999)).toBe(9);
	});

	it("moves forward and backward deterministically", () => {
		const initial = { pageIndex: 0, disk: "/dev/vda", mode: "ab" as const };
		expect(currentPage(initial)).toBe("welcome");
		expect(currentPage(nextPage(initial))).toBe("target-disk");
		expect(currentPage(prevPage(nextPage(initial)))).toBe("welcome");
	});

	it("blocks advance on target-disk when disk is empty", () => {
		const state = { pageIndex: 1, disk: "   ", mode: "ab" as const };
		expect(currentPage(state)).toBe("target-disk");
		expect(canAdvance(state)).toBe(false);
	});

	it("updates mode without mutating page position", () => {
		const state = { pageIndex: 2, disk: "/dev/vda", mode: "ab" as const };
		const changed = setMode(state, "mutable");
		expect(changed.pageIndex).toBe(2);
		expect(changed.mode).toBe("mutable");
	});
});
