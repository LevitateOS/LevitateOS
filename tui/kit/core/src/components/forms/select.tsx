import { Text } from "ink";
import { useMemo, useState } from "react";
import { useHotkeys } from "../../hooks/use-hotkeys";
import { clampIndex } from "../../utils/clamp";

export type SelectProps = {
	items: ReadonlyArray<string>;
	initialIndex?: number;
	onSelect: (index: number) => void;
	onCancel?: () => void;
	emptyLabel?: string;
};

export function Select({
	items,
	initialIndex = 0,
	onSelect,
	onCancel,
	emptyLabel = "(no items)",
}: SelectProps) {
	const [selected, setSelected] = useState(() => clampIndex(initialIndex, items.length));

	useHotkeys(["up", "k"], () => {
		setSelected((value) => clampIndex(value - 1, items.length));
	});

	useHotkeys(["down", "j"], () => {
		setSelected((value) => clampIndex(value + 1, items.length));
	});

	useHotkeys(["enter"], () => {
		if (items.length === 0) {
			return;
		}

		onSelect(clampIndex(selected, items.length));
	});

	useHotkeys(["escape", "q"], () => {
		onCancel?.();
	});

	const rendered = useMemo(() => {
		if (items.length === 0) {
			return emptyLabel;
		}

		return items.map((item, index) => `${index === selected ? ">" : " "} ${item}`).join("\n");
	}, [emptyLabel, items, selected]);

	return <Text>{rendered}</Text>;
}
