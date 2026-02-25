import { describe, expect, it } from "bun:test";
import { canAdvancePage } from "./guards";

const base = {
	hasDisk: true,
	planReady: true,
	commandPreviewReady: true,
	preflightPassing: true,
	confirmUnlocked: true,
	applyFinished: true,
};

describe("page guards", () => {
	it("blocks target-disk without disk", () => {
		expect(canAdvancePage("target-disk", { ...base, hasDisk: false })).toBe(false);
	});

	it("blocks destructive-confirm until token unlocked", () => {
		expect(canAdvancePage("destructive-confirm", { ...base, confirmUnlocked: false })).toBe(false);
		expect(canAdvancePage("destructive-confirm", base)).toBe(true);
	});

	it("blocks apply-progress until apply completes", () => {
		expect(canAdvancePage("apply-progress", { ...base, applyFinished: false })).toBe(false);
		expect(canAdvancePage("apply-progress", base)).toBe(true);
	});

	it("allows welcome by default", () => {
		expect(canAdvancePage("welcome", base)).toBe(true);
	});
});
