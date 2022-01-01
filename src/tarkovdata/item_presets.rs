use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;


pub type ItemPresets = HashMap<String, ItemPresetsValue>;

use crate::tarkovdata::{items_en, REPO_DIR};
use cached::proc_macro::once;
use crate::tarkovdata::items_en::ItemsEnValue;

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
    #[serde(deserialize_with = "items_from_number")]
    pub(crate) base_id: ItemsEnValue,
    pub(crate) parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(deserialize_with = "items_from_number")]
    pub(crate) id: ItemsEnValue,
    pub(crate) quantity: i64,
}


fn items_from_number<'de, D>(deserializer: D) -> Result<ItemsEnValue, D::Error>
    where
        D: Deserializer<'de>,
{
    let n: &str = Deserialize::deserialize(deserializer)?;

    let items = items_en::from_json();
    Ok(items
        .iter()
        .filter(|t| t.1.id == n)
        .next()
        .unwrap()
        .1
        .clone())
}