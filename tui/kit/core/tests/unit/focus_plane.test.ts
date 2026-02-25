import { describe, expect, it } from "bun:test";
import { toggleFocusPlane } from "../../src/hooks/use-focus-plane";

describe("focus plane", () => {
	it("toggles navigation/content deterministically", () => {
		expect(toggleFocusPlane("content")).toBe("navigation");
		expect(toggleFocusPlane("navigation")).toBe("content");
	});
});
