import { describe, expect, it } from "bun:test";
import {
	normalizeTextWidth,
	prefixWrappedText,
	truncateBoundedLine,
	wrapBoundedText,
} from "../../src/utils/strings";

describe("bounded string helpers", () => {
	it("enforces minimum width", () => {
		expect(normalizeTextWidth(0)).toBe(20);
		expect(normalizeTextWidth(4, 8)).toBe(8);
		expect(normalizeTextWidth(30, 8)).toBe(30);
	});

	it("wraps and truncates text to bounded width", () => {
		const wrapped = wrapBoundedText("alpha beta gamma", 7, 4);
		expect(wrapped.length).toBeGreaterThan(1);
		expect(wrapped.every((line) => line.length <= 7)).toBe(true);
	});

	it("wraps prefixed text with aligned continuation", () => {
		const lines = prefixWrappedText("Q:", "this is a wrapped answer", 12, 8);
		expect(lines[0]?.startsWith("Q: ")).toBe(true);
		expect(lines.slice(1).every((line) => line.startsWith("   "))).toBe(true);
		expect(lines.every((line) => line.length <= 12)).toBe(true);
	});

	it("truncates bounded lines", () => {
		expect(truncateBoundedLine("abcdefghij", 6, 4)).toBe("abcde…");
	});
});
