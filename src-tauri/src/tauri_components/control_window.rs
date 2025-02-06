use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_control_selector_win(
    app: tauri::AppHandle,
    plate_format: u32,
    modify: bool,
    id: u32,
    inital_wells: String,
) {
    let label = format!("control-selector-{}", id);

    // window open?
    if app.get_webview_window(&label).is_none() {
        let _ = WebviewWindowBuilder::new(
            &app,
            label,
            WebviewUrl::App(
                format!(
                    "/control-selector?plate={}&modify={}&id={}&initalWells={}",
                    plate_format, modify, id, inital_wells
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
