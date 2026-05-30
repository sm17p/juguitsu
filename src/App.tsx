import DevAgentation from "./components/DevAgentation";
import WorkbenchLayout from "./components/WorkbenchLayout";
import "./App.css";

export default function App() {
  return (
    <div className="app">
      <WorkbenchLayout />
      <DevAgentation />
    </div>
  );
}
