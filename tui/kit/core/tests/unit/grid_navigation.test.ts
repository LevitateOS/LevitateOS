import { describe, expect, it } from "bun:test";
import { transitionGridIndex } from "../../src/hooks/use-grid-navigation";

describe("grid navigation", () => {
	it("clamps left and right within a row", () => {
		expect(transitionGridIndex(0, 5, 2, "left")).toBe(0);
		expect(transitionGridIndex(1, 5, 2, "right")).toBe(1);
		expect(transitionGridIndex(2, 5, 2, "right")).toBe(3);
		expect(transitionGridIndex(4, 5, 2, "right")).toBe(4);
	});

	it("moves up/down by column count and clamps at boundaries", () => {
		expect(transitionGridIndex(0, 6, 2, "down")).toBe(2);
		expect(transitionGridIndex(2, 6, 2, "up")).toBe(0);
		expect(transitionGridIndex(4, 5, 2, "down")).toBe(4);
	});

	it("keeps horizontal movement clamped on partial final rows", () => {
		expect(transitionGridIndex(4, 5, 2, "left")).toBe(4);
		expect(transitionGridIndex(4, 5, 2, "right")).toBe(4);
	});

	it("clamps vertical movement into existing final-row cells", () => {
		expect(transitionGridIndex(1, 5, 2, "down")).toBe(3);
		expect(transitionGridIndex(3, 5, 2, "down")).toBe(4);
	});

	it("returns zero when count is empty", () => {
		expect(transitionGridIndex(10, 0, 2, "down")).toBe(0);
	});
});
