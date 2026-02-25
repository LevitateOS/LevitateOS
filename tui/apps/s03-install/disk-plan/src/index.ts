#!/usr/bin/env bun

export {
	RECPART_MANIFEST,
	RECPART_ROOT,
	buildPlanCommand,
	extractInlineJson,
	runRecpartPlan,
} from "./app/plan-runner";
export { cliHelpText, parseCliArgs } from "./cli/parse-args";
export { runCli } from "./cli/run";

import { runCli } from "./cli/run";

if (import.meta.main) {
	runCli();
}
