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

/// Stub — will be expanded in Phase 6
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NpcDef {
    pub id: u16,
}

/// Stub — will be expanded in Phase 6
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventDef {
    pub id: u16,
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
