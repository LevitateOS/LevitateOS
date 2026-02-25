import { Box } from "ink";
import type { ReactNode } from "react";
import { useTuiViewport } from "../../app/app-provider";
import { toPositiveInt } from "../../utils/clamp";

export type GridColumnBreakpoint = {
	minWidth: number;
	columns: number;
};

export type GridColumnSpec =
	| number
	| {
			base: number;
			breakpoints?: ReadonlyArray<GridColumnBreakpoint>;
	  };

export type GridProps = {
	items: ReadonlyArray<ReactNode>;
	columns?: GridColumnSpec;
	gap?: number;
	gapX?: number;
	gapY?: number;
	viewportColumns?: number;
	itemWidth?: number | string;
	fillRow?: boolean;
};

export type GridItemProps = {
	children: ReactNode;
	width?: number | string;
	marginRight?: number;
	flexGrow?: number;
	flexBasis?: number | string;
};

function sanitizeBreakpoint(value: GridColumnBreakpoint): GridColumnBreakpoint {
	return {
		minWidth: Math.max(1, Math.floor(value.minWidth)),
		columns: toPositiveInt(value.columns, 1),
	};
}

export function resolveGridColumns(
	spec: GridColumnSpec | undefined,
	viewportColumns: number,
	fallback = 1,
): number {
	const safeFallback = toPositiveInt(fallback, 1);
	if (typeof spec === "number" || spec === undefined) {
		return toPositiveInt(spec ?? safeFallback, safeFallback);
	}
	const safeBase = toPositiveInt(spec.base, safeFallback);
	const safeViewport = toPositiveInt(viewportColumns, 80);
	const breakpoints = (spec.breakpoints ?? [])
		.map((item) => sanitizeBreakpoint(item))
		.sort((a, b) => a.minWidth - b.minWidth);
	let resolved = safeBase;
	for (const point of breakpoints) {
		if (safeViewport >= point.minWidth) {
			resolved = point.columns;
		}
	}
	return toPositiveInt(resolved, safeFallback);
}

export function groupGridRows<T>(
	items: ReadonlyArray<T>,
	columns: number,
): ReadonlyArray<ReadonlyArray<T>> {
	const safeColumns = toPositiveInt(columns, 1);
	if (items.length === 0) {
		return [];
	}
	const rows: Array<ReadonlyArray<T>> = [];
	for (let index = 0; index < items.length; index += safeColumns) {
		rows.push(items.slice(index, index + safeColumns));
	}
	return rows;
}

export function GridItem({ children, width, marginRight = 0, flexGrow, flexBasis }: GridItemProps) {
	return (
		<Box width={width} marginRight={marginRight} flexGrow={flexGrow} flexBasis={flexBasis}>
			{children}
		</Box>
	);
}

export function Grid({
	items,
	columns = 2,
	gap = 1,
	gapX,
	gapY,
	viewportColumns,
	itemWidth,
	fillRow = false,
}: GridProps) {
	const viewport = useTuiViewport();
	const safeColumns = resolveGridColumns(columns, viewportColumns ?? viewport.columns, 2);
	const horizontalGap = Math.max(0, Math.floor(gapX ?? gap));
	const verticalGap = Math.max(0, Math.floor(gapY ?? gap));
	const rows = groupGridRows(items, safeColumns);

	return (
		<Box flexDirection="column">
			{rows.map((rowItems, rowIndex) => (
				<Box
					key={`row-${rowIndex}`}
					flexDirection="row"
					marginBottom={rowIndex < rows.length - 1 ? verticalGap : 0}
				>
					{rowItems.map((item, columnIndex) => (
						<GridItem
							key={`cell-${rowIndex}-${columnIndex}`}
							marginRight={columnIndex < safeColumns - 1 ? horizontalGap : 0}
							width={itemWidth}
							flexGrow={fillRow ? 1 : undefined}
							flexBasis={fillRow ? 0 : undefined}
						>
							{item}
						</GridItem>
					))}
					{!fillRow && itemWidth !== undefined && rowItems.length < safeColumns
						? Array.from({ length: safeColumns - rowItems.length }).map((_, padIndex) => {
								const columnIndex = rowItems.length + padIndex;
								return (
									<GridItem
										key={`pad-${rowIndex}-${columnIndex}`}
										marginRight={columnIndex < safeColumns - 1 ? horizontalGap : 0}
										width={itemWidth}
									>
										<Box />
									</GridItem>
								);
							})
						: null}
				</Box>
			))}
		</Box>
	);
}
