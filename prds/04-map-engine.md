# PRD 04 — Map Engine & JSON Loading (Phase 2A)

## Goal
Replace the hardcoded map array with a JSON-based map system. Create the first two real maps (Boxfresh Town and Route 1). The engine loads map data from JSON strings passed from JS.

## Dependencies
- PRD 03 (GameState)

## Deliverables

### Files to Create

**`engine/src/world/mod.rs`** — re-exports

**`engine/src/world/map.rs`**
- `MapData` struct (Deserialize from JSON):
  ```rust
  pub struct MapData {
      pub id: String,
      pub name: String,
      pub width: u16,
      pub height: u16,
      pub collision: Vec<u8>,      // flat array: 0=walkable, 1=solid, 2=tall_grass, 3=door, 4=warp
      pub ground: Vec<u16>,        // tile IDs for rendering (flat array)
      pub overlay: Vec<u16>,       // tile IDs drawn over player
      pub connections: MapConnections,
      pub wild_encounters: Vec<WildEncounterEntry>,
      pub npcs: Vec<NpcDef>,       // stub for now
      pub events: Vec<EventDef>,   // stub for now
      pub music: String,
  }
  ```
- `MapConnections`: `north: Option<String>, south: Option<String>, east: Option<String>, west: Option<String>`
- `WildEncounterEntry`: `species_id: u16, level_min: u8, level_max: u8, weight: u32`
- `NpcDef` / `EventDef` — stub structs (filled in Phase 6)
- `TileType` enum: `Walkable, Solid, TallGrass, Door, Warp`
- Method: `is_walkable(&self, x: u16, y: u16) -> bool`
- Method: `tile_type_at(&self, x: u16, y: u16) -> TileType`
- Method: `from_json(json: &str) -> Result<MapData, String>`

**`engine/src/world/encounters.rs`**
- `check_wild_encounter(table: &[WildEncounterEntry], rng: &mut SeededRng) -> Option<(u16, u8)>`
  - 15% base chance (caller checks this before calling)
  - Weighted random selection from encounter table
  - Returns (species_id, level) where level is random within the entry's range
- `generate_wild_sneaker(species_id: u16, level: u8, rng: &mut SeededRng) -> SneakerInstance`
  - Random IVs (0-31 each), random Condition, appropriate moves for level from learnset

**`client/public/maps/boxfresh_town.json`**
- 30×20 tile map
- Collision layer: walls around border, building footprints, a few interior walls
- Some tall grass patches near the edges (for testing)
- Connection: north edge → "route_1"
- No wild encounters (town)
- Player start position: (5, 10)

**`client/public/maps/route_1.json`**
- 50×15 tile map
- Collision: path down the middle, walls/fences on sides, tall grass patches on both sides
- Connection: south → "boxfresh_town"
- Wild encounters:
  - Classic Dunk (ID 4), Lv.3-5, weight 40
  - Grip Tape (ID 20), Lv.3-5, weight 35
  - Foam Cell (ID 12), Lv.4-6, weight 15
  - Retro Runner (wild, ID 1), Lv.4-6, weight 10

### Files to Modify

**`engine/src/lib.rs`**
- Add `mod world;`
- Add WASM method: `pub fn load_map_data(&mut self, json: &str) -> Result<(), JsValue>`
  - Parses JSON into MapData, stores in engine
  - Updates map dimensions, collision data
- Refactor `tick()` to use MapData for collision instead of hardcoded `self.map`
- Refactor encounter check to use `self.current_map.wild_encounters` and `world::encounters`
- Keep `get_tile()` working — return collision byte from MapData
- Update `state_json()` to include `map_width` and `map_height` from loaded map
- On construction: still build the hardcoded map as fallback (if no JSON loaded yet)

**`client/src/hooks/useWasm.ts`**
- After `init()` and `new GameEngine(seed)`, fetch `/maps/boxfresh_town.json` and call `engine.load_map_data(json)`
- Store current map ID for later use

**`client/src/App.tsx`**
- Handle variable map sizes (no longer hardcoded 20×15)

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_2a {
    // Map parsing
    - Parse a minimal valid JSON map → MapData with correct dimensions
    - Invalid JSON → error
    - Missing fields → error

    // Collision
    - is_walkable on walkable tile → true
    - is_walkable on solid tile → false
    - is_walkable out of bounds → false
    - tile_type_at on tall grass → TallGrass

    // Encounters
    - With encounter table and 100% chance, always returns a result
    - Species and levels are within table ranges
    - Weighted selection: over 1000 rolls, most common entry appears most
    - generate_wild_sneaker produces valid instance with IVs in 0-31 range
    - Generated sneaker has appropriate moves for its level

    // Integration
    - Load map, move player into tall grass, verify encounter can trigger
    - Load map, move player into wall, verify position unchanged
}
```

## Verification
```bash
cd engine && cargo test tests_phase_2a && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Maps load from JSON
- [ ] Boxfresh Town and Route 1 JSON files exist and parse correctly
- [ ] Collision works against JSON map data
- [ ] Wild encounter generation produces valid sneakers with moves
- [ ] Client loads starting map on init
- [ ] `./verify.sh` exits 0
