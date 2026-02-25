export class AppError extends Error {
	readonly code: string;
	readonly exitCode: number;

	constructor(code: string, message: string, exitCode = 2) {
		super(message);
		this.name = "AppError";
		this.code = code;
		this.exitCode = exitCode;
	}
}

export function formatErrorMessage(error: unknown): string {
	if (error instanceof Error) {
		return error.message;
	}

	return String(error);
}
