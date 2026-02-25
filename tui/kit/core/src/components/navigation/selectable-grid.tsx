import type { ReactNode } from "react";
import { clampIndex, toPositiveInt } from "../../utils/clamp";
import { Grid, resolveGridColumns, type GridColumnSpec } from "../../primitives/layout/grid";
import { useTuiViewport } from "../../app/app-provider";

type RenderItemContext = {
	index: number;
	selected: boolean;
};

export type SelectableGridProps<T> = {
	items: ReadonlyArray<T>;
	selectedIndex: number;
	columns?: GridColumnSpec;
	gapX?: number;
	gapY?: number;
	viewportColumns?: number;
	equalWidth?: boolean;
	fillRow?: boolean;
	renderItem: (item: T, context: RenderItemContext) => ReactNode;
};

export function mapSelectableGridItems<T>(
	items: ReadonlyArray<T>,
	selectedIndex: number,
	renderItem: (item: T, context: RenderItemContext) => ReactNode,
): ReadonlyArray<ReactNode> {
	const safeSelected = clampIndex(selectedIndex, items.length);
	return items.map((item, index) => renderItem(item, { index, selected: index === safeSelected }));
}

function widthForColumns(columns: number): string | undefined {
	const safeColumns = toPositiveInt(columns, 1);
	if (safeColumns <= 1) {
		return undefined;
	}
	return `${Math.floor(100 / safeColumns)}%`;
}

export function SelectableGrid<T>({
	items,
	selectedIndex,
	columns = 2,
	gapX = 1,
	gapY = 1,
	viewportColumns,
	equalWidth = true,
	fillRow = false,
	renderItem,
}: SelectableGridProps<T>) {
	const viewport = useTuiViewport();
	const resolvedColumns = resolveGridColumns(columns, viewportColumns ?? viewport.columns, 1);
	const itemWidth = !fillRow && equalWidth ? widthForColumns(resolvedColumns) : undefined;
	const renderedItems = mapSelectableGridItems(items, selectedIndex, renderItem);

	return (
		<Grid
			items={renderedItems}
			columns={resolvedColumns}
			gapX={gapX}
			gapY={gapY}
			viewportColumns={viewportColumns}
			itemWidth={itemWidth}
			fillRow={fillRow}
		/>
	);
}
