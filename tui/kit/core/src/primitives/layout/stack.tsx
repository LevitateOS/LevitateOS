import { Box } from "ink";
import type { ReactNode } from "react";

export type StackProps = {
	children: ReactNode;
	paddingX?: number;
	paddingY?: number;
	flexGrow?: number;
	width?: number | string;
};

export function Stack({ children, paddingX, paddingY, flexGrow, width }: StackProps) {
	return (
		<Box
			flexDirection="column"
			paddingX={paddingX}
			paddingY={paddingY}
			flexGrow={flexGrow}
			width={width}
		>
			{children}
		</Box>
	);
}
