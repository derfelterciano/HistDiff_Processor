use std::collections::HashMap;

use cp_hierarchical_clustering::{ClusterHierarchy, DendrogramNode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Node {
    pub cid: usize,
    pub name: Option<String>,
    pub dist: f64,
    pub children: Vec<D3Node>,
}

impl D3Node {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Can't convert D3Node into string")
    }
}

pub(in crate::analysis) fn convert_to_d3(
    cluster: &ClusterHierarchy,
    name_map: &HashMap<usize, String>,
) -> D3Node {
    let node: &DendrogramNode = &cluster.get_raw_nodes().unwrap();

    return build_d3_tree(node, name_map);
}

fn build_d3_tree(root: &DendrogramNode, name_map: &HashMap<usize, String>) -> D3Node {
    // child array
    let mut kids: Vec<D3Node> = Vec::new();
    if let Some(ref left_child) = root.left {
        kids.push(build_d3_tree(left_child, name_map));
    }

    if let Some(ref right_child) = root.right {
        kids.push(build_d3_tree(right_child, name_map));
    }

    return D3Node {
        cid: root.cid.clone(),
        dist: root.distance.clone(),
        name: name_map.get(&root.cid).cloned(),
        children: kids,
    };
}
