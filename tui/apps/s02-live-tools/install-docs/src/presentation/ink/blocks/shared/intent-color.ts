import { resolveIntentColor, type ColorIntent, useTuiColors, useTuiTheme } from "@levitate/tui-kit";

const INHERIT_TERMINAL_BACKGROUND_INTENTS = new Set<ColorIntent>([
	"background",
	"sidebarBackground",
	"contentBackground",
	"headerBackground",
	"headerAccentBackground",
	"footerBackground",
	"sidebarItemActiveBackground",
	"commandBarBackground",
	"warningBackground",
	"cardBackground",
]);

export function useIntentColor(intent: ColorIntent): string | undefined {
	if (INHERIT_TERMINAL_BACKGROUND_INTENTS.has(intent)) {
		return undefined;
	}
	const theme = useTuiTheme();
	const colors = useTuiColors();
	return resolveIntentColor(theme, intent, colors);
}
