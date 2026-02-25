import { describe, expect, it } from "bun:test";
import { parseApplySummary, parseErrorSummary } from "./apply-model";

describe("apply model", () => {
	it("parses valid apply payload with schema version 1", () => {
		const parsed = parseApplySummary(
			JSON.stringify({
				schema_version: 1,
				mode: "ab",
				dry_run: true,
				steps: [
					{
						phase: "partition",
						command: "wipefs -a /dev/vda",
						status: null,
						dry_run: true,
					},
				],
				mounted: [{ path: "/mnt/sysroot", device: "/dev/vda2" }],
				handoff: {
					schema_version: 1,
					install_target: "/mnt/sysroot",
					mount_map: [{ path: "/mnt/sysroot", device: "/dev/vda2" }],
					next_commands: ["recstrap /mnt/sysroot"],
					mode_context: { notes: ["note"] },
				},
				warnings: [],
			}),
		);
		expect(parsed).not.toBeNull();
		expect(parsed?.handoff.installTarget).toBe("/mnt/sysroot");
	});

	it("fails contract on wrong schema version", () => {
		expect(
			parseApplySummary(
				JSON.stringify({
					schema_version: 2,
					mode: "ab",
				}),
			),
		).toBeNull();
	});

	it("parses structured error payload", () => {
		const parsed = parseErrorSummary(
			JSON.stringify({
				schema_version: 1,
				code: "E002",
				component: "preflight",
				expectation: "tool exists",
				observed: "missing",
				remediation: "install tool",
			}),
		);
		expect(parsed?.code).toBe("E002");
	});
});
