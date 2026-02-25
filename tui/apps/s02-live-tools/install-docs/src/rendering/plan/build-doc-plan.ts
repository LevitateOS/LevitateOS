import type { RichText } from "@levitate/docs-content";
import type { DocsContentLike } from "../../domain/content/contracts";
import type { DocRenderPlan } from "../../domain/render/types";

function hasInlineContent(value: string | RichText | undefined): value is string | RichText {
	if (typeof value === "string") {
		return value.trim().length > 0;
	}
	if (Array.isArray(value)) {
		return value.length > 0;
	}
	return false;
}

export function buildDocRenderPlan(content: DocsContentLike): DocRenderPlan {
	const items: DocRenderPlan["items"] = [];

	if (hasInlineContent(content.intro)) {
		items.push({
			kind: "intro",
			key: "intro",
			content: content.intro,
		});
	}

	for (const [sectionIndex, section] of content.sections.entries()) {
		const sectionLevel = section.level === 3 ? 3 : 2;
		items.push({
			kind: "section",
			key: `section-heading-${sectionIndex}`,
			title: section.title,
			level: sectionLevel,
			sectionIndex,
		});

		for (const [blockIndex, block] of section.content.entries()) {
			items.push({
				kind: "block",
				key: `section-${sectionIndex}-block-${blockIndex}`,
				block,
				sectionIndex,
				sectionTitle: section.title,
				sectionLevel,
				blockIndex,
			});
		}
	}

	return { items };
}
