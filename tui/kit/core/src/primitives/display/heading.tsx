import { UiText } from "./text";

export type HeadingProps = {
	title: string;
	level?: 1 | 2 | 3;
};

export function Heading({ title, level = 1 }: HeadingProps) {
	const intent = level === 1 ? "sectionHeading" : "sectionSubheading";
	return (
		<UiText intent={intent} bold>
			{title}
		</UiText>
	);
}
