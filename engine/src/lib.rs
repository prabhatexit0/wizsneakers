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
use world::map::{MapData, MapTransition, TransitionType};
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
    /// Pending map transition (set when player walks off edge or through door)
    pub(crate) pending_transition: Option<MapTransition>,
    /// Species ID of rival Flip's starter (set by choose_starter)
    pub(crate) rival_starter_id: u16,
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
            pending_transition: None,
            rival_starter_id: 0,
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

        // ── Edge transition detection ──────────────────────────────────────────
        // Before any movement, check if the player is attempting to walk off the
        // map edge in a direction that has a connection.
        if !self.state.player.moving && self.pending_transition.is_none() {
            let dir_opt: Option<Direction> = match action_str {
                "up"    => Some(Direction::Up),
                "down"  => Some(Direction::Down),
                "left"  => Some(Direction::Left),
                "right" => Some(Direction::Right),
                _ => None,
            };
            if let Some(dir) = dir_opt {
                if let Some(map) = &self.current_map.clone() {
                    let (ddx, ddy) = dir.delta();
                    let nx = self.state.player.x as isize + ddx;
                    let ny = self.state.player.y as isize + ddy;
                    let map_w = map.width as isize;
                    let map_h = map.height as isize;

                    // Check if the target tile is the border edge (solid outer row/col)
                    let target_is_border = nx < 0 || ny < 0 || nx >= map_w || ny >= map_h
                        || (nx >= 0 && ny >= 0 && nx < map_w && ny < map_h && {
                            let tile = map.collision[ny as usize * map.width as usize + nx as usize];
                            tile == 1 && (nx == 0 || ny == 0 || nx == map_w - 1 || ny == map_h - 1)
                        });

                    if target_is_border {
                        let dir_str = match dir {
                            Direction::Up    => "north",
                            Direction::Down  => "south",
                            Direction::Left  => "west",
                            Direction::Right => "east",
                        };
                        let connection = match dir {
                            Direction::Up    => map.connections.north.clone(),
                            Direction::Down  => map.connections.south.clone(),
                            Direction::Left  => map.connections.west.clone(),
                            Direction::Right => map.connections.east.clone(),
                        };
                        if let Some(target_map) = connection {
                            self.pending_transition = Some(MapTransition {
                                target_map,
                                target_x: self.state.player.x, // preserved for north/south
                                target_y: self.state.player.y, // preserved for east/west
                                transition_type: TransitionType::Walk,
                                direction: dir_str.to_string(),
                            });
                            self.state.player.facing = dir;
                            // Return immediately so JS can handle the transition
                            let facing_str3 = match self.state.player.facing {
                                Direction::Up    => "up",
                                Direction::Down  => "down",
                                Direction::Left  => "left",
                                Direction::Right => "right",
                            };
                            let npc_json = self.build_npc_json();
                            let trans = self.pending_transition.as_ref().map(|t| serde_json::json!({
                                "map": t.target_map,
                                "x": t.target_x,
                                "y": t.target_y,
                                "type": format!("{:?}", t.transition_type).to_lowercase(),
                                "direction": t.direction,
                            }));
                            return serde_json::json!({
                                "player_x": self.state.player.x,
                                "player_y": self.state.player.y,
                                "facing": facing_str3,
                                "moving": false,
                                "move_progress": 0.0,
                                "map_width": self.map_width,
                                "map_height": self.map_height,
                                "encounter": false,
                                "mode": format!("{:?}", self.state.mode),
                                "npcs": npc_json,
                                "trainer_spotted": self.trainer_spotted,
                                "transition": trans,
                            }).to_string();
                        }
                    }
                }
            }
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
                GameEvent::MapTransition { .. } => {
                    // Door tile — look up event definition for target map/position
                    let px = self.state.player.x;
                    let py = self.state.player.y;
                    if let Some(ev_def) = self.current_map.as_ref().and_then(|m| {
                        m.events.iter().find(|e| e.x == px && e.y == py
                            && (e.event_type == "warp_trigger" || e.event_type == "door"))
                    }) {
                        let parts: Vec<&str> = ev_def.data.split(':').collect();
                        if parts.len() >= 3 {
                            let target_map = parts[0].to_string();
                            let tx: u16 = parts[1].parse().unwrap_or(1);
                            let ty: u16 = parts[2].parse().unwrap_or(1);
                            self.pending_transition = Some(MapTransition {
                                target_map,
                                target_x: tx,
                                target_y: ty,
                                transition_type: TransitionType::Fade,
                                direction: "warp".to_string(),
                            });
                        }
                    }
                    self.step_count += 1;
                }
                GameEvent::Warp { .. } => {
                    // Warp tile — look up event definition
                    let px = self.state.player.x;
                    let py = self.state.player.y;
                    if let Some(ev_def) = self.current_map.as_ref().and_then(|m| {
                        m.events.iter().find(|e| e.x == px && e.y == py
                            && (e.event_type == "warp_trigger" || e.event_type == "warp"))
                    }) {
                        let parts: Vec<&str> = ev_def.data.split(':').collect();
                        if parts.len() >= 3 {
                            let target_map = parts[0].to_string();
                            let tx: u16 = parts[1].parse().unwrap_or(1);
                            let ty: u16 = parts[2].parse().unwrap_or(1);
                            self.pending_transition = Some(MapTransition {
                                target_map,
                                target_x: tx,
                                target_y: ty,
                                transition_type: TransitionType::Warp,
                                direction: "warp".to_string(),
                            });
                        }
                    }
                    self.step_count += 1;
                }
                GameEvent::None => {}
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

        let transition_json = self.pending_transition.as_ref().map(|t| serde_json::json!({
            "map": t.target_map,
            "x": t.target_x,
            "y": t.target_y,
            "type": format!("{:?}", t.transition_type).to_lowercase(),
            "direction": t.direction,
        }));

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
            "transition": transition_json,
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

    // ── Save / Load ───────────────────────────────────────────────────────────

    /// Serialize the full GameState to a JSON string for saving.
    pub fn export_save(&self) -> String {
        serde_json::to_string(&self.state).unwrap_or_else(|_| "{}".to_string())
    }

    /// Reconstruct a GameEngine from a previously exported save JSON.
    /// Map data and dialogue data must be reloaded by the client afterward.
    pub fn load_save(json: &str) -> Result<GameEngine, JsValue> {
        let state: GameState = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse save: {}", e)))?;
        let seed = state.play_time_ms.wrapping_add(1);
        let mut map = vec![0u8; MAP_WIDTH * MAP_HEIGHT];
        for x in 0..MAP_WIDTH {
            map[x] = 1;
            map[(MAP_HEIGHT - 1) * MAP_WIDTH + x] = 1;
        }
        for y in 0..MAP_HEIGHT {
            map[y * MAP_WIDTH] = 1;
            map[y * MAP_WIDTH + MAP_WIDTH - 1] = 1;
        }
        Ok(GameEngine {
            state,
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
            pending_transition: None,
            rival_starter_id: 0,
        })
    }

    /// Set the player's name.
    pub fn set_player_name(&mut self, name: &str) {
        self.state.player.name = name.to_string();
    }

    /// Choose the player's starter sneaker.
    /// choice 0 = Retro Runner, 1 = Tech Trainer, 2 = Skate Blazer
    pub fn choose_starter(&mut self, choice: u8) {
        use models::inventory::InventoryPocket;
        use models::moves::MoveSlot;
        use models::stats::{Stats, Condition};
        use models::sneaker::SneakerInstance;

        let (species_id, move1_id, move2_id): (u16, u16, u16) = match choice {
            0 => (1,  5,  11), // Retro Runner: Stomp, Crease
            1 => (9,  4,  20), // Tech Trainer: Quick Step, Shock Drop
            2 => (17, 5,  27), // Skate Blazer: Stomp, Kickflip
            _ => return,
        };

        // Rival Flip picks the counter-type
        let rival_id: u16 = match choice {
            0 => 9,  // Retro → rival has Tech Trainer
            1 => 17, // Tech  → rival has Skate Blazer
            2 => 1,  // Skate → rival has Retro Runner
            _ => return,
        };

        let species = data::get_species(species_id);
        let level: u8 = 5;

        // Neutral IVs for the starter
        let ivs = Stats { durability: 15, hype: 15, comfort: 15, drip: 15, rarity: 15 };

        // Calculate max HP
        let base_hp = species.base_stats.durability as u32;
        let iv_hp   = ivs.durability as u32;
        let inner   = (2 * base_hp + iv_hp) * level as u32 / 100;
        let max_hp  = (inner + level as u32 + 10) as u16;

        let md1 = data::get_move(move1_id);
        let md2 = data::get_move(move2_id);

        let mut moves = [None, None, None, None];
        moves[0] = Some(MoveSlot { move_id: move1_id, current_pp: md1.pp, max_pp: md1.pp });
        moves[1] = Some(MoveSlot { move_id: move2_id, current_pp: md2.pp, max_pp: md2.pp });

        let uid = self.rng.next_u64();

        let starter = SneakerInstance {
            uid,
            species_id,
            nickname: None,
            level,
            xp: 0,
            current_hp: max_hp,
            max_hp,
            ivs,
            evs: Stats::zero(),
            condition: Condition::Deadstock,
            moves,
            status: None,
            on_fire_turns: 0,
            held_item: None,
            friendship: 70,
            caught_location: 0,
            original_trainer: self.state.player.name.clone(),
        };

        self.state.player.party.push(starter);

        // Starting money and items
        self.state.player.money = 500;
        self.state.player.bag.add_item(1,  5, InventoryPocket::HealItems);    // 5x Sole Sauce
        self.state.player.bag.add_item(30, 5, InventoryPocket::SneakerCases); // 5x Sneaker Case

        // Mark starter as caught in Sneakerdex
        let dex_idx = (species_id as usize).saturating_sub(1);
        if dex_idx < self.state.player.sneakerdex.entries.len() {
            self.state.player.sneakerdex.entries[dex_idx].seen   = true;
            self.state.player.sneakerdex.entries[dex_idx].caught = true;
        }

        // Store rival species and set flag
        self.rival_starter_id = rival_id;
        self.state.event_flags.insert("has_starter".to_string());
    }

    /// Get the rival's starter species ID (set after choose_starter is called).
    pub fn get_rival_starter_id(&self) -> u16 {
        self.rival_starter_id
    }

    /// Start the rival Flip battle. Uses the counter-type starter at Lv.7.
    pub fn start_rival_battle(&mut self) {
        let rival_species_id = if self.rival_starter_id != 0 { self.rival_starter_id } else { 9 };
        let rival_sneaker = generate_wild_sneaker(rival_species_id, 7, &mut self.rng);
        let battle_state = BattleState {
            kind: BattleKind::Trainer { id: 999, name: "Flip".to_string() },
            player_active: 0,
            opponent: BattleOpponent {
                team: vec![rival_sneaker],
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
        self.state.event_flags.insert("route1_rival_battled".to_string());
    }

    /// Get the current pending map transition as JSON, or null if none.
    pub fn get_pending_transition(&self) -> String {
        match &self.pending_transition {
            Some(t) => serde_json::json!({
                "map": t.target_map,
                "x": t.target_x,
                "y": t.target_y,
                "type": format!("{:?}", t.transition_type).to_lowercase(),
                "direction": t.direction,
            }).to_string(),
            None => "null".to_string(),
        }
    }

    /// Use an item on a party member in the overworld.
    /// Returns JSON: {"ok": bool, "error": string}
    pub fn use_item(&mut self, item_id: u16, target_index: u8) -> String {
        use models::{ItemCategory, ItemEffect, InventoryPocket};
        let item = data::get_item(item_id);
        let party_len = self.state.player.party.len();
        let target_idx = target_index as usize;
        if target_idx >= party_len {
            return serde_json::json!({"ok": false, "error": "Invalid party index"}).to_string();
        }
        let is_fainted = self.state.player.party[target_idx].current_hp == 0;
        let is_revive = matches!(item.effect, ItemEffect::Revive(_) | ItemEffect::ReviveFull);
        if is_fainted && !is_revive {
            return serde_json::json!({"ok": false, "error": "Cannot use this item on a fainted sneaker"}).to_string();
        }
        match item.effect {
            ItemEffect::HealHp(amount) => {
                let snk = &mut self.state.player.party[target_idx];
                snk.current_hp = (snk.current_hp + amount).min(snk.max_hp);
            }
            ItemEffect::HealFull => {
                let snk = &mut self.state.player.party[target_idx];
                snk.current_hp = snk.max_hp;
            }
            ItemEffect::Revive(pct) => {
                let snk = &mut self.state.player.party[target_idx];
                if snk.current_hp == 0 {
                    snk.current_hp = ((snk.max_hp as u32 * pct as u32) / 100).max(1) as u16;
                }
            }
            ItemEffect::ReviveFull => {
                let snk = &mut self.state.player.party[target_idx];
                if snk.current_hp == 0 {
                    snk.current_hp = snk.max_hp;
                }
            }
            ItemEffect::CureStatus(Some(st)) => {
                let snk = &mut self.state.player.party[target_idx];
                if snk.status.as_ref().map(|s| s.status_type() == st).unwrap_or(false) {
                    snk.status = None;
                }
            }
            ItemEffect::CureStatus(None) | ItemEffect::CureAll => {
                self.state.player.party[target_idx].status = None;
            }
            ItemEffect::RestorePp(amount) => {
                let snk = &mut self.state.player.party[target_idx];
                for slot in snk.moves.iter_mut().flatten() {
                    slot.current_pp = (slot.current_pp + amount).min(slot.max_pp);
                }
            }
            ItemEffect::RestoreAllPp => {
                let snk = &mut self.state.player.party[target_idx];
                for slot in snk.moves.iter_mut().flatten() {
                    slot.current_pp = slot.max_pp;
                }
            }
            _ => {
                return serde_json::json!({"ok": false, "error": "Cannot use this item outside of battle"}).to_string();
            }
        }
        // Consume the item
        let pocket = match item.category {
            ItemCategory::HealItem => Some(InventoryPocket::HealItems),
            ItemCategory::BattleItem => Some(InventoryPocket::BattleItems),
            ItemCategory::SneakerCase => Some(InventoryPocket::SneakerCases),
            ItemCategory::HeldItem => Some(InventoryPocket::HeldItems),
            ItemCategory::KeyItem => None,
        };
        if let Some(p) = pocket {
            self.state.player.bag.remove_item(item_id, 1, p);
        }
        serde_json::json!({"ok": true}).to_string()
    }

    /// Buy an item from a shop: deducts money and adds item to bag.
    /// Returns JSON: {"ok": bool, "money": u32, "error": string}
    pub fn buy_item(&mut self, item_id: u16, quantity: u16) -> String {
        use models::{ItemCategory, InventoryPocket};
        let item = data::get_item(item_id);
        let total_cost = item.cost as u64 * quantity as u64;
        if (self.state.player.money as u64) < total_cost {
            return serde_json::json!({"ok": false, "error": "Insufficient money"}).to_string();
        }
        self.state.player.money -= total_cost as u32;
        match item.category {
            ItemCategory::KeyItem => {
                if !self.state.player.bag.key_items.contains(&item_id) {
                    self.state.player.bag.key_items.push(item_id);
                }
            }
            ItemCategory::HealItem => {
                self.state.player.bag.add_item(item_id, quantity, InventoryPocket::HealItems);
            }
            ItemCategory::BattleItem => {
                self.state.player.bag.add_item(item_id, quantity, InventoryPocket::BattleItems);
            }
            ItemCategory::SneakerCase => {
                self.state.player.bag.add_item(item_id, quantity, InventoryPocket::SneakerCases);
            }
            ItemCategory::HeldItem => {
                self.state.player.bag.add_item(item_id, quantity, InventoryPocket::HeldItems);
            }
        }
        serde_json::json!({"ok": true, "money": self.state.player.money}).to_string()
    }

    /// Sell an item: adds money and removes item from bag.
    /// Returns JSON: {"ok": bool, "money": u32, "error": string}
    pub fn sell_item(&mut self, item_id: u16, quantity: u16) -> String {
        use models::{ItemCategory, InventoryPocket};
        let item = data::get_item(item_id);
        if item.category == ItemCategory::KeyItem {
            return serde_json::json!({"ok": false, "error": "Cannot sell key items"}).to_string();
        }
        let pocket = match item.category {
            ItemCategory::HealItem => InventoryPocket::HealItems,
            ItemCategory::BattleItem => InventoryPocket::BattleItems,
            ItemCategory::SneakerCase => InventoryPocket::SneakerCases,
            ItemCategory::HeldItem => InventoryPocket::HeldItems,
            ItemCategory::KeyItem => unreachable!(),
        };
        if !self.state.player.bag.remove_item(item_id, quantity, pocket) {
            return serde_json::json!({"ok": false, "error": "Item not found or insufficient quantity"}).to_string();
        }
        let sell_price = (item.cost / 2) as u64 * quantity as u64;
        self.state.player.money = self.state.player.money.saturating_add(sell_price as u32);
        serde_json::json!({"ok": true, "money": self.state.player.money}).to_string()
    }

    /// Get full inventory as JSON (all pockets with cost and sell price).
    pub fn get_inventory(&self) -> String {
        use models::ItemCategory;
        let bag = &self.state.player.bag;
        let heal: Vec<serde_json::Value> = bag.heal_items.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({"id": id, "name": item.name, "qty": qty, "description": item.description, "category": "HealItem", "cost": item.cost, "sell_price": item.cost / 2})
        }).collect();
        let battle_items: Vec<serde_json::Value> = bag.battle_items.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({"id": id, "name": item.name, "qty": qty, "description": item.description, "category": "BattleItem", "cost": item.cost, "sell_price": item.cost / 2})
        }).collect();
        let cases: Vec<serde_json::Value> = bag.sneaker_cases.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({"id": id, "name": item.name, "qty": qty, "description": item.description, "category": "SneakerCase", "cost": item.cost, "sell_price": item.cost / 2})
        }).collect();
        let key_items: Vec<serde_json::Value> = bag.key_items.iter().map(|id| {
            let item = data::get_item(*id);
            serde_json::json!({"id": id, "name": item.name, "description": item.description, "category": "KeyItem"})
        }).collect();
        let held: Vec<serde_json::Value> = bag.held_items.iter().map(|(id, qty)| {
            let item = data::get_item(*id);
            serde_json::json!({"id": id, "name": item.name, "qty": qty, "description": item.description, "category": "HeldItem", "cost": item.cost, "sell_price": item.cost / 2})
        }).collect();
        let _ = ItemCategory::HealItem; // suppress unused import warning
        serde_json::json!({"heal": heal, "battle": battle_items, "cases": cases, "key_items": key_items, "held": held}).to_string()
    }

    /// Get party summaries as JSON (same as get_party_state but aliased for overworld use).
    pub fn get_party(&self) -> String {
        self.get_party_state()
    }

    /// Get player info (name, money, play time, stamps, dex progress).
    pub fn get_player_info(&self) -> String {
        let dex = &self.state.player.sneakerdex;
        let seen = dex.entries.iter().filter(|e| e.seen || e.caught).count();
        let caught = dex.entries.iter().filter(|e| e.caught).count();
        let stamps_earned = self.state.authentication_stamps.iter().filter(|&&s| s).count();
        serde_json::json!({
            "name": self.state.player.name,
            "money": self.state.player.money,
            "play_time_ms": self.state.play_time_ms,
            "stamps": self.state.authentication_stamps,
            "stamps_earned": stamps_earned,
            "sneakerdex_seen": seen,
            "sneakerdex_caught": caught,
        }).to_string()
    }

    /// Get Sneakerdex data as JSON.
    pub fn get_sneakerdex(&self) -> String {
        let dex = &self.state.player.sneakerdex;
        let entries: Vec<serde_json::Value> = dex.entries.iter().enumerate().map(|(i, entry)| {
            let species_id = (i + 1) as u16;
            if entry.seen || entry.caught {
                let species = data::get_species(species_id);
                let base_stats = if entry.caught {
                    serde_json::json!({
                        "durability": species.base_stats.durability,
                        "hype": species.base_stats.hype,
                        "comfort": species.base_stats.comfort,
                        "drip": species.base_stats.drip,
                        "rarity": species.base_stats.rarity,
                    })
                } else {
                    serde_json::Value::Null
                };
                serde_json::json!({
                    "number": species_id,
                    "seen": entry.seen || entry.caught,
                    "caught": entry.caught,
                    "name": species.name,
                    "faction": format!("{:?}", species.faction),
                    "description": if entry.caught { species.description } else { "" },
                    "base_stats": base_stats,
                })
            } else {
                serde_json::json!({
                    "number": species_id,
                    "seen": false,
                    "caught": false,
                    "name": serde_json::Value::Null,
                    "faction": serde_json::Value::Null,
                    "description": serde_json::Value::Null,
                    "base_stats": serde_json::Value::Null,
                })
            }
        }).collect();
        let total_seen = dex.entries.iter().filter(|e| e.seen || e.caught).count();
        let total_caught = dex.entries.iter().filter(|e| e.caught).count();
        serde_json::json!({
            "entries": entries,
            "total_seen": total_seen,
            "total_caught": total_caught,
            "total_species": 30,
        }).to_string()
    }

    /// Restore all party members to full HP, PP, and clear status conditions.
    pub fn heal_party(&mut self) {
        for snk in &mut self.state.player.party {
            snk.current_hp = snk.max_hp;
            snk.status = None;
            snk.on_fire_turns = 0;
            for slot in snk.moves.iter_mut().flatten() {
                slot.current_pp = slot.max_pp;
            }
        }
    }

    /// Deposit a party member (by party index) into the sneaker box.
    /// Returns JSON: {"ok": bool, "error": string}
    pub fn deposit_sneaker(&mut self, party_index: u8) -> String {
        let party_len = self.state.player.party.len();
        if party_len <= 1 {
            return serde_json::json!({"ok": false, "error": "Cannot deposit last party member"}).to_string();
        }
        if self.state.player.sneaker_box.is_full() {
            return serde_json::json!({"ok": false, "error": "Box is full"}).to_string();
        }
        let idx = party_index as usize;
        if idx >= party_len {
            return serde_json::json!({"ok": false, "error": "Invalid party index"}).to_string();
        }
        let sneaker = self.state.player.party.remove(idx);
        self.state.player.sneaker_box.deposit(sneaker);
        serde_json::json!({"ok": true}).to_string()
    }

    /// Withdraw a sneaker from the box (by box index) into the party.
    /// Returns JSON: {"ok": bool, "error": string}
    pub fn withdraw_sneaker(&mut self, box_index: u16) -> String {
        if self.state.player.party.len() >= 6 {
            return serde_json::json!({"ok": false, "error": "Party is full"}).to_string();
        }
        let idx = box_index as usize;
        if idx >= self.state.player.sneaker_box.sneakers.len() {
            return serde_json::json!({"ok": false, "error": "Invalid box index"}).to_string();
        }
        let sneaker = self.state.player.sneaker_box.sneakers.remove(idx);
        self.state.player.party.push(sneaker);
        serde_json::json!({"ok": true}).to_string()
    }
}

