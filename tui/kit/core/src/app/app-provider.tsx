import { createContext, useContext, useEffect, useState, type PropsWithChildren } from "react";
import { useStdout } from "ink";
import { readScreenSize, type TerminalSize } from "../runtime/terminal/screen-size";
import type { ColorRuntime, TuiTheme } from "../theme";

export type TuiAppContextValue = {
	title: string;
	theme: TuiTheme;
	colors: ColorRuntime;
};

const AppContext = createContext<TuiAppContextValue | null>(null);

export type AppProviderProps = PropsWithChildren<{
	app: TuiAppContextValue;
}>;

export function AppProvider({ app, children }: AppProviderProps) {
	return <AppContext.Provider value={app}>{children}</AppContext.Provider>;
}

export function useTuiApp(): TuiAppContextValue {
	const value = useContext(AppContext);
	if (!value) {
		throw new Error("useTuiApp must be used inside <AppProvider>.");
	}
	return value;
}

export function useTuiTheme(): TuiTheme {
	return useTuiApp().theme;
}

export function useTuiColors(): ColorRuntime {
	return useTuiApp().colors;
}

export function useTuiViewport(): TerminalSize {
	const { stdout } = useStdout();
	const processViewport = readScreenSize({
		columns: process.stdout.columns,
		rows: process.stdout.rows,
	});
	const readViewport = (): TerminalSize => readScreenSize(stdout, processViewport);
	const [size, setSize] = useState<TerminalSize>(() => readViewport());

	useEffect(() => {
		setSize((previous) => {
			const next = readViewport();
			if (previous.columns === next.columns && previous.rows === next.rows) {
				return previous;
			}
			return next;
		});

		if (!stdout) {
			return;
		}

		const onResize = () => {
			setSize((previous) => {
				const next = readViewport();
				if (previous.columns === next.columns && previous.rows === next.rows) {
					return previous;
				}
				return next;
			});
		};

		stdout.on("resize", onResize);

		return () => {
			if (typeof stdout.off === "function") {
				stdout.off("resize", onResize);
				return;
			}
			stdout.removeListener?.("resize", onResize);
		};
	}, [stdout]);

	return size;
}
