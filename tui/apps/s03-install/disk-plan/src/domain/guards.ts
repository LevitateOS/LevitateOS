import type { DiskPlanPageId } from "./flow";

export type AdvanceContext = {
	hasDisk: boolean;
	planReady: boolean;
	commandPreviewReady: boolean;
	preflightPassing: boolean;
	confirmUnlocked: boolean;
	applyFinished: boolean;
};

export function canAdvancePage(page: DiskPlanPageId, ctx: AdvanceContext): boolean {
	if (page === "target-disk") {
		return ctx.hasDisk;
	}
	if (page === "plan-preview") {
		return ctx.planReady;
	}
	if (page === "command-preview") {
		return ctx.commandPreviewReady;
	}
	if (page === "preflight-checks") {
		return ctx.preflightPassing;
	}
	if (page === "destructive-confirm") {
		return ctx.confirmUnlocked;
	}
	if (page === "apply-progress") {
		return ctx.applyFinished;
	}
	return true;
}