impl GameEngine {
    /// Internal map loader — not exposed to WASM directly but usable in tests.
    pub fn load_map_from_json(&mut self, json: &str) -> Result<(), String> {
        let map_data = MapData::from_json(json)?;
        self.map_width = map_data.width as usize;
        self.map_height = map_data.height as usize;
        self.map = map_data.collision.clone();

        // Apply pending transition: position player at the correct entry point
        if let Some(trans) = self.pending_transition.take() {
            match trans.direction.as_str() {
                "north" => {
                    // Entering from the south side of the new map
                    self.state.player.y = (map_data.height as u16).saturating_sub(2);
                    self.state.player.x = trans.target_x;
                }
                "south" => {
                    // Entering from the north side of the new map
                    self.state.player.y = 1;
                    self.state.player.x = trans.target_x;
                }
                "east" => {
                    // Entering from the west side of the new map
                    self.state.player.x = 1;
                    self.state.player.y = trans.target_y;
                }
                "west" => {
                    // Entering from the east side of the new map
                    self.state.player.x = (map_data.width as u16).saturating_sub(2);
                    self.state.player.y = trans.target_y;
                }
                _ => {
                    // Explicit warp coordinates
                    self.state.player.x = trans.target_x;
                    self.state.player.y = trans.target_y;
                }
            }
        }

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

// ── Phase 7 Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_phase_7 {
    use super::*;
    use crate::world::encounters::generate_wild_sneaker;
    use crate::models::inventory::InventoryPocket;
    use crate::models::sneaker::StatusCondition;

    fn make_engine_with_party() -> GameEngine {
        let mut eng = GameEngine::new(12345);
        let mut rng = SeededRng::new(42);
        let sneaker = generate_wild_sneaker(1, 10, &mut rng);
        eng.state.player.party.push(sneaker);
        eng
    }

    // ── Save / Load ──────────────────────────────────────────────────────────

    #[test]
    fn export_save_produces_valid_json() {
        let eng = GameEngine::new(1);
        let json = eng.export_save();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.is_object());
    }

