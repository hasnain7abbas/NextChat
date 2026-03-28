use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

pub fn create_system_tray() -> SystemTray {
    let quick_actions = SystemTraySubmenu::new(
        "Quick Actions",
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("summarize", "Summarize Clipboard"))
            .add_item(CustomMenuItem::new("rewrite", "Rewrite Clipboard"))
            .add_item(CustomMenuItem::new("translate", "Translate Clipboard"))
            .add_item(CustomMenuItem::new("fix_grammar", "Fix Grammar"))
            .add_item(CustomMenuItem::new("explain", "Explain Clipboard")),
    );

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show Window"))
        .add_item(CustomMenuItem::new("new_chat", "New Chat"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(quick_actions)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            show_main_window(app);
        }
        SystemTrayEvent::DoubleClick { .. } => {
            show_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => show_main_window(app),
            "new_chat" => {
                show_main_window(app);
                let _ = app.emit_all("tray-action", "new_chat");
            }
            "summarize" => {
                let _ = app.emit_all("clipboard-action", "summarize");
            }
            "rewrite" => {
                let _ = app.emit_all("clipboard-action", "rewrite");
            }
            "translate" => {
                let _ = app.emit_all("clipboard-action", "translate");
            }
            "fix_grammar" => {
                let _ = app.emit_all("clipboard-action", "fix_grammar");
            }
            "explain" => {
                let _ = app.emit_all("clipboard-action", "explain");
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}
