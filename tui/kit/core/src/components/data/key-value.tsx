import { Text } from "ink";

export type KeyValueEntry = {
	key: string;
	value: string;
};

export type KeyValueProps = {
	entries: ReadonlyArray<KeyValueEntry>;
};

export function renderKeyValue(entries: ReadonlyArray<KeyValueEntry>): string {
	if (entries.length === 0) {
		return "";
	}

	const maxKey = entries.reduce((size, entry) => Math.max(size, entry.key.length), 1);
	return entries.map((entry) => `${entry.key.padEnd(maxKey, " ")} : ${entry.value}`).join("\n");
}

export function KeyValue({ entries }: KeyValueProps) {
	return <Text>{renderKeyValue(entries)}</Text>;
}
