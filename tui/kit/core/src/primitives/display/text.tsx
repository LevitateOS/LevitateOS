import { Text } from "ink";
import type { ReactNode } from "react";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor, type ColorIntent } from "../../theme";

export type UiTextProps = {
	children: ReactNode;
	intent?: ColorIntent;
	bold?: boolean;
};

export function UiText({ children, intent = "text", bold = false }: UiTextProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const color = resolveIntentColor(theme, intent, colors);

	if (typeof children === "string") {
		return (
			<Text color={color} bold={bold}>
				{children}
			</Text>
		);
	}

	return (
		<Text color={color} bold={bold}>
			{children}
		</Text>
	);
}
