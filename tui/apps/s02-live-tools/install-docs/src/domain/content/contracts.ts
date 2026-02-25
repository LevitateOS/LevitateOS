import type {
	DocsPageMeta,
	DocsProduct,
	DocsScope,
	NavSection,
	DocsContent,
	RichText,
} from "@levitate/docs-content";
import type { ColorRuntime, TuiTheme } from "@levitate/tui-kit";

export type { DocsPageMeta, DocsProduct, DocsScope, NavSection };

export type DocsSource = {
	docsNav: ReadonlyArray<NavSection>;
	contentBySlug: Record<string, DocsContent>;
	metaBySlug: Record<string, DocsPageMeta>;
};

export type FlatDocsNavItem = {
	sectionTitle: string;
	title: string;
	href: string;
	slug: string;
};

export type DocsRenderStyleContext = {
	theme: TuiTheme;
	colors: ColorRuntime;
};

export type DocsContentLike = DocsContent;
export type RichTextLike = RichText;
