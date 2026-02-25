import { Box } from "ink";
import { UiText } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { DocRenderItem } from "../../../domain/render/types";
import { renderDocItemWithRegistry, type DocsRendererRegistry } from "./renderer-registry";

type DocsContentViewProps = {
	items: ReadonlyArray<DocRenderItem>;
	contentWidth: number;
	renderers: DocsRendererRegistry;
};

export function DocsContentView({
	items,
	contentWidth,
	renderers,
}: DocsContentViewProps): ReactNode {
	return (
		<Box flexDirection="column" width={contentWidth} flexShrink={0}>
			{items.map((item, index) => (
				<Box key={item.key} flexDirection="column" width={contentWidth} flexShrink={0}>
					{renderDocItemWithRegistry(item, renderers, contentWidth)}
					{index < items.length - 1 ? <UiText intent="dimText"> </UiText> : null}
				</Box>
			))}
		</Box>
	);
}
