import { Text } from "ink";
import { useState } from "react";
import { useHotkeys } from "../../hooks/use-hotkeys";

export type ConfirmProps = {
	prompt: string;
	defaultYes?: boolean;
	onConfirm: (value: boolean) => void;
	onCancel?: () => void;
};

export function Confirm({ prompt, defaultYes = true, onConfirm, onCancel }: ConfirmProps) {
	const [selectedYes, setSelectedYes] = useState(defaultYes);

	useHotkeys(["left", "right", "tab", "S-tab", "h", "l"], () => {
		setSelectedYes((value) => !value);
	});

	useHotkeys(["y", "Y"], () => {
		setSelectedYes(true);
	});

	useHotkeys(["n", "N"], () => {
		setSelectedYes(false);
	});

	useHotkeys(["enter"], () => {
		onConfirm(selectedYes);
	});

	useHotkeys(["escape", "q"], () => {
		onCancel?.();
	});

	const yesMarker = selectedYes ? "[x]" : "[ ]";
	const noMarker = selectedYes ? "[ ]" : "[x]";

	return <Text>{[prompt, "", `${yesMarker} Yes`, `${noMarker} No`].join("\n")}</Text>;
}
