import { invoke } from "@tauri-apps/api/core";

/** Names of every Tauri command the webview may invoke. */
export const Commands = {
  injectText: "inject_text",
} as const;

/** Arguments for the `inject_text` command (mirrors the Rust signature). */
export interface InjectTextArgs {
  text: string;
}

/** Inject text at the cursor via the Rust core. */
export function injectText(text: string): Promise<void> {
  const args: InjectTextArgs = { text };
  // Pass a fresh object literal so it satisfies Tauri's `InvokeArgs`
  // (`Record<string, unknown>`), which a named interface type does not.
  return invoke(Commands.injectText, { ...args });
}
