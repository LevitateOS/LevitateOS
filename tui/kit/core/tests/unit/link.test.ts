import { describe, expect, it } from "bun:test";
import { linkRuns } from "../../src/primitives/display/link";

describe("link primitive helpers", () => {
	it("renders unselected link runs with href suffix when different", () => {
		const runs = linkRuns("Install", {
			href: "/docs/installation",
		});

		expect(runs.length).toBe(2);
		expect(runs[0]?.intent).toBe("linkText");
		expect(runs[0]?.underline).toBe(true);
		expect(runs[0]?.backgroundIntent).toBeUndefined();
		expect(runs[1]?.text).toContain("/docs/installation");
	});

	it("renders selected link runs with active background", () => {
		const runs = linkRuns("Install", {
			href: "/docs/installation",
			isSelected: true,
		});

		expect(runs[0]?.intent).toBe("linkActiveText");
		expect(runs[0]?.backgroundIntent).toBe("linkActiveBackground");
		expect(runs[0]?.bold).toBe(true);
		expect(runs[1]?.backgroundIntent).toBe("linkActiveBackground");
	});
});
