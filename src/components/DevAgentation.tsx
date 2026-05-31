import { Agentation } from "agentation";

export default function DevAgentation() {
  if (!import.meta.env.DEV) {
    return null;
  }

  return <Agentation endpoint="http://localhost:4747" />;
}
