const invoke = window.__TAURI__.invoke;

// @3
export async function invokeSetWindowDecorations(decoration) {
  return await invoke("set_window_decorations", { decoration })
}
