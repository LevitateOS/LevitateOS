import { Text } from "ink";

export type EmptyStateProps = {
	title: string;
	hint?: string;
};

export function EmptyState({ title, hint }: EmptyStateProps) {
	return <Text>{hint && hint.length > 0 ? `${title}\n${hint}` : title}</Text>;
}
