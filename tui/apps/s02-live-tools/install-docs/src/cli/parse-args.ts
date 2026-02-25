const REMOVED_FLAGS = new Set(["--list", "-l", "--page", "-p", "--all", "-a", "--tmux-mode"]);

type CliOptions = {
	help: boolean;
	slug?: string;
	error?: string;
};

export function parseCliArgs(args: string[]): CliOptions {
	let slug: string | undefined;

	for (let index = 0; index < args.length; index += 1) {
		const arg = args[index];
		const [flag] = arg.split("=", 1);

		if (arg === "--help" || arg === "-h") {
			return { help: true };
		}

		if (REMOVED_FLAGS.has(flag)) {
			return {
				help: false,
				error: `Flag '${flag}' was removed. Run interactive mode with no flags or use '--slug <page-slug>'.`,
			};
		}

		if (arg.startsWith("--slug=")) {
			const inlineSlug = arg.slice("--slug=".length).trim();
			if (inlineSlug.length === 0) {
				return {
					help: false,
					error: "Flag '--slug' requires a non-empty page slug.",
				};
			}
			slug = inlineSlug;
			continue;
		}

		if (arg === "--slug" || arg === "-s") {
			const value = args[index + 1];
			if (!value || value.startsWith("-")) {
				return {
					help: false,
					error: "Flag '--slug' requires a non-empty page slug.",
				};
			}
			slug = value;
			index += 1;
			continue;
		}

		return {
			help: false,
			error: `Unknown argument '${arg}'. Use '--help' for usage.`,
		};
	}

	return {
		help: false,
		slug,
	};
}
