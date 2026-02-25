import { Text } from "ink";
import { useEffect, useState } from "react";

const FRAMES = ["-", "\\", "|", "/"];

export type SpinnerProps = {
	label?: string;
	intervalMs?: number;
};

export function Spinner({ label = "Loading", intervalMs = 80 }: SpinnerProps) {
	const [frame, setFrame] = useState(0);

	useEffect(() => {
		const timer = setInterval(() => {
			setFrame((value) => (value + 1) % FRAMES.length);
		}, intervalMs);

		return () => {
			clearInterval(timer);
		};
	}, [intervalMs]);

	return <Text>{`${FRAMES[frame]} ${label}`}</Text>;
}
