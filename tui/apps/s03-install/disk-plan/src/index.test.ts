import { existsSync } from "node:fs";
import { basename } from "node:path";
import { describe, expect, it } from "bun:test";
import { RECPART_MANIFEST, RECPART_ROOT, buildPlanCommand, extractInlineJson, parseCliArgs } from "./index";

describe("frontend pathing", () => {
	it("resolves recpart crate root and manifest", () => {
		expect(basename(RECPART_ROOT)).toBe("recpart");
		expect(existsSync(RECPART_MANIFEST)).toBe(true);
	});
});

describe("json extraction", () => {
	it("returns full payload when content is pure JSON", () => {
		const input = '{"ok":true}';
		expect(extractInlineJson(input)).toBe(input);
	});

	it("extracts final inline JSON line from mixed output", () => {
		const input = 'info: warmup\n{"schema_version":1,"code":"E001"}';
		expect(extractInlineJson(input)).toBe('{"schema_version":1,"code":"E001"}');
	});

	it("returns null when no JSON line exists", () => {
		expect(extractInlineJson("plain text output")).toBeNull();
	});
});

describe("command building", () => {
	it("builds recpart plan command deterministically", () => {
		expect(buildPlanCommand("/dev/sda", "ab")).toEqual([
			"recpart",
			"plan",
			"--disk",
			"/dev/sda",
			"--mode",
			"ab",
			"--json",
		]);
	});
});

describe("cli args", () => {
	it("defaults mode to ab", () => {
		const parsed = parseCliArgs(["--disk", "/dev/sdb"]);
		expect(parsed.mode).toBe("ab");
		expect(parsed.error).toBeNull();
	});

	it("accepts mutable mode", () => {
		const parsed = parseCliArgs(["--disk", "/dev/sdb", "--mode", "mutable"]);
		expect(parsed.mode).toBe("mutable");
		expect(parsed.error).toBeNull();
	});

	it("rejects unknown mode", () => {
		const parsed = parseCliArgs(["--disk", "/dev/sdb", "--mode", "bad"]);
		expect(parsed.error).toBe("--mode must be one of: ab, mutable");
	});
});
