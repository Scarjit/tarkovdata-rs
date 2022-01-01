use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type Traders = HashMap<String, Trader>;
use cached::proc_macro::once;
#[once]
pub(crate) fn from_json(path: &PathBuf) -> Traders {
    let jstr =
        std::fs::read_to_string(path.join("traders.json")).expect("Failed to read traders.json");
    serde_json::from_str(&jstr).expect("Failed to parse traders.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trader {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) locale: Locale,
    pub(crate) wiki: String,
    pub(crate) description: String,
    pub(crate) currencies: Vec<String>,
    #[serde(rename = "salesCurrency")]
    pub(crate) sales_currency: String,
    pub(crate) loyalty: Vec<Loyalty>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Locale {
    pub(crate) en: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loyalty {
    pub(crate) level: i64,
    #[serde(rename = "requiredLevel")]
    pub(crate) required_level: i64,
    #[serde(rename = "requiredReputation")]
    pub(crate) required_reputation: f64,
    #[serde(rename = "requiredSales")]
    pub(crate) required_sales: i64,
}
