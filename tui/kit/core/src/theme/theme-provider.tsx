import { createContext, useContext, type PropsWithChildren } from "react";
import { defaultTuiTheme, type TuiTheme } from "./tokens";

const ThemeContext = createContext<TuiTheme>(defaultTuiTheme);

export type ThemeProviderProps = PropsWithChildren<{
	theme?: TuiTheme;
}>;

export function ThemeProvider({ theme, children }: ThemeProviderProps) {
	return <ThemeContext.Provider value={theme ?? defaultTuiTheme}>{children}</ThemeContext.Provider>;
}

export function useThemeTokens(): TuiTheme {
	return useContext(ThemeContext);
}
