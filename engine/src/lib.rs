use wasm_bindgen::prelude::*;

pub mod models;
pub mod util;
pub mod data;
pub mod state;

// Re-export commonly used types
pub use models::{Faction, Stats};
pub use state::{GameState, GameMode};
pub use util::rng::SeededRng;

use state::player::Direction;

// ── Tile types ──

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;

// ── Game Engine — single source of truth ──

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
    rng: SeededRng,
    map: Vec<u8>, // 0=floor, 1=wall, 2=tall_grass
    map_width: usize,
    map_height: usize,
    // Transient per-tick state (not persisted to GameState yet — PRD 04+)
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
            step_count: 0,
            encounter_triggered: false,
        }
    }

    /// Process one game tick. direction: 0=none, 1=up, 2=down, 3=left, 4=right
    pub fn tick(&mut self, direction: u8) {
        self.encounter_triggered = false;

        let opt_dir = match direction {
            1 => Some(Direction::Up),
            2 => Some(Direction::Down),
            3 => Some(Direction::Left),
            4 => Some(Direction::Right),
            _ => None,
        };

        let dir = match opt_dir {
            Some(d) => d,
            None => return,
        };

        let (dx, dy) = dir.delta();
        self.state.player.facing = dir;

        let nx = self.state.player.x as isize + dx;
        let ny = self.state.player.y as isize + dy;

        if nx < 0 || ny < 0 || nx >= self.map_width as isize || ny >= self.map_height as isize {
            return;
        }

        let (nx, ny) = (nx as usize, ny as usize);
        let tile = self.map[ny * self.map_width + nx];

        if tile == 1 {
            return;
        }

        self.state.player.x = nx as u16;
        self.state.player.y = ny as u16;
        self.step_count += 1;

        // Tall grass encounter check (~15% chance) using SeededRng
        if tile == 2 {
            if self.rng.chance(15) {
                self.encounter_triggered = true;
            }
        }
    }

    // ── Getters for JS ──

    pub fn player_x(&self) -> usize { self.state.player.x as usize }
    pub fn player_y(&self) -> usize { self.state.player.y as usize }
    pub fn map_width(&self) -> usize { self.map_width }
    pub fn map_height(&self) -> usize { self.map_height }
    pub fn step_count(&self) -> u32 { self.step_count }
    pub fn encounter_triggered(&self) -> bool { self.encounter_triggered }

    /// Get tile at position
    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        if x >= self.map_width || y >= self.map_height {
            return 1;
        }
        self.map[y * self.map_width + x]
    }

    /// Get state as JSON for UI overlays
    pub fn state_json(&self) -> String {
        serde_json::json!({
            "player_x": self.state.player.x,
            "player_y": self.state.player.y,
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
