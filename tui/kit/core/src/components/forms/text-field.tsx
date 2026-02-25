import { Text } from "ink";
import { useState } from "react";
import { useHotkeys } from "../../hooks/use-hotkeys";

function isPrintableInput(input: string): boolean {
	if (typeof input !== "string" || input.length === 0) {
		return false;
	}

	const code = input.charCodeAt(0);
	return code >= 32 && code !== 127;
}

export type TextFieldProps = {
	label: string;
	instruction?: string;
	initialValue?: string;
	allowEmpty?: boolean;
	onSubmit: (value: string) => void;
	onCancel?: () => void;
};

export function TextField({
	label,
	instruction,
	initialValue = "",
	allowEmpty = true,
	onSubmit,
	onCancel,
}: TextFieldProps) {
	const [value, setValue] = useState(initialValue);

	useHotkeys(["backspace"], () => {
		setValue((current) => current.slice(0, -1));
	});

	useHotkeys(["enter"], () => {
		const trimmed = value.trim();
		if (!allowEmpty && trimmed.length === 0) {
			return;
		}
		onSubmit(trimmed);
	});

	useHotkeys(["escape", "q"], () => {
		onCancel?.();
	});

	useHotkeys(undefined, ({ input, key }) => {
		if (
			key.ctrl ||
			key.meta ||
			key.return ||
			key.escape ||
			key.tab ||
			key.backspace ||
			key.delete
		) {
			return;
		}

		if (!isPrintableInput(input)) {
			return;
		}

		setValue((current) => current + input);
	});

	const prompt = `${label}: ${value}_`;
	const text =
		instruction && instruction.trim().length > 0 ? `${instruction}\n\n${prompt}` : prompt;

	return <Text>{text}</Text>;
}
