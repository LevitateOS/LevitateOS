import { Text } from "ink";
import { Component, type ErrorInfo, type ReactNode } from "react";

type ErrorBoundaryProps = {
	fallback?: ReactNode;
	onError?: (error: Error, errorInfo: ErrorInfo) => void;
	children: ReactNode;
};

type ErrorBoundaryState = {
	error: Error | null;
};

export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
	public state: ErrorBoundaryState = {
		error: null,
	};

	public static getDerivedStateFromError(error: Error): ErrorBoundaryState {
		return {
			error,
		};
	}

	public componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
		this.props.onError?.(error, errorInfo);
	}

	public render(): ReactNode {
		if (!this.state.error) {
			return this.props.children;
		}

		if (this.props.fallback) {
			return this.props.fallback;
		}

		return <Text color="red">Unhandled TUI error: {this.state.error.message}</Text>;
	}
}
