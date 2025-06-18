use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_control_selector_win(
    app: tauri::AppHandle,
    plate_format: u32,
    modify: bool,
    id: u32,
    initial_wells: String,
) {
    let label = format!("control-selector-{}", id);

    // window open?
    if app.get_webview_window(&label).is_none() {
        let _ = WebviewWindowBuilder::new(
            &app,
            label,
            WebviewUrl::App(
                format!(
                    "/control-selector?plate={}&modify={}&id={}&initialWells={}",
                    plate_format, modify, id, initial_wells
                )
                .into(),
            ),
        )
        .title("Select controls")
        .resizable(false)
        .inner_size(1024.0, 768.0)
        .build();
    }
}

#[tauri::command]
pub async fn open_logging_window(app: tauri::AppHandle) {
    let label = format!("Logs");

    // check if window is open
    if app.get_webview_window(&label).is_none() {
        _ = WebviewWindowBuilder::new(&app, label, WebviewUrl::App(format!("/logs").into()))
            .title("HistDiff Logs")
            .resizable(false)
            .inner_size(600.0, 400.0)
            .build();
    }
}

#[tauri::command]
pub async fn open_analysis(app: tauri::AppHandle) {
    let label = format!("analysis");

    if app.get_webview_window(&label).is_none() {
        _ = WebviewWindowBuilder::new(&app, label, WebviewUrl::App(format!("/analysis").into()))
            .title("Analysis")
            .resizable(true)
            .inner_size(1024.0, 800.0)
            .build();
    }
}