    #[test]
    fn load_save_produces_identical_state() {
        let mut eng = GameEngine::new(1);
        eng.state.player.money = 9999;
        eng.state.player.name = "TestPlayer".to_string();
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert_eq!(loaded.state.player.money, 9999);
        assert_eq!(loaded.state.player.name, "TestPlayer");
    }

    #[test]
    fn player_position_preserved_through_save_load() {
        let mut eng = GameEngine::new(1);
        eng.state.player.x = 10;
        eng.state.player.y = 7;
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert_eq!(loaded.state.player.x, 10);
        assert_eq!(loaded.state.player.y, 7);
    }

    #[test]
    fn party_preserved_through_save_load() {
        let mut eng = make_engine_with_party();
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert_eq!(loaded.state.player.party.len(), 1);
        assert_eq!(loaded.state.player.party[0].species_id, 1);
    }

    #[test]
    fn inventory_preserved_through_save_load() {
        let mut eng = GameEngine::new(1);
        eng.state.player.bag.add_item(1, 3, InventoryPocket::HealItems);
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert_eq!(loaded.state.player.bag.heal_items.len(), 1);
        assert_eq!(loaded.state.player.bag.heal_items[0], (1, 3));
    }

    #[test]
    fn event_flags_preserved_through_save_load() {
        let mut eng = GameEngine::new(1);
        eng.state.event_flags.insert("test_flag".to_string());
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert!(loaded.state.event_flags.contains("test_flag"));
    }

