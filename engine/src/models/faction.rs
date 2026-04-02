use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Faction {
    Normal,
    Retro,
    Techwear,
    Skate,
    HighFashion,
}

impl Faction {
    pub fn effectiveness_against(&self, defender: Faction) -> f64 {
        match (self, defender) {
            (Faction::Retro, Faction::Skate) => 2.0,
            (Faction::Retro, Faction::Techwear) => 0.5,
            (Faction::Techwear, Faction::Retro) => 2.0,
            (Faction::Techwear, Faction::Skate) => 0.5,
            (Faction::Skate, Faction::Techwear) => 2.0,
            (Faction::Skate, Faction::Retro) => 0.5,
            (Faction::HighFashion, Faction::HighFashion) => 0.5,
            _ => 1.0,
        }
    }
}
