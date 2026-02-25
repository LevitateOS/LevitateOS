export const spacing = {
	xs: 0,
	sm: 1,
	md: 2,
	lg: 3,
	xl: 4,
} as const;

export type SpacingToken = keyof typeof spacing;

export function resolveSpacing(token: SpacingToken | number): number {
	if (typeof token === "number") {
		return Number.isFinite(token) ? Math.max(0, Math.floor(token)) : 0;
	}

	return spacing[token] ?? 0;
}
