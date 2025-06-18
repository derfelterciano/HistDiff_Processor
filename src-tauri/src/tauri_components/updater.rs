use log;
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_ln, cont_len| {
                    downloaded += chunk_ln;
                    log::info!("downloaded {downloaded} from {cont_len:?}");
                },
                || {
                    log::info!("download finished");
                },
            )
            .await?;

        log::info!("updated installed!");
        app.restart();
    }

    return Ok(());
}
