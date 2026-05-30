import { Agentation } from "agentation";

const agentationEndpoint = "http://localhost:4747";

export default function DevAgentation() {
  if (!import.meta.env.DEV) {
    return null;
  }

  return <Agentation endpoint={agentationEndpoint} />;
}
