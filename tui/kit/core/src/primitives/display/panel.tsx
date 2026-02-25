import { Box, Text } from "ink";
import type { ReactNode } from "react";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";

export type PanelProps = {
	title?: string;
	children: ReactNode;
	borderIntent?: "border" | "sidebarBorder" | "cardBorder";
	width?: number | string;
	minWidth?: number;
	flexGrow?: number;
};

export function Panel({
	title,
	children,
	borderIntent = "border",
	width,
	minWidth,
	flexGrow,
}: PanelProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const borderColor = resolveIntentColor(theme, borderIntent, colors);

	return (
		<Box
			borderStyle="single"
			borderColor={borderColor}
			flexDirection="column"
			width={width}
			minWidth={minWidth}
			flexGrow={flexGrow}
		>
			{title ? (
				<Box paddingX={1}>
					<Text bold>{title}</Text>
				</Box>
			) : null}
			<Box flexDirection="column" paddingX={1}>
				{children}
			</Box>
		</Box>
	);
}
