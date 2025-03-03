use std::sync::Mutex;

use super::HistDiffState;

impl HistDiffState {
    pub fn new() -> Self {
        return Self {
            hd_res: Mutex::new(None),
        };
    }
}
