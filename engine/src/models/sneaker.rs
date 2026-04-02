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

/// Result returned by add_xp().
pub struct XpResult {
    pub leveled_up: bool,
    pub new_level: u8,
    pub new_moves: Vec<u16>,
    pub can_evolve: Option<u16>,
}

/// XP required to reach a given level.
/// Formula: (6 * level^3 / 5) - 15 * level^2 + 100 * level - 140
/// Returns 0 for level <= 1.
pub fn xp_needed(level: u8) -> u32 {
    if level <= 1 {
        return 0;
    }
    let l = level as i64;
    let v = (6 * l * l * l / 5) - 15 * l * l + 100 * l - 140;
    v.max(0) as u32
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

    /// Add XP, handle level-ups, return result describing what happened.
    pub fn add_xp(&mut self, amount: u32, species: &SneakerSpecies) -> XpResult {
        self.xp = self.xp.saturating_add(amount);
        let old_level = self.level;
        let old_max_hp = self.calc_max_hp(species);

        // Level up while XP >= threshold and level < 100
        while self.level < 100 && self.xp >= xp_needed(self.level + 1) {
            self.level += 1;
        }

        let leveled_up = self.level > old_level;

        if leveled_up {
            let new_max_hp = self.calc_max_hp(species);
            let hp_increase = new_max_hp.saturating_sub(old_max_hp);
            self.max_hp = new_max_hp;
            self.current_hp = self.current_hp.saturating_add(hp_increase).min(new_max_hp);
        }

        // Collect newly learnable moves at the new level range
        let new_moves: Vec<u16> = species
            .learnset
            .iter()
            .filter(|(lv, _)| *lv > old_level && *lv <= self.level)
            .map(|(_, move_id)| *move_id)
            .collect();

        let can_evolve = self.check_evolution(species);

        XpResult {
            leveled_up,
            new_level: self.level,
            new_moves,
            can_evolve,
        }
    }

    /// Check if this sneaker can evolve based on species data.
    pub fn check_evolution(&self, species: &SneakerSpecies) -> Option<u16> {
        if let Some((evo_level, target_id)) = species.evolution {
            if self.level >= evo_level {
                return Some(target_id);
            }
        }
        None
    }

    /// Perform evolution: update species_id and recalculate stats.
    pub fn evolve(&mut self, new_species_id: u16, new_species: &SneakerSpecies) {
        let old_max_hp = self.max_hp;
        self.species_id = new_species_id;
        let new_max_hp = self.calc_max_hp(new_species);
        let hp_increase = new_max_hp.saturating_sub(old_max_hp);
        self.max_hp = new_max_hp;
        self.current_hp = self.current_hp.saturating_add(hp_increase).min(new_max_hp);
    }
}
