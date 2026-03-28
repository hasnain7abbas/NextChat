/**
 * Clipboard Pipeline — Intercepts clipboard contents, processes through
 * the configured LLM, and writes the result back to the clipboard.
 *
 * Activated via:
 *  1. System tray quick actions (summarize, rewrite, translate, etc.)
 *  2. Global hotkey (future: dedicated clipboard hotkey)
 */

const PROMPTS: Record<string, string> = {
  summarize:
    "Summarize the following text concisely in a few bullet points:\n\n",
  rewrite:
    "Rewrite the following text to be clearer and more professional. Return only the rewritten text:\n\n",
  translate:
    "Translate the following text to English. If it is already in English, translate it to Chinese. Return only the translation:\n\n",
  fix_grammar:
    "Fix the grammar and spelling in the following text. Return only the corrected text:\n\n",
  explain: "Explain the following text in simple terms:\n\n",
};

export function getClipboardPrompt(action: string): string | null {
  return PROMPTS[action] ?? null;
}

export async function readClipboard(): Promise<string | null> {
  try {
    if (window.__TAURI__) {
      const result = await window.__TAURI__.invoke("read_clipboard");
      return result as string;
    }
    return await navigator.clipboard.readText();
  } catch {
    return null;
  }
}

export async function writeClipboard(text: string): Promise<void> {
  try {
    if (window.__TAURI__) {
      await window.__TAURI__.invoke("write_clipboard", { text });
      return;
    }
    await navigator.clipboard.writeText(text);
  } catch (e) {
    console.error("Failed to write clipboard:", e);
  }
}