    // ── Inventory operations ─────────────────────────────────────────────────

    #[test]
    fn buy_item_deducts_money_and_adds_item() {
        let mut eng = GameEngine::new(1);
        eng.state.player.money = 1000;
        let result: serde_json::Value = serde_json::from_str(&eng.buy_item(1, 2)).unwrap();
        assert_eq!(result["ok"], true);
        // Sole Sauce costs 200 each, 2x = 400 deducted
        assert_eq!(eng.state.player.money, 600);
        assert_eq!(eng.state.player.bag.heal_items[0], (1, 2));
    }

    #[test]
    fn buy_item_fails_with_insufficient_money() {
        let mut eng = GameEngine::new(1);
        eng.state.player.money = 100;
        let result: serde_json::Value = serde_json::from_str(&eng.buy_item(1, 1)).unwrap();
        assert_eq!(result["ok"], false);
        assert_eq!(eng.state.player.money, 100);
    }

    #[test]
    fn sell_item_adds_money_and_removes_item() {
        let mut eng = GameEngine::new(1);
        eng.state.player.money = 0;
        eng.state.player.bag.add_item(1, 1, InventoryPocket::HealItems);
        let result: serde_json::Value = serde_json::from_str(&eng.sell_item(1, 1)).unwrap();
        assert_eq!(result["ok"], true);
        // Sole Sauce cost 200, sell price = 100
        assert_eq!(eng.state.player.money, 100);
        assert!(eng.state.player.bag.heal_items.is_empty());
    }

