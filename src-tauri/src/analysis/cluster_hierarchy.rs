use std::collections::HashMap;

use super::convert_to_d3;
use cp_hierarchical_clustering::{
    create_hierarchy_from_df, ClusterHierarchy, DendrogramNode, LinkageMethod, Metric,
};
use polars::prelude::*;
use tauri::AppHandle;

use crate::hd_interface::retrieve_state;

#[tauri::command]
pub fn cluster_hd(app: AppHandle, mat_metric: Metric, linkage: LinkageMethod) -> Option<String> {
    let hd = retrieve_state(&app);

    if let Some(res) = hd {
        let data = res.dataframe_scores?;

        let id_col = grab_col_idx_as_str(&data, 0).unwrap();

        let cluster = create_hierarchy_from_df(&data, mat_metric, linkage, &Some(vec![0])).unwrap();

        let d3 = convert_to_d3(&cluster, &id_col);
        log::warn!("{}", d3.to_json());
        log::warn!("{:?}", cluster.leaf_ordering());
        return Some(d3.to_json());
    } else {
        return None;
    }
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
