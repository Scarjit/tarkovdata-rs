



use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type ItemsEn = HashMap<String, ItemsEnValue>;
pub fn from_json(path: &PathBuf) -> ItemsEn{
    let jstr = std::fs::read_to_string(path.join("items.en.json")).expect("Failed to read items.en.json");
    serde_json::from_str(&jstr).expect("Failed to parse items.en.json")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsEnValue {
    //TODO: Remove when https://github.com/TarkovTracker/tarkovdata/pull/198 is merged
    #[serde(alias = "id:")]
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "shortName")]
    pub(crate) short_name: String,
}