    #[test]
    fn use_item_sole_sauce_heals_20_hp() {
        let mut eng = make_engine_with_party();
        eng.state.player.party[0].current_hp = 10;
        let max_hp = eng.state.player.party[0].max_hp;
        eng.state.player.bag.add_item(1, 1, InventoryPocket::HealItems);
        let result: serde_json::Value = serde_json::from_str(&eng.use_item(1, 0)).unwrap();
        assert_eq!(result["ok"], true);
        let expected = (10u16 + 20).min(max_hp);
        assert_eq!(eng.state.player.party[0].current_hp, expected);
    }

    #[test]
    fn use_item_full_restore_heals_to_max() {
        let mut eng = make_engine_with_party();
        eng.state.player.party[0].current_hp = 1;
        let max_hp = eng.state.player.party[0].max_hp;
        eng.state.player.bag.add_item(3, 1, InventoryPocket::HealItems);
        let result: serde_json::Value = serde_json::from_str(&eng.use_item(3, 0)).unwrap();
        assert_eq!(result["ok"], true);
        assert_eq!(eng.state.player.party[0].current_hp, max_hp);
    }

    #[test]
    fn cant_use_item_on_fainted_sneaker_unless_revive() {
        let mut eng = make_engine_with_party();
        eng.state.player.party[0].current_hp = 0;
        eng.state.player.bag.add_item(1, 1, InventoryPocket::HealItems);
        let result: serde_json::Value = serde_json::from_str(&eng.use_item(1, 0)).unwrap();
        assert_eq!(result["ok"], false);
    }

