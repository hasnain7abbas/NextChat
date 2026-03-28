use tauri::{ClipboardManager, Manager};

/// Read clipboard text and return it to the frontend for LLM processing.
/// The actual LLM call happens on the frontend side since it has access
/// to the configured provider and API keys.
#[tauri::command]
pub fn read_clipboard(app_handle: tauri::AppHandle) -> Result<String, String> {
    app_handle
        .clipboard_manager()
        .read_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?
        .ok_or_else(|| "Clipboard is empty".to_string())
}

/// Write processed text back to the clipboard
#[tauri::command]
pub fn write_clipboard(app_handle: tauri::AppHandle, text: String) -> Result<(), String> {
    let mut clipboard = app_handle.clipboard_manager();
    clipboard
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))
}
