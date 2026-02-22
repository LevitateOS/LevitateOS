import { toFiniteNumber, toPositiveInt } from "../internal/numbers";

export type ProgressState = {
  current: number;
  total: number;
};

export function progressRatio(state: ProgressState): number {
  const total = toFiniteNumber(state.total, 0);
  const current = toFiniteNumber(state.current, 0);

  if (total <= 0) {
    return 0;
  }

  return Math.max(0, Math.min(1, current / total));
}

export function renderProgress(state: ProgressState): string {
  const pct = Math.round(progressRatio(state) * 100);
  return `${pct}%`;
}

export function renderProgressBar(state: ProgressState, width = 20): string {
  const safeWidth = Math.max(3, toPositiveInt(width, 20));
  const pctText = renderProgress(state);
  const ratio = progressRatio(state);
  const filled = Math.round(ratio * safeWidth);
  return `[${"#".repeat(filled)}${"-".repeat(Math.max(0, safeWidth - filled))}] ${pctText}`;
}
