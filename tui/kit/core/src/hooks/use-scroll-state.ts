import { useState } from "react";
import { clampNumber, toNonNegativeInt } from "../utils/clamp";

export type ScrollState = {
	scrollOffset: number;
	scrollBy: (delta: number, maxScroll: number) => void;
	scrollToTop: () => void;
	scrollToBottom: (maxScroll?: number) => void;
	reset: () => void;
};

export function useScrollState(initialOffset = 0): ScrollState {
	const [scrollOffset, setScrollOffset] = useState(() => toNonNegativeInt(initialOffset, 0));

	const scrollBy = (delta: number, maxScroll: number) => {
		const safeMaxScroll = toNonNegativeInt(maxScroll, 0);
		setScrollOffset((value) => clampNumber(value + delta, 0, safeMaxScroll));
	};

	return {
		scrollOffset,
		scrollBy,
		scrollToTop: () => setScrollOffset(0),
		scrollToBottom: (maxScroll = Number.MAX_SAFE_INTEGER) =>
			setScrollOffset(toNonNegativeInt(maxScroll, Number.MAX_SAFE_INTEGER)),
		reset: () => setScrollOffset(0),
	};
}
