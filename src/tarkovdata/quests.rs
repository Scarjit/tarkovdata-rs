use serde::{Deserialize, Serialize};


pub type Quests = Vec<QuestClass>;

use crate::tarkovdata::REPO_DIR;
use cached::proc_macro::once;

#[once]
pub(crate) fn from_json() -> Quests {
    let jstr =
        std::fs::read_to_string(REPO_DIR.join("quests.json")).expect("Failed to read quests.json");
    serde_json::from_str(&jstr).expect("Failed to parse quests.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestClass {
    pub(crate) id: i64,
    pub(crate) require: Require,
    pub(crate) giver: i64,
    pub(crate) turnin: i64,
    pub(crate) title: String,
    pub(crate) locales: Locales,
    pub(crate) wiki: String,
    pub(crate) exp: i64,
    pub(crate) unlocks: Vec<Unlock>,
    pub(crate) reputation: Vec<Reputation>,
    pub(crate) objectives: Vec<Objective>,
    #[serde(rename = "gameId")]
    pub(crate) game_id: String,
    #[serde(rename = "reputationFailure")]
    pub(crate) reputation_failure: Option<Vec<Reputation>>,
    pub(crate) alternatives: Option<Vec<i64>>,
    pub(crate) nokappa: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Locales {
    pub(crate) en: String,
    pub(crate) ru: Option<String>,
    pub(crate) cs: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objective {
    #[serde(rename = "type")]
    pub(crate) objective_type: ObjectiveType,
    pub(crate) target: Target,
    pub(crate) number: i64,
    pub(crate) location: Option<i64>,
    pub(crate) id: i64,
    pub(crate) gps: Option<Gps>,
    pub(crate) tool: Option<Tool>,
    pub(crate) hint: Option<String>,
    pub(crate) have: Option<i64>,
    pub(crate) with: Option<Vec<WithElement>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gps {
    #[serde(rename = "leftPercent")]
    pub(crate) left_percent: Option<f64>,
    #[serde(rename = "topPercent")]
    pub(crate) top_percent: Option<f64>,
    pub(crate) floor: Floor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithClass {
    #[serde(rename = "type")]
    pub(crate) with_type: WithType,
    pub(crate) name: Option<Name>,
    pub(crate) value: Option<Value>,
    pub(crate) id: Option<IdUnion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdElement {
    pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reputation {
    pub(crate) trader: i64,
    pub(crate) rep: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Require {
    pub(crate) level: Option<i64>,
    pub(crate) quests: Vec<QuestUnion>,
    pub(crate) loyalty: Option<Vec<Loyalty>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loyalty {
    pub(crate) trader: i64,
    pub(crate) stage: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Target {
    Integer(i64),
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WithElement {
    String(String),
    WithClass(WithClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IdUnion {
    IdElementArray(Vec<IdElement>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum QuestUnion {
    Integer(i64),
    IntegerArray(Vec<i64>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Unlock {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Floor {
    Basement,
    Bunkers,
    #[serde(rename = "First_Floor")]
    FirstFloor,
    #[serde(rename = "Ground_Floor")]
    GroundFloor,
    #[serde(rename = "Ground_Level")]
    GroundLevel,
    #[serde(rename = "Second_Floor")]
    SecondFloor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ObjectiveType {
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "collect")]
    Collect,
    #[serde(rename = "find")]
    Find,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "kill")]
    Kill,
    #[serde(rename = "locate")]
    Locate,
    #[serde(rename = "mark")]
    Mark,
    #[serde(rename = "pickup")]
    Pickup,
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "reputation")]
    Reputation,
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "warning")]
    Warning,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tool {
    #[serde(rename = "5991b51486f77447b112d44f")]
    The5991B51486F77447B112D44F,
    #[serde(rename = "5ac78a9b86f7741cca0bbd8d")]
    The5Ac78A9B86F7741Cca0Bbd8D,
    #[serde(rename = "5b4391a586f7745321235ab2")]
    The5B4391A586F7745321235Ab2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Name {
    #[serde(rename = "durability")]
    Durability,
    #[serde(rename = "ergonomics")]
    Ergonomics,
    #[serde(rename = "Extended magazine")]
    ExtendedMagazine,
    Foregrip,
    #[serde(rename = "recoil")]
    Recoil,
    #[serde(rename = "sighting range")]
    SightingRange,
    Suppressor,
    #[serde(rename = "Tactical device")]
    TacticalDevice,
    #[serde(rename = "weight")]
    Weight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WithType {
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "cells")]
    Cells,
    #[serde(rename = "part")]
    Part,
    #[serde(rename = "stat")]
    Stat,
}
