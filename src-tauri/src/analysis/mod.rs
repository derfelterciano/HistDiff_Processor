mod cluster_hierarchy;
mod node_utilities;
use serde::{Deserialize, Serialize};
use std::io::Result;
use std::{fs::File, path::Path};

pub use cluster_hierarchy::{cluster_hd, get_cluster_res};
use node_utilities::*;
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClusterRes {
    pub row_cluster: Option<String>,
    pub col_cluster: Option<String>,
}

impl ClusterRes {
    pub fn new(row_clust: Option<String>, col_clust: Option<String>) -> Self {
        return ClusterRes {
            row_cluster: row_clust,
            col_cluster: col_clust,
        };
    }
    pub fn write_as_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;

        _ = serde_json::to_writer_pretty(file, &self);

        return Ok(());
    }
}
