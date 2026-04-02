use serde::{Deserialize, Serialize};
use crate::models::stats::{StatStages, StatKind};
use crate::models::sneaker::SneakerInstance;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleKind {
    Wild,
    Trainer { id: u16, name: String },
    Boss { id: u16, name: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AiLevel {
    Random,
    Basic,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleOpponent {
    pub team: Vec<SneakerInstance>,
    pub items: Vec<(u16, u16)>,
    pub ai_level: AiLevel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattlePrompt {
    MoveLearn { move_id: u16 },
    Evolution { species_id: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleAction {
    Fight { move_index: u8 },
    Bag { item_id: u16 },
    Switch { party_index: u8 },
    Run,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleSide {
    Player,
    Opponent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effectiveness {
    SuperEffective,
    Normal,
    NotVeryEffective,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleResult {
    PlayerWin,
    PlayerLose,
    PlayerFlee,
    PlayerCapture,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleState {
    pub kind: BattleKind,
    pub player_active: usize,
    pub opponent: BattleOpponent,
    pub opponent_active: usize,
    pub turn_number: u16,
    pub player_stages: StatStages,
    pub opponent_stages: StatStages,
    pub turn_log: Vec<BattleTurnEvent>,
    pub flee_attempts: u8,
    pub can_flee: bool,
    pub waiting_for: Option<BattlePrompt>,
    /// Player's active sneaker must skip their next action (after using SkipNextTurn move).
    #[serde(default)]
    pub player_skip_turn: bool,
    /// Opponent's active sneaker must skip their next action.
    #[serde(default)]
    pub opponent_skip_turn: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BattleTurnEvent {
    MoveUsed { side: BattleSide, move_id: u16 },
    Damage { side: BattleSide, amount: u16, effectiveness: Effectiveness, is_critical: bool },
    StatChange { side: BattleSide, stat: StatKind, stages: i8 },
    StatusApplied { side: BattleSide, status: String },
    StatusDamage { side: BattleSide, amount: u16 },
    Healed { side: BattleSide, amount: u16 },
    Fainted { side: BattleSide },
    SwitchedIn { side: BattleSide, species_id: u16 },
    ItemUsed { item_id: u16 },
    FleeAttempt { success: bool },
    CaptureAttempt { shakes: u8, success: bool },
    XpGained { amount: u32 },
    LevelUp { new_level: u8 },
    MoveLearnPrompt { move_id: u16 },
    EvolutionPrompt { species_id: u16 },
    BattleEnd { result: BattleResult },
    Message { text: String },
}
