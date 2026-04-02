use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapConnections {
    pub north: Option<String>,
    pub south: Option<String>,
    pub east: Option<String>,
    pub west: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WildEncounterEntry {
    pub species_id: u16,
    pub level_min: u8,
    pub level_max: u8,
    pub weight: u32,
}

/// NPC movement pattern — used in both map definition (JSON) and runtime state
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NpcMovement {
    Stationary,
    RandomWalk { radius: u8 },
    Patrol { path: Vec<(u16, u16)> },
    FacePlayer,
}

/// NPC definition as loaded from map JSON
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NpcDef {
    pub id: String,
    pub x: u16,
    pub y: u16,
    #[serde(default = "default_facing")]
    pub facing: String,
    #[serde(default)]
    pub sprite: String,
    #[serde(default = "default_movement")]
    pub movement: NpcMovement,
    #[serde(default)]
    pub dialogue_id: String,
    #[serde(default)]
    pub is_trainer: bool,
    #[serde(default)]
    pub sight_range: u8,
    #[serde(default)]
    pub trainer_id: u16,
    #[serde(default)]
    pub defeated_flag: Option<String>,
}

fn default_facing() -> String {
    "down".to_string()
}

fn default_movement() -> NpcMovement {
    NpcMovement::Stationary
}

/// Map event definition as loaded from map JSON
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventDef {
    pub id: String,
    pub x: u16,
    pub y: u16,
    /// Type: "sign", "shop", "heal", "sneaker_box", "warp_trigger", etc.
    #[serde(default)]
    pub event_type: String,
    /// Event data: sign text, shop id, target map, etc.
    #[serde(default)]
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapData {
    pub id: String,
    pub name: String,
    pub width: u16,
    pub height: u16,
    /// Flat array (row-major): 0=walkable, 1=solid, 2=tall_grass, 3=door, 4=warp
    pub collision: Vec<u8>,
    /// Tile IDs for ground layer rendering
    pub ground: Vec<u16>,
    /// Tile IDs drawn over the player
    pub overlay: Vec<u16>,
    pub connections: MapConnections,
    pub wild_encounters: Vec<WildEncounterEntry>,
    pub npcs: Vec<NpcDef>,
    pub events: Vec<EventDef>,
    pub music: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Walkable,
    Solid,
    TallGrass,
    Door,
    Warp,
}

impl MapData {
    pub fn is_walkable(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let tile = self.collision[y as usize * self.width as usize + x as usize];
        // Solid (1) is the only impassable type
        tile != 1
    }

    pub fn tile_type_at(&self, x: u16, y: u16) -> TileType {
        if x >= self.width || y >= self.height {
            return TileType::Solid;
        }
        match self.collision[y as usize * self.width as usize + x as usize] {
            0 => TileType::Walkable,
            1 => TileType::Solid,
            2 => TileType::TallGrass,
            3 => TileType::Door,
            4 => TileType::Warp,
            _ => TileType::Walkable,
        }
    }

    pub fn from_json(json: &str) -> Result<MapData, String> {
        serde_json::from_str(json).map_err(|e| format!("Failed to parse map JSON: {}", e))
    }
}
