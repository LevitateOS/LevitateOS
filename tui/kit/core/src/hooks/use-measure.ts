import { useTuiViewport } from "../app/app-provider";

export type ViewportMeasure = ReturnType<typeof useTuiViewport>;

export function useMeasure(): ViewportMeasure {
	return useTuiViewport();
}