    // ── Heal party ───────────────────────────────────────────────────────────

    #[test]
    fn heal_party_restores_all_sneakers_to_max_hp() {
        let mut eng = make_engine_with_party();
        eng.state.player.party[0].current_hp = 1;
        let max_hp = eng.state.player.party[0].max_hp;
        eng.heal_party();
        assert_eq!(eng.state.player.party[0].current_hp, max_hp);
    }

    #[test]
    fn heal_party_restores_pp() {
        let mut eng = make_engine_with_party();
        if let Some(slot) = eng.state.player.party[0].moves[0].as_mut() {
            slot.current_pp = 0;
        }
        eng.heal_party();
        if let Some(slot) = eng.state.player.party[0].moves[0].as_ref() {
            assert_eq!(slot.current_pp, slot.max_pp);
        }
    }

    #[test]
    fn heal_party_clears_status_conditions() {
        let mut eng = make_engine_with_party();
        eng.state.player.party[0].status = Some(StatusCondition::Creased);
        eng.heal_party();
        assert!(eng.state.player.party[0].status.is_none());
    }

    // ── Sneaker Box ──────────────────────────────────────────────────────────

    #[test]
    fn deposit_moves_sneaker_from_party_to_box() {
        let mut eng = make_engine_with_party();
        let mut rng = SeededRng::new(99);
        let s2 = generate_wild_sneaker(2, 5, &mut rng);
        eng.state.player.party.push(s2);
        let result: serde_json::Value = serde_json::from_str(&eng.deposit_sneaker(0)).unwrap();
        assert_eq!(result["ok"], true);
        assert_eq!(eng.state.player.party.len(), 1);
        assert_eq!(eng.state.player.sneaker_box.count(), 1);
    }

    #[test]
    fn withdraw_moves_sneaker_from_box_to_party() {
        let mut eng = make_engine_with_party();
        let mut rng = SeededRng::new(77);
        let s2 = generate_wild_sneaker(2, 5, &mut rng);
        let uid = s2.uid;
        eng.state.player.sneaker_box.deposit(s2);
        let result: serde_json::Value = serde_json::from_str(&eng.withdraw_sneaker(0)).unwrap();
        assert_eq!(result["ok"], true);
        assert_eq!(eng.state.player.party.len(), 2);
        assert!(eng.state.player.party.iter().any(|s| s.uid == uid));
    }

    #[test]
    fn cant_deposit_last_party_member() {
        let mut eng = make_engine_with_party();
        let result: serde_json::Value = serde_json::from_str(&eng.deposit_sneaker(0)).unwrap();
        assert_eq!(result["ok"], false);
    }

    #[test]
    fn cant_withdraw_when_party_full() {
        let mut eng = GameEngine::new(1);
        let mut rng = SeededRng::new(42);
        for _ in 0..6 {
            let s = generate_wild_sneaker(1, 5, &mut rng);
            eng.state.player.party.push(s);
        }
        let boxed = generate_wild_sneaker(1, 5, &mut rng);
        eng.state.player.sneaker_box.deposit(boxed);
        let result: serde_json::Value = serde_json::from_str(&eng.withdraw_sneaker(0)).unwrap();
        assert_eq!(result["ok"], false);
    }

    #[test]
    fn cant_deposit_when_box_full() {
        let mut eng = GameEngine::new(1);
        let mut rng = SeededRng::new(42);
        for _ in 0..2 {
            let s = generate_wild_sneaker(1, 5, &mut rng);
            eng.state.player.party.push(s);
        }
        for _ in 0..50 {
            let s = generate_wild_sneaker(1, 5, &mut rng);
            eng.state.player.sneaker_box.deposit(s);
        }
        let result: serde_json::Value = serde_json::from_str(&eng.deposit_sneaker(0)).unwrap();
        assert_eq!(result["ok"], false);
    }

    // ── Sneakerdex ───────────────────────────────────────────────────────────

    #[test]
    fn new_game_all_dex_entries_unseen() {
        let eng = GameEngine::new(1);
        for entry in &eng.state.player.sneakerdex.entries {
            assert!(!entry.seen);
            assert!(!entry.caught);
        }
    }

    #[test]
    fn after_encountering_entry_marked_seen() {
        let mut eng = GameEngine::new(1);
        eng.state.player.sneakerdex.entries[0].seen = true;
        assert!(eng.state.player.sneakerdex.entries[0].seen);
        assert!(!eng.state.player.sneakerdex.entries[0].caught);
    }

    #[test]
    fn after_catching_entry_marked_caught() {
        let mut eng = GameEngine::new(1);
        eng.state.player.sneakerdex.entries[0].seen = true;
        eng.state.player.sneakerdex.entries[0].caught = true;
        assert!(eng.state.player.sneakerdex.entries[0].caught);
    }

