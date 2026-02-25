#!/usr/bin/env bun

import { runCli } from "./cli/run";

if (import.meta.main) {
	runCli();
}
