use serde::{Deserialize, Serialize};

mod config;
pub use config::process_hd;

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
