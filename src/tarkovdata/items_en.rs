use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type ItemsEn = HashMap<String, ItemsEnValue>;
use crate::tarkovdata::REPO_DIR;
use cached::proc_macro::once;

#[once]
pub(crate) fn from_json() -> ItemsEn {
    let jstr = std::fs::read_to_string(REPO_DIR.join("items.en.json"))
        .expect("Failed to read items.en.json");
    serde_json::from_str(&jstr).expect("Failed to parse items.en.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsEnValue {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "shortName")]
    pub(crate) short_name: String,
}
