import { toPositiveIntOrFallback } from "../../utils/clamp";

export type TerminalSize = {
	columns: number;
	rows: number;
};

export const DEFAULT_TERMINAL_SIZE: TerminalSize = {
	columns: 80,
	rows: 24,
};

function normalizeDimension(value: unknown, fallback: number): number {
	return toPositiveIntOrFallback(value, fallback);
}

export function normalizeTerminalSize(
	size: Partial<TerminalSize>,
	fallback: TerminalSize = DEFAULT_TERMINAL_SIZE,
): TerminalSize {
	return {
		columns: normalizeDimension(size.columns, fallback.columns),
		rows: normalizeDimension(size.rows, fallback.rows),
	};
}

export function readScreenSize(
	target?: { columns?: number; rows?: number },
	fallback: TerminalSize = DEFAULT_TERMINAL_SIZE,
): TerminalSize {
	const safeFallback = normalizeTerminalSize(fallback, DEFAULT_TERMINAL_SIZE);

	return normalizeTerminalSize(
		{
			columns: target?.columns,
			rows: target?.rows,
		},
		safeFallback,
	);
}

export function getTerminalSize(fallback: TerminalSize = DEFAULT_TERMINAL_SIZE): TerminalSize {
	return readScreenSize(
		{
			columns: process.stdout.columns,
			rows: process.stdout.rows,
		},
		fallback,
	);
}

export function terminalMeetsMinimum(
	size: TerminalSize,
	minimum: { columns: number; rows: number },
): boolean {
	const normalizedSize = normalizeTerminalSize(size, DEFAULT_TERMINAL_SIZE);
	const normalizedMinimum = normalizeTerminalSize(minimum, DEFAULT_TERMINAL_SIZE);

	return (
		normalizedSize.columns >= normalizedMinimum.columns &&
		normalizedSize.rows >= normalizedMinimum.rows
	);
}
