use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type Levels = HashMap<String, LevelsValue>;

use cached::proc_macro::once;
#[once]
pub(crate) fn from_json(path: &PathBuf) -> Levels {
    let jstr =
        std::fs::read_to_string(path.join("levels.json")).expect("Failed to read levels.json");
    serde_json::from_str(&jstr).expect("Failed to parse levels.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LevelsValue {
    pub(crate) exp: i64,
    pub(crate) group: String,
}
