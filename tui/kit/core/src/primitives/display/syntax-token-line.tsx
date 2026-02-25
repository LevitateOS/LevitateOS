import { Text } from "ink";
import type { ColorIntent } from "../../theme";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";

const ESCAPED_OPEN = "\u0000TUI_ESCAPED_OPEN\u0000";

export type SyntaxToken = {
	text: string;
	color?: string;
};

function normalizeHexColor(value: string): string | undefined {
	const candidate = value.trim().toLowerCase();
	if (/^#[0-9a-f]{6}$/.test(candidate)) {
		return candidate;
	}
	if (/^#[0-9a-f]{3}$/.test(candidate)) {
		const [, r, g, b] = candidate;
		return `#${r}${r}${g}${g}${b}${b}`;
	}
	if (/^#[0-9a-f]{8}$/.test(candidate)) {
		return candidate.slice(0, 7);
	}
	return undefined;
}

function restoreEscapes(value: string): string {
	return value.replaceAll(ESCAPED_OPEN, "[[").replaceAll("\\{", "{").replaceAll("\\}", "}");
}

function pushToken(target: SyntaxToken[], token: SyntaxToken): void {
	if (token.text.length === 0) {
		return;
	}
	const previous = target[target.length - 1];
	if (previous && previous.color === token.color) {
		previous.text += token.text;
		return;
	}
	target.push(token);
}

export function parseSyntaxTokenLine(line: string): SyntaxToken[] {
	const source = line.replaceAll("\\[[", ESCAPED_OPEN);
	const tokens: SyntaxToken[] = [];
	let offset = 0;
	let activeColor: string | undefined;

	while (offset < source.length) {
		const open = source.indexOf("[[", offset);
		if (open < 0) {
			pushToken(tokens, {
				text: restoreEscapes(source.slice(offset)),
				color: activeColor,
			});
			break;
		}

		if (open > offset) {
			pushToken(tokens, {
				text: restoreEscapes(source.slice(offset, open)),
				color: activeColor,
			});
		}

		const close = source.indexOf("]]", open + 2);
		if (close < 0) {
			pushToken(tokens, {
				text: restoreEscapes(source.slice(open)),
				color: activeColor,
			});
			break;
		}

		const tag = source.slice(open + 2, close).trim();
		if (tag === "/") {
			activeColor = undefined;
		} else if (tag.startsWith("fg=")) {
			const color = normalizeHexColor(tag.slice(3));
			if (color) {
				activeColor = color;
			}
		}

		offset = close + 2;
	}

	if (tokens.length === 0) {
		return [{ text: restoreEscapes(source) }];
	}

	return tokens;
}

export type SyntaxTokenLineProps = {
	line: string;
	fallbackIntent?: ColorIntent;
	bold?: boolean;
};

export function SyntaxTokenLine({
	line,
	fallbackIntent = "text",
	bold = false,
}: SyntaxTokenLineProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const fallbackColor = resolveIntentColor(theme, fallbackIntent, colors);
	const tokens = parseSyntaxTokenLine(line);

	return (
		<Text>
			{tokens.map((token, index) => (
				<Text key={`${index}-${token.text}`} color={token.color ?? fallbackColor} bold={bold}>
					{token.text}
				</Text>
			))}
		</Text>
	);
}
