use wasm_bindgen::prelude::*;

pub mod models;
pub mod util;
pub mod data;
pub mod state;
pub mod world;
pub mod battle;

// Re-export commonly used types
pub use models::{Faction, Stats};
pub use state::{GameState, GameMode};
pub use util::rng::SeededRng;

use state::player::Direction;
use world::map::MapData;
use world::movement::{parse_input, process_movement, GameEvent};
use world::encounters::generate_wild_sneaker;
use battle::{BattleEngine, BattleState, BattleAction, BattleTurnEvent, BattleResult};
use battle::types::{BattlePrompt};

// ── Tile types ──

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;

// ── Game Engine — single source of truth ──

#[wasm_bindgen]
pub struct GameEngine {
    pub(crate) state: GameState,
    pub(crate) rng: SeededRng,
    map: Vec<u8>, // current collision data (updated by load_map_data)
    map_width: usize,
    map_height: usize,
    current_map: Option<MapData>, // loaded map metadata (encounters, connections, etc.)
    step_count: u32,
    encounter_triggered: bool,
    pub(crate) battle: Option<BattleState>,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> Self {
        let mut map = vec![0u8; MAP_WIDTH * MAP_HEIGHT];

        // Border walls
        for x in 0..MAP_WIDTH {
            map[x] = 1;
            map[(MAP_HEIGHT - 1) * MAP_WIDTH + x] = 1;
        }
        for y in 0..MAP_HEIGHT {
            map[y * MAP_WIDTH] = 1;
            map[y * MAP_WIDTH + MAP_WIDTH - 1] = 1;
        }

        // Interior walls
        for y in 3..7 {
            map[y * MAP_WIDTH + 5] = 1;
        }
        for x in 10..15 {
            map[4 * MAP_WIDTH + x] = 1;
        }

        // Tall grass patches
        for y in 2..5 {
            for x in 8..10 {
                map[y * MAP_WIDTH + x] = 2;
            }
        }
        for y in 8..12 {
            for x in 12..17 {
                map[y * MAP_WIDTH + x] = 2;
            }
        }

        Self {
            state: GameState::new(),
            rng: SeededRng::new(seed),
            map,
            map_width: MAP_WIDTH,
            map_height: MAP_HEIGHT,
            current_map: None,
            step_count: 0,
            encounter_triggered: false,
            battle: None,
        }
    }

    /// Load map data from a JSON string. Updates collision, dimensions, and encounter table.
    pub fn load_map_data(&mut self, json: &str) -> Result<(), JsValue> {
        self.load_map_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    /// Process one game tick.
    ///
    /// `dt_ms` — delta time in milliseconds since last frame.
    /// `input` — string input: "up"/"down"/"left"/"right"/"action"/"cancel"/"menu"/"none"
    ///           Prefix with "sprint_" to activate sprint mode (e.g. "sprint_right").
    ///
    /// Returns a JSON string with the full player + map state.
    pub fn tick(&mut self, dt_ms: f64, input: &str) -> String {
        self.encounter_triggered = false;

        let facing_str = match self.state.player.facing {
            Direction::Up    => "up",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Right => "right",
        };

        // In battle mode, skip overworld movement processing
        if self.state.mode == GameMode::Battle {
            return serde_json::json!({
                "player_x": self.state.player.x,
                "player_y": self.state.player.y,
                "facing": facing_str,
                "moving": false,
                "move_progress": 0.0,
                "map_width": self.map_width,
                "map_height": self.map_height,
                "encounter": false,
                "mode": "Battle",
            })
            .to_string();
        }

        // Parse sprint modifier and actual action
        let (sprint, action_str) = if let Some(rest) = input.strip_prefix("sprint_") {
            (true, rest)
        } else {
            (false, input)
        };

        let action = parse_input(action_str);

        // Get encounter table (may be empty if no map loaded)
        let wild_encounters: Vec<world::map::WildEncounterEntry> = self.current_map
            .as_ref()
            .map(|m| m.wild_encounters.clone())
            .unwrap_or_default();

        let events = process_movement(
            &mut self.state.player,
            action,
            dt_ms,
            sprint,
            &self.map,
            self.map_width,
            self.map_height,
            &wild_encounters,
            &mut self.rng,
        );

        for ev in &events {
            match ev {
                GameEvent::WildEncounter { species_id, level } => {
                    self.encounter_triggered = true;
                    // Enter battle mode if player has a party
                    if !self.state.player.party.is_empty() {
                        let wild = generate_wild_sneaker(*species_id, *level, &mut self.rng);
                        let battle_state = BattleEngine::new_wild(wild);
                        self.battle = Some(battle_state);
                        self.state.mode = GameMode::Battle;
                    }
                    self.step_count += 1;
                }
                GameEvent::None => {}
                _ => {
                    self.step_count += 1;
                }
            }
        }

        // Count non-encounter steps too
        if !self.state.player.moving && events.is_empty() {
            // No step completed this tick (either idle or mid-movement)
        }

        let facing_str = match self.state.player.facing {
            Direction::Up    => "up",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Right => "right",
        };

        serde_json::json!({
            "player_x": self.state.player.x,
            "player_y": self.state.player.y,
            "facing": facing_str,
            "moving": self.state.player.moving,
            "move_progress": self.state.player.move_progress,
            "map_width": self.map_width,
            "map_height": self.map_height,
            "encounter": self.encounter_triggered,
            "mode": format!("{:?}", self.state.mode),
        })
        .to_string()
    }

    // ── Battle methods ────────────────────────────────────────────────────────

    /// Get the current game mode as a string ("Overworld", "Battle", etc.).
    pub fn mode(&self) -> String {
        format!("{:?}", self.state.mode)
    }

    /// Submit a battle action and receive events.
    /// Input JSON: {"type":"fight","move_index":0} | {"type":"run"} | {"type":"switch","party_index":1} | {"type":"bag","item_id":1}
    /// Returns JSON array of BattleTurnEvent.
    pub fn battle_action(&mut self, action_json: &str) -> String {
        let action = match parse_battle_action(action_json) {
            Some(a) => a,
            None => {
                return serde_json::json!([{"Message": {"text": "Invalid battle action"}}])
                    .to_string()
            }
        };

        let mut events = if let Some(battle) = self.battle.as_mut() {
            BattleEngine::submit_action(
                battle,
                &mut self.state.player.party,
                action,
                &mut self.rng,
            )
        } else {
            return "[]".to_string();
        };

        // Check for PlayerWin — award XP and money
        let player_won = events
            .iter()
            .any(|e| matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin }));
        if player_won {
            if let Some(battle) = self.battle.as_mut() {
                let xp_events = BattleEngine::award_xp_and_money(
                    battle,
                    &mut self.state.player.party,
                    &mut self.state.player.money,
                );
                events.extend(xp_events);
            }
        }

