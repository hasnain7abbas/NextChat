// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod stream;
mod tray;

use tauri::{GlobalShortcutManager, Manager};

fn main() {
    tauri::Builder::default()
        .system_tray(tray::create_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .setup(|app| {
            // Register global hotkey: Ctrl+Shift+A to toggle window
            let handle = app.handle();
            let mut shortcut_manager = app.global_shortcut_manager();

            let hotkey_handle = handle.clone();
            shortcut_manager
                .register("CmdOrCtrl+Shift+A", move || {
                    if let Some(window) = hotkey_handle.get_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                })
                .unwrap_or_else(|e| {
                    eprintln!("Failed to register global shortcut: {}", e);
                });

            // Emit hotkey-registered event so frontend knows it's active
            if let Some(window) = app.get_window("main") {
                let _ = window.emit("hotkey-registered", "CmdOrCtrl+Shift+A");
            }

            Ok(())
        })
        // Minimize to tray instead of closing
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap_or_default();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            stream::stream_fetch,
            clipboard::read_clipboard,
            clipboard::write_clipboard,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
