use serde::{Deserialize, Serialize};
use crate::models::faction::Faction;
use crate::models::stats::StatKind;
use crate::models::moves::StatusType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemCategory {
    HealItem,
    BattleItem,
    SneakerCase,
    KeyItem,
    HeldItem,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemEffect {
    HealHp(u16),
    HealFull,
    Revive(u8),   // percent of max HP to restore
    ReviveFull,
    CureStatus(Option<StatusType>),
    CureAll,
    RestorePp(u8),
    RestoreAllPp,
    StatBoost(StatKind, i8),
    BoostAll,
    GuaranteedCrit,
    SurviveFatalHit,
    /// Catch multiplier stored as x100 (e.g. 150 = 1.5x)
    CatchMultiplier(u16),
    /// Faction-specific catch multiplier stored as x100
    CatchMultiplierFaction(Faction, u16),
    GuaranteedCatch,
    LevelUp,
    Repel(u16),
    EscapeDungeon,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub id: u16,
    pub name: &'static str,
    pub category: ItemCategory,
    pub cost: u32,
    pub effect: ItemEffect,
    pub description: &'static str,
}
