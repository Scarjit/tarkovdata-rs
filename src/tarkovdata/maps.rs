use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub type Maps = HashMap<String, Map>;
pub fn from_json(path: &PathBuf) -> Maps{
    let jstr = std::fs::read_to_string(path.join("maps.json")).expect("Failed to read maps.json");
    serde_json::from_str(&jstr).expect("Failed to parse maps.json")
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub(crate) id: i64,
    pub(crate) locale: Locale,
    pub(crate) wiki: String,
    pub(crate) description: String,
    pub(crate) enemies: Vec<String>,
    #[serde(rename = "raidDuration")]
    pub(crate) raid_duration: RaidDuration,
    pub(crate) svg: Option<Svg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale {
    pub(crate) en: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaidDuration {
    pub(crate) day: i64,
    pub(crate) night: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Svg {
    pub(crate) file: String,
    pub(crate) floors: Vec<String>,
    #[serde(rename = "defaultFloor")]
    pub(crate) default_floor: String,
}
