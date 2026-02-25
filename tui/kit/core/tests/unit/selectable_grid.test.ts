import { describe, expect, it } from "bun:test";
import { mapSelectableGridItems } from "../../src/components/navigation/selectable-grid";

describe("selectable grid", () => {
	it("maps selected context to one item only", () => {
		const values = mapSelectableGridItems(["a", "b", "c"], 1, (item, context) => ({
			item,
			selected: context.selected,
			index: context.index,
		}));
		expect(values).toEqual([
			{ item: "a", selected: false, index: 0 },
			{ item: "b", selected: true, index: 1 },
			{ item: "c", selected: false, index: 2 },
		]);
	});

	it("clamps out-of-range selection index", () => {
		const values = mapSelectableGridItems(["a", "b"], 999, (_item, context) => context.selected);
		expect(values).toEqual([false, true]);
	});
});
