use num_derive::FromPrimitive;
use serde::Deserialize;

mod impls;

/// Damage structured as an array of possible damage types.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub struct DamageArray([f64; 20]);

/// Ordinal corresponds to weapon damage array index.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum DamageType {
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
