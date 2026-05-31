export default function openShortcutLabel() {
  if (typeof navigator === "undefined") return "Ctrl+O";
  return /Mac|iPhone|iPod|iPad/.test(navigator.userAgent) ? "⌘O" : "Ctrl+O";
}
