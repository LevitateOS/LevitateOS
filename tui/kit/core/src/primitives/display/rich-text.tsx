import { Text } from "ink";
import type { ColorIntent } from "../../theme";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";

export type RichTextRun = {
	text: string;
	intent?: ColorIntent;
	backgroundIntent?: ColorIntent;
	literalColor?: string;
	bold?: boolean;
	italic?: boolean;
	underline?: boolean;
};

export type RichTextLineProps = {
	runs: ReadonlyArray<RichTextRun>;
	fallbackIntent?: ColorIntent;
};

function resolveRunColor(
	run: RichTextRun,
	theme: ReturnType<typeof useTuiTheme>,
	colors: ReturnType<typeof useTuiColors>,
	fallbackIntent: ColorIntent,
) {
	if (typeof run.literalColor === "string" && run.literalColor.length > 0) {
		return run.literalColor;
	}
	return resolveIntentColor(theme, run.intent ?? fallbackIntent, colors);
}

function resolveRunBackground(
	run: RichTextRun,
	theme: ReturnType<typeof useTuiTheme>,
	colors: ReturnType<typeof useTuiColors>,
) {
	if (!run.backgroundIntent) {
		return undefined;
	}
	return resolveIntentColor(theme, run.backgroundIntent, colors);
}

export function RichTextLine({ runs, fallbackIntent = "text" }: RichTextLineProps) {
	if (runs.length === 0) {
		return <Text> </Text>;
	}

	const theme = useTuiTheme();
	const colors = useTuiColors();

	return (
		<Text>
			{runs.map((run, index) => (
				<Text
					key={`${index}-${run.text}`}
					color={resolveRunColor(run, theme, colors, fallbackIntent)}
					backgroundColor={resolveRunBackground(run, theme, colors)}
					bold={run.bold}
					italic={run.italic}
					underline={run.underline}
				>
					{run.text.length > 0 ? run.text : " "}
				</Text>
			))}
		</Text>
	);
}
