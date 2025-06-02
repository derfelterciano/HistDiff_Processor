use std::error::Error;
use std::sync::Mutex;

use tauri::AppHandle;
use tauri::State;

use super::HistDiffState;

impl HistDiffState {
    pub fn new() -> Self {
        return Self {
            hd_res: Mutex::new(None),
        };
    }
}

use super::ClusterState;

impl ClusterState {
    pub fn new() -> Self {
        return Self {
            cluster_res: Mutex::new(None),
        };
    }
}

/// Resets all app states
#[tauri::command]
pub fn reset_state(
    hd_state: State<'_, HistDiffState>,
    cluster_state: State<'_, ClusterState>,
) -> Result<(), String> {
    {
        let mut guard = hd_state.hd_res.lock().unwrap();
        *guard = None;
    }

    {
        let mut guard = cluster_state.cluster_res.lock().unwrap();
        *guard = None;
    }

    return Ok(());
}
