import { useCallback, useEffect, useMemo, useState } from "react";
import { clampIndex, toPositiveInt } from "../utils/clamp";

export type GridDirection = "left" | "right" | "up" | "down";

export type GridNavigationOptions = {
	count: number;
	columns: number;
	initialIndex?: number;
	disabled?: boolean;
};

export type GridNavigationState = {
	currentIndex: number;
	safeIndex: number;
	row: number;
	column: number;
	setIndex: (index: number) => void;
	move: (direction: GridDirection) => void;
	moveLeft: () => void;
	moveRight: () => void;
	moveUp: () => void;
	moveDown: () => void;
};

function safeCount(count: number): number {
	if (!Number.isFinite(count) || count <= 0) {
		return 0;
	}
	return Math.floor(count);
}

export function transitionGridIndex(
	currentIndex: number,
	count: number,
	columns: number,
	direction: GridDirection,
): number {
	const safeTotal = safeCount(count);
	if (safeTotal <= 0) {
		return 0;
	}
	const safeColumns = toPositiveInt(columns, 1);
	const safeCurrent = clampIndex(currentIndex, safeTotal);
	const lastIndex = safeTotal - 1;
	const rowStart = Math.floor(safeCurrent / safeColumns) * safeColumns;
	const rowEnd = Math.min(rowStart + safeColumns - 1, lastIndex);

	if (direction === "left") {
		return Math.max(safeCurrent - 1, rowStart);
	}
	if (direction === "right") {
		return Math.min(safeCurrent + 1, rowEnd);
	}
	if (direction === "up") {
		return Math.max(safeCurrent - safeColumns, 0);
	}
	return Math.min(safeCurrent + safeColumns, lastIndex);
}

export function useGridNavigation({
	count,
	columns,
	initialIndex = 0,
	disabled = false,
}: GridNavigationOptions): GridNavigationState {
	const safeTotal = safeCount(count);
	const safeColumns = toPositiveInt(columns, 1);
	const [currentIndex, setCurrentIndex] = useState(() => clampIndex(initialIndex, safeTotal));

	useEffect(() => {
		setCurrentIndex((value) => clampIndex(value, safeTotal));
	}, [safeTotal]);

	const safeIndex = clampIndex(currentIndex, safeTotal);

	const setIndex = useCallback(
		(index: number) => {
			setCurrentIndex(clampIndex(index, safeTotal));
		},
		[safeTotal],
	);

	const move = useCallback(
		(direction: GridDirection) => {
			if (disabled) {
				return;
			}
			setCurrentIndex((value) => transitionGridIndex(value, safeTotal, safeColumns, direction));
		},
		[disabled, safeColumns, safeTotal],
	);

	const moveLeft = useCallback(() => move("left"), [move]);
	const moveRight = useCallback(() => move("right"), [move]);
	const moveUp = useCallback(() => move("up"), [move]);
	const moveDown = useCallback(() => move("down"), [move]);
	const row = Math.floor(safeIndex / safeColumns);
	const column = safeIndex % safeColumns;

	return useMemo(
		() => ({
			currentIndex,
			safeIndex,
			row,
			column,
			setIndex,
			move,
			moveLeft,
			moveRight,
			moveUp,
			moveDown,
		}),
		[column, currentIndex, move, moveDown, moveLeft, moveRight, moveUp, row, safeIndex, setIndex],
	);
}
