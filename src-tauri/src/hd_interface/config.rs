use histdiff_core::{calculate_scores, HistDiffRes, UserConfig};
use log;
use rayon::ThreadPoolBuilder;
use serde_json::Value;
use std::sync::Arc;
use tauri::{Emitter, Manager, State};

use super::{HistDiffState, SvelteConfig};

#[tauri::command]
pub fn process_hd(app: tauri::AppHandle, config: SvelteConfig) {
    let hd_config = svelte_to_hd_config(config);
    // println!("Max threads: {:?}", num_cpus::get());

    log::info!("Max threads: {:?}", num_cpus::get());
    log::info!("Begin HistDiff...");
    let mut hd_res: Option<HistDiffRes> = None;

    std::thread::spawn(move || {
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() - 2)
            .build()
            .unwrap();

        pool.install(|| {
            hd_res = Some(calculate_scores(&hd_config).expect("HistDiff could not be calculated"));
        });

        if let Some(hd) = hd_res {
            let state = app.state::<HistDiffState>();
            let mut guard = state.hd_res.lock().unwrap();
            *guard = Some(hd);
        }

        // log::info!("{:?}", &hd_res.unwrap().dataframe_scores); // TODO: send this to a global struct
        _ = app.emit("hd-completed", ());
    });
}

fn svelte_to_hd_config(config: SvelteConfig) -> UserConfig {
    let path = config.dataset_path.as_str();
    let id = vec![config.well_name.clone()];
    let useless_meta = config.add_meta_cols.clone();
    let plate = &config.plate_format; // TODO: Convert between the different plates
    let ref_cntrls = config.negative_control.wells.clone();

    return UserConfig::new(path, id, useless_meta, true, None, None, ref_cntrls, None);
}

#[tauri::command]
pub fn write_res(app: tauri::AppHandle, out_path: String) {
    let state = app.state::<HistDiffState>();

    let guard = state.hd_res.lock().unwrap();

    match &*guard {
        Some(res) => {
            log::info!("{:?}", res.dataframe_scores);
            res.clone().to_csv(out_path);
        }
        None => {
            log::error!("Couldn't get scores");
        }
    }
}