        // Check for PlayerCapture — add captured sneaker to party or box
        let player_captured = events
            .iter()
            .any(|e| matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerCapture }));
        if player_captured {
            if let Some(battle) = &self.battle {
                let opp_idx = battle.opponent_active;
                let captured = battle.opponent.team[opp_idx].clone();
                let species_id = captured.species_id;
                if self.state.player.party.len() < 6 {
                    self.state.player.party.push(captured);
                } else {
                    self.state.player.sneaker_box.deposit(captured);
                }
                // Update dex
                if (species_id as usize) <= self.state.player.sneakerdex.entries.len() {
                    let idx = (species_id as usize).saturating_sub(1);
                    self.state.player.sneakerdex.entries[idx].seen = true;
                    self.state.player.sneakerdex.entries[idx].caught = true;
                }
            }
        }

        // If battle ended with no pending prompts, return to overworld
        let battle_ended = events
            .iter()
            .any(|e| matches!(e, BattleTurnEvent::BattleEnd { .. }));
        let has_waiting = self.battle.as_ref().map(|b| b.waiting_for.is_some()).unwrap_or(false);
        if battle_ended && !has_waiting {
            self.battle = None;
            self.state.mode = GameMode::Overworld;
        }

        serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string())
    }

    /// Respond to a move-learn prompt. slot 0-3 replaces that slot, slot 4 skips.
    /// Returns remaining events as JSON.
    pub fn battle_learn_move(&mut self, slot: u8) -> String {
        let battle = match self.battle.as_mut() {
            Some(b) => b,
            None => return "[]".to_string(),
        };

        let move_id = match &battle.waiting_for {
            Some(BattlePrompt::MoveLearn { move_id }) => *move_id,
            _ => return "[]".to_string(),
        };

        battle.waiting_for = None;

        let mut events = Vec::new();

        if slot < 4 {
            // Replace the move at slot
            let active_idx = battle.player_active;
            if let Some(sneaker) = self.state.player.party.get_mut(active_idx) {
                let md = data::get_move(move_id);
                sneaker.moves[slot as usize] = Some(crate::models::moves::MoveSlot {
                    move_id,
                    current_pp: md.pp,
                    max_pp: md.pp,
                });
            }
        }
        // slot == 4 means skip (don't learn)

        // Check if battle is actually over now
        let battle_over_without_prompts = battle.turn_log
            .iter()
            .any(|e| matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin }))
            && battle.waiting_for.is_none();

        if battle_over_without_prompts {
            events.push(BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin });
            self.battle = None;
            self.state.mode = GameMode::Overworld;
        }

        serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string())
    }

    /// Respond to an evolution prompt. accept=true to evolve, false to cancel.
    /// Returns remaining events as JSON.
    pub fn battle_evolution_choice(&mut self, accept: bool) -> String {
        let battle = match self.battle.as_mut() {
            Some(b) => b,
            None => return "[]".to_string(),
        };

        let target_species_id = match &battle.waiting_for {
            Some(BattlePrompt::Evolution { species_id }) => *species_id,
            _ => return "[]".to_string(),
        };

        battle.waiting_for = None;

        let mut events = Vec::new();

        if accept {
            let active_idx = battle.player_active;
            if let Some(sneaker) = self.state.player.party.get_mut(active_idx) {
                let new_species = data::get_species(target_species_id);
                sneaker.evolve(target_species_id, new_species);
                events.push(BattleTurnEvent::Message {
                    text: format!("Evolved into {}!", new_species.name),
                });
            }
        } else {
            events.push(BattleTurnEvent::Message {
                text: "Evolution cancelled.".to_string(),
            });
        }

        // Check if battle is actually over now
        let battle_over_without_prompts = battle.turn_log
            .iter()
            .any(|e| matches!(e, BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin }))
            && battle.waiting_for.is_none();

        if battle_over_without_prompts {
            events.push(BattleTurnEvent::BattleEnd { result: BattleResult::PlayerWin });
            self.battle = None;
            self.state.mode = GameMode::Overworld;
        }

        serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get current battle state as JSON for the UI.
    /// Returns JSON with player and opponent sneaker summaries.
    pub fn get_battle_state(&self) -> String {
        let battle = match &self.battle {
            Some(b) => b,
            None => return "{}".to_string(),
        };

        if self.state.player.party.is_empty() {
            return "{}".to_string();
        }

        let player_sneaker = &self.state.player.party[battle.player_active];
        let player_species = data::get_species(player_sneaker.species_id);

        let opp_sneaker = &battle.opponent.team[battle.opponent_active];
        let opp_species = data::get_species(opp_sneaker.species_id);

        let player_moves: Vec<serde_json::Value> = player_sneaker
            .moves
            .iter()
            .filter_map(|slot| {
                slot.as_ref().map(|s| {
                    let md = data::get_move(s.move_id);
                    serde_json::json!({
                        "name": md.name,
                        "pp": s.current_pp,
                        "max_pp": s.max_pp,
                        "faction": format!("{:?}", md.faction),
                    })
                })
            })
            .collect();

        serde_json::json!({
            "player": {
                "name": player_sneaker.display_name(player_species),
                "level": player_sneaker.level,
                "current_hp": player_sneaker.current_hp,
                "max_hp": player_sneaker.max_hp,
                "moves": player_moves,
            },
            "opponent": {
                "name": opp_sneaker.display_name(opp_species),
                "level": opp_sneaker.level,
                "current_hp": opp_sneaker.current_hp,
                "max_hp": opp_sneaker.max_hp,
            },
        })
        .to_string()
    }

    // ── Getters for JS ──

    pub fn player_x(&self) -> usize { self.state.player.x as usize }
    pub fn player_y(&self) -> usize { self.state.player.y as usize }
    pub fn map_width(&self) -> usize { self.map_width }
    pub fn map_height(&self) -> usize { self.map_height }
    pub fn step_count(&self) -> u32 { self.step_count }
    pub fn encounter_triggered(&self) -> bool { self.encounter_triggered }

    /// Get player facing direction: 1=up, 2=down, 3=left, 4=right
    pub fn player_facing(&self) -> u8 {
        match self.state.player.facing {
            Direction::Up    => 1,
            Direction::Down  => 2,
            Direction::Left  => 3,
            Direction::Right => 4,
        }
    }

    pub fn player_moving(&self) -> bool { self.state.player.moving }

    pub fn player_move_progress(&self) -> f32 { self.state.player.move_progress }

    /// Get tile at position (returns collision byte)
    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        if x >= self.map_width || y >= self.map_height {
            return 1;
        }
        self.map[y * self.map_width + x]
    }

    /// Get state as JSON for UI overlays
    pub fn state_json(&self) -> String {
        let facing_str = match self.state.player.facing {
            Direction::Up    => "up",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Right => "right",
        };
        serde_json::json!({
            "player_x": self.state.player.x,
            "player_y": self.state.player.y,
            "facing": facing_str,
            "moving": self.state.player.moving,
            "move_progress": self.state.player.move_progress,
            "step_count": self.step_count,
            "encounter": self.encounter_triggered,
            "map_width": self.map_width,
            "map_height": self.map_height,
            "mode": format!("{:?}", self.state.mode),
            "money": self.state.player.money,
            "story_progress": self.state.story_progress,
        })
        .to_string()
    }
}

impl GameEngine {
    /// Internal map loader — not exposed to WASM directly but usable in tests.
    pub fn load_map_from_json(&mut self, json: &str) -> Result<(), String> {
        let map_data = MapData::from_json(json)?;
        self.map_width = map_data.width as usize;
        self.map_height = map_data.height as usize;
        self.map = map_data.collision.clone();
        self.current_map = Some(map_data);
        Ok(())
    }
}

// ── Battle action JSON parsing ────────────────────────────────────────────────

fn parse_battle_action(json: &str) -> Option<BattleAction> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let action_type = v["type"].as_str()?;
    match action_type {
        "fight" => {
            let move_index = v["move_index"].as_u64()? as u8;
            Some(BattleAction::Fight { move_index })
        }
        "run" => Some(BattleAction::Run),
        "switch" => {
            let party_index = v["party_index"].as_u64()? as u8;
            Some(BattleAction::Switch { party_index })
        }
        "bag" => {
            let item_id = v["item_id"].as_u64()? as u16;
            Some(BattleAction::Bag { item_id })
        }
        _ => None,
    }
}
