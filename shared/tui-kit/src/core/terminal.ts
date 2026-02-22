import { toPositiveIntOrFallback } from "../internal/numbers";

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

export function getTerminalSize(fallback: TerminalSize = DEFAULT_TERMINAL_SIZE): TerminalSize {
  const safeFallback = normalizeTerminalSize(fallback, DEFAULT_TERMINAL_SIZE);
  const columns = process.stdout.columns;
  const rows = process.stdout.rows;

  return normalizeTerminalSize(
    {
      columns,
      rows,
    },
    safeFallback,
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
