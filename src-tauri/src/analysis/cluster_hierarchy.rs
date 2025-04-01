use cp_hierarchical_clustering::{create_hierarchy_from_df, ClusterHierarchy};
use tauri::AppHandle;

use crate::hd_interface::retrieve_state;

#[tauri::command]
pub fn cluster_hd(app: AppHandle) -> Option<ClusterHierarchy> {
    let hd = retrieve_state(&app);

    if let Some(res) = hd {
        let data = res.dataframe_scores?;
        log::info!("{}", data);

        return None;
    } else {
        return None;
    }
}
