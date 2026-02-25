import { describe, expect, it } from "bun:test";
import { createFocusManager } from "../../src/runtime/ink/focus-manager";

describe("focus manager", () => {
	it("cycles next/prev inside range", () => {
		const focus = createFocusManager(3, 0);

		expect(focus.current()).toBe(0);
		expect(focus.next()).toBe(1);
		expect(focus.next()).toBe(2);
		expect(focus.next()).toBe(0);
		expect(focus.prev()).toBe(2);
	});

	it("clamps and updates when count changes", () => {
		const focus = createFocusManager(5, 4);

		expect(focus.current()).toBe(4);
		focus.setCount(2);
		expect(focus.current()).toBe(1);

		focus.set(10);
		expect(focus.current()).toBe(1);

		focus.setCount(0);
		expect(focus.current()).toBe(0);
		expect(focus.next()).toBe(0);
		expect(focus.prev()).toBe(0);
	});
});
