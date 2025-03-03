use std::sync::Mutex;

use histdiff_core::HistDiffRes;
use serde::{Deserialize, Serialize};

mod config;
mod states;
pub use config::*;
pub use states::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SvelteConfig {
    pub dataset_path: String,
    pub plate_format: u32,
    pub well_name: String,
    pub add_meta_cols: Option<Vec<String>>,
    pub negative_control: ControlSelection,
    pub add_controls: Option<Vec<ControlSelection>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControlSelection {
    pub name: String,
    pub wells: Vec<String>,
}

pub struct HistDiffState {
    pub hd_res: Mutex<Option<HistDiffRes>>,
}