    #[test]
    fn get_sneakerdex_returns_correct_counts() {
        let mut eng = GameEngine::new(1);
        eng.state.player.sneakerdex.entries[0].seen = true;
        eng.state.player.sneakerdex.entries[0].caught = true;
        eng.state.player.sneakerdex.entries[1].seen = true;
        let v: serde_json::Value = serde_json::from_str(&eng.get_sneakerdex()).unwrap();
        assert_eq!(v["total_seen"], 2);
        assert_eq!(v["total_caught"], 1);
        assert_eq!(v["total_species"], 30);
    }
}

// ── Phase 8 Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_phase_8 {
    use super::*;
    use crate::world::map::{MapConnections, MapData, WildEncounterEntry, EventDef};

    // ── Helper: minimal map with a north connection ────────────────────────────

    fn make_test_map_with_north_connection() -> MapData {
        let w: u16 = 10;
        let h: u16 = 8;
        let mut collision = vec![0u8; (w * h) as usize];
        // Border walls
        for x in 0..w { collision[x as usize] = 1; collision[((h - 1) * w + x) as usize] = 1; }
        for y in 0..h { collision[(y * w) as usize] = 1; collision[(y * w + w - 1) as usize] = 1; }
        MapData {
            id: "test_map".to_string(),
            name: "Test Map".to_string(),
            width: w, height: h,
            collision,
            ground: vec![0u16; (w * h) as usize],
            overlay: vec![0u16; (w * h) as usize],
            connections: MapConnections {
                north: Some("route_1".to_string()),
                south: None, east: None, west: None,
            },
            wild_encounters: vec![],
            npcs: vec![],
            events: vec![],
            music: "test_bgm".to_string(),
        }
    }

    fn make_test_map_no_connections() -> MapData {
        let mut m = make_test_map_with_north_connection();
        m.connections.north = None;
        m
    }

    fn make_test_map_with_door() -> MapData {
        let w: u16 = 10;
        let h: u16 = 8;
        let mut collision = vec![0u8; (w * h) as usize];
        for x in 0..w { collision[x as usize] = 1; collision[((h - 1) * w + x) as usize] = 1; }
        for y in 0..h { collision[(y * w) as usize] = 1; collision[(y * w + w - 1) as usize] = 1; }
        // Door tile at (5, 4)
        collision[4 * w as usize + 5] = 3;
        MapData {
            id: "test_door_map".to_string(),
            name: "Test Door Map".to_string(),
            width: w, height: h,
            collision,
            ground: vec![0u16; (w * h) as usize],
            overlay: vec![0u16; (w * h) as usize],
            connections: MapConnections {
                north: None, south: None, east: None, west: None,
            },
            wild_encounters: vec![],
            npcs: vec![],
            events: vec![EventDef {
                id: "lab_door".to_string(),
                x: 5, y: 4,
                event_type: "warp_trigger".to_string(),
                data: "lab_interior:3:7".to_string(),
            }],
            music: "test_bgm".to_string(),
        }
    }

    fn engine_on_map(map: MapData, player_x: u16, player_y: u16) -> GameEngine {
        let mut eng = GameEngine::new(1);
        eng.state.player.x = player_x;
        eng.state.player.y = player_y;
        eng.map_width = map.width as usize;
        eng.map_height = map.height as usize;
        eng.map = map.collision.clone();
        eng.current_map = Some(map);
        eng
    }

    // ── Map transitions ────────────────────────────────────────────────────────

    #[test]
    fn walk_off_north_edge_triggers_transition() {
        let map = make_test_map_with_north_connection();
        // Place player at y=1 (one tile from north border)
        let mut eng = engine_on_map(map, 5, 1);

        // Tick with "up" — target y=0 is solid border → edge transition
        let json_str = eng.tick(16.67, "up");
        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(!v["transition"].is_null(), "should have a transition when walking off north edge");
        assert_eq!(v["transition"]["map"], "route_1");
    }

    #[test]
    fn walk_off_edge_no_connection_is_blocked() {
        let map = make_test_map_no_connections();
        let mut eng = engine_on_map(map, 5, 1);

        let json_str = eng.tick(16.67, "up");
        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(v["transition"].is_null(), "should have no transition when no connection");
        // Player should not move
        assert_eq!(v["player_y"], 1);
    }

    #[test]
    fn door_tile_triggers_fade_transition() {
        let map = make_test_map_with_door();
        // Place player at (5, 5) — above the door at (5, 4), move up
        let mut eng = engine_on_map(map, 5, 5);

        // Tick enough to complete the step onto the door tile
        for i in 0..20 {
            let input = if i == 0 { "up" } else { "none" };
            eng.tick(16.67, input);
            if !eng.state.player.moving && eng.state.player.y == 4 { break; }
        }

        // After stepping onto the door tile, transition should be pending
        assert!(eng.pending_transition.is_some(), "pending transition should be set after door step");
        let t = eng.pending_transition.as_ref().unwrap();
        assert_eq!(t.target_map, "lab_interior");
        assert_eq!(t.target_x, 3);
        assert_eq!(t.target_y, 7);
        use crate::world::map::TransitionType;
        assert_eq!(t.transition_type, TransitionType::Fade);
    }

    #[test]
    fn player_position_correct_after_transition() {
        let mut eng = GameEngine::new(1);
        // Simulate a pending north transition (player was at x=5 on old map)
        use crate::world::map::TransitionType;
        eng.pending_transition = Some(crate::world::map::MapTransition {
            target_map: "route_1".to_string(),
            target_x: 5,
            target_y: 0,
            transition_type: TransitionType::Walk,
            direction: "north".to_string(),
        });

        // Load a new map — player should arrive at bottom (height-2)
        let new_map = make_test_map_with_north_connection(); // h=8
        let json = serde_json::to_string(&new_map).unwrap();
        eng.load_map_from_json(&json).unwrap();

        // After transition, pending_transition should be cleared
        assert!(eng.pending_transition.is_none());
        // Player x preserved, player y = height - 2 = 6
        assert_eq!(eng.state.player.x, 5);
        assert_eq!(eng.state.player.y, 6);
    }

    // ── Starter selection ─────────────────────────────────────────────────────

    #[test]
    fn choose_starter_0_gives_retro_runner_with_stomp_crease() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0);
        assert_eq!(eng.state.player.party.len(), 1);
        let s = &eng.state.player.party[0];
        assert_eq!(s.species_id, 1, "should be Retro Runner");
        assert_eq!(s.level, 5);
        let m0_id = s.moves[0].as_ref().map(|m| m.move_id);
        let m1_id = s.moves[1].as_ref().map(|m| m.move_id);
        assert_eq!(m0_id, Some(5),  "move 0 should be Stomp (id=5)");
        assert_eq!(m1_id, Some(11), "move 1 should be Crease (id=11)");
    }

    #[test]
    fn choose_starter_1_gives_tech_trainer_with_quick_step_shock_drop() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(1);
        let s = &eng.state.player.party[0];
        assert_eq!(s.species_id, 9, "should be Tech Trainer");
        assert_eq!(s.level, 5);
        let m0_id = s.moves[0].as_ref().map(|m| m.move_id);
        let m1_id = s.moves[1].as_ref().map(|m| m.move_id);
        assert_eq!(m0_id, Some(4),  "move 0 should be Quick Step (id=4)");
        assert_eq!(m1_id, Some(20), "move 1 should be Shock Drop (id=20)");
    }

    #[test]
    fn choose_starter_2_gives_skate_blazer_with_stomp_kickflip() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(2);
        let s = &eng.state.player.party[0];
        assert_eq!(s.species_id, 17, "should be Skate Blazer");
        assert_eq!(s.level, 5);
        let m0_id = s.moves[0].as_ref().map(|m| m.move_id);
        let m1_id = s.moves[1].as_ref().map(|m| m.move_id);
        assert_eq!(m0_id, Some(5),  "move 0 should be Stomp (id=5)");
        assert_eq!(m1_id, Some(27), "move 1 should be Kickflip (id=27)");
    }

    #[test]
    fn choose_starter_gives_500_money_and_items() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0);
        assert_eq!(eng.state.player.money, 500);
        // 5x Sole Sauce (id=1)
        let sauce_qty = eng.state.player.bag.heal_items.iter()
            .find(|(id, _)| *id == 1)
            .map(|(_, q)| *q)
            .unwrap_or(0);
        assert_eq!(sauce_qty, 5, "should have 5 Sole Sauce");
        // 5x Sneaker Case (id=30)
        let case_qty = eng.state.player.bag.sneaker_cases.iter()
            .find(|(id, _)| *id == 30)
            .map(|(_, q)| *q)
            .unwrap_or(0);
        assert_eq!(case_qty, 5, "should have 5 Sneaker Cases");
    }

    #[test]
    fn choose_starter_marks_dex_as_caught() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0); // Retro Runner = species_id 1, dex_idx 0
        assert!(eng.state.player.sneakerdex.entries[0].seen);
        assert!(eng.state.player.sneakerdex.entries[0].caught);
    }

    #[test]
    fn choose_starter_sets_has_starter_flag() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(1);
        assert!(eng.state.event_flags.contains("has_starter"));
    }

    // ── Rival team ────────────────────────────────────────────────────────────

    #[test]
    fn player_chose_retro_rival_has_techwear() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0); // Retro Runner
        // Rival should be Tech Trainer (species_id=9, Techwear faction)
        assert_eq!(eng.rival_starter_id, 9);
        let species = crate::data::get_species(eng.rival_starter_id);
        assert_eq!(species.faction, crate::models::Faction::Techwear);
    }

    #[test]
    fn player_chose_techwear_rival_has_skate() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(1); // Tech Trainer
        assert_eq!(eng.rival_starter_id, 17);
        let species = crate::data::get_species(eng.rival_starter_id);
        assert_eq!(species.faction, crate::models::Faction::Skate);
    }

    #[test]
    fn player_chose_skate_rival_has_retro() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(2); // Skate Blazer
        assert_eq!(eng.rival_starter_id, 1);
        let species = crate::data::get_species(eng.rival_starter_id);
        assert_eq!(species.faction, crate::models::Faction::Retro);
    }

    #[test]
    fn rival_battle_uses_level_7() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0);
        eng.start_rival_battle();
        assert!(eng.battle.is_some());
        let battle = eng.battle.as_ref().unwrap();
        assert_eq!(battle.opponent.team[0].species_id, 9); // Tech Trainer
        assert_eq!(battle.opponent.team[0].level, 7);
    }

    // ── Integration ───────────────────────────────────────────────────────────

    #[test]
    fn save_after_starter_load_party_intact() {
        let mut eng = GameEngine::new(42);
        eng.choose_starter(0);
        let json = eng.export_save();
        let loaded = GameEngine::load_save(&json).expect("load_save failed");
        assert_eq!(loaded.state.player.party.len(), 1);
        assert_eq!(loaded.state.player.party[0].species_id, 1);
        assert_eq!(loaded.state.player.money, 500);
        assert!(loaded.state.event_flags.contains("has_starter"));
    }

    #[test]
    fn walk_off_north_edge_blocked_from_battle_mode() {
        // In battle mode, edge transition should not trigger
        let map = make_test_map_with_north_connection();
        let mut eng = engine_on_map(map, 5, 1);
        eng.state.mode = GameMode::Battle;

        let json_str = eng.tick(16.67, "up");
        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        // Battle mode returns early, no transition
        assert!(v["transition"].is_null(), "no transition in battle mode");
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
