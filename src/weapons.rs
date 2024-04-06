use serde::Deserialize;

use crate::download_json;

fn get_weapons(weapons_url: &str) -> Vec<Weapon> {
    let weapons = download_json::<Weapons>(weapons_url);
    weapons.weapons
}

#[derive(Debug, Deserialize)]
struct Weapons {
    #[serde(rename="ExportWeapons")]
    weapons: Vec<Weapon>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Weapon {
    name: String, // Will be in uppercase
    #[serde(rename="uniqueName")]
    path: String,
    codex_secret: bool,
    damage_per_shot: [f64; 20],
    total_damage: f64,
    description: String,
    critical_chance: f64,
    critical_multiplier: f64,
    #[serde(rename="procChance")]
    status_chance: f64,
    fire_rate: f64,
    mastery_req: i8,
    product_category: ProductCategory,
    exclude_from_codex: bool,
    slot: Slot,
    accuracy: f64,
    #[serde(rename="omegaAttenuation")]
    riven_disposition: f64,
    max_level_cap: Option<i32>, // Reserved for Kuva and Tenet weapons
    
}

/*
Gun Specific

noise: Noise,
trigger: Trigger,
magazine_size: i32,
reload_time: f64,
sentinel: bool, // Whether or not this is a robotic (sentinel or moa) weapon
multishot: i32,

Melee Specific

blocking_angle: i32 // Melee's blocking angle
combo_duration: i32 // Melee Combo duration
follow_through: f64 // Melee's follow through
range: f64 // Melee's range
slam_attack: f64 // Melee's direct slam attack damage
slam_radial_damage: f64 // Melee's radial slam attack damage
slam_radius: f64 // Melee's slam radius
slide_attack: f64 // Melee's slide attack damage
heavy_attack_damage: f64 // Melee's heavy attack damage
heavy_slam_attack: f64 // Melee's heavy direct slam attack damage
heavy_slam_radial_damage: f64 // Melee's heavy radial slam attack damage
heavy_slam_radius: f64 // Melee's heavy slam radius
wind_up: f64 // Melee's heavy attack wind-up time
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all="UPPERCASE")]
enum Noise {
    Silent,
    Alarming
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="UPPERCASE")]
enum Trigger {
    Semi,
    Auto,
    Burst,
    Held,
    Charge
}

#[derive(Debug, Deserialize)]
enum ProductCategory {
    Pistols,
    LongGuns,
    Melee,
    SpaceGuns,
    SpaceMelee,
    SpecialItems,
    CrewShipWeapons
}

#[derive(Debug, Deserialize)]
enum Slot {
    Secondary = 0,
    PrimaryArchgun = 1,
    Melee = 5,
    Exalted = 7,
    Railjack = 13
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
    True = 19
}