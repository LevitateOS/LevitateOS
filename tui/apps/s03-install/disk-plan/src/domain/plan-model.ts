type UnknownRecord = Record<string, unknown>;

export type PlanPartition = {
	index: number;
	name: string;
	filesystem: string;
	label: string;
	mountpoint: string;
	sizeMb: number | null;
};

export type PlanSummary = {
	mode: "ab" | "mutable";
	diskPath: string;
	diskSizeBytes: number;
	partitions: PlanPartition[];
	sfdiskScript: string;
};

function isRecord(value: unknown): value is UnknownRecord {
	return typeof value === "object" && value !== null;
}

function asString(value: unknown): string | null {
	return typeof value === "string" ? value : null;
}

function asNumber(value: unknown): number | null {
	return typeof value === "number" && Number.isFinite(value) ? value : null;
}

export function formatBytes(value: number): string {
	if (!Number.isFinite(value) || value <= 0) {
		return "0 B";
	}
	const units = ["B", "KB", "MB", "GB", "TB"];
	let size = value;
	let unitIndex = 0;
	while (size >= 1024 && unitIndex < units.length - 1) {
		size /= 1024;
		unitIndex += 1;
	}
	return `${size.toFixed(unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
}

export function parsePlanSummary(jsonLine: string): PlanSummary | null {
	let parsed: unknown;
	try {
		parsed = JSON.parse(jsonLine);
	} catch {
		return null;
	}
	if (!isRecord(parsed)) {
		return null;
	}

	const schemaVersion = asNumber(parsed.schema_version);
	if (schemaVersion !== 1) {
		return null;
	}

	const mode = asString(parsed.mode);
	if (mode !== "ab" && mode !== "mutable") {
		return null;
	}

	const disk = isRecord(parsed.disk) ? parsed.disk : null;
	if (!disk) {
		return null;
	}
	const diskPath = asString(disk.path);
	const diskSizeBytes = asNumber(disk.size_bytes);
	if (!diskPath || diskSizeBytes === null) {
		return null;
	}

	const sfdiskScript = asString(parsed.sfdisk_script);
	if (!sfdiskScript) {
		return null;
	}

	const partitionsRaw = Array.isArray(parsed.partitions) ? parsed.partitions : null;
	if (!partitionsRaw) {
		return null;
	}

	const partitions: PlanPartition[] = [];
	for (const item of partitionsRaw) {
		if (!isRecord(item)) {
			return null;
		}
		const index = asNumber(item.index);
		const name = asString(item.name);
		const filesystem = asString(item.filesystem);
		const label = asString(item.label);
		const mountpoint = asString(item.mountpoint);
		const rawSize = item.size_mb;
		const sizeMb = rawSize === null ? null : asNumber(rawSize);
		if (
			index === null ||
			!name ||
			!filesystem ||
			!label ||
			!mountpoint ||
			(rawSize !== null && sizeMb === null)
		) {
			return null;
		}
		partitions.push({
			index,
			name,
			filesystem,
			label,
			mountpoint,
			sizeMb,
		});
	}

	return {
		mode,
		diskPath,
		diskSizeBytes,
		partitions,
		sfdiskScript,
	};
}
