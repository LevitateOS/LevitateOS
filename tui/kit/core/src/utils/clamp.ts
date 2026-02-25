export function toFiniteNumber(value: unknown, fallback: number): number {
	if (typeof value !== "number" || !Number.isFinite(value)) {
		return fallback;
	}

	return value;
}

export function toInt(value: unknown, fallback: number): number {
	return Math.floor(toFiniteNumber(value, fallback));
}

export function toPositiveInt(value: unknown, fallback: number): number {
	return Math.max(1, toInt(value, fallback));
}

export function toPositiveIntOrFallback(value: unknown, fallback: number): number {
	const numeric = toFiniteNumber(value, Number.NaN);
	if (!Number.isFinite(numeric) || numeric < 1) {
		return fallback;
	}
	return Math.floor(numeric);
}

export function toNonNegativeInt(value: unknown, fallback = 0): number {
	return Math.max(0, toInt(value, fallback));
}

export function clampNumber(value: number, min: number, max: number): number {
	const lower = Math.min(min, max);
	const upper = Math.max(min, max);
	return Math.max(lower, Math.min(upper, value));
}

export function clampIndex(index: number, count: number): number {
	if (!Number.isFinite(count) || count <= 0) {
		return 0;
	}

	return clampNumber(index, 0, Math.floor(count) - 1);
}
