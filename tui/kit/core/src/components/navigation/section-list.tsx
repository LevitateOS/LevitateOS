import { Text } from "ink";
import { clampNumber, toNonNegativeInt } from "../../utils/clamp";
import { normalizeTextWidth, truncateBoundedLine } from "../../utils/strings";

export type SectionListItem = {
	section: string;
	label: string;
};

export type RenderSectionListOptions = {
	maxWidth?: number;
	emptyLabel?: string;
	marker?: string;
	inactiveMarker?: string;
	sectionPrefix?: string;
};

function marker(token: string | undefined, fallback: string): string {
	if (typeof token === "string") {
		return token.length > 0 ? token.slice(0, 1) : "";
	}
	return fallback;
}

export function renderSectionList(
	items: ReadonlyArray<SectionListItem>,
	selectedIndex: number,
	options: RenderSectionListOptions = {},
): string {
	if (items.length === 0) {
		return options.emptyLabel ?? "(no items)";
	}

	const safeWidth = normalizeTextWidth(options.maxWidth ?? 30, 1);
	const safeSelected = clampNumber(toNonNegativeInt(selectedIndex, 0), 0, items.length - 1);
	const activeMarker = marker(options.marker, ">");
	const inactiveMarker = marker(options.inactiveMarker, " ");
	const sectionPrefix = typeof options.sectionPrefix === "string" ? options.sectionPrefix : "";
	const lines: string[] = [];
	let currentSection = "";

	for (const [index, item] of items.entries()) {
		const section = item.section.trim();
		if (section !== currentSection) {
			currentSection = section;
			if (lines.length > 0) {
				lines.push("");
			}
			lines.push(
				truncateBoundedLine(
					sectionPrefix.length > 0 ? `${sectionPrefix} ${currentSection}` : currentSection,
					safeWidth,
					1,
				),
			);
		}

		const selectedMarker = index === safeSelected ? activeMarker : inactiveMarker;
		lines.push(truncateBoundedLine(`  ${selectedMarker} ${item.label}`, safeWidth, 1));
	}

	return lines.join("\n");
}

export type SectionListProps = {
	items: ReadonlyArray<SectionListItem>;
	selectedIndex: number;
	maxWidth?: number;
	emptyLabel?: string;
	marker?: string;
	inactiveMarker?: string;
	sectionPrefix?: string;
};

export function SectionList({
	items,
	selectedIndex,
	maxWidth,
	emptyLabel,
	marker,
	inactiveMarker,
	sectionPrefix,
}: SectionListProps) {
	return (
		<Text>
			{renderSectionList(items, selectedIndex, {
				maxWidth,
				emptyLabel,
				marker,
				inactiveMarker,
				sectionPrefix,
			})}
		</Text>
	);
}
