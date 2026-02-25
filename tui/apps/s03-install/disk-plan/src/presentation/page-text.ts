import type { ErrorSummary } from "../domain/apply-model";

export function confirmStatusLine(token: string): string {
	return token === "DESTROY" ? "Status: unlocked" : "Status: locked";
}

export function progressStepLine(phase: string, command: string, status: number | null): string {
	const state = status === null || status === 0 ? "OK" : "FAIL";
	return `[${state}] ${phase} :: ${command}`;
}

export function failureDiagnosticLines(failure: {
	source: string;
	detail: ErrorSummary | null;
	observed: string;
	remediation: string;
}): string[] {
	if (failure.detail) {
		return [
			`Failure Diagnostics (${failure.source})`,
			`Code: ${failure.detail.code}`,
			`Component: ${failure.detail.component}`,
			`Expectation: ${failure.detail.expectation}`,
			`Observed: ${failure.detail.observed}`,
			`Remediation: ${failure.detail.remediation}`,
		];
	}
	return [
		`Failure Diagnostics (${failure.source})`,
		`Observed: ${failure.observed}`,
		`Remediation: ${failure.remediation}`,
	];
}
