import { Text } from "ink";
import { useEffect, useMemo, useState, type ReactNode } from "react";
import { useHotkeys } from "../hooks/use-hotkeys";
import { useAsyncTask } from "../hooks/use-async-task";
import { clampIndex } from "../utils/clamp";
import { PageFrame } from "./page-frame";
import { TextField } from "../components/forms/text-field";
import { ReviewScreen } from "./review-screen";

export type WizardCard = {
	title: string;
	lines: ReadonlyArray<string>;
};

export function wizardClampIndex(index: number, count: number): number {
	return clampIndex(index, count);
}

export function formatWizardCard(card: WizardCard, selected: boolean): string[] {
	const marker = selected ? ">" : " ";
	const title = typeof card.title === "string" ? card.title : "";
	const lines = Array.isArray(card.lines) ? card.lines : [];
	return [`${marker} ${title}`, ...lines.map((line) => `  ${line}`), ""];
}

// Backward-compatible helper names for tests and callsites during the cutover.
export const pageClampIndex = wizardClampIndex;
export const formatPageCard = formatWizardCard;

export type WizardFlowProps = {
	title: string;
	instruction: string;
	cards: ReadonlyArray<WizardCard>;
	sidebar?: ReactNode;
	footerText?: string;
	defaultIndex?: number;
	onSelect: (index: number) => void;
	onCancel?: () => void;
};

export function WizardFlow({
	title,
	instruction,
	cards,
	sidebar,
	footerText,
	defaultIndex = 0,
	onSelect,
	onCancel,
}: WizardFlowProps) {
	const [selected, setSelected] = useState(() => wizardClampIndex(defaultIndex, cards.length));

	useEffect(() => {
		setSelected((index) => wizardClampIndex(index, cards.length));
	}, [cards.length]);

	useHotkeys(["up", "k"], () => {
		setSelected((value) => wizardClampIndex(value - 1, cards.length));
	});

	useHotkeys(["down", "j"], () => {
		setSelected((value) => wizardClampIndex(value + 1, cards.length));
	});

	useHotkeys(["enter"], () => {
		if (cards.length === 0) {
			return;
		}

		onSelect(wizardClampIndex(selected, cards.length));
	});

	useHotkeys(["escape", "q"], () => {
		onCancel?.();
	});

	const rendered = useMemo(() => {
		const lines: string[] = [instruction, ""];

		if (cards.length === 0) {
			lines.push("No items available.");
			return lines.join("\n");
		}

		cards.forEach((card, index) => {
			lines.push(...formatWizardCard(card, index === selected));
		});

		return lines.join("\n").trimEnd();
	}, [cards, instruction, selected]);

	return (
		<PageFrame
			title={title}
			sidebar={sidebar}
			footer={footerText ?? "Arrows/j/k move | Enter select | q/Esc cancel"}
		>
			<Text>{rendered}</Text>
		</PageFrame>
	);
}

export type AsyncWizardSelection<T> = {
	index: number;
	item: T;
};

export type AsyncWizardFlowProps<T> = {
	title: string;
	instruction: string;
	load: (signal: AbortSignal) => Promise<ReadonlyArray<T>>;
	toCard: (item: T, index: number) => WizardCard;
	sidebar?: ReactNode;
	footerText?: string;
	defaultIndex?: number;
	loadingMessage?: string;
	emptyMessage?: string;
	refreshKeys?: ReadonlyArray<string>;
	onSelect: (selection: AsyncWizardSelection<T>) => void;
	onCancel?: () => void;
};

export function AsyncWizardFlow<T>({
	title,
	instruction,
	load,
	toCard,
	sidebar,
	footerText,
	defaultIndex = 0,
	loadingMessage,
	emptyMessage,
	refreshKeys,
	onSelect,
	onCancel,
}: AsyncWizardFlowProps<T>) {
	const [selected, setSelected] = useState(() => wizardClampIndex(defaultIndex, 1));
	const task = useAsyncTask(load, []);
	const items = task.value ?? [];

	useEffect(() => {
		setSelected((current) => wizardClampIndex(current, Math.max(1, items.length)));
	}, [items.length]);

	useHotkeys(["up", "k"], () => {
		if (task.loading || task.error || items.length === 0) {
			return;
		}
		setSelected((value) => wizardClampIndex(value - 1, items.length));
	});

	useHotkeys(["down", "j"], () => {
		if (task.loading || task.error || items.length === 0) {
			return;
		}
		setSelected((value) => wizardClampIndex(value + 1, items.length));
	});

	useHotkeys(refreshKeys ?? ["r"], () => {
		task.reload();
	});

	useHotkeys(["enter"], () => {
		if (task.loading || task.error || items.length === 0) {
			return;
		}

		const index = wizardClampIndex(selected, items.length);
		onSelect({
			index,
			item: items[index]!,
		});
	});

	useHotkeys(["escape", "q"], () => {
		onCancel?.();
	});

	const cards = useMemo(() => items.map((item, index) => toCard(item, index)), [items, toCard]);

	const rendered = useMemo(() => {
		const lines: string[] = [instruction, ""];

		if (task.loading) {
			lines.push(loadingMessage ?? "Loading...");
			return lines.join("\n");
		}

		if (task.error) {
			lines.push("Load failed:");
			lines.push(task.error);
			lines.push("");
			lines.push("Press r to retry.");
			return lines.join("\n");
		}

		if (cards.length === 0) {
			lines.push(emptyMessage ?? "No items available.");
			lines.push("");
			lines.push("Press r to refresh.");
			return lines.join("\n");
		}

		cards.forEach((card, index) => {
			lines.push(...formatWizardCard(card, index === selected));
		});

		return lines.join("\n").trimEnd();
	}, [cards, emptyMessage, instruction, loadingMessage, selected, task.error, task.loading]);

	return (
		<PageFrame
			title={title}
			sidebar={sidebar}
			footer={footerText ?? "Arrows/j/k move | Enter select | r refresh | q/Esc cancel"}
		>
			<Text>{rendered}</Text>
		</PageFrame>
	);
}

export type TextEntryScreenProps = {
	title: string;
	instruction: string;
	label: string;
	sidebar?: ReactNode;
	footerText?: string;
	initialValue?: string;
	allowEmpty?: boolean;
	onSubmit: (value: string) => void;
	onCancel?: () => void;
};

export function TextEntryScreen({
	title,
	instruction,
	label,
	sidebar,
	footerText,
	initialValue,
	allowEmpty,
	onSubmit,
	onCancel,
}: TextEntryScreenProps) {
	return (
		<PageFrame
			title={title}
			sidebar={sidebar}
			footer={footerText ?? "Type value | Backspace delete | Enter submit | q/Esc cancel"}
		>
			<TextField
				label={label}
				instruction={instruction}
				initialValue={initialValue}
				allowEmpty={allowEmpty}
				onSubmit={onSubmit}
				onCancel={onCancel}
			/>
		</PageFrame>
	);
}

export type ReviewScreenFlowProps = {
	title: string;
	instruction: string;
	content: string;
	sidebar?: ReactNode;
	footerText?: string;
	onClose?: () => void;
};

export function ReviewScreenFlow({
	title,
	instruction,
	content,
	sidebar,
	footerText,
	onClose,
}: ReviewScreenFlowProps) {
	const fullContent = `${instruction}\n\n${content}`.trimEnd();

	return (
		<ReviewScreen
			title={title}
			sidebar={sidebar}
			content={fullContent}
			footerText={footerText ?? "Arrows/PageUp/PageDown scroll | Enter/q/Esc close"}
			onClose={onClose}
		/>
	);
}
