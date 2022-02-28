#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use tauri::WindowBuilder;

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
        .setup(|app| {
            app.create_window(
                "overlay",
                tauri::WindowUrl::App("index.html".into()),
                move |window_builder, attributes| {
                    (
                        window_builder
                            .transparent(true)
                            .visible(false)
                            .always_on_top(true)
                            .skip_taskbar(true),
                        attributes,
                    )
                },
            )
            .map(|_| ())
            .map_err(|err| Box::new(err) as Box<dyn Error + Send>)
        })
        .run(tauri::generate_context!())
        .unwrap();
}
