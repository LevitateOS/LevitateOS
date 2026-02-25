import type { ContentBlock } from "@levitate/docs-content";
import type { ReactNode } from "react";
import type { DocBlockItem, DocRenderItem } from "../../../domain/render/types";
import { DEFAULT_INSTALL_BLOCK_PLUGINS } from "../blocks/plugins";
import { IntroItem } from "./intro-item";
import { SectionHeadingItem } from "./section-heading-item";

type DocNodeRenderContext = {
	contentWidth: number;
	renderBlock: (block: ContentBlock, indent?: number) => ReactNode;
};

type DocNodeKind = DocRenderItem["kind"];

type DocNodePlugin<TKind extends DocNodeKind = DocNodeKind> = {
	kind: TKind;
	render: (
		item: Extract<DocRenderItem, { kind: TKind }>,
		context: DocNodeRenderContext,
	) => ReactNode;
};

type DocNodePluginMap = {
	[K in DocNodeKind]: DocNodePlugin<K>;
};

export type DocsRendererRegistry = Readonly<DocNodePluginMap>;

function docsRendererError(detail: string): never {
	throw new Error(`docs.render ${detail}`);
}

function renderDocNode(
	item: DocRenderItem,
	registry: DocsRendererRegistry,
	context: DocNodeRenderContext,
): ReactNode {
	const nodePlugin = registry[item.kind];
	if (!nodePlugin) {
		docsRendererError(
			`missing node renderer for kind '${item.kind}'. Remediation: register this node plugin in the install docs renderer registry.`,
		);
	}
	return nodePlugin.render(item as never, context);
}

const DEFAULT_INSTALL_DOC_NODE_PLUGINS: DocNodePluginMap = Object.freeze({
	intro: {
		kind: "intro",
		render: (item, context) => (
			<IntroItem content={item.content} contentWidth={context.contentWidth} />
		),
	},
	section: {
		kind: "section",
		render: (item) => <SectionHeadingItem title={item.title} level={item.level} />,
	},
	block: {
		kind: "block",
		render: (item, context) => {
			const blockPlugin = DEFAULT_INSTALL_BLOCK_PLUGINS[item.block.type];
			if (!blockPlugin) {
				docsRendererError(
					`missing block plugin for block type '${item.block.type}'. Remediation: register this block plugin in DEFAULT_INSTALL_BLOCK_PLUGINS.`,
				);
			}

			const indent =
				typeof item.indent === "number" && Number.isFinite(item.indent)
					? Math.max(0, item.indent)
					: 0;

			return blockPlugin.render(
				item.block as never,
				{
					contentWidth: context.contentWidth,
					renderBlock: context.renderBlock,
				},
				indent,
			);
		},
	},
});

export function createInstallDocsRendererRegistry(
	overrides: Partial<DocNodePluginMap> = {},
): DocsRendererRegistry {
	return {
		...DEFAULT_INSTALL_DOC_NODE_PLUGINS,
		...overrides,
	};
}

function nestedBlockNode(block: ContentBlock, indent: number): DocBlockItem {
	return {
		kind: "block",
		key: `nested-block-${block.type}`,
		block,
		sectionIndex: -1,
		sectionTitle: "(nested)",
		sectionLevel: 2,
		blockIndex: -1,
		indent,
	};
}

export function renderDocItemWithRegistry(
	item: DocRenderItem,
	registry: DocsRendererRegistry,
	contentWidth: number,
): ReactNode {
	const renderBlock = (block: ContentBlock, indent = 0): ReactNode =>
		renderDocNode(nestedBlockNode(block, indent), registry, {
			contentWidth,
			renderBlock,
		});

	return renderDocNode(item, registry, {
		contentWidth,
		renderBlock,
	});
}
