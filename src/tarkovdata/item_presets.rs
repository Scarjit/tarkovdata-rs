use serde::{Deserialize, Serialize};
use std::collections::HashMap;


pub type ItemPresets = HashMap<String, ItemPresetsValue>;

use crate::tarkovdata::REPO_DIR;
use cached::proc_macro::once;

#[once]
pub(crate) fn from_json() -> ItemPresets {
    let jstr = std::fs::read_to_string(REPO_DIR.join("item_presets.json"))
        .expect("Failed to read item_presets.json");
    serde_json::from_str(&jstr).expect("Failed to parse item_presets.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemPresetsValue {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "appendName")]
    pub(crate) append_name: String,
    #[serde(rename = "default")]
    pub(crate) item_presets_default: bool,
    #[serde(rename = "baseId")]
    pub(crate) base_id: String,
    pub(crate) parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub(crate) id: String,
    pub(crate) quantity: i64,
}
