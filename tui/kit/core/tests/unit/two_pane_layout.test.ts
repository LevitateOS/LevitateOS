import { describe, expect, it } from "bun:test";
import { resolveTwoPaneGeometry } from "../../src/patterns/two-pane";

describe("two pane geometry", () => {
	it("keeps docs legible at half-screen dimensions", () => {
		const geometry = resolveTwoPaneGeometry({
			columns: 80,
			rows: 24,
			requestedSidebarWidth: 28,
			headerHeight: 2,
			footerHeight: 2,
			hasFooter: true,
		});

		expect(geometry.frameColumns).toBe(80);
		expect(geometry.sidebarOuterWidth).toBe(28);
		expect(geometry.contentOuterWidth).toBe(52);
		expect(geometry.sidebarTextColumns).toBeGreaterThanOrEqual(20);
		expect(geometry.contentTextColumns).toBeGreaterThanOrEqual(40);
		expect(geometry.contentTextRows).toBe(18);
	});

	it("allows oversized sidebar requests when no layout guards are supplied", () => {
		const geometry = resolveTwoPaneGeometry({
			columns: 90,
			rows: 28,
			requestedSidebarWidth: 60,
			headerHeight: 2,
			footerHeight: 2,
			hasFooter: true,
		});

		expect(geometry.sidebarOuterWidth).toBe(60);
		expect(geometry.contentOuterWidth).toBe(30);
	});

	it("caps oversized sidebar requests when layout guards are provided", () => {
		const geometry = resolveTwoPaneGeometry({
			columns: 90,
			rows: 28,
			requestedSidebarWidth: 60,
			headerHeight: 2,
			footerHeight: 2,
			hasFooter: true,
			layoutGuards: {
				minLeftOuterWidth: 20,
				minRightOuterWidth: 42,
				maxLeftWidthRatio: 0.4,
			},
		});

		expect(geometry.sidebarOuterWidth).toBeLessThanOrEqual(Math.floor(90 * 0.4));
		expect(geometry.contentOuterWidth).toBeGreaterThanOrEqual(42);
	});
});
