import { useSyncExternalStore } from "react";

const narrowQuery = "(max-width: 48rem)";

function subscribeNarrow(onStoreChange: () => void) {
  const media = window.matchMedia(narrowQuery);
  media.addEventListener("change", onStoreChange);
  return () => media.removeEventListener("change", onStoreChange);
}

function getNarrowSnapshot() {
  return window.matchMedia(narrowQuery).matches;
}

function getNarrowServerSnapshot() {
  return false;
}

export default function useMediaNarrow() {
  return useSyncExternalStore(subscribeNarrow, getNarrowSnapshot, getNarrowServerSnapshot);
}
