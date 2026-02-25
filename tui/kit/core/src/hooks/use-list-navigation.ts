import { useEffect, useState } from "react";
import { clampIndex } from "../utils/clamp";

export type ListNavigationState = {
	currentIndex: number;
	safeIndex: number;
	moveBy: (delta: number) => void;
	setIndex: (index: number) => void;
};

function safeCount(count: number): number {
	if (!Number.isFinite(count) || count <= 0) {
		return 0;
	}

	return Math.floor(count);
}

export function useListNavigation(count: number, initialIndex = 0): ListNavigationState {
	const boundedCount = safeCount(count);
	const [currentIndex, setCurrentIndex] = useState(() => clampIndex(initialIndex, boundedCount));

	useEffect(() => {
		setCurrentIndex((value) => clampIndex(value, boundedCount));
	}, [boundedCount]);

	return {
		currentIndex,
		safeIndex: clampIndex(currentIndex, boundedCount),
		moveBy: (delta: number) => {
			setCurrentIndex((value) => clampIndex(value + delta, boundedCount));
		},
		setIndex: (index: number) => {
			setCurrentIndex(clampIndex(index, boundedCount));
		},
	};
}
