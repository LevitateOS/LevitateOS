import { clampIndex } from "../../utils/clamp";

export type FocusManager = {
	current: () => number;
	next: () => number;
	prev: () => number;
	setCount: (count: number) => void;
	set: (index: number) => number;
};

export function createFocusManager(initialCount: number, initialIndex = 0): FocusManager {
	let count = Number.isFinite(initialCount) ? Math.max(0, Math.floor(initialCount)) : 0;
	let index = clampIndex(initialIndex, count);

	const ensure = () => {
		index = clampIndex(index, count);
		return index;
	};

	return {
		current: () => ensure(),
		next: () => {
			if (count <= 0) {
				index = 0;
				return index;
			}
			index = (index + 1) % count;
			return index;
		},
		prev: () => {
			if (count <= 0) {
				index = 0;
				return index;
			}
			index = (index - 1 + count) % count;
			return index;
		},
		setCount: (nextCount: number) => {
			count = Number.isFinite(nextCount) ? Math.max(0, Math.floor(nextCount)) : 0;
			ensure();
		},
		set: (nextIndex: number) => {
			index = clampIndex(nextIndex, count);
			return index;
		},
	};
}
