import { Box } from "ink";
import type { DocsViewport } from "../../../rendering/pipeline/viewport";
import { DocsContentView } from "../document/docs-content-view";
import type { DocsRendererRegistry } from "../document/renderer-registry";

export function InstallContentPane({
	viewport,
	renderers,
}: {
	viewport: DocsViewport;
	renderers: DocsRendererRegistry;
}) {
	return (
		<Box
			flexDirection="column"
			width={viewport.contentWidth}
			height={viewport.visibleCount}
			overflowY="hidden"
			flexShrink={0}
		>
			<Box
				flexDirection="column"
				width={viewport.contentWidth}
				marginTop={-viewport.scrollOffset}
				flexShrink={0}
			>
				<DocsContentView
					items={viewport.visibleItems}
					contentWidth={viewport.contentWidth}
					renderers={renderers}
				/>
			</Box>
		</Box>
	);
}
