use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_control_selector_win(app: tauri::AppHandle) {
    let label = "control-selector";

    // window open?
    if app.get_webview_window(label).is_none() {
        let _ = WebviewWindowBuilder::new(&app, label, WebviewUrl::App("/control-selector".into()))
            .title("Select controls")
            .resizable(true)
            .inner_size(600.0, 400.0)
            .build();
    }
}
