use std::{collections::HashMap, error::Error, fs::File, io::Write};

use super::{convert_to_d3, D3Node};
use cp_hierarchical_clustering::{
    create_hierarchy_from_df, ClusterHierarchy, DendrogramNode, LinkageMethod, Metric,
};
use histdiff_core::HistDiffRes;
use polars::prelude::*;
use rayon::ThreadPoolBuilder;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{analysis::ClusterRes, hd_interface::retrieve_state, ClusterState};

#[tauri::command]
pub fn cluster_hd(
    app: AppHandle,
    mat_metric: Metric,
    linkage: LinkageMethod,
    features: bool,
) -> Result<(), String> {
    let hd = retrieve_state(&app);

    std::thread::spawn(move || {
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get().saturating_sub(2))
            .build()
            .unwrap();

        pool.install(|| {
            let res = hd.expect("HistDiff has not been calculated yet!");

            let data = res.dataframe_scores.unwrap();
            let id_col = grab_col_idx_as_str(&data, 0).unwrap();

            let cluster =
                create_hierarchy_from_df(&data, mat_metric, linkage, &Some(vec![0])).unwrap();
            let row_d3 = convert_to_d3(&cluster, &id_col);

            // WARNING: Remove utility lines
            // _ = write_raw_scores_json("./scores.json", &res.raw_scores);
            // _ = row_d3.write_json("./row_tree.json");

            // log::warn!("{}", row_d3.to_json());

            let mut feat_clust: Option<String> = None;
            if features {
                // get rid of if col
                let id_col_name = &data.get_column_names()[0];
                let mut data_no_id = data.drop(id_col_name).unwrap();

                // rename cols to if col names
                let id_series = data.column(id_col_name).unwrap().as_series().unwrap();
                let id_col_names: Vec<String> = id_series.iter().map(|s| s.to_string()).collect();

                // idx - 1 because we dropped first row
                let feature_map: HashMap<usize, String> = data
                    .get_column_names()
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != 0)
                    .map(|(idx, name)| (idx - 1, name.to_string()))
                    .collect();

                let data_t = data_no_id
                    .transpose(None, Some(rayon::iter::Either::Right(id_col_names)))
                    .unwrap();

                let cluster_features =
                    create_hierarchy_from_df(&data_t, mat_metric, linkage, &None).unwrap();

                let d3_features = convert_to_d3(&cluster_features, &feature_map);
                feat_clust = Some(d3_features.to_json());

                //WARNING: Remove utility lines
                // _ = d3_features.write_json("./feat_tree.json");

                // log::warn!("{}", d3_features.to_json());
            }

            let clust_res = ClusterRes::new(Some(row_d3.to_json()), feat_clust);
            let state = app.state::<ClusterState>();
            let mut guard = state.cluster_res.lock().unwrap();
            *guard = Some(clust_res);

            _ = app.emit("cluster-complete", ());
        });
    });

    return Ok(());
}

#[tauri::command]
pub fn get_cluster_res(state: State<'_, ClusterState>) -> Option<ClusterRes> {
    let res = state.cluster_res.lock().unwrap().clone();
    // if let Some(ref clust) = res {
    //     let dbg_str = serde_json::to_string(&clust).unwrap();
    //     log::warn!("{}", dbg_str);
    // }
    return res;
}

/// Helper function to write scores to json
fn write_raw_scores_json<K, V>(fp: &str, hd_res: &HashMap<K, V>) -> Result<(), Box<dyn Error>>
where
    K: Serialize,
    V: Serialize,
{
    let j_str =
        serde_json::to_string(&hd_res).map_err(|e| format!("Serialization error: {}", e))?;

    let mut file = File::create(fp)?;
    _ = file.write_all(j_str.as_bytes());

    return Ok(());
}

fn grab_col_idx_as_str(df: &DataFrame, idx: usize) -> PolarsResult<HashMap<usize, String>> {
    let series = df
        .select_at_idx(idx)
        .ok_or_else(|| PolarsError::NoData("DataFrame has no columns".into()))?;

    let str_col = series.str()?;

    let vec: HashMap<usize, String> = str_col
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (idx, val.unwrap_or_default().to_string()))
        .collect();

    return Ok(vec);
}

#[cfg(test)]
mod cluster_hierarchy_test {
    use super::*;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_write_raw_scores_json_creates_valid_json_file() {
        let mut scores = HashMap::new();
        scores.insert("foo", 1);
        scores.insert("bar", 2);

        let path = "test_scores.json";
        write_raw_scores_json(path, &scores).unwrap();

        // Check file content
        let data = fs::read_to_string(path).unwrap();
        assert!(data.contains("\"foo\":1"));
        assert!(data.contains("\"bar\":2"));

        // Clean up
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_grab_col_idx_as_str_extracts_strings() {
        let ids = Series::new("ids".into(), &["a", "b", "c"]);
        let vals = Series::new("vals".into(), &[10, 20, 30]);
        let df = DataFrame::new(vec![ids.clone().into(), vals.clone().into()]).unwrap();

        let res = grab_col_idx_as_str(&df, 0).unwrap();
        assert_eq!(res.len(), 3);
        assert_eq!(res.get(&0), Some(&"a".to_string()));
        assert_eq!(res.get(&1), Some(&"b".to_string()));
        assert_eq!(res.get(&2), Some(&"c".to_string()));
    }
}
