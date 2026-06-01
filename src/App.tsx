import { RegistryProvider } from "@effect-atom/atom-react";

import DevAgentation from "@/components/DevAgentation";
import WorkbenchLayout from "@/components/WorkbenchLayout";

export default function App() {
  return (
    <RegistryProvider>
      <div className="flex h-screen min-h-0 flex-col overflow-hidden">
        <WorkbenchLayout />
        <DevAgentation />
      </div>
    </RegistryProvider>
  );
}
