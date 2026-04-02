use serde::{Deserialize, Serialize};
use crate::models::faction::Faction;
use crate::models::stats::StatKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveTarget {
    Self_,
    Opponent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusType {
    Creased,
    Scuffed,
    SoldOut,
    Hypnotized,
    Deflated,
    OnFire,
}

/// Move effects. Static game data — not deserialized from save files.
/// MultiStatChange uses a static slice reference for zero-cost game data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveEffect {
    None,
    StatChange { target: MoveTarget, stat: StatKind, stages: i8 },
    MultiStatChange { target: MoveTarget, changes: &'static [(StatKind, i8)] },
    StatusInflict { status: StatusType, chance: u8 },
    Recoil { percent: u8 },
    DrainHp { percent: u8 },
    HealPercent { percent: u8 },
    MultiHit { times: u8 },
    HighCrit,
    AlwaysCritOnSuperEffective,
    FlinchChance { percent: u8 },
    SkipNextTurn,
    SwapStatChanges,
    RemoveBuffs,
    PowerEqualsLevel,
    PercentCurrentHp { percent: u8 },
    HealPercentDamage { percent: u8 },
    SelfStatusOnMiss { status: StatusType },
    PriorityPlus,
    RemoveStatusDealDamage,
}

/// Static move data — not serialized/deserialized (IDs only travel in save files).
#[derive(Clone, Debug)]
pub struct MoveData {
    pub id: u16,
    pub name: &'static str,
    pub faction: Faction,
    pub category: MoveCategory,
    pub power: Option<u8>,
    pub accuracy: u8,
    pub pp: u8,
    pub priority: i8,
    pub effect: MoveEffect,
    pub description: &'static str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveSlot {
    pub move_id: u16,
    pub current_pp: u8,
    pub max_pp: u8,
}
