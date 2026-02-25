import type { DocsPageMeta, DocsProduct, DocsScope } from "../content/contracts";

export type DistroId = "levitate" | "acorn" | "ralph";

export type DistroProfile = {
	id: DistroId;
	title: string;
	allowedProducts: ReadonlyArray<DocsProduct>;
	allowedScopes: ReadonlyArray<DocsScope>;
	defaultSlug: string;
	allowsMeta: (meta: DocsPageMeta) => boolean;
};
