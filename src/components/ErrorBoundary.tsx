import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import { Component, type ErrorInfo, type ReactNode } from "react";

type Props = {
  children: ReactNode;
};

type State = {
  hasError: boolean;
  message: string;
};

export default class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, message: "" };

  static getDerivedStateFromError(err: Error): State {
    return { hasError: true, message: err.message };
  }

  componentDidCatch(err: Error, info: ErrorInfo) {
    void error(`[react] ${err.message}`);
    void invoke("log_crash", {
      source: "react",
      message: err.message,
      stack: info.componentStack ?? null,
    });
  }

  render() {
    if (this.state.hasError) {
      return (
        <div
          role="alert"
          className="flex h-screen min-h-0 flex-col items-center justify-center gap-2 bg-bg p-4 text-fg"
        >
          <p className="text-sm font-medium">Something went wrong.</p>
          <p className="text-xs text-fg-muted">{this.state.message}</p>
        </div>
      );
    }

    return this.props.children;
  }
}
