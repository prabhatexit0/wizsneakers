use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// ── Faction (type) system ──

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Normal,
    Retro,
    Techwear,
    Skate,
    HighFashion,
}

// ── Sneaker stats ──

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub durability: u32, // HP
    pub hype: u32,       // Attack
    pub comfort: u32,    // Defense
    pub drip: u32,       // Special Attack
    pub rarity: u32,     // Speed
}

// ── Sneaker species ──

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sneaker {
    pub id: u16,
    pub name: String,
    pub faction: Faction,
    pub base_stats: Stats,
    pub level: u32,
    pub current_hp: u32,
    pub max_hp: u32,
}

impl Sneaker {
    pub fn new(id: u16, name: &str, faction: Faction, base: Stats, level: u32) -> Self {
        let max_hp = ((2 * base.durability + 15) * level / 100) + level + 10;
        Self {
            id,
            name: name.to_string(),
            faction,
            base_stats: base,
            level,
            current_hp: max_hp,
            max_hp,
        }
    }
}

// ── Tile types ──

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;

// ── Game Engine — single source of truth ──

#[wasm_bindgen]
pub struct GameEngine {
    player_x: usize,
    player_y: usize,
    map: Vec<u8>, // 0=floor, 1=wall, 2=tall_grass
    map_width: usize,
    map_height: usize,
    step_count: u32,
    encounter_triggered: bool,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
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
            player_x: 3,
            player_y: 3,
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

        let (dx, dy): (isize, isize) = match direction {
            1 => (0, -1),  // up
            2 => (0, 1),   // down
            3 => (-1, 0),  // left
            4 => (1, 0),   // right
            _ => return,
        };

        let nx = self.player_x as isize + dx;
        let ny = self.player_y as isize + dy;

        if nx < 0 || ny < 0 || nx >= self.map_width as isize || ny >= self.map_height as isize {
            return;
        }

        let (nx, ny) = (nx as usize, ny as usize);
        let tile = self.map[ny * self.map_width + nx];

        // Wall collision
        if tile == 1 {
            return;
        }

        self.player_x = nx;
        self.player_y = ny;
        self.step_count += 1;

        // Tall grass encounter check (~15% chance)
        if tile == 2 {
            let pseudo_rand = (self.step_count.wrapping_mul(2654435761)) % 100;
            if pseudo_rand < 15 {
                self.encounter_triggered = true;
            }
        }
    }

    // ── Getters for JS ──

    pub fn player_x(&self) -> usize { self.player_x }
    pub fn player_y(&self) -> usize { self.player_y }
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
            "player_x": self.player_x,
            "player_y": self.player_y,
            "step_count": self.step_count,
            "encounter": self.encounter_triggered,
            "map_width": self.map_width,
            "map_height": self.map_height,
        })
        .to_string()
    }
}
