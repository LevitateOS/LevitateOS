import { describe, expect, it } from "bun:test";
import { formatWizardCard, wizardClampIndex } from "../../src/patterns/wizard-flow";

describe("wizard flow helpers", () => {
	it("clamps index within bounds", () => {
		expect(wizardClampIndex(-1, 3)).toBe(0);
		expect(wizardClampIndex(10, 3)).toBe(2);
		expect(wizardClampIndex(1, 3)).toBe(1);
	});

	it("returns 0 when count is non-finite or non-positive", () => {
		expect(wizardClampIndex(2, 0)).toBe(0);
		expect(wizardClampIndex(2, Number.NaN)).toBe(0);
	});

	it("formats selected card with marker and indented lines", () => {
		const lines = formatWizardCard(
			{
				title: "Disk",
				lines: ["line 1", "line 2"],
			},
			true,
		);

		expect(lines[0]).toBe("> Disk");
		expect(lines[1]).toBe("  line 1");
		expect(lines[2]).toBe("  line 2");
	});
});
