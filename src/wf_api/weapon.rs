use super::content_server;
use super::download_json;
use crate::damage::DamageArray;
use crate::wf_api::manifest::Category;
use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub fn get_weapons(path: &str) -> Vec<Weapon> {
    let mut value = download_json::<serde_json::Value>(&content_server(path));

    let weapons = value[Category::Weapons.as_str()].take();
    let weapons: Vec<Weapon> =
        serde_json::from_value(weapons).expect("Could not deserialize weapons");

    weapons
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Weapon {
    pub name: String, // Will be in uppercase
    #[serde(rename = "uniqueName")]
    pub path: String,
    pub codex_secret: bool,
    pub damage_per_shot: DamageArray,
    pub total_damage: f64,
    pub description: String,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    #[serde(rename = "procChance")]
    pub status_chance: f64,
    pub fire_rate: f64,
    pub mastery_req: i8,
    pub product_category: ProductCategory,
    pub exclude_from_codex: Option<bool>,
    pub slot: Option<Slot>,
    #[serde(rename = "omegaAttenuation")]
    pub riven_disposition: f64,
    #[serde(rename = "primeOmegaAttenuation")]
    pub prime_riven_disposition: Option<f64>,
    pub max_level_cap: Option<i32>, // Reserved for Kuva and Tenet weapons

    // Gun Specific
    pub accuracy: Option<f64>,
    pub noise: Option<Noise>,
    pub trigger: Option<Trigger>,
    pub magazine_size: Option<i32>,
    pub reload_time: Option<f64>,
    pub sentinel: Option<bool>, // Whether or not this is a robotic (sentinel or moa) weapon
    pub multishot: Option<i32>,

    // Melee Specific
    pub blocking_angle: Option<i32>, // blocking angle in degrees
    pub combo_duration: Option<i32>, // Combo duration in seconds
    pub follow_through: Option<f64>,
    pub range: Option<f64>,
    pub slam_attack: Option<f64>, // direct slam attack damage
    pub slam_radial_damage: Option<f64>,
    pub slam_radius: Option<f64>,
    pub slide_attack: Option<f64>,
    pub heavy_attack_damage: Option<f64>,
    pub heavy_slam_attack: Option<f64>, // heavy direct slam attack damage
    pub heavy_slam_radial_damage: Option<f64>,
    pub heavy_slam_radius: Option<f64>,
    pub wind_up: Option<f64>, // heavy attack wind-up time
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Noise {
    Silent,
    Alarming,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Trigger {
    Semi,
    Auto,
    Burst,
    Held,
    Charge,
    Active,
    Duplex,
    #[serde(rename = "Auto Burst")]
    AutoBurst,
}

#[derive(Debug, Deserialize)]
pub enum ProductCategory {
    Pistols,
    LongGuns,
    Melee,
    SpaceGuns,
    SpaceMelee,
    SpecialItems,
    CrewShipWeapons,
    OperatorAmps,
    SentinelWeapons,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Slot {
    Secondary = 0,
    PrimaryArchgun = 1,
    Melee = 5,
    Exalted = 7,
    Railjack = 13,
}
