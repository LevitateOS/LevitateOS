import { describe, expect, it } from "bun:test";
import { keyNamesFromInput, normalizeKeySpec } from "../../src/hooks/use-hotkeys";

describe("hotkey helpers", () => {
	it("normalizes and deduplicates key specs", () => {
		expect(normalizeKeySpec("  enter  ")).toEqual(["enter"]);
		expect(normalizeKeySpec([" j", "j ", "k", ""])).toEqual(["j", "k"]);
	});

	it("derives names for printable and modifier keys", () => {
		const names = keyNamesFromInput("G", {
			shift: true,
			end: true,
		});

		expect(names.includes("G")).toBe(true);
		expect(names.includes("g")).toBe(true);
		expect(names.includes("S-g")).toBe(true);
		expect(names.includes("end")).toBe(true);
	});

	it("maps control and navigation keys", () => {
		const names = keyNamesFromInput("c", {
			ctrl: true,
			pageDown: true,
			return: true,
		});

		expect(names.includes("C-c")).toBe(true);
		expect(names.includes("pagedown")).toBe(true);
		expect(names.includes("enter")).toBe(true);
	});

	it("maps control characters to C-* names", () => {
		const names = keyNamesFromInput("\u0003", {
			ctrl: true,
		});

		expect(names.includes("c")).toBe(true);
		expect(names.includes("C-c")).toBe(true);
	});

	it("maps raw VT escape sequences for arrows and paging", () => {
		expect(keyNamesFromInput("\u001b[A", {}).includes("up")).toBe(true);
		expect(keyNamesFromInput("\u001bOB", {}).includes("down")).toBe(true);
		expect(keyNamesFromInput("\u001bOC", {}).includes("right")).toBe(true);
		expect(keyNamesFromInput("\u001b[D", {}).includes("left")).toBe(true);
		expect(keyNamesFromInput("\u001b[5~", {}).includes("pageup")).toBe(true);
		expect(keyNamesFromInput("\u001b[6~", {}).includes("pagedown")).toBe(true);
		expect(keyNamesFromInput("\u001bOH", {}).includes("home")).toBe(true);
		expect(keyNamesFromInput("\u001b[F", {}).includes("end")).toBe(true);
	});
});
