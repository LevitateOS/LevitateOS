import { Text } from "ink";
import { resolveChromeGlyphSet } from "../../chrome";
import { useTuiColors, useTuiTheme, useTuiViewport } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";
import { horizontalRule } from "../../utils/strings";

export type DividerProps = {
	width?: number;
};

export function resolveDividerGlyph(borderStyle: "single" | "double" | "round" | "bold"): string {
	return resolveChromeGlyphSet(borderStyle).h;
}

export function Divider({ width }: DividerProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const viewport = useTuiViewport();
	const lineWidth = width ?? viewport.columns;
	const color = resolveIntentColor(theme, "footerSeparator", colors);
	const glyph = resolveDividerGlyph(theme.chrome.borderGlyphSet);

	return <Text color={color}>{horizontalRule(Math.max(1, lineWidth), glyph)}</Text>;
}
