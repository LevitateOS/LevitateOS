import { useCallback, useEffect, useMemo, useState } from "react";

export type AsyncTaskState<T> = {
	loading: boolean;
	value: T | null;
	error: string | null;
};

export type AsyncTaskResult<T> = AsyncTaskState<T> & {
	reload: () => void;
};

export function useAsyncTask<T>(
	load: (signal: AbortSignal) => Promise<T>,
	deps: ReadonlyArray<unknown> = [],
): AsyncTaskResult<T> {
	const [revision, setRevision] = useState(0);
	const [state, setState] = useState<AsyncTaskState<T>>({
		loading: true,
		value: null,
		error: null,
	});

	const reload = useCallback(() => {
		setRevision((value) => value + 1);
	}, []);

	useEffect(() => {
		const controller = new AbortController();
		let active = true;

		setState((current) => ({
			...current,
			loading: true,
			error: null,
		}));

		void Promise.resolve(load(controller.signal))
			.then((value) => {
				if (!active || controller.signal.aborted) {
					return;
				}

				setState({
					loading: false,
					value,
					error: null,
				});
			})
			.catch((error: unknown) => {
				if (!active || controller.signal.aborted) {
					return;
				}

				setState({
					loading: false,
					value: null,
					error: error instanceof Error ? error.message : String(error),
				});
			});

		return () => {
			active = false;
			controller.abort();
		};
	}, [load, revision, ...deps]);

	return useMemo(
		() => ({
			...state,
			reload,
		}),
		[reload, state],
	);
}
