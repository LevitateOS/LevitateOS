import { Text } from "ink";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";
import type { RichTextRun } from "./rich-text";

export type UiLinkProps = {
	label: string;
	href?: string;
	isSelected?: boolean;
	underline?: boolean;
	boldWhenSelected?: boolean;
	onActivate?: () => void;
};

type LinkRunOptions = {
	href?: string;
	isSelected?: boolean;
	includeHrefWhenDifferent?: boolean;
};

export function linkRuns(label: string, options: LinkRunOptions = {}): RichTextRun[] {
	const href = options.href?.trim() ?? "";
	const isSelected = options.isSelected ?? false;
	const includeHref =
		(options.includeHrefWhenDifferent ?? true) && href.length > 0 && href !== label;
	const foregroundIntent = isSelected ? "linkActiveText" : "linkText";
	const backgroundIntent = isSelected ? "linkActiveBackground" : undefined;

	const runs: RichTextRun[] = [
		{
			text: label,
			intent: foregroundIntent,
			backgroundIntent,
			underline: true,
			bold: isSelected,
		},
	];

	if (includeHref) {
		runs.push({
			text: ` (${href})`,
			intent: foregroundIntent,
			backgroundIntent,
			bold: false,
		});
	}

	return runs;
}

export function UiLink({
	label,
	href,
	isSelected = false,
	underline = true,
	boldWhenSelected = true,
	onActivate,
}: UiLinkProps) {
	void onActivate;
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const textColor = resolveIntentColor(
		theme,
		isSelected ? "linkActiveText" : "linkText",
		colors,
	);
	const backgroundColor = isSelected
		? resolveIntentColor(theme, "linkActiveBackground", colors)
		: undefined;
	const suffix = href && href.trim().length > 0 && href.trim() !== label ? ` (${href.trim()})` : "";

	return (
		<Text
			color={textColor}
			backgroundColor={backgroundColor}
			underline={underline}
			bold={boldWhenSelected && isSelected}
		>
			{`${label}${suffix}`}
		</Text>
	);
}
