import { describe, expect, it } from "bun:test";
import { parseSyntaxTokenLine } from "../../src/primitives/display/syntax-token-line";

describe("syntax token line", () => {
	it("parses fg color segments and resets", () => {
		const tokens = parseSyntaxTokenLine("[[fg=#b392f0]]recstrap[[/]][[fg=#e1e4e8]] /mnt[[/]]");

		expect(tokens.length).toBeGreaterThan(1);
		expect(tokens[0]?.text).toBe("recstrap");
		expect(tokens[0]?.color).toBe("#b392f0");
		expect(tokens[1]?.text).toContain("/mnt");
		expect(tokens[1]?.color).toBe("#e1e4e8");
	});

	it("restores escaped style delimiters", () => {
		const tokens = parseSyntaxTokenLine("\\[[literal]] [[fg=#9ecbff]]ok[[/]]");
		const text = tokens.map((token) => token.text).join("");

		expect(text).toContain("[[literal]]");
		expect(tokens.some((token) => token.color === "#9ecbff")).toBe(true);
	});
});
