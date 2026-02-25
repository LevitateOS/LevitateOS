import { Select, type SelectProps } from "../forms/select";

export type ListProps = SelectProps;

export function List(props: ListProps) {
	return <Select {...props} />;
}
