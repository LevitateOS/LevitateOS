import { Box, Text, useApp, useInput } from "ink";
import { useEffect, useMemo, useState } from "react";
import type { ReactNode } from "react";
import {
	SelectableGrid,
	SegmentedStatusLine,
	SurfaceFrame,
	UiText,
	hotkeyStatusSegment,
	resolveGridColumns,
	scopeStatusSegment,
	textStatusSegment,
	useGridNavigation,
	useHotkeys,
	useTuiViewport,
} from "@levitate/tui-kit";
import type { DiskMode } from "../cli/parse-args";
import {
	listHostDisks,
	runRecpartApply,
	runRecpartPlan,
	type DiskCandidate,
} from "../app/plan-runner";
import { runPreflightChecks, type PreflightCheck } from "../app/preflight";
import {
	currentPage,
	nextPage,
	prevPage,
	setMode,
	type DiskPlanFlowState,
	type DiskPlanPageId,
	DISK_PLAN_PAGES,
} from "../domain/flow";
import { canAdvancePage } from "../domain/guards";
import { formatBytes, parsePlanSummary, type PlanSummary } from "../domain/plan-model";
import {
	parseApplySummary,
	parseErrorSummary,
	type ApplySummary,
	type ErrorSummary,
} from "../domain/apply-model";
import { confirmStatusLine, failureDiagnosticLines, progressStepLine } from "./page-text";

type AsyncState<T> =
	| { kind: "idle" }
	| { kind: "running" }
	| { kind: "success"; value: T }
	| {
			kind: "failure";
			raw: { stdout: string; stderr: string; jsonLine: string | null; exitCode: number };
	  }
	| { kind: "error"; message: string };

type FailureDiagnostic = {
	source: "plan" | "apply-dry-run" | "preflight" | "apply";
	detail: ErrorSummary | null;
	observed: string;
	remediation: string;
};

type DiskPlanScreenProps = {
	disk: string;
	mode: DiskMode;
};

function limitLines(input: string, maxLines = 16): string {
	const lines = input.split(/\r?\n/);
	if (lines.length <= maxLines) {
		return input.trimEnd();
	}
	return `${lines.slice(0, maxLines).join("\n")}\n... (${lines.length - maxLines} more lines)`;
}

function pageLabel(page: DiskPlanPageId): string {
	return page;
}

function isPlanReady(plan: AsyncState<PlanSummary>): boolean {
	return plan.kind === "success";
}

function isDryRunReady(applyDryRun: AsyncState<ApplySummary>): boolean {
	return applyDryRun.kind === "success";
}

function arePreflightChecksPassing(preflight: AsyncState<PreflightCheck[]>): boolean {
	return preflight.kind === "success" && preflight.value.every((check) => check.ok);
}

function isApplyFinished(apply: AsyncState<ApplySummary>): boolean {
	return apply.kind === "success" || apply.kind === "failure" || apply.kind === "error";
}

function displayVendor(vendor: string): string | null {
	const normalized = vendor.trim();
	if (normalized.length === 0) {
		return null;
	}
	const generic = new Set(["ATA", "NVME", "SCSI"]);
	if (generic.has(normalized.toUpperCase())) {
		return null;
	}
	return normalized;
}

function hasInteractiveContent(page: DiskPlanPageId): boolean {
	return page === "target-disk" || page === "mode-selection" || page === "destructive-confirm";
}

