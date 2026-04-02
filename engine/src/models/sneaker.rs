use serde::{Deserialize, Serialize};
use crate::models::faction::Faction;
use crate::models::stats::{Stats, StatKind, Condition};
use crate::models::moves::{MoveSlot, StatusType};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RarityTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

pub struct SneakerSpecies {
    pub id: u16,
    pub name: &'static str,
    pub faction: Faction,
    pub base_stats: Stats,
    pub rarity_tier: RarityTier,
    pub base_catch_rate: u8,
    pub base_xp_yield: u16,
    pub ev_yield: Stats,
    pub learnset: &'static [(u8, u16)],   // (level, move_id)
    pub evolution: Option<(u8, u16)>,      // (level, target_species_id)
    pub description: &'static str,
}

/// Major status conditions — only one can be active at a time.
/// OnFire is volatile and tracked separately via `on_fire_turns` on SneakerInstance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusCondition {
    Creased,
    Scuffed { turns_left: u8 },
    SoldOut { turns_left: u8 },
    Hypnotized { turns_left: u8 },
    Deflated,
}

impl StatusCondition {
    pub fn status_type(&self) -> StatusType {
        match self {
            StatusCondition::Creased => StatusType::Creased,
            StatusCondition::Scuffed { .. } => StatusType::Scuffed,
            StatusCondition::SoldOut { .. } => StatusType::SoldOut,
            StatusCondition::Hypnotized { .. } => StatusType::Hypnotized,
            StatusCondition::Deflated => StatusType::Deflated,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SneakerInstance {
    pub uid: u64,
    pub species_id: u16,
    pub nickname: Option<String>,
    pub level: u8,
    pub xp: u32,
    pub current_hp: u16,
    pub max_hp: u16,
    pub ivs: Stats,
    pub evs: Stats,
    pub condition: Condition,
    pub moves: [Option<MoveSlot>; 4],
    pub status: Option<StatusCondition>,
    /// Volatile OnFire status — can coexist with a major status.
    /// Counts down each turn; 0 = not on fire.
    #[serde(default)]
    pub on_fire_turns: u8,
    pub held_item: Option<u16>,
    pub friendship: u8,
    pub caught_location: u16,
    pub original_trainer: String,
}

impl SneakerInstance {
    /// Calculate a stat value for this instance.
    /// HP formula: (2*base + iv + ev/4) * level/100 + level + 10
    /// Other: ((2*base + iv + ev/4) * level/100 + 5) * condition_mod
    pub fn calc_stat(&self, species: &SneakerSpecies, stat: StatKind) -> u16 {
        let base = species.base_stats.get(stat) as u32;
        let iv = self.ivs.get(stat) as u32;
        let ev = self.evs.get(stat) as u32;
        let level = self.level as u32;

        let inner = (2 * base + iv + ev / 4) * level / 100;

        if stat == StatKind::Durability {
            (inner + level + 10) as u16
        } else {
            let raw = (inner + 5) as f64 * self.condition.modifier(stat);
            raw as u16
        }
    }

    pub fn calc_max_hp(&self, species: &SneakerSpecies) -> u16 {
        self.calc_stat(species, StatKind::Durability)
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn display_name<'a>(&'a self, species: &'a SneakerSpecies) -> &'a str {
        self.nickname.as_deref().unwrap_or(species.name)
    }
}
