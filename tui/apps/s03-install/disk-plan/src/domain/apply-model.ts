type UnknownRecord = Record<string, unknown>;

function isRecord(value: unknown): value is UnknownRecord {
	return typeof value === "object" && value !== null;
}

function asString(value: unknown): string | null {
	return typeof value === "string" ? value : null;
}

function asNumber(value: unknown): number | null {
	return typeof value === "number" && Number.isFinite(value) ? value : null;
}

function asBoolean(value: unknown): boolean | null {
	return typeof value === "boolean" ? value : null;
}

export type ApplyStep = {
	phase: string;
	command: string;
	status: number | null;
	dryRun: boolean;
};

export type MountEntry = {
	path: string;
	device: string;
};

export type HandoffSummary = {
	installTarget: string;
	mountMap: MountEntry[];
	nextCommands: string[];
	notes: string[];
};

export type ApplySummary = {
	mode: "ab" | "mutable";
	dryRun: boolean;
	steps: ApplyStep[];
	mounted: MountEntry[];
	handoff: HandoffSummary;
	warnings: string[];
};

export function parseApplySummary(jsonLine: string): ApplySummary | null {
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

	const dryRun = asBoolean(parsed.dry_run);
	if (dryRun === null) {
		return null;
	}

	const stepsRaw = Array.isArray(parsed.steps) ? parsed.steps : null;
	if (!stepsRaw) {
		return null;
	}

	const steps: ApplyStep[] = [];
	for (const item of stepsRaw) {
		if (!isRecord(item)) {
			return null;
		}
		const phase = asString(item.phase);
		const command = asString(item.command);
		const statusRaw = item.status;
		const status = statusRaw === null ? null : asNumber(statusRaw);
		const stepDryRun = asBoolean(item.dry_run);
		if (!phase || !command || (statusRaw !== null && status === null) || stepDryRun === null) {
			return null;
		}
		steps.push({
			phase,
			command,
			status,
			dryRun: stepDryRun,
		});
	}

	const mountedRaw = Array.isArray(parsed.mounted) ? parsed.mounted : null;
	if (!mountedRaw) {
		return null;
	}
	const mounted: MountEntry[] = [];
	for (const item of mountedRaw) {
		if (!isRecord(item)) {
			return null;
		}
		const path = asString(item.path);
		const device = asString(item.device);
		if (!path || !device) {
			return null;
		}
		mounted.push({ path, device });
	}

	const handoffRaw = isRecord(parsed.handoff) ? parsed.handoff : null;
	if (!handoffRaw) {
		return null;
	}
	const handoffSchema = asNumber(handoffRaw.schema_version);
	if (handoffSchema !== 1) {
		return null;
	}
	const installTarget = asString(handoffRaw.install_target);
	const nextCommandsRaw = Array.isArray(handoffRaw.next_commands) ? handoffRaw.next_commands : null;
	const mountMapRaw = Array.isArray(handoffRaw.mount_map) ? handoffRaw.mount_map : null;
	const modeContextRaw = isRecord(handoffRaw.mode_context) ? handoffRaw.mode_context : null;
	if (!installTarget || !nextCommandsRaw || !mountMapRaw || !modeContextRaw) {
		return null;
	}
	const nextCommands = nextCommandsRaw.map(asString).filter((value): value is string => value !== null);
	if (nextCommands.length !== nextCommandsRaw.length) {
		return null;
	}
	const mountMap: MountEntry[] = [];
	for (const item of mountMapRaw) {
		if (!isRecord(item)) {
			return null;
		}
		const path = asString(item.path);
		const device = asString(item.device);
		if (!path || !device) {
			return null;
		}
		mountMap.push({ path, device });
	}
	const notesRaw = Array.isArray(modeContextRaw.notes) ? modeContextRaw.notes : null;
	if (!notesRaw) {
		return null;
	}
	const notes = notesRaw.map(asString).filter((value): value is string => value !== null);
	if (notes.length !== notesRaw.length) {
		return null;
	}

	const warningsRaw = Array.isArray(parsed.warnings) ? parsed.warnings : null;
	if (!warningsRaw) {
		return null;
	}
	const warnings = warningsRaw.map(asString).filter((value): value is string => value !== null);
	if (warnings.length !== warningsRaw.length) {
		return null;
	}

	return {
		mode,
		dryRun,
		steps,
		mounted,
		handoff: {
			installTarget,
			mountMap,
			nextCommands,
			notes,
		},
		warnings,
	};
}

export type ErrorSummary = {
	code: string;
	component: string;
	expectation: string;
	observed: string;
	remediation: string;
};

export function parseErrorSummary(jsonLine: string): ErrorSummary | null {
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
	const code = asString(parsed.code);
	const component = asString(parsed.component);
	const expectation = asString(parsed.expectation);
	const observed = asString(parsed.observed);
	const remediation = asString(parsed.remediation);
	if (!code || !component || !expectation || !observed || !remediation) {
		return null;
	}
	return {
		code,
		component,
		expectation,
		observed,
		remediation,
	};
}