export function DiskPlanScreen({ disk, mode }: DiskPlanScreenProps) {
	const { exit } = useApp();
	const [flow, setFlow] = useState<DiskPlanFlowState>({
		pageIndex: 0,
		mode,
		disk,
	});
	const [planState, setPlanState] = useState<AsyncState<PlanSummary>>({ kind: "idle" });
	const [applyDryRunState, setApplyDryRunState] = useState<AsyncState<ApplySummary>>({
		kind: "idle",
	});
	const [preflightState, setPreflightState] = useState<AsyncState<PreflightCheck[]>>({
		kind: "idle",
	});
	const [applyState, setApplyState] = useState<AsyncState<ApplySummary>>({ kind: "idle" });
	const [confirmToken, setConfirmToken] = useState("");
	const [failure, setFailure] = useState<FailureDiagnostic | null>(null);
	const [diskCandidates, setDiskCandidates] = useState<DiskCandidate[]>([]);
	const [activePane, setActivePane] = useState<"nav" | "content">("nav");
	const viewport = useTuiViewport();

	const page = currentPage(flow);
	const diskColumnsSpec = useMemo(
		() => ({
			base: 1,
			breakpoints: [
				{ minWidth: 110, columns: 2 },
				{ minWidth: 180, columns: 4 },
			],
		}),
		[],
	);
	const diskGridColumns = resolveGridColumns(diskColumnsSpec, viewport.columns, 1);
	const diskNav = useGridNavigation({
		count: diskCandidates.length,
		columns: diskGridColumns,
		initialIndex: 0,
		disabled: page !== "target-disk",
	});

	const selectDiskAtIndex = (index: number) => {
		if (diskCandidates.length === 0) {
			return;
		}
		const bounded = Math.max(0, Math.min(index, diskCandidates.length - 1));
		const nextDisk = diskCandidates[bounded]?.path ?? "";
		diskNav.setIndex(bounded);
		if (nextDisk.length > 0) {
			setFlow((state) => ({ ...state, disk: nextDisk }));
		}
	};

	const setFailureFromRun = (
		source: FailureDiagnostic["source"],
		run: { stderr: string; stdout: string; jsonLine: string | null },
	) => {
		const detail = run.jsonLine ? parseErrorSummary(run.jsonLine) : null;
		setFailure({
			source,
			detail,
			observed: (run.stderr || run.stdout || "(no output)").trim(),
			remediation:
				detail?.remediation ?? "Inspect command stderr and fix prerequisites before retry.",
		});
	};

	useEffect(() => {
		void listHostDisks().then((devices) => {
			setDiskCandidates(devices);
			if (devices.length === 0) {
				return;
			}
			const exact = devices.findIndex((item) => item.path === flow.disk);
			const resolved = exact >= 0 ? exact : 0;
			diskNav.setIndex(resolved);
			if (flow.disk.trim().length === 0) {
				const first = devices[resolved]?.path ?? "";
				if (first.length > 0) {
					setFlow((state) => ({ ...state, disk: first }));
				}
			}
		});
	}, [diskNav.setIndex, flow.disk]);

	useEffect(() => {
		if (page !== "target-disk") {
			return;
		}
		const selectedPath = diskCandidates[diskNav.safeIndex]?.path ?? "";
		if (selectedPath.length === 0 || selectedPath === flow.disk) {
			return;
		}
		setFlow((state) => ({ ...state, disk: selectedPath }));
	}, [diskCandidates, diskNav.safeIndex, flow.disk, page]);

	useInput((input, key) => {
		if (page !== "destructive-confirm") {
			return;
		}
		if (key.backspace || key.delete) {
			setConfirmToken((value) => value.slice(0, -1));
			return;
		}
		if (key.return) {
			return;
		}
		if (key.ctrl || key.meta || input.length !== 1) {
			return;
		}
		if (!/[a-zA-Z]/.test(input)) {
			return;
		}
		setConfirmToken((value) => (value + input.toUpperCase()).slice(0, 7));
	});

	useEffect(() => {
		if (page !== "plan-preview") {
			return;
		}
		let cancelled = false;
		setPlanState({ kind: "running" });
		void runRecpartPlan(flow.disk, flow.mode)
			.then((result) => {
				if (cancelled) {
					return;
				}
				if (result.exitCode !== 0) {
					setPlanState({
						kind: "failure",
						raw: {
							stdout: result.stdout,
							stderr: result.stderr,
							jsonLine: result.jsonLine,
							exitCode: result.exitCode,
						},
					});
					setFailureFromRun("plan", result);
					return;
				}
				if (!result.jsonLine) {
					setPlanState({
						kind: "error",
						message: "plan command succeeded but no JSON payload was emitted",
					});
					return;
				}
				const summary = parsePlanSummary(result.jsonLine);
				if (!summary) {
					setPlanState({
						kind: "error",
						message: "plan JSON failed schema validation (expected schema_version=1)",
					});
					return;
				}
				setPlanState({ kind: "success", value: summary });
			})
			.catch((error: unknown) => {
				if (cancelled) {
					return;
				}
				const message = error instanceof Error ? error.message : String(error);
				setPlanState({ kind: "error", message });
			});
		return () => {
			cancelled = true;
		};
	}, [flow.disk, flow.mode, page]);

	useEffect(() => {
		if (page !== "command-preview") {
			return;
		}
		let cancelled = false;
		setApplyDryRunState({ kind: "running" });
		void runRecpartApply(flow.disk, flow.mode, { dryRun: true })
			.then((result) => {
				if (cancelled) {
					return;
				}
				if (result.exitCode !== 0) {
					setApplyDryRunState({
						kind: "failure",
						raw: {
							stdout: result.stdout,
							stderr: result.stderr,
							jsonLine: result.jsonLine,
							exitCode: result.exitCode,
						},
					});
					setFailureFromRun("apply-dry-run", result);
					return;
				}
				if (!result.jsonLine) {
					setApplyDryRunState({
						kind: "error",
						message: "apply dry-run succeeded but no JSON payload was emitted",
					});
					return;
				}
				const summary = parseApplySummary(result.jsonLine);
				if (!summary) {
					setApplyDryRunState({
						kind: "error",
						message: "apply dry-run JSON failed schema validation (expected schema_version=1)",
					});
					return;
				}
				setApplyDryRunState({ kind: "success", value: summary });
			})
			.catch((error: unknown) => {
				if (cancelled) {
					return;
				}
				const message = error instanceof Error ? error.message : String(error);
				setApplyDryRunState({ kind: "error", message });
			});
		return () => {
			cancelled = true;
		};
	}, [flow.disk, flow.mode, page]);

	useEffect(() => {
		if (page !== "preflight-checks") {
			return;
		}
		let cancelled = false;
		setPreflightState({ kind: "running" });
		void runPreflightChecks(flow.disk)
			.then((checks) => {
				if (cancelled) {
					return;
				}
				setPreflightState({ kind: "success", value: checks });
				const failed = checks.find((check) => !check.ok);
				if (failed) {
					setFailure({
						source: "preflight",
						detail: null,
						observed: `${failed.expectation}; observed: ${failed.observed}`,
						remediation: failed.remediation,
					});
				}
			})
			.catch((error: unknown) => {
				if (cancelled) {
					return;
				}
				const message = error instanceof Error ? error.message : String(error);
				setPreflightState({ kind: "error", message });
			});
		return () => {
			cancelled = true;
		};
	}, [flow.disk, page]);

	useEffect(() => {
		if (page !== "apply-progress") {
			return;
		}
		if (confirmToken !== "DESTROY") {
			setApplyState({
				kind: "error",
				message: "destructive apply is locked until confirmation token matches DESTROY",
			});
			return;
		}
		let cancelled = false;
		setApplyState({ kind: "running" });
		void runRecpartApply(flow.disk, flow.mode, { dryRun: false, confirmDestroy: true })
			.then((result) => {
				if (cancelled) {
					return;
				}
				if (result.exitCode !== 0) {
					setApplyState({
						kind: "failure",
						raw: {
							stdout: result.stdout,
							stderr: result.stderr,
							jsonLine: result.jsonLine,
							exitCode: result.exitCode,
						},
					});
					setFailureFromRun("apply", result);
					return;
				}
				if (!result.jsonLine) {
					setApplyState({
						kind: "error",
						message: "apply succeeded but no JSON payload was emitted",
					});
					return;
				}
				const summary = parseApplySummary(result.jsonLine);
				if (!summary) {
					setApplyState({
						kind: "error",
						message: "apply JSON failed schema validation (expected schema_version=1)",
					});
					return;
				}
				setApplyState({ kind: "success", value: summary });
			})
			.catch((error: unknown) => {
				if (cancelled) {
					return;
				}
				const message = error instanceof Error ? error.message : String(error);
				setApplyState({ kind: "error", message });
			});

		return () => {
			cancelled = true;
		};
	}, [confirmToken, flow.disk, flow.mode, page]);

	const quit = () => exit();
	const isDiskPage = page === "target-disk";
	const isModePage = page === "mode-selection";
	const isNavFocus = activePane === "nav";
	const isContentFocus = activePane === "content";
	const pageAllowsContentPane = hasInteractiveContent(page);
	const toggleMode = () => {
		setFlow((state) => setMode(state, state.mode === "ab" ? "mutable" : "ab"));
	};
	const clampPage = (index: number) =>
		Math.max(0, Math.min(DISK_PLAN_PAGES.length - 1, index));
	const onNext = () => {
		if (
			!canAdvancePage(page, {
				hasDisk: flow.disk.trim().length > 0,
				planReady: isPlanReady(planState),
				commandPreviewReady: isDryRunReady(applyDryRunState),
				preflightPassing: arePreflightChecksPassing(preflightState),
				confirmUnlocked: confirmToken === "DESTROY",
				applyFinished: isApplyFinished(applyState),
			})
		) {
			return;
		}
		if (page === "apply-progress") {
			if (applyState.kind === "success") {
				setFlow((state) => ({ ...state, pageIndex: DISK_PLAN_PAGES.indexOf("result-handoff") }));
			} else {
				setFlow((state) => ({
					...state,
					pageIndex: DISK_PLAN_PAGES.indexOf("failure-diagnostics"),
				}));
			}
			return;
		}
		setFlow((state) => nextPage(state));
	};

	const onPrev = () => {
		setFlow((state) => prevPage(state));
	};

	useEffect(() => {
		if (activePane === "content" && !pageAllowsContentPane) {
			setActivePane("nav");
		}
	}, [activePane, pageAllowsContentPane]);

	useHotkeys(["q", "escape", "C-c"], quit);
	useHotkeys(["tab", "S-tab"], () => {
		if (!pageAllowsContentPane) {
			setActivePane("nav");
			return;
		}
		setActivePane((pane) => (pane === "nav" ? "content" : "nav"));
	});
	useHotkeys(["n"], () => {
		if (!isContentFocus) {
			return;
		}
		onNext();
	});
	useHotkeys(["p"], () => {
		if (!isContentFocus) {
			return;
		}
		onPrev();
	});
	useHotkeys(["right", "l"], () => {
		if (isNavFocus) {
			setFlow((state) => ({ ...state, pageIndex: clampPage(state.pageIndex + 1) }));
			return;
		}
		if (!isContentFocus) {
			return;
		}
		if (isDiskPage) {
			diskNav.moveRight();
			return;
		}
		if (isModePage) {
			toggleMode();
			return;
		}
	});
	useHotkeys(["left", "h"], () => {
		if (isNavFocus) {
			setFlow((state) => ({ ...state, pageIndex: clampPage(state.pageIndex - 1) }));
			return;
		}
		if (!isContentFocus) {
			return;
		}
		if (isDiskPage) {
			diskNav.moveLeft();
			return;
		}
		if (isModePage) {
			toggleMode();
			return;
		}
	});
	useHotkeys(["down", "j"], () => {
		if (isNavFocus) {
			setFlow((state) => ({ ...state, pageIndex: clampPage(state.pageIndex + 1) }));
			return;
		}
		if (!isContentFocus) {
			return;
		}
		if (isDiskPage) {
			diskNav.moveDown();
			return;
		}
		if (isModePage) {
			toggleMode();
			return;
		}
	});
	useHotkeys(["up", "k"], () => {
		if (isNavFocus) {
			setFlow((state) => ({ ...state, pageIndex: clampPage(state.pageIndex - 1) }));
			return;
		}
		if (!isContentFocus) {
			return;
		}
		if (isDiskPage) {
			diskNav.moveUp();
			return;
		}
		if (isModePage) {
			toggleMode();
			return;
		}
	});
	useHotkeys(["enter"], () => {
		if (isNavFocus) {
			if (pageAllowsContentPane) {
				setActivePane("content");
			}
			return;
		}
		if (!isContentFocus) {
			return;
		}
		if (isDiskPage) {
			selectDiskAtIndex(diskNav.safeIndex);
		}
		onNext();
	});
	useHotkeys(["m"], () => {
		if (!isModePage) {
			return;
		}
		toggleMode();
	});
	useHotkeys(["F"], () => {
		if (!failure) {
			return;
		}
		setFlow((state) => ({ ...state, pageIndex: DISK_PLAN_PAGES.indexOf("failure-diagnostics") }));
	});

	const footer = useMemo(() => {
		const nextKeys = isNavFocus ? "j/k/h/l" : "enter/n";
		const backKeys = isNavFocus ? "tab" : "p";
		const segments = [
			scopeStatusSegment("s03-disk-plan"),
			textStatusSegment("focus", activePane, isNavFocus ? "warning" : "accent"),
			textStatusSegment(
				"page",
				`${page} (${flow.pageIndex + 1}/${DISK_PLAN_PAGES.length})`,
				"accent",
			),
			hotkeyStatusSegment("next", nextKeys, "next"),
			hotkeyStatusSegment("back", backKeys, "back"),
			hotkeyStatusSegment(
				"focus",
				pageAllowsContentPane ? "tab" : "-",
				pageAllowsContentPane ? "toggle pane" : "nav-only page",
			),
			hotkeyStatusSegment("quit", "q", "quit"),
		];
		if (isModePage) {
			segments.push(hotkeyStatusSegment("mode", "m/h/l", "toggle"));
		}
		if (isDiskPage) {
			segments.push(hotkeyStatusSegment("pick", "enter/h/j/k/l", "select disk"));
		}
		if (isNavFocus) {
			segments.push(hotkeyStatusSegment("nav", "j/k/h/l", "browse pages"));
			if (pageAllowsContentPane) {
				segments.push(hotkeyStatusSegment("open", "enter", "activate content"));
			}
		}
		if (failure) {
			segments.push(hotkeyStatusSegment("fail", "Shift+f", "failure"));
		}
		return <SegmentedStatusLine segments={segments} />;
	}, [
		activePane,
		failure,
		flow.pageIndex,
		isDiskPage,
		isModePage,
		isNavFocus,
		page,
		pageAllowsContentPane,
	]);

	const sidebar = (
		<Box flexDirection="column">
			{DISK_PLAN_PAGES.map((item, index) => (
				<UiText key={item}>
					{index === flow.pageIndex ? "▸ " : "  "}
					{item}
				</UiText>
			))}
			<UiText> </UiText>
			<UiText>Disk: {flow.disk || "(unset)"}</UiText>
			<UiText>Mode: {flow.mode}</UiText>
		</Box>
	);

	let body: ReactNode = <UiText>Unknown page state.</UiText>;

	if (page === "welcome") {
		body = (
			<Box flexDirection="column">
				<Text bold>Stage 03 Disk Plan</Text>
				<UiText> </UiText>
				<UiText>UI owns flow and evidence presentation.</UiText>
				<UiText>recpart backend owns partition/apply logic and contracts.</UiText>
				<UiText> </UiText>
				<UiText>Proceed with explicit, contract-validated steps only.</UiText>
			</Box>
		);
	}

	if (page === "target-disk") {
		body = (
			<Box flexDirection="column">
				<Text bold>Target Disk</Text>
				<UiText>Selected path: {flow.disk || "(unset)"}</UiText>
				<UiText>Discovered disks (card grid: {diskGridColumns} columns):</UiText>
				{diskCandidates.length === 0 ? (
					<UiText>- (none discovered)</UiText>
				) : (
						<SelectableGrid
							items={diskCandidates}
							selectedIndex={diskNav.safeIndex}
							columns={diskColumnsSpec}
							gapX={1}
							gapY={0}
							renderItem={(item, context) => {
								const vendor = displayVendor(item.vendor);
								return (
								<Box
									width="100%"
									borderStyle="single"
									borderColor={context.selected ? "cyan" : "gray"}
									flexDirection="column"
									paddingX={1}
								>
									<Text bold color={context.selected ? "cyan" : undefined}>
										{context.selected ? "▸ " : "  "}
										{item.path}
									</Text>
									<UiText>size: {item.size}</UiText>
									{vendor ? <UiText>vendor: {vendor}</UiText> : null}
									<UiText>model: {item.model || "(unknown)"}</UiText>
									<UiText>transport: {item.transport || "(unknown)"}</UiText>
									<UiText>serial: {item.serial || "(unknown)"}</UiText>
									<UiText>
										media:{" "}
										{item.rotational === null
											? "(unknown)"
											: item.rotational
												? "hdd"
												: "ssd"}
										{item.removable === null
											? ""
											: item.removable
												? " | removable"
												: " | fixed"}
									</UiText>
								</Box>
								);
							}}
						/>
				)}
				<UiText> </UiText>
				<UiText>
					{flow.disk.trim().length > 0
						? "Ready to continue. Use h/j/k/l (or arrows) to move between cards."
						: "Missing --disk value."}
				</UiText>
			</Box>
		);
	}

	if (page === "mode-selection") {
		body = (
			<Box flexDirection="column">
				<Text bold>Mode Selection</Text>
				<UiText>{flow.mode === "ab" ? "▸" : " "} ab: A/B immutable-ready layout</UiText>
				<UiText>{flow.mode === "mutable" ? "▸" : " "} mutable: classic mutable root</UiText>
				<UiText>Press `m` to toggle mode.</UiText>
			</Box>
		);
	}

	if (page === "plan-preview") {
		if (planState.kind === "running" || planState.kind === "idle") {
			body = <UiText>Running recpart plan --json ...</UiText>;
		} else if (planState.kind === "success") {
			const plan = planState.value;
			body = (
				<Box flexDirection="column">
					<Text bold>Plan Summary</Text>
					<UiText>Disk: {plan.diskPath}</UiText>
					<UiText>Size: {formatBytes(plan.diskSizeBytes)}</UiText>
					<UiText>Mode: {plan.mode}</UiText>
					<UiText>Partitions: {plan.partitions.length}</UiText>
					<UiText> </UiText>
					{plan.partitions.map((partition) => (
						<UiText key={`${partition.index}-${partition.name}`}>
							#{partition.index} {partition.name} {partition.filesystem} {partition.mountpoint}{" "}
							{partition.sizeMb === null ? "(rest)" : `(${partition.sizeMb}MB)`}
						</UiText>
					))}
				</Box>
			);
		} else if (planState.kind === "failure") {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Plan Failed
					</Text>
					<UiText>Exit: {planState.raw.exitCode}</UiText>
					<UiText>{limitLines(planState.raw.stderr || planState.raw.stdout)}</UiText>
				</Box>
			);
		} else {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Plan Error
					</Text>
					<UiText>{planState.message}</UiText>
				</Box>
			);
		}
	}

	if (page === "command-preview") {
		if (applyDryRunState.kind === "running" || applyDryRunState.kind === "idle") {
			body = <UiText>Loading dry-run command sequence ...</UiText>;
		} else if (applyDryRunState.kind === "success") {
			const applySummary = applyDryRunState.value;
			body = (
				<Box flexDirection="column">
					<Text bold>Exact Command Sequence (No Hidden Commands)</Text>
					<UiText>
						recpart plan --disk {flow.disk} --mode {flow.mode} --json
					</UiText>
					<UiText>
						recpart apply --disk {flow.disk} --mode {flow.mode} --dry-run --json
					</UiText>
					<UiText> </UiText>
					{applySummary.steps.map((step, index) => (
						<UiText key={`${step.phase}-${index}`}>
							[{step.phase}] {step.command}
						</UiText>
					))}
				</Box>
			);
		} else if (applyDryRunState.kind === "failure") {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Dry-run Failed
					</Text>
					<UiText>Exit: {applyDryRunState.raw.exitCode}</UiText>
					<UiText>{limitLines(applyDryRunState.raw.stderr || applyDryRunState.raw.stdout)}</UiText>
				</Box>
			);
		} else {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Dry-run Error
					</Text>
					<UiText>{applyDryRunState.message}</UiText>
				</Box>
			);
		}
	}

	if (page === "preflight-checks") {
		if (preflightState.kind === "running" || preflightState.kind === "idle") {
			body = <UiText>Running preflight checks ...</UiText>;
		} else if (preflightState.kind === "success") {
			body = (
				<Box flexDirection="column">
					<Text bold>Preflight Checks</Text>
					{preflightState.value.map((check) => (
						<Box key={check.id} flexDirection="column">
							<UiText>
								{check.ok ? "[OK]" : "[FAIL]"} {check.expectation}
							</UiText>
							<UiText>observed: {check.observed}</UiText>
							{check.ok ? null : <UiText>remediation: {check.remediation}</UiText>}
						</Box>
					))}
					<UiText> </UiText>
					<UiText>
						{arePreflightChecksPassing(preflightState)
							? "All checks passed."
							: "Fail-fast: fix failed checks before continuing."}
					</UiText>
				</Box>
			);
		} else if (preflightState.kind === "error") {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Preflight Error
					</Text>
					<UiText>{preflightState.message}</UiText>
				</Box>
			);
		} else {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Preflight Failure
					</Text>
					<UiText>{limitLines(preflightState.raw.stderr || preflightState.raw.stdout)}</UiText>
				</Box>
			);
		}
	}

	if (page === "destructive-confirm") {
		const unlocked = confirmToken === "DESTROY";
		body = (
			<Box flexDirection="column">
				<Text bold color={unlocked ? "green" : "yellow"}>
					Destructive Apply Confirmation
				</Text>
				<UiText>Type token to unlock destructive apply: DESTROY</UiText>
				<UiText>Token: {confirmToken || "(empty)"}</UiText>
				<UiText>{confirmStatusLine(confirmToken)}</UiText>
				<UiText>Press Backspace/Delete to edit.</UiText>
			</Box>
		);
	}

	if (page === "apply-progress") {
		if (applyState.kind === "running") {
			body = (
				<Box flexDirection="column">
					<Text bold>Applying Changes</Text>
					<UiText>
						Running: recpart apply --disk {flow.disk} --mode {flow.mode} --confirm DESTROY --json
					</UiText>
					{applyDryRunState.kind === "success" ? (
						<>
							<UiText> </UiText>
							<Text bold>Step Boundaries</Text>
							{applyDryRunState.value.steps.map((step, index) => (
								<UiText key={`${step.phase}-${index}`}>
									[pending] {step.phase} :: {step.command}
								</UiText>
							))}
						</>
					) : null}
				</Box>
			);
		} else if (applyState.kind === "success") {
			body = (
				<Box flexDirection="column">
					<Text bold color="green">
						Apply Completed
					</Text>
					{applyState.value.steps.map((step, index) => (
						<UiText key={`${step.phase}-${index}`}>
							{progressStepLine(step.phase, step.command, step.status)}
						</UiText>
					))}
					<UiText>Next: result-handoff page</UiText>
				</Box>
			);
		} else if (applyState.kind === "failure") {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Apply Failed
					</Text>
					<UiText>Exit: {applyState.raw.exitCode}</UiText>
					<UiText>{limitLines(applyState.raw.stderr || applyState.raw.stdout)}</UiText>
					<UiText>Next: failure-diagnostics page</UiText>
				</Box>
			);
		} else if (applyState.kind === "error") {
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						Apply Error
					</Text>
					<UiText>{applyState.message}</UiText>
					<UiText>Next: failure-diagnostics page</UiText>
				</Box>
			);
		} else {
			body = <UiText>Waiting to start apply...</UiText>;
		}
	}

	if (page === "result-handoff") {
		if (applyState.kind !== "success") {
			body = <UiText>No successful apply result available.</UiText>;
		} else {
			const handoff = applyState.value.handoff;
			body = (
				<Box flexDirection="column">
					<Text bold>Handoff</Text>
					<UiText>Install target: {handoff.installTarget}</UiText>
					<UiText> </UiText>
					<Text bold>Mount Topology</Text>
					{handoff.mountMap.map((entry) => (
						<UiText key={`${entry.path}-${entry.device}`}>
							{entry.path}
							{" <- "}
							{entry.device}
						</UiText>
					))}
					<UiText> </UiText>
					<Text bold>Next Commands</Text>
					{handoff.nextCommands.map((command) => (
						<UiText key={command}>{command}</UiText>
					))}
				</Box>
			);
		}
	}

	if (page === "failure-diagnostics") {
		if (!failure) {
			body = <UiText>No failure captured.</UiText>;
		} else {
			const lines = failureDiagnosticLines(failure);
			body = (
				<Box flexDirection="column">
					<Text bold color="red">
						{lines[0]}
					</Text>
					{lines.slice(1).map((line) => (
						<UiText key={line}>{line}</UiText>
					))}
				</Box>
			);
		}
	}

	return (
		<SurfaceFrame
			title="LevitateOS S03 Disk Plan"
			showHeader={false}
			leftWidth={38}
			footer={footer}
			leftPane={{
				title: "Pages",
				titleMode: "inline",
				body: sidebar,
				borderIntent: "sidebarBorder",
				textIntent: "sidebarItemText",
				titleIntent: "sidebarSectionText",
			}}
			rightPane={{
				title: pageLabel(page),
				titleMode: "inline",
				body,
				borderIntent: "cardBorder",
				textIntent: "text",
				titleIntent: "sectionHeading",
			}}
		/>
	);
}
