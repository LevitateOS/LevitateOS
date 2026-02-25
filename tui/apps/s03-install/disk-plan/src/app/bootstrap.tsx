import { createTuiApp, renderApp, type RenderedTuiApp } from "@levitate/tui-kit";
import type { DiskMode } from "../cli/parse-args";
import { DiskPlanScreen } from "../presentation/disk-plan-screen";

type StartDiskPlanOptions = {
	disk?: string;
	mode: DiskMode;
};

export function startDiskPlanApp(options: StartDiskPlanOptions): RenderedTuiApp {
	const app = createTuiApp({ title: "levitate-s03-disk-plan" });
	return renderApp(<DiskPlanScreen disk={options.disk ?? ""} mode={options.mode} />, {
		app,
		exitOnCtrlC: false,
	});
}
