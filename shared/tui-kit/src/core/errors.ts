export type TuiKitErrorCode = "TERMINAL_TOO_SMALL" | "CANCELLED" | "INTERNAL";

export const TUI_KIT_ERROR_CODES: ReadonlySet<TuiKitErrorCode> = new Set([
  "TERMINAL_TOO_SMALL",
  "CANCELLED",
  "INTERNAL",
]);

export type TuiKitErrorContext = Record<string, string | number | boolean | null | undefined>;

export class TuiKitError extends Error {
  readonly code: TuiKitErrorCode;
  readonly context: TuiKitErrorContext;

  constructor(code: TuiKitErrorCode, message: string, context: TuiKitErrorContext = {}) {
    super(message);
    this.name = "TuiKitError";
    this.code = code;
    this.context = { ...context };
  }
}

export function isTuiKitError(value: unknown): value is TuiKitError {
  if (value instanceof TuiKitError) {
    return true;
  }

  if (!value || typeof value !== "object") {
    return false;
  }

  const candidate = value as {
    name?: unknown;
    message?: unknown;
    code?: unknown;
  };

  return (
    candidate.name === "TuiKitError" &&
    typeof candidate.message === "string" &&
    typeof candidate.code === "string" &&
    TUI_KIT_ERROR_CODES.has(candidate.code as TuiKitErrorCode)
  );
}
