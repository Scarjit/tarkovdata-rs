use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type Ammuniton = HashMap<String, AmmunitonValue>;

pub fn from_json(path: &PathBuf) -> Ammuniton{
    let jstr = std::fs::read_to_string(path.join("ammunition.json")).expect("Failed to read ammunition.json");
    serde_json::from_str(&jstr).expect("Failed to parse ammunition.json")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmunitonValue {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "shortName")]
    pub(crate) short_name: String,
    pub(crate) weight: Option<f64>,
    pub(crate) caliber: String,
    #[serde(rename = "stackMaxSize")]
    pub(crate) stack_max_size: Option<i64>,
    pub(crate) tracer: bool,
    #[serde(rename = "tracerColor")]
    pub(crate) tracer_color: TracerColor,
    #[serde(rename = "ammoType")]
    pub(crate) ammo_type: AmmoType,
    #[serde(rename = "projectileCount")]
    pub(crate) projectile_count: i64,
    pub(crate) ballistics: Ballistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ballistics {
    pub(crate) damage: i64,
    #[serde(rename = "armorDamage")]
    pub(crate) armor_damage: i64,
    #[serde(rename = "fragmentationChance")]
    pub(crate) fragmentation_chance: f64,
    #[serde(rename = "ricochetChance")]
    pub(crate) ricochet_chance: f64,
    #[serde(rename = "penetrationChance")]
    pub(crate) penetration_chance: f64,
    #[serde(rename = "penetrationPower")]
    pub(crate) penetration_power: i64,
    pub(crate) accuracy: i64,
    pub(crate) recoil: i64,
    #[serde(rename = "initialSpeed")]
    pub(crate) initial_speed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AmmoType {
    #[serde(rename = "buckshot")]
    Buckshot,
    #[serde(rename = "bullet")]
    Bullet,
    #[serde(rename = "grenade")]
    Grenade,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TracerColor {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "tracerGreen")]
    TracerGreen,
    #[serde(rename = "tracerRed")]
    TracerRed,
}
