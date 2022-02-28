#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .system_tray(
            tauri::SystemTray::new()
                .with_icon(tauri::Icon::Raw(
                    include_bytes!("../icons/icon.ico").to_vec(),
                ))
                .with_menu(
                    tauri::SystemTrayMenu::new()
                        .add_item(tauri::CustomMenuItem::new("quit", "Quit")),
                ),
        )
        .on_system_tray_event(move |app, event| {
            if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => app.exit(0),
                    _ => {}
                }
            }
        })
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .unwrap();
}
