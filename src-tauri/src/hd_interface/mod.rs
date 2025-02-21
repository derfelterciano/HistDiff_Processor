use histdiff_core::UserConfig;
use std::collections::HashMap;

pub struct SvelteConfig {
    pub dataset_path: String,
    pub plate_format: u32,
    pub well_name: String,
    pub add_meta_cols: Option<Vec<String>>,
    pub negative_control: ControlSelection,
    pub add_controls: Option<Vec<ControlSelection>>,
}

pub struct ControlSelection {
    pub name: String,
    pub wells: Vec<String>,
}
