use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};


pub type Traders = HashMap<String, Trader>;
pub fn from_json(path: &PathBuf) -> Traders{
    let jstr = std::fs::read_to_string(path.join("traders.json")).expect("Failed to read traders.json");
    serde_json::from_str(&jstr).expect("Failed to parse traders.json")
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale {
    pub(crate) en: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loyalty {
    pub(crate) level: i64,
    #[serde(rename = "requiredLevel")]
    pub(crate) required_level: i64,
    #[serde(rename = "requiredReputation")]
    pub(crate) required_reputation: f64,
    #[serde(rename = "requiredSales")]
    pub(crate) required_sales: i64,
}
