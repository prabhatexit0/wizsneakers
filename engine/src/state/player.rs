use serde::{Deserialize, Serialize};
use crate::models::sneaker::SneakerInstance;
use crate::models::inventory::{Inventory, SneakerBox};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns (dx, dy) delta for this direction
    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DexEntry {
    pub seen: bool,
    pub caught: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SneakerdexData {
    pub entries: Vec<DexEntry>,
}

impl SneakerdexData {
    pub fn new(species_count: usize) -> Self {
        Self {
            entries: (0..species_count)
                .map(|_| DexEntry { seen: false, caught: false })
                .collect(),
        }
    }
}

impl Default for SneakerdexData {
    fn default() -> Self {
        // 30 species defined in PRD 02
        Self::new(30)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub facing: Direction,
    pub party: Vec<SneakerInstance>,
    pub sneaker_box: SneakerBox,
    pub bag: Inventory,
    pub money: u32,
    pub sneakerdex: SneakerdexData,
    pub moving: bool,
    pub move_progress: f32,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            name: String::from("Player"),
            x: 3,
            y: 3,
            facing: Direction::Down,
            party: Vec::new(),
            sneaker_box: SneakerBox::default(),
            bag: Inventory::default(),
            money: 0,
            sneakerdex: SneakerdexData::default(),
            moving: false,
            move_progress: 0.0,
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}
