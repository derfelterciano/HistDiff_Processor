use cp_hierarchical_clustering::DendrogramNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Node {
    pub cid: usize,
    pub name: String,
    pub dist: f64,
    pub children: Vec<D3Node>,
}
