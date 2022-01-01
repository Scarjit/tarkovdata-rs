use serde::{Deserialize, Deserializer, Serialize};


use crate::tarkovdata::traders::Trader;
use crate::tarkovdata::{traders, REPO_DIR};
use cached::proc_macro::once;

#[once]
pub(crate) fn from_json() -> Hideout {
    let jstr = std::fs::read_to_string(REPO_DIR.join("hideout.json"))
        .expect("Failed to read hideout.json");
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
    String(String),
    #[serde(deserialize_with = "trader_from_number")]
    Trader(Trader),
}

fn trader_from_number<'de, D>(deserializer: D) -> Result<Trader, D::Error>
where
    D: Deserializer<'de>,
{
    let n: i64 = Deserialize::deserialize(deserializer)?;

    let traders = traders::from_json();
    Ok(traders
        .iter()
        .filter(|t| t.1.id == n)
        .next()
        .unwrap()
        .1
        .clone())
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
