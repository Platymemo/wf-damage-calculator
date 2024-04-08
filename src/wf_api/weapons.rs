use std::ops::Index;

use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::wf_api::manifest::Category;

use super::content_server;
use super::download_json;

pub fn get_weapons(path: &str) -> Vec<Weapon> {
    let mut value = download_json::<serde_json::Value>(&content_server(path));

    let weapons = value[Category::Weapons.as_str()].take();
    let weapons: Vec<Weapon> =
        serde_json::from_value(weapons).expect("Could not deserialize weapons");

    weapons
}

#[derive(Debug, Deserialize)]
pub struct DamageArray([f64; 20]);

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Weapon {
    name: String, // Will be in uppercase
    #[serde(rename = "uniqueName")]
    path: String,
    codex_secret: bool,
    damage_per_shot: DamageArray,
    total_damage: f64,
    description: String,
    critical_chance: f64,
    critical_multiplier: f64,
    #[serde(rename = "procChance")]
    status_chance: f64,
    fire_rate: f64,
    mastery_req: i8,
    product_category: ProductCategory,
    exclude_from_codex: Option<bool>,
    slot: Option<Slot>,
    accuracy: Option<f64>,
    #[serde(rename = "omegaAttenuation")]
    riven_disposition: f64,
    #[serde(rename = "primeOmegaAttenuation")]
    prime_riven_disposition: Option<f64>,
    max_level_cap: Option<i32>, // Reserved for Kuva and Tenet weapons

    // Gun Specific
    noise: Option<Noise>,
    trigger: Option<Trigger>,
    magazine_size: Option<i32>,
    reload_time: Option<f64>,
    sentinel: Option<bool>, // Whether or not this is a robotic (sentinel or moa) weapon
    multishot: Option<i32>,

    // Melee Specific
    blocking_angle: Option<i32>, // blocking angle in degrees
    combo_duration: Option<i32>, // Combo duration in seconds
    follow_through: Option<f64>,
    range: Option<f64>,
    slam_attack: Option<f64>, // direct slam attack damage
    slam_radial_damage: Option<f64>,
    slam_radius: Option<f64>,
    slide_attack: Option<f64>,
    heavy_attack_damage: Option<f64>,
    heavy_slam_attack: Option<f64>, // heavy direct slam attack damage
    heavy_slam_radial_damage: Option<f64>,
    heavy_slam_radius: Option<f64>,
    wind_up: Option<f64>, // heavy attack wind-up time
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Noise {
    Silent,
    Alarming,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Trigger {
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
enum ProductCategory {
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Slot {
    Secondary = 0,
    PrimaryArchgun = 1,
    Melee = 5,
    Exalted = 7,
    Railjack = 13,
}

// Ordinal corresponds to weapon damage array index
#[derive(Debug)]
enum DamageType {
    Impact = 0,
    Puncture = 1,
    Slash = 2,
    Heat = 3,
    Cold = 4,
    Electricity = 5,
    Toxin = 6,
    Blast = 7,
    Radiation = 8,
    Gas = 9,
    Magnetic = 10,
    Viral = 11,
    Corrosive = 12,
    Void = 13,
    Tau = 14,
    Cinematic = 15,
    ShieldDrain = 16,
    HealthDrain = 17,
    EnergyDrain = 18,
    True = 19,
}

impl Index<DamageType> for DamageArray {
    type Output = f64;

    fn index(&self, index: DamageType) -> &Self::Output {
        &self.0[index as usize]
    }
}
