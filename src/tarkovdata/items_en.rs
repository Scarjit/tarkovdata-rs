



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
    pub(crate) id: Option<String>,
    pub(crate) name: String,
    #[serde(rename = "shortName")]
    pub(crate) short_name: String,
    #[serde(rename = "id:")]
    pub(crate) items_en_id: Option<String>,
}
