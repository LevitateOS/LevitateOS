import { Box } from "ink";
import type { ReactNode } from "react";

export type InlineProps = {
	children: ReactNode;
	paddingX?: number;
	paddingY?: number;
	flexGrow?: number;
	width?: number | string;
};

export function Inline({ children, paddingX, paddingY, flexGrow, width }: InlineProps) {
	return (
		<Box
			flexDirection="row"
			paddingX={paddingX}
			paddingY={paddingY}
			flexGrow={flexGrow}
			width={width}
		>
			{children}
		</Box>
	);
}
