import {
	defaultTuiTheme,
	detectColorRuntime,
	type ColorMode,
	type ColorRuntime,
	type TuiTheme,
} from "../theme";
import type { TuiAppContextValue } from "./app-provider";

export type CreateTuiAppOptions = {
	title?: string;
	theme?: TuiTheme;
	colorMode?: ColorMode;
	colorEnabled?: boolean;
};

export type TuiApp = TuiAppContextValue;

function resolveTitle(title?: string): string {
	const normalized = typeof title === "string" ? title.trim() : "";
	return normalized.length > 0 ? normalized : "tui-kit";
}

export function createTuiApp(options: CreateTuiAppOptions = {}): TuiApp {
	const theme = options.theme ?? defaultTuiTheme;
	const colors: ColorRuntime = detectColorRuntime({
		mode: options.colorMode,
		enabled: options.colorEnabled,
	});

	return {
		title: resolveTitle(options.title),
		theme,
		colors,
	};
}

export function disposeTuiApp(_app: TuiApp): void {}
