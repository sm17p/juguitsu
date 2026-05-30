import AppTabs from "./components/AppTabs";
import DevAgentation from "./components/DevAgentation";
import "./App.css";

export default function App() {
  return (
    <div className="app">
      <AppTabs />
      <DevAgentation />
    </div>
  );
}
