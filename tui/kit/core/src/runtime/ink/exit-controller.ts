export type ExitSubscriber = () => void;

export type ExitController = {
	requestExit: () => void;
	subscribe: (subscriber: ExitSubscriber) => () => void;
};

export function createExitController(onExit?: () => void): ExitController {
	const subscribers = new Set<ExitSubscriber>();

	const requestExit = () => {
		onExit?.();
		for (const subscriber of subscribers) {
			subscriber();
		}
	};

	return {
		requestExit,
		subscribe: (subscriber: ExitSubscriber) => {
			subscribers.add(subscriber);
			return () => {
				subscribers.delete(subscriber);
			};
		},
	};
}
