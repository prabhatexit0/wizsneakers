use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::state::player::PlayerState;

pub const SAVE_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Overworld,
    Battle,
    Dialogue,
    Menu,
    Cutscene,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub mode: GameMode,
    pub player: PlayerState,
    pub current_map: u16,
    pub event_flags: HashSet<String>,
    pub story_progress: u8,
    pub play_time_ms: u64,
    pub authentication_stamps: [bool; 8],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Overworld,
            player: PlayerState::new(),
            current_map: 0,
            event_flags: HashSet::new(),
            story_progress: 0,
            play_time_ms: 0,
            authentication_stamps: [false; 8],
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
