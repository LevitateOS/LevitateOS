import { startInstallDocsApp } from "../app/bootstrap";
import { AppError, formatErrorMessage } from "../app/errors";
import { installDocsCliHelpText } from "./help";
import { parseCliArgs } from "./parse-args";

export function runCli(args: string[] = process.argv.slice(2)): void {
	const options = parseCliArgs(args);

	if (options.help) {
		console.log(installDocsCliHelpText());
		return;
	}

	if (options.error) {
		console.error(options.error);
		process.exit(2);
	}

	try {
		const mounted = startInstallDocsApp({ slug: options.slug });
		void mounted.waitUntilExit().catch((error: unknown) => {
			console.error(`Docs TUI exited with error: ${formatErrorMessage(error)}`);
			process.exit(1);
		});
	} catch (error: unknown) {
		if (error instanceof AppError) {
			console.error(error.message);
			process.exit(error.exitCode);
		}

		console.error(`Failed to start Docs TUI: ${formatErrorMessage(error)}`);
		process.exit(1);
	}
}
