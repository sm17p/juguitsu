import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { initCrashLogger } from "@/lib/crash-logger";

import App from "@/App";
import ErrorBoundary from "@/components/ErrorBoundary";

import "./index.css";

initCrashLogger();

const rootElement = document.getElementById("root");
if (rootElement == null) {
  throw new Error("Root element not found");
}

createRoot(rootElement).render(
  <StrictMode>
    <ErrorBoundary>
      <App />
    </ErrorBoundary>
  </StrictMode>,
);
