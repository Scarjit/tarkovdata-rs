use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use cached::proc_macro::once;
#[once]
pub(crate) fn from_json(path: &PathBuf) -> Hideout {
    let jstr =
        std::fs::read_to_string(path.join("hideout.json")).expect("Failed to read hideout.json");
    serde_json::from_str(&jstr).expect("Failed to parse hideout.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hideout {
    pub(crate) stations: Vec<Station>,
    pub(crate) modules: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub(crate) module: String,
    pub(crate) level: i64,
    pub(crate) require: Vec<Require>,
    pub(crate) id: i64,
    #[serde(rename = "stationId")]
    pub(crate) station_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Require {
    #[serde(rename = "type")]
    pub(crate) require_type: Type,
    pub(crate) name: Name,
    pub(crate) quantity: i64,
    pub(crate) id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub(crate) id: i64,
    pub(crate) locales: Locales,
    pub(crate) function: String,
    #[serde(rename = "imgSource")]
    pub(crate) img_source: String,
    pub(crate) disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Locales {
    pub(crate) en: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Name {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "trader")]
    Trader,
}
