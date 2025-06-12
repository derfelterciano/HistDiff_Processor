use histdiff_core::{calculate_scores, HistDiffRes, UserConfig};
use log::{self, error};
use rayon::ThreadPoolBuilder;
use serde_json::Value;
use std::{collections::HashMap, error::Error, sync::Arc};
use tauri::{Emitter, Manager, State};

use super::{clean_well_names, HistDiffState, NegControlState, SvelteConfig};

#[tauri::command]
pub fn process_hd(app: tauri::AppHandle, config: SvelteConfig) {
    let hd_config = svelte_to_hd_config(config);
    // println!("Max threads: {:?}", num_cpus::get());

    {
        let state = app.state::<NegControlState>();
        let mut guard = state.cntrls.lock().unwrap();
        *guard = Some(hd_config.vehicle_cntrls.clone());
    }

    log::info!("Max threads: {:?}", num_cpus::get());
    log::info!("Begin HistDiff...");
    let mut hd_res: Option<HistDiffRes> = None;

    std::thread::spawn(move || {
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() - 2)
            .build()
            .unwrap();

        pool.install(|| {
            match calculate_scores(&hd_config) {
                Ok(result) => {
                    hd_res = Some(result);
                }
                Err(e) => {
                    log::error!("HistDiff failes: {}", e);
                    hd_res = None;
                }
            }
            // hd_res = Some(calculate_scores(&hd_config).expect("HistDiff could not be calculated"));
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
    let ref_cntrls = clean_well_names(&config.negative_control.wells);

    let mut plate_def: Option<Vec<String>> = None;
    if *plate == (96 as u32) {
        plate_def = Some(plate_96_def());
    }

    return UserConfig::new(
        path,
        id,
        useless_meta,
        true,
        None,
        plate_def,
        ref_cntrls,
        None,
    );
}

fn plate_96_def() -> Vec<String> {
    const WELL_96_LETTERS: std::ops::RangeInclusive<u8> = ('A' as u8)..=('H' as u8);
    const WELL_96_NUMBERS: std::ops::RangeInclusive<i32> = 1..=12;

    let mut res: Vec<String> = Vec::new();

    for letter in WELL_96_LETTERS {
        for num in WELL_96_NUMBERS {
            let format_str = format!("{}{}", letter as char, num);
            res.push(format_str);
        }
    }

    return res;
}

#[tauri::command]
pub fn get_hd_scores(
    state: State<'_, HistDiffState>,
) -> Option<HashMap<String, HashMap<String, f64>>> {
    let guard = state.hd_res.lock().unwrap();

    match &*guard {
        Some(res) => {
            return Some(res.raw_scores.clone());
        }
        None => {
            log::error!("Can't retrieve json scores");
            return None;
        }
    }
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

#[tauri::command]
pub fn get_neg_controls(neg_cntrls: State<'_, NegControlState>) -> Option<Vec<String>> {
    let guard = neg_cntrls.cntrls.lock().unwrap();

    match &*guard {
        Some(cntrl) => {
            return Some(cntrl.clone());
        }

        None => {
            return None;
        }
    }
}
