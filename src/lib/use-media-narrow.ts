import { useSyncExternalStore } from "react";

export default function useMediaNarrow() {
  return useSyncExternalStore(
    (onStoreChange) => {
      const media = window.matchMedia("(max-width: 48rem)");
      media.addEventListener("change", onStoreChange);
      return () => media.removeEventListener("change", onStoreChange);
    },
    () => window.matchMedia("(max-width: 48rem)").matches,
    () => false,
  );
}
