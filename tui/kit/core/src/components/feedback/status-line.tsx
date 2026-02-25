import { Text } from "ink";
import type { ColorIntent } from "../../theme";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";
import { clampNumber, toNonNegativeInt } from "../../utils/clamp";

export type StatusLineProps = {
	text: string;
	intent?: ColorIntent;
};

export function StatusLine({ text, intent = "dimText" }: StatusLineProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const color = resolveIntentColor(theme, intent, colors);

	return <Text color={color}>{text}</Text>;
}

export type StatusAtom = {
	text: string;
	intent?: ColorIntent;
	bold?: boolean;
	italic?: boolean;
	underline?: boolean;
};

export type StatusSegment = {
	id: string;
	atoms: ReadonlyArray<StatusAtom>;
};

export type FlattenStatusSegmentsOptions = {
	separator?: string;
	separatorIntent?: ColorIntent;
};

export function flattenStatusSegments(
	segments: ReadonlyArray<StatusSegment>,
	options: FlattenStatusSegmentsOptions = {},
): StatusAtom[] {
	const separator = options.separator ?? " | ";
	const separatorIntent = options.separatorIntent ?? "dimText";
	const atoms: StatusAtom[] = [];

	for (const [index, segment] of segments.entries()) {
		if (index > 0 && separator.length > 0) {
			atoms.push({
				text: separator,
				intent: separatorIntent,
			});
		}
		for (const atom of segment.atoms) {
			if (atom.text.length === 0) {
				continue;
			}
			atoms.push(atom);
		}
	}

	return atoms;
}

export type SegmentedStatusLineProps = {
	segments: ReadonlyArray<StatusSegment>;
	separator?: string;
	separatorIntent?: ColorIntent;
};

export function SegmentedStatusLine({
	segments,
	separator,
	separatorIntent,
}: SegmentedStatusLineProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const atoms = flattenStatusSegments(segments, {
		separator,
		separatorIntent,
	});

	return (
		<Text>
			{atoms.map((atom, index) => (
				<Text
					key={`${index}-${atom.text}`}
					color={resolveIntentColor(theme, atom.intent ?? "dimText", colors)}
					bold={atom.bold}
					italic={atom.italic}
					underline={atom.underline}
				>
					{atom.text}
				</Text>
			))}
		</Text>
	);
}

export function scopeStatusSegment(scope: string, intent: ColorIntent = "accent"): StatusSegment {
	const normalized = scope.trim().length > 0 ? scope.trim() : "tui";
	return {
		id: `scope-${normalized}`,
		atoms: [
			{
				text: `[${normalized}]`,
				intent,
				bold: true,
			},
		],
	};
}

export function hotkeyStatusSegment(
	id: string,
	key: string,
	label: string,
	keyIntent: ColorIntent = "accent",
	labelIntent: ColorIntent = "dimText",
): StatusSegment {
	const normalizedKey = key.trim();
	const normalizedLabel = label.trim();

	return {
		id,
		atoms: [
			{
				text: normalizedKey.length > 0 ? normalizedKey : "?",
				intent: keyIntent,
				bold: true,
			},
			{
				text: normalizedLabel.length > 0 ? ` ${normalizedLabel}` : "",
				intent: labelIntent,
			},
		],
	};
}

export function textStatusSegment(
	id: string,
	text: string,
	intent: ColorIntent = "dimText",
	bold = false,
): StatusSegment {
	return {
		id,
		atoms: [
			{
				text,
				intent,
				bold,
			},
		],
	};
}

export type PagedStatusLineOptions = {
	currentIndex: number;
	itemCount: number;
	startLine: number;
	endLine: number;
	totalLines: number;
	note?: string;
	keymap?: string;
};

export function formatPagedStatusLine(options: PagedStatusLineOptions): string {
	const safeItemCount = Math.max(1, toNonNegativeInt(options.itemCount, 1));
	const safeCurrentIndex = clampNumber(
		toNonNegativeInt(options.currentIndex, 0),
		0,
		safeItemCount - 1,
	);
	const safeTotalLines = Math.max(1, toNonNegativeInt(options.totalLines, 1));
	const safeStartLine = clampNumber(toNonNegativeInt(options.startLine, 0), 0, safeTotalLines);
	const safeEndLine = clampNumber(
		toNonNegativeInt(options.endLine, 0),
		safeStartLine,
		safeTotalLines,
	);

	const keymap =
		typeof options.keymap === "string" && options.keymap.trim().length > 0
			? options.keymap.trim()
			: "q quit | h/l page | j/k scroll | g/G top/end";
	const extra =
		typeof options.note === "string" && options.note.trim().length > 0
			? ` | ${options.note.trim()}`
			: "";
	return `${keymap} | page ${safeCurrentIndex + 1}/${safeItemCount} | lines ${safeStartLine}-${safeEndLine}/${safeTotalLines}${extra}`;
}
