import { resolve } from "node:path";
import type { DiskMode } from "../cli/parse-args";

export const REPO_ROOT = resolve(import.meta.dir, "../../../../../../");
export const RECPART_ROOT = resolve(REPO_ROOT, "tools/recpart");
export const RECPART_MANIFEST = resolve(RECPART_ROOT, "Cargo.toml");

export type PlanRunResult = {
	command: string[];
	exitCode: number;
	stdout: string;
	stderr: string;
	jsonLine: string | null;
};

export type ApplyRunResult = {
	command: string[];
	exitCode: number;
	stdout: string;
	stderr: string;
	jsonLine: string | null;
};

export type DiskCandidate = {
	path: string;
	size: string;
	model: string;
	vendor: string;
	serial: string;
	transport: string;
	rotational: boolean | null;
	removable: boolean | null;
	label: string;
};

export function buildPlanCommand(disk: string, mode: DiskMode): string[] {
	return ["recpart", "plan", "--disk", disk, "--mode", mode, "--json"];
}

export function buildApplyCommand(
	disk: string,
	mode: DiskMode,
	options: { dryRun: boolean; confirmDestroy?: boolean },
): string[] {
	const command = ["recpart", "apply", "--disk", disk, "--mode", mode, "--json"];
	if (options.dryRun) {
		command.push("--dry-run");
	} else if (options.confirmDestroy) {
		command.push("--confirm", "DESTROY");
	}
	return command;
}

export function extractInlineJson(output: string): string | null {
	const lines = output.split(/\r?\n/).map((line) => line.trim());
	for (let index = lines.length - 1; index >= 0; index -= 1) {
		const line = lines[index];
		if (line.startsWith("{") && line.endsWith("}")) {
			return line;
		}
	}
	return null;
}

export async function runRecpartPlan(disk: string, mode: DiskMode): Promise<PlanRunResult> {
	const command = buildPlanCommand(disk, mode);
	const proc = Bun.spawn(command, {
		cwd: REPO_ROOT,
		stdout: "pipe",
		stderr: "pipe",
	});

	const [stdout, stderr, exitCode] = await Promise.all([
		new Response(proc.stdout).text(),
		new Response(proc.stderr).text(),
		proc.exited,
	]);

	const combined = [stdout, stderr].filter((part) => part.trim().length > 0).join("\n");
	return {
		command,
		exitCode,
		stdout,
		stderr,
		jsonLine: extractInlineJson(combined),
	};
}

export async function runRecpartApply(
	disk: string,
	mode: DiskMode,
	options: { dryRun: boolean; confirmDestroy?: boolean },
): Promise<ApplyRunResult> {
	const command = buildApplyCommand(disk, mode, options);
	const proc = Bun.spawn(command, {
		cwd: REPO_ROOT,
		stdout: "pipe",
		stderr: "pipe",
	});

	const [stdout, stderr, exitCode] = await Promise.all([
		new Response(proc.stdout).text(),
		new Response(proc.stderr).text(),
		proc.exited,
	]);

	const combined = [stdout, stderr].filter((part) => part.trim().length > 0).join("\n");
	return {
		command,
		exitCode,
		stdout,
		stderr,
		jsonLine: extractInlineJson(combined),
	};
}

type LsblkDevice = {
	path?: string;
	size?: string;
	model?: string;
	vendor?: string;
	serial?: string;
	tran?: string;
	rota?: boolean | number | string;
	rm?: boolean | number | string;
	type?: string;
};

type LsblkJson = {
	blockdevices?: LsblkDevice[];
};

type UdevProps = Record<string, string>;

function humanizeDeviceText(input: string): string {
	return input
		.replaceAll("_", " ")
		.replaceAll(/\s+/g, " ")
		.trim();
}

function parseUdevProperties(raw: string): UdevProps {
	const properties: UdevProps = {};
	for (const line of raw.split(/\r?\n/)) {
		const trimmed = line.trim();
		if (trimmed.length === 0 || !trimmed.includes("=")) {
			continue;
		}
		const pivot = trimmed.indexOf("=");
		const key = trimmed.slice(0, pivot).trim();
		const value = trimmed.slice(pivot + 1).trim();
		if (key.length > 0) {
			properties[key] = value;
		}
	}
	return properties;
}

async function readUdevProperties(devicePath: string): Promise<UdevProps | null> {
	const proc = Bun.spawn(["udevadm", "info", "--query=property", "--name", devicePath], {
		cwd: REPO_ROOT,
		stdout: "pipe",
		stderr: "pipe",
	});
	const [stdout, exitCode] = await Promise.all([new Response(proc.stdout).text(), proc.exited]);
	if (exitCode !== 0) {
		return null;
	}
	return parseUdevProperties(stdout);
}

export async function listHostDisks(): Promise<DiskCandidate[]> {
	const proc = Bun.spawn(
		["lsblk", "-J", "-d", "-o", "PATH,SIZE,MODEL,VENDOR,SERIAL,TRAN,ROTA,RM,TYPE"],
		{
		cwd: REPO_ROOT,
		stdout: "pipe",
		stderr: "pipe",
		},
	);
	const [stdout, exitCode] = await Promise.all([new Response(proc.stdout).text(), proc.exited]);
	if (exitCode !== 0) {
		return [];
	}

	let parsed: LsblkJson | null = null;
	try {
		parsed = JSON.parse(stdout) as LsblkJson;
	} catch {
		return [];
	}

	const raw = parsed.blockdevices ?? [];
	const disks = raw
		.filter((device) => device.type === "disk" && typeof device.path === "string")
		.map((device) => {
			const path = (device.path ?? "").trim();
			const size = (device.size ?? "?").trim();
			const model = (device.model ?? "").trim();
			const vendor = (device.vendor ?? "").trim();
			const serial = (device.serial ?? "").trim();
			const transport = (device.tran ?? "").trim();
			const rotational =
				device.rota === true ||
				device.rota === 1 ||
				device.rota === "1" ||
				device.rota === "true";
			const removable =
				device.rm === true ||
				device.rm === 1 ||
				device.rm === "1" ||
				device.rm === "true";
			const modelPart = model.length > 0 ? ` ${model}` : "";
			return {
				path,
				size,
				model,
				vendor,
				serial,
				transport,
				rotational:
					device.rota === undefined || device.rota === null
						? null
						: rotational,
				removable:
					device.rm === undefined || device.rm === null ? null : removable,
				label: `${path} (${size})${modelPart}`,
			};
		})
		.filter((device) => device.path.length > 0);

	const enriched = await Promise.all(
		disks.map(async (disk) => {
			const udev = await readUdevProperties(disk.path);
			if (!udev) {
				return disk;
			}
			const vendor = humanizeDeviceText(
				udev.ID_VENDOR_FROM_DATABASE ?? udev.ID_VENDOR ?? disk.vendor,
			);
			const model = humanizeDeviceText(
				udev.ID_MODEL_FROM_DATABASE ?? udev.ID_MODEL ?? disk.model,
			);
			const transport = (udev.ID_BUS ?? disk.transport).trim();
			const serial = (udev.ID_SERIAL_SHORT ?? disk.serial).trim();
			const labelModel = model.length > 0 ? ` ${model}` : "";
			return {
				...disk,
				vendor,
				model,
				transport,
				serial,
				label: `${disk.path} (${disk.size})${labelModel}`,
			};
		}),
	);

	return enriched;
}
