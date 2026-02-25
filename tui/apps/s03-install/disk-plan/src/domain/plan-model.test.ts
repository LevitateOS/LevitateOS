import { describe, expect, it } from "bun:test";
import { formatBytes, parsePlanSummary } from "./plan-model";

describe("plan model", () => {
	it("parses minimal valid recpart plan payload", () => {
		const parsed = parsePlanSummary(
			JSON.stringify({
				schema_version: 1,
				mode: "ab",
				disk: {
					path: "/dev/vda",
					size_bytes: 1000,
				},
				partitions: [
					{
						index: 1,
						name: "efi",
						filesystem: "vfat",
						label: "EFI",
						mountpoint: "/boot",
						size_mb: 1024,
					},
				],
				sfdisk_script: "label:gpt",
			}),
		);
		expect(parsed).not.toBeNull();
		expect(parsed?.diskPath).toBe("/dev/vda");
		expect(parsed?.partitions.length).toBe(1);
	});

	it("returns null on invalid payload", () => {
		expect(parsePlanSummary("not-json")).toBeNull();
		expect(parsePlanSummary(JSON.stringify({ mode: "x" }))).toBeNull();
	});

	it("formats bytes for human display", () => {
		expect(formatBytes(0)).toBe("0 B");
		expect(formatBytes(1024)).toBe("1.0 KB");
		expect(formatBytes(1024 * 1024)).toBe("1.0 MB");
	});
});
