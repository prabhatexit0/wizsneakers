use wasm_bindgen::prelude::*;
use std::collections::HashMap;

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
use world::dialogue::{DialogueData, DialogueState};
use world::npc::{NpcState, TrainerNpcData, tick_npcs, check_trainer_triggers};
use world::events::{interact as do_interact, InteractionResult};
use battle::{BattleEngine, BattleState, BattleAction, BattleTurnEvent, BattleResult, BattleKind};
use battle::types::{BattlePrompt, BattleOpponent, AiLevel};
use models::sneaker::xp_needed;

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
    /// Active NPC runtime states
    pub(crate) npcs: Vec<NpcState>,
    /// Active dialogue session
    pub(crate) dialogue_state: Option<DialogueState>,
    /// In-memory dialogue database loaded from JSON
    pub(crate) dialogue_db: HashMap<String, DialogueData>,
    /// NPC id of a trainer that has spotted the player (approach in progress)
    trainer_spotted: Option<String>,
    /// Timer for trainer approach sequence (ms)
    trainer_approach_timer: f64,
    /// Deduplicate action key presses (prevent holding from spamming)
    action_consumed: bool,
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
            npcs: Vec::new(),
            dialogue_state: None,
            dialogue_db: HashMap::new(),
            trainer_spotted: None,
            trainer_approach_timer: 0.0,
            action_consumed: false,
        }
    }

    /// Load map data from a JSON string. Updates collision, dimensions, and encounter table.
    pub fn load_map_data(&mut self, json: &str) -> Result<(), JsValue> {
        self.load_map_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    /// Load dialogue data from a JSON array string.
    /// Format: `[{"id": "...", "pages": [...]}, ...]`
    pub fn load_dialogue_data(&mut self, json: &str) -> Result<(), JsValue> {
        self.load_dialogue_json(json).map_err(|e| JsValue::from_str(&e))
    }

    /// Trigger interaction with what is in front of the player.
    /// Returns JSON: {"type": "dialogue"|"sign"|"shop"|"heal"|"sneaker_box"|"none", ...}
    pub fn interact(&mut self) -> String {
        let (px, py) = (self.state.player.x, self.state.player.y);
        let (fdx, fdy) = self.state.player.facing.delta();

        let events_defs = self.current_map
            .as_ref()
            .map(|m| m.events.as_slice())
            .unwrap_or(&[]);

        let result = do_interact(px, py, fdx, fdy, &self.npcs, events_defs, &self.dialogue_db);

        match result {
            Some(InteractionResult::Dialogue(data)) => {
                let page = data.pages.first().cloned();
                let page_json = page.map(|p| self.page_to_json_with_replacements(&p));
                let page_json = page_json.unwrap_or(serde_json::Value::Null);
                self.dialogue_state = Some(DialogueState::new(data));
                self.state.mode = GameMode::Dialogue;
                serde_json::json!({
                    "type": "dialogue",
                    "page": page_json,
                }).to_string()
            }
            Some(InteractionResult::Sign(text)) => {
                serde_json::json!({
                    "type": "sign",
                    "text": text,
                }).to_string()
            }
            Some(InteractionResult::Shop(shop_id)) => {
                serde_json::json!({
                    "type": "shop",
                    "shop_id": shop_id,
                }).to_string()
            }
            Some(InteractionResult::Heal) => {
                serde_json::json!({ "type": "heal" }).to_string()
            }
            Some(InteractionResult::SneakerBox) => {
                serde_json::json!({ "type": "sneaker_box" }).to_string()
            }
            None => {
                serde_json::json!({ "type": "none" }).to_string()
            }
        }
    }

    /// Advance to the next dialogue page.
    /// Returns JSON: {"status": "continue"|"end", "page": {...}}
    pub fn advance_dialogue(&mut self) -> String {
        let ds = match self.dialogue_state.as_mut() {
            Some(s) => s,
            None => return serde_json::json!({"status": "end"}).to_string(),
        };

        if ds.advance() {
            let page = ds.current().cloned();
            let page_json = page.map(|p| self.page_to_json_with_replacements(&p));
            serde_json::json!({
                "status": "continue",
                "page": page_json.unwrap_or(serde_json::Value::Null),
            }).to_string()
        } else {
            // End of dialogue
            self.dialogue_state = None;
            self.state.mode = GameMode::Overworld;
            serde_json::json!({"status": "end"}).to_string()
        }
    }

    /// Select a dialogue choice by index.
    /// Returns JSON: {"status": "continue"|"end", "page": {...}}
    pub fn select_choice(&mut self, index: u8) -> String {
        let choice = {
            let ds = match self.dialogue_state.as_ref() {
                Some(s) => s,
                None => return serde_json::json!({"status": "end"}).to_string(),
            };
            let page = match ds.current() {
                Some(p) => p,
                None => return serde_json::json!({"status": "end"}).to_string(),
            };
            let choices = match &page.choices {
                Some(c) => c,
                None => return serde_json::json!({"status": "end"}).to_string(),
            };
            choices.get(index as usize).cloned()
        };

        let choice = match choice {
            Some(c) => c,
            None => return serde_json::json!({"status": "end"}).to_string(),
        };

        // Set flag if specified
        if let Some(flag) = &choice.set_flag {
            self.state.event_flags.insert(flag.clone());
        }

        // Handle action
        if let Some(action) = &choice.action {
            match action.as_str() {
                "heal_party" => {
                    for sneaker in &mut self.state.player.party {
                        sneaker.current_hp = sneaker.max_hp;
                        sneaker.status = None;
                    }
                }
                _ => {}
            }
        }

        // Follow next_dialogue if specified
        if let Some(next_id) = &choice.next_dialogue.clone() {
            if let Some(data) = self.dialogue_db.get(next_id).cloned() {
                let page = data.pages.first().cloned();
                let page_json = page.map(|p| self.page_to_json_with_replacements(&p));
                self.dialogue_state = Some(DialogueState::new(data));
                return serde_json::json!({
                    "status": "continue",
                    "page": page_json.unwrap_or(serde_json::Value::Null),
                }).to_string();
            }
        }

        // End dialogue
        self.dialogue_state = None;
        self.state.mode = GameMode::Overworld;
        serde_json::json!({"status": "end"}).to_string()
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
                "npcs": [],
                "trainer_spotted": null,
            })
            .to_string();
        }

        // In dialogue mode, handle action key to advance
        if self.state.mode == GameMode::Dialogue {
            let action_pressed = input == "action" && !self.action_consumed;
            if action_pressed {
                self.action_consumed = true;
                let advance_result = self.advance_dialogue_internal();
                let _ = advance_result; // result communicated via state change
            }
            if input != "action" {
                self.action_consumed = false;
            }

            let dialogue_page = self.dialogue_state.as_ref()
                .and_then(|ds| ds.current())
                .map(|p| self.page_to_json_with_replacements(p));

            let npc_json = self.build_npc_json();
            return serde_json::json!({
                "player_x": self.state.player.x,
                "player_y": self.state.player.y,
                "facing": facing_str,
                "moving": false,
                "move_progress": 0.0,
                "map_width": self.map_width,
                "map_height": self.map_height,
                "encounter": false,
                "mode": format!("{:?}", self.state.mode),
                "npcs": npc_json,
                "trainer_spotted": self.trainer_spotted,
                "dialogue_page": dialogue_page,
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

        // Handle trainer approach sequence
        if self.trainer_spotted.is_some() {
            self.trainer_approach_timer -= dt_ms;
            let trainer_id = self.trainer_spotted.clone().unwrap();

            // Find trainer NPC and move toward player
            let player_x = self.state.player.x;
            let player_y = self.state.player.y;

            let trainer_adjacent = {
                let npc = self.npcs.iter().find(|n| n.id == trainer_id);
                if let Some(npc) = npc {
                    let dx = (npc.x as isize - player_x as isize).abs();
                    let dy = (npc.y as isize - player_y as isize).abs();
                    dx + dy <= 1
                } else {
                    true // trainer not found, start battle
                }
            };

            if trainer_adjacent {
                // Start trainer battle
                self.start_trainer_battle(&trainer_id);
                self.trainer_spotted = None;
                let facing_str2 = match self.state.player.facing {
                    Direction::Up    => "up",
                    Direction::Down  => "down",
                    Direction::Left  => "left",
                    Direction::Right => "right",
                };
                let npc_json = self.build_npc_json();
                return serde_json::json!({
                    "player_x": self.state.player.x,
                    "player_y": self.state.player.y,
                    "facing": facing_str2,
                    "moving": false,
                    "move_progress": 0.0,
                    "map_width": self.map_width,
                    "map_height": self.map_height,
                    "encounter": false,
                    "mode": format!("{:?}", self.state.mode),
                    "npcs": npc_json,
                    "trainer_spotted": serde_json::Value::Null,
                })
                .to_string();
            } else if self.trainer_approach_timer <= 0.0 {
                // Move trainer one step toward player
                self.move_trainer_toward_player(&trainer_id);
                self.trainer_approach_timer = 300.0; // move every 300ms
            }

            // Player is frozen during trainer approach
            let npc_json = self.build_npc_json();
            return serde_json::json!({
                "player_x": self.state.player.x,
                "player_y": self.state.player.y,
                "facing": facing_str,
                "moving": false,
                "move_progress": 0.0,
                "map_width": self.map_width,
                "map_height": self.map_height,
                "encounter": false,
                "mode": "Overworld",
                "npcs": npc_json,
                "trainer_spotted": self.trainer_spotted,
            })
            .to_string();
        }

        // Handle action key (with edge detection)
        if action_str == "action" && !self.action_consumed {
            self.action_consumed = true;
            // Try to interact with what's in front of the player
            let interact_result_json = self.interact();
            let v: serde_json::Value = serde_json::from_str(&interact_result_json).unwrap_or_default();
            if v["type"] != "none" {
                // Interaction occurred — return updated state
                let facing_str2 = match self.state.player.facing {
                    Direction::Up    => "up",
                    Direction::Down  => "down",
                    Direction::Left  => "left",
                    Direction::Right => "right",
                };
                let npc_json = self.build_npc_json();
                return serde_json::json!({
                    "player_x": self.state.player.x,
                    "player_y": self.state.player.y,
                    "facing": facing_str2,
                    "moving": false,
                    "move_progress": 0.0,
                    "map_width": self.map_width,
                    "map_height": self.map_height,
                    "encounter": false,
                    "mode": format!("{:?}", self.state.mode),
                    "npcs": npc_json,
                    "trainer_spotted": self.trainer_spotted,
                    "interaction": v,
                })
                .to_string();
            }
        }
        if action_str != "action" {
            self.action_consumed = false;
        }

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

        // Tick NPCs
        if let Some(map) = &self.current_map.clone() {
            let player_pos = (self.state.player.x, self.state.player.y);
            tick_npcs(&mut self.npcs, player_pos, map, dt_ms, &mut self.rng);

            // Check trainer line-of-sight
            if self.trainer_spotted.is_none() {
                if let Some(spotted_id) = check_trainer_triggers(&self.npcs, player_pos, map) {
                    self.trainer_spotted = Some(spotted_id);
                    self.trainer_approach_timer = 500.0; // brief pause before approach
                }
            }
        }

        let facing_str = match self.state.player.facing {
            Direction::Up    => "up",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Right => "right",
        };

        let npc_json = self.build_npc_json();

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
            "npcs": npc_json,
            "trainer_spotted": self.trainer_spotted,
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
    /// Returns full BattleRenderState with sneaker summaries, stages, moves, and metadata.
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

        // Compute XP progress for player sneaker
        let xp_current_level = xp_needed(player_sneaker.level);
        let xp_next_level = xp_needed(player_sneaker.level.saturating_add(1));

        // Available moves with full detail
        let available_moves: Vec<serde_json::Value> = player_sneaker
            .moves
            .iter()
            .filter_map(|slot| {
                slot.as_ref().map(|s| {
                    let md = data::get_move(s.move_id);
                    serde_json::json!({
                        "id": md.id,
                        "name": md.name,
                        "faction": format!("{:?}", md.faction),
                        "category": format!("{:?}", md.category),
                        "power": md.power.unwrap_or(0),
                        "accuracy": md.accuracy,
                        "current_pp": s.current_pp,
                        "max_pp": s.max_pp,
                    })
                })
            })
            .collect();

        // Opponent moves (for move name lookup during animations)
        let opponent_moves: Vec<serde_json::Value> = opp_sneaker
            .moves
            .iter()
            .filter_map(|slot| {
                slot.as_ref().map(|s| {
                    let md = data::get_move(s.move_id);
                    serde_json::json!({
                        "id": md.id,
                        "name": md.name,
                        "faction": format!("{:?}", md.faction),
                    })
                })
            })
            .collect();

        let player_status = player_sneaker.status.as_ref().map(|s| format!("{:?}", s.status_type()));
        let opp_status = opp_sneaker.status.as_ref().map(|s| format!("{:?}", s.status_type()));

        let is_wild = matches!(battle.kind, BattleKind::Wild);

        let waiting_for = battle.waiting_for.as_ref().map(|p| match p {
            BattlePrompt::MoveLearn { move_id } => serde_json::json!({ "type": "MoveLearn", "move_id": move_id }),
            BattlePrompt::Evolution { species_id } => serde_json::json!({ "type": "Evolution", "species_id": species_id }),
        });

        serde_json::json!({
            "player_sneaker": {
                "uid": player_sneaker.uid,
                "species_id": player_sneaker.species_id,
                "name": player_sneaker.display_name(player_species),
                "level": player_sneaker.level,
                "current_hp": player_sneaker.current_hp,
                "max_hp": player_sneaker.max_hp,
                "faction": format!("{:?}", player_species.faction),
                "rarity_tier": format!("{:?}", player_species.rarity_tier),
                "status": player_status,
                "current_xp": player_sneaker.xp,
                "xp_current_level": xp_current_level,
                "xp_next_level": xp_next_level,
            },
            "opponent_sneaker": {
                "uid": opp_sneaker.uid,
                "species_id": opp_sneaker.species_id,
                "name": opp_sneaker.display_name(opp_species),
                "level": opp_sneaker.level,
                "current_hp": opp_sneaker.current_hp,
                "max_hp": opp_sneaker.max_hp,
                "faction": format!("{:?}", opp_species.faction),
                "rarity_tier": format!("{:?}", opp_species.rarity_tier),
                "status": opp_status,
            },
            "player_stages": {
                "hype": battle.player_stages.hype,
                "comfort": battle.player_stages.comfort,
                "drip": battle.player_stages.drip,
                "rarity": battle.player_stages.rarity,
            },
            "opponent_stages": {
                "hype": battle.opponent_stages.hype,
                "comfort": battle.opponent_stages.comfort,
                "drip": battle.opponent_stages.drip,
                "rarity": battle.opponent_stages.rarity,
            },
            "available_moves": available_moves,
            "opponent_moves": opponent_moves,
            "can_flee": battle.can_flee,
            "is_wild": is_wild,
            "waiting_for": waiting_for,
        })
        .to_string()
    }

    /// Get bag items as JSON for the BagScreen UI.
    /// is_wild: if true, include sneaker cases.
    pub fn get_bag_items(&self, is_wild: bool) -> String {
        let bag = &self.state.player.bag;

        let mut heal: Vec<serde_json::Value> = bag.heal_items.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({
                "id": id,
                "name": item.name,
                "qty": qty,
                "description": item.description,
                "category": format!("{:?}", item.category),
            })
        }).collect();

        let mut battle_items: Vec<serde_json::Value> = bag.battle_items.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({
                "id": id,
                "name": item.name,
                "qty": qty,
                "description": item.description,
                "category": format!("{:?}", item.category),
            })
        }).collect();

        let mut cases: Vec<serde_json::Value> = Vec::new();
        if is_wild {
            cases = bag.sneaker_cases.iter().map(|(id, qty)| {
                let item = data::get_item(*id);
                serde_json::json!({
                    "id": id,
                    "name": item.name,
                    "qty": qty,
                    "description": item.description,
                    "category": format!("{:?}", item.category),
                })
            }).collect();
        }

        // Sort each pocket by item id for stable display
        heal.sort_by_key(|v| v["id"].as_u64().unwrap_or(0));
        battle_items.sort_by_key(|v| v["id"].as_u64().unwrap_or(0));
        cases.sort_by_key(|v| v["id"].as_u64().unwrap_or(0));

        serde_json::json!({
            "heal": heal,
            "battle": battle_items,
            "cases": cases,
        })
        .to_string()
    }

    /// Get party state as JSON for the PartyScreen UI.
    pub fn get_party_state(&self) -> String {
        let active_idx = self.battle.as_ref().map(|b| b.player_active).unwrap_or(0);

        let party: Vec<serde_json::Value> = self.state.player.party.iter().enumerate().map(|(i, snk)| {
            let species = data::get_species(snk.species_id);
            let status = snk.status.as_ref().map(|s| format!("{:?}", s.status_type()));
            serde_json::json!({
                "uid": snk.uid,
                "species_id": snk.species_id,
                "name": snk.display_name(species),
                "level": snk.level,
                "current_hp": snk.current_hp,
                "max_hp": snk.max_hp,
                "faction": format!("{:?}", species.faction),
                "rarity_tier": format!("{:?}", species.rarity_tier),
                "status": status,
                "is_active": i == active_idx,
                "is_fainted": snk.current_hp == 0,
            })
        }).collect();

        serde_json::to_string(&party).unwrap_or_else(|_| "[]".to_string())
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

        // Initialize NPC runtime state from map definitions
        self.npcs = map_data.npcs.iter().map(|def| {
            let facing = NpcState::parse_facing(&def.facing);
            let trainer_data = if def.is_trainer {
                Some(TrainerNpcData {
                    trainer_id: def.trainer_id,
                    sight_range: def.sight_range,
                })
            } else {
                None
            };
            NpcState {
                id: def.id.clone(),
                x: def.x,
                y: def.y,
                facing,
                sprite: def.sprite.clone(),
                movement: def.movement.clone(),
                dialogue_id: def.dialogue_id.clone(),
                is_trainer: def.is_trainer,
                trainer_data,
                defeated: def.defeated_flag.as_ref()
                    .map(|f| self.state.event_flags.contains(f))
                    .unwrap_or(false),
                moving: false,
                move_progress: 0.0,
                move_timer: 2000.0,
                home_x: def.x,
                home_y: def.y,
                patrol_index: 0,
            }
        }).collect();

        self.current_map = Some(map_data);
        Ok(())
    }

    /// Internal dialogue loader — not exposed to WASM directly but usable in tests.
    pub fn load_dialogue_json(&mut self, json: &str) -> Result<(), String> {
        let dialogues: Vec<DialogueData> = serde_json::from_str(json)
            .map_err(|e| format!("Failed to parse dialogue JSON: {}", e))?;
        for d in dialogues {
            self.dialogue_db.insert(d.id.clone(), d);
        }
        Ok(())
    }

    /// Advance dialogue without returning JSON (used internally from tick).
    fn advance_dialogue_internal(&mut self) {
        let has_next = self.dialogue_state.as_mut().map(|ds| ds.advance()).unwrap_or(false);
        if !has_next {
            self.dialogue_state = None;
            self.state.mode = GameMode::Overworld;
        }
    }

    /// Build NPC JSON array for tick return.
    fn build_npc_json(&self) -> Vec<serde_json::Value> {
        self.npcs.iter().map(|npc| {
            serde_json::json!({
                "id": npc.id,
                "x": npc.x,
                "y": npc.y,
                "facing": npc.facing_str(),
                "sprite": npc.sprite,
                "moving": npc.moving,
                "move_progress": npc.move_progress,
                "is_trainer": npc.is_trainer,
                "defeated": npc.defeated,
            })
        }).collect()
    }

    /// Convert a DialoguePage to JSON with template replacements applied.
    fn page_to_json_with_replacements(&self, page: &world::dialogue::DialoguePage) -> serde_json::Value {
        let text = world::dialogue::replace_template_vars(&page.text, &self.state.player.name);
        let speaker = page.speaker.as_ref().map(|s| {
            world::dialogue::replace_template_vars(s, &self.state.player.name)
        });
        serde_json::json!({
            "speaker": speaker,
            "text": text,
            "choices": page.choices,
        })
    }

    /// Move the spotted trainer one step toward the player.
    fn move_trainer_toward_player(&mut self, trainer_id: &str) {
        let player_x = self.state.player.x;
        let player_y = self.state.player.y;

        let npc_pos = self.npcs.iter().find(|n| n.id == trainer_id)
            .map(|n| (n.x, n.y));

        if let Some((nx, ny)) = npc_pos {
            let dx = player_x as isize - nx as isize;
            let dy = player_y as isize - ny as isize;

            let dir = if dx.abs() >= dy.abs() {
                if dx > 0 { Direction::Right } else { Direction::Left }
            } else {
                if dy > 0 { Direction::Down } else { Direction::Up }
            };

            let (ddx, ddy) = dir.delta();
            let new_x = (nx as isize + ddx) as u16;
            let new_y = (ny as isize + ddy) as u16;

            if let Some(npc) = self.npcs.iter_mut().find(|n| n.id == trainer_id) {
                npc.facing = dir;
                if self.map.get(new_y as usize * self.map_width + new_x as usize)
                    .map(|&t| t != 1)
                    .unwrap_or(false)
                {
                    npc.x = new_x;
                    npc.y = new_y;
                }
            }
        }
    }

    /// Start a trainer battle when they reach the player.
    fn start_trainer_battle(&mut self, trainer_id: &str) {
        let trainer_info = self.npcs.iter().find(|n| n.id == trainer_id)
            .and_then(|n| n.trainer_data.as_ref())
            .map(|td| (td.trainer_id, format!("Trainer {}", td.trainer_id)));

        let (tid, tname) = trainer_info.unwrap_or((0, "Trainer".to_string()));

        // Generate a simple opponent team (level 5 sneaker)
        let opp_sneaker = generate_wild_sneaker(1, 5, &mut self.rng);
        let battle_state = BattleState {
            kind: BattleKind::Trainer { id: tid, name: tname },
            player_active: 0,
            opponent: BattleOpponent {
                team: vec![opp_sneaker],
                items: vec![],
                ai_level: AiLevel::Basic,
            },
            opponent_active: 0,
            turn_number: 0,
            player_stages: Default::default(),
            opponent_stages: Default::default(),
            turn_log: vec![],
            flee_attempts: 0,
            can_flee: false,
            waiting_for: None,
            player_skip_turn: false,
            opponent_skip_turn: false,
        };

        self.battle = Some(battle_state);
        self.state.mode = GameMode::Battle;

        // Mark trainer as defeated (will be set properly when battle ends)
        if let Some(npc) = self.npcs.iter_mut().find(|n| n.id == trainer_id) {
            npc.defeated = true;
        }
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
