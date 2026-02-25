export function uniqueStrings(values: ReadonlyArray<string>): string[] {
	return Array.from(
		new Set(
			values
				.map((value) => (typeof value === "string" ? value.trim() : ""))
				.filter((value) => value.length > 0),
		),
	);
}

export function chunkArray<T>(values: ReadonlyArray<T>, size: number): T[][] {
	if (!Number.isFinite(size) || size < 1) {
		return [Array.from(values)];
	}

	const chunks: T[][] = [];
	for (let index = 0; index < values.length; index += size) {
		chunks.push(Array.from(values.slice(index, index + size)));
	}

	return chunks;
}
