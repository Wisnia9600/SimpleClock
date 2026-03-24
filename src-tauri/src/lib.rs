// Suppress console window in release builds on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            toggle_maximize,
            resize_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running SimpleClock");
}

/// Close the current window
#[tauri::command]
fn close_window(window: tauri::WebviewWindow) {
    let _ = window.close();
}

/// Minimize the current window
#[tauri::command]
fn minimize_window(window: tauri::WebviewWindow) {
    let _ = window.minimize();
}

/// Toggle maximize / restore
#[tauri::command]
fn toggle_maximize(window: tauri::WebviewWindow) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

/// Resize the window based on DOM content height
#[tauri::command]
fn resize_window(window: tauri::WebviewWindow, height: f64) {
    let _ = window.set_size(tauri::LogicalSize::new(340.0, height));
}
