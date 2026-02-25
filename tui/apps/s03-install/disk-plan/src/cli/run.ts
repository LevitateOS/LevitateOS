import { startDiskPlanApp } from "../app/bootstrap";
import { cliHelpText, parseCliArgs } from "./parse-args";

export function runCli(args: string[] = process.argv.slice(2)): void {
	const options = parseCliArgs(args);

	if (options.help) {
		console.log(cliHelpText());
		return;
	}
	if (options.error) {
		console.error(options.error);
		process.exit(2);
	}
	const mounted = startDiskPlanApp({
		disk: options.disk ?? undefined,
		mode: options.mode,
	});
	void mounted.waitUntilExit().catch((error: unknown) => {
		const message = error instanceof Error ? error.message : String(error);
		console.error(`Disk-plan TUI exited with error: ${message}`);
		process.exit(1);
	});
}
