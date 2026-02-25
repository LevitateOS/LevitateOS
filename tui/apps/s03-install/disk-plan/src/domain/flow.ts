import type { DiskMode } from "../cli/parse-args";

export const DISK_PLAN_PAGES = [
	"welcome",
	"target-disk",
	"mode-selection",
	"plan-preview",
	"command-preview",
	"preflight-checks",
	"destructive-confirm",
	"apply-progress",
	"result-handoff",
	"failure-diagnostics",
] as const;

export type DiskPlanPageId = (typeof DISK_PLAN_PAGES)[number];

export type DiskPlanFlowState = {
	pageIndex: number;
	mode: DiskMode;
	disk: string;
};

export function clampPageIndex(value: number): number {
	if (!Number.isFinite(value)) {
		return 0;
	}
	if (value < 0) {
		return 0;
	}
	if (value >= DISK_PLAN_PAGES.length) {
		return DISK_PLAN_PAGES.length - 1;
	}
	return Math.floor(value);
}

export function currentPage(state: DiskPlanFlowState): DiskPlanPageId {
	return DISK_PLAN_PAGES[clampPageIndex(state.pageIndex)] ?? "welcome";
}

export function nextPage(state: DiskPlanFlowState): DiskPlanFlowState {
	return {
		...state,
		pageIndex: clampPageIndex(state.pageIndex + 1),
	};
}

export function prevPage(state: DiskPlanFlowState): DiskPlanFlowState {
	return {
		...state,
		pageIndex: clampPageIndex(state.pageIndex - 1),
	};
}

export function setMode(state: DiskPlanFlowState, mode: DiskMode): DiskPlanFlowState {
	return {
		...state,
		mode,
	};
}

export function canAdvance(state: DiskPlanFlowState): boolean {
	const page = currentPage(state);
	if (page === "target-disk") {
		return state.disk.trim().length > 0;
	}
	return true;
}
