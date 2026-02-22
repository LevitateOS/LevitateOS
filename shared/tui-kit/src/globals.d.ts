type TuiKitProcessStream = {
  columns?: number;
  rows?: number;
  isTTY?: boolean;
  write?: (chunk: string | Uint8Array) => boolean;
  on?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  once?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  off?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  removeListener?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  setRawMode?: (mode: boolean) => void;
  resume?: () => void;
  pause?: () => void;
};

declare const process: {
  argv: string[];
  env: Record<string, string | undefined>;
  cwd: () => string;
  stdout: TuiKitProcessStream;
  stderr: TuiKitProcessStream;
  stdin: TuiKitProcessStream;
  on: (event: string, listener: (...args: unknown[]) => void) => unknown;
  once?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  off?: (event: string, listener: (...args: unknown[]) => void) => unknown;
  exit: (code?: number) => never;
};
