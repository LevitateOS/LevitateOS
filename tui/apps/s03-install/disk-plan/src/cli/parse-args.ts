export type DiskMode = "ab" | "mutable";

export type CliOptions = {
	disk: string | null;
	mode: DiskMode;
	help: boolean;
	error: string | null;
};

function parseMode(value: string | undefined): DiskMode | null {
	if (value === "ab" || value === "mutable") {
		return value;
	}
	return null;
}

export function parseCliArgs(args: string[]): CliOptions {
	let disk: string | null = null;
	let mode: DiskMode = "ab";
	let help = false;
	let error: string | null = null;

	for (let index = 0; index < args.length; index += 1) {
		const arg = args[index];

		if (arg === "--help" || arg === "-h") {
			help = true;
			continue;
		}

		if (arg === "--disk") {
			const value = args[index + 1];
			if (!value || value.startsWith("-")) {
				error = "--disk requires a value";
				break;
			}
			disk = value;
			index += 1;
			continue;
		}

		if (arg === "--mode") {
			const value = args[index + 1];
			const parsed = parseMode(value);
			if (!parsed) {
				error = "--mode must be one of: ab, mutable";
				break;
			}
			mode = parsed;
			index += 1;
			continue;
		}

		error = `unknown argument: ${arg}`;
		break;
	}

	return { disk, mode, help, error };
}

export function cliHelpText(): string {
	return [
		"s03 disk-plan (recpart-backed thin shell)",
		"",
		"Usage:",
		"  just tui-s03-disk-plan",
		"  just tui-s03-disk-plan -- --disk /dev/sdX [--mode ab|mutable]",
		"  bun run src/index.ts [--disk /dev/sdX] [--mode ab|mutable]",
		"",
		"Notes:",
		"  - mode defaults to 'ab'",
		"  - --disk is optional (interactive selection if omitted)",
		"  - command runs: recpart plan --disk <disk> --mode <mode> --json",
		"  - q/esc/ctrl-c exits the TUI",
	].join("\n");
}
