import { detectColorRuntime, type ColorMode, type ColorRuntime } from "../../theme";

export type DetectTtyColorsOptions = {
	env?: Record<string, string | undefined>;
	isTTY?: boolean;
	mode?: ColorMode;
	enabled?: boolean;
};

export function detectTtyColors(options: DetectTtyColorsOptions = {}): ColorRuntime {
	return detectColorRuntime(options);
}

export function ttySupportsColor(runtime: ColorRuntime): boolean {
	return runtime.enabled && runtime.mode !== "mono";
}
