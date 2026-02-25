import { describe, expect, it } from "bun:test";
import { confirmStatusLine, failureDiagnosticLines, progressStepLine } from "./page-text";

describe("page text snapshots", () => {
	it("confirm page status is stable", () => {
		expect(confirmStatusLine("DEST")).toBe("Status: locked");
		expect(confirmStatusLine("DESTROY")).toBe("Status: unlocked");
	});

	it("progress page step line format is stable", () => {
		expect(progressStepLine("partition", "sfdisk /dev/vda", null)).toBe(
			"[OK] partition :: sfdisk /dev/vda",
		);
		expect(progressStepLine("mount", "mount /dev/vda2 /mnt", 1)).toBe(
			"[FAIL] mount :: mount /dev/vda2 /mnt",
		);
	});

	it("failure page text is stable", () => {
		const withDetail = failureDiagnosticLines({
			source: "apply",
			detail: {
				code: "E002",
				component: "preflight",
				expectation: "tool exists",
				observed: "missing",
				remediation: "install tool",
			},
			observed: "x",
			remediation: "y",
		});
		expect(withDetail).toEqual([
			"Failure Diagnostics (apply)",
			"Code: E002",
			"Component: preflight",
			"Expectation: tool exists",
			"Observed: missing",
			"Remediation: install tool",
		]);
	});
});
