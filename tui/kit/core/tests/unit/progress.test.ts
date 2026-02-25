import { describe, expect, it } from "bun:test";
import {
	progressRatio,
	renderProgress,
	renderProgressBar,
} from "../../src/components/feedback/progress";

describe("progress helpers", () => {
	it("computes bounded ratio", () => {
		expect(progressRatio({ current: 5, total: 10 })).toBe(0.5);
		expect(progressRatio({ current: 12, total: 10 })).toBe(1);
		expect(progressRatio({ current: -1, total: 10 })).toBe(0);
		expect(progressRatio({ current: 1, total: 0 })).toBe(0);
	});

	it("renders percentage string", () => {
		expect(renderProgress({ current: 1, total: 2 })).toBe("50%");
	});

	it("renders bounded progress bar", () => {
		expect(renderProgressBar({ current: 1, total: 2 }, 6)).toContain("50%");
		expect(renderProgressBar({ current: 1, total: 2 }, 1)).toContain("50%");
	});
});
