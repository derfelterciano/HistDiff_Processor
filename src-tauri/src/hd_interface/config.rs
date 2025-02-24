use histdiff_core::{calculate_scores, UserConfig};
use serde_json::Value;

use super::SvelteConfig;

#[tauri::command]
pub fn process_hd(config: SvelteConfig) {
    let hd_config = svelte_to_hd_config(config);

    let hd_res = calculate_scores(&hd_config).expect("HistDiff could not be calculated");
    println!("{:?}", hd_res);
}

fn svelte_to_hd_config(config: SvelteConfig) -> UserConfig {
    let path = config.dataset_path.as_str();
    let id = vec![config.well_name.clone()];
    let useless_meta = config.add_meta_cols.clone();
    let plate = &config.plate_format; // TODO: Convert between the different plates
    let ref_cntrls = config.negative_control.wells.clone();

    return UserConfig::new(path, id, useless_meta, true, None, None, ref_cntrls, None);
}
