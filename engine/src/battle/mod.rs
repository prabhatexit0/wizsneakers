pub mod types;
pub mod damage;
pub mod status;
pub mod engine;

pub use types::{
    AiLevel, BattleAction, BattleKind, BattleOpponent, BattlePrompt, BattleResult,
    BattleSide, BattleState, BattleTurnEvent, Effectiveness,
};
pub use engine::BattleEngine;
pub use damage::{calculate_damage, calculate_damage_ex, calculate_damage_with_override, DamageResult};
