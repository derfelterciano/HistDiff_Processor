use std::{collections::HashMap, error::Error, fs::File, io::Write};

use super::{convert_to_d3, D3Node};
use cp_hierarchical_clustering::{
    create_hierarchy_from_df, ClusterHierarchy, DendrogramNode, LinkageMethod, Metric,
};
use histdiff_core::HistDiffRes;
use polars::prelude::*;
use serde::Serialize;
use tauri::AppHandle;

use crate::hd_interface::retrieve_state;

#[tauri::command]
pub fn cluster_hd(
    app: AppHandle,
    mat_metric: Metric,
    linkage: LinkageMethod,
    features: bool,
) -> Option<String> {
    let hd = retrieve_state(&app);

    if let Some(res) = hd {
        let mut data = res.dataframe_scores?;

        let id_col = grab_col_idx_as_str(&data, 0).unwrap();

        let cluster = create_hierarchy_from_df(&data, mat_metric, linkage, &Some(vec![0])).unwrap();

        let d3 = convert_to_d3(&cluster, &id_col);
        log::warn!("{}", d3.to_json());
        log::warn!("{:?}", cluster.leaf_ordering());

        if features {
            let id_col_name = &data.get_column_names()[0];
            let mut data_no_id = data.drop(&id_col_name).unwrap();

            let feature_names = data
                .get_column_names()
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx != 0)
                .map(|(_, n)| n.to_string())
                .collect::<Vec<_>>();

            let id_series = data.column(id_col_name).unwrap().as_series().unwrap();
            let id_col_names: Vec<String> = id_series.iter().map(|s| s.to_string()).collect();

            let mut transposed_data = data_no_id
                .transpose(None, Some(rayon::iter::Either::Right(id_col_names)))
                .unwrap();

            let feat_col = Column::new("features".into(), feature_names);
            _ = transposed_data.insert_column(0, feat_col).unwrap();

            log::info!("ORIGINAL: {}", data);
            log::info!("{}", transposed_data);
        }

        // WARN: Remove below utility lines
        // _ = write_raw_scores_json("./scores.json", &res.raw_scores);
        // _ = d3.write_json("./tree.json");

        return Some(d3.to_json());
    } else {
        return None;
    }
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
