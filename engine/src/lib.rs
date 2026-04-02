use wasm_bindgen::prelude::*;

pub mod models;
pub mod util;
pub mod data;
pub mod state;
pub mod world;

// Re-export commonly used types
pub use models::{Faction, Stats};
pub use state::{GameState, GameMode};
pub use util::rng::SeededRng;

use state::player::Direction;
use world::map::MapData;
use world::movement::{parse_input, process_movement, GameEvent};

// ── Tile types ──

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;

// ── Game Engine — single source of truth ──

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
    rng: SeededRng,
    map: Vec<u8>, // current collision data (updated by load_map_data)
    map_width: usize,
    map_height: usize,
    current_map: Option<MapData>, // loaded map metadata (encounters, connections, etc.)
    step_count: u32,
    encounter_triggered: bool,
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
                GameEvent::WildEncounter { .. } => {
                    self.encounter_triggered = true;
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
