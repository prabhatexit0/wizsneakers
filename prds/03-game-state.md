# PRD 03 — Game State & Refactored WASM API (Phase 1C)

## Goal
Replace the ad-hoc state in `GameEngine` with the proper `GameState`/`PlayerState` architecture. Integrate SeededRng. Refactor the WASM constructor to accept a seed. The client must still work identically.

## Dependencies
- PRD 01 (models), PRD 02 (data)

## Deliverables

### Files to Create

**`engine/src/state/mod.rs`** — re-exports

**`engine/src/state/game_state.rs`**
- `GameMode` enum: `Overworld, Battle, Dialogue, Menu, Cutscene`
- `GameState` struct:
  ```rust
  pub struct GameState {
      pub mode: GameMode,
      pub player: PlayerState,
      pub current_map: u16,
      pub event_flags: HashSet<String>,
      pub story_progress: u8,
      pub play_time_ms: u64,
      pub authentication_stamps: [bool; 8],
  }
  ```
- Derive `Serialize, Deserialize` for save/load later

**`engine/src/state/player.rs`**
- `PlayerState` struct:
  ```rust
  pub struct PlayerState {
      pub name: String,
      pub x: u16,
      pub y: u16,
      pub facing: Direction,
      pub party: Vec<SneakerInstance>,  // up to 6
      pub sneaker_box: SneakerBox,
      pub bag: Inventory,
      pub money: u32,
      pub sneakerdex: SneakerdexData,
      pub moving: bool,
      pub move_progress: f32,
  }
  ```
- `Direction` enum: `Up, Down, Left, Right` (with delta methods)
- `SneakerdexData`: `entries: Vec<DexEntry>` where `DexEntry` has `seen: bool, caught: bool`

**`engine/src/state/flags.rs`**
- Functions on `GameState`:
  - `set_flag(&mut self, flag: &str)`
  - `has_flag(&self, flag: &str) -> bool`
  - `clear_flag(&mut self, flag: &str)`

### Files to Modify

**`engine/src/lib.rs`**
- Add `mod state;`
- Refactor `GameEngine`:
  ```rust
  #[wasm_bindgen]
  pub struct GameEngine {
      state: GameState,
      rng: SeededRng,
      // map data stored here temporarily (moved to world module in PRD 04)
      map: Vec<u8>,
      map_width: usize,
      map_height: usize,
  }
  ```
- Constructor: `pub fn new(seed: u64) -> Self` — initialize SeededRng with seed, create default GameState with player at (3,3), build the same hardcoded map as before
- `state_json()` — serialize from GameState fields
- `tick()` — use `self.rng` for encounter checks instead of the `wrapping_mul` hack
- Keep all existing getter methods working (`player_x`, `player_y`, `get_tile`, etc.)

**`client/src/hooks/useWasm.ts`**
- Change `new GameEngine()` to `new GameEngine(BigInt(Date.now()))` to pass seed

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_1c {
    // GameState initialization
    - New GameState has mode = Overworld
    - Player starts at expected position
    - Party is empty initially
    - Money is 0
    - No stamps earned
    - story_progress is 0

    // Event flags
    - set_flag then has_flag returns true
    - has_flag on unset returns false
    - clear_flag removes the flag

    // Direction
    - Up delta = (0, -1)
    - Down delta = (0, 1)
    - Left delta = (-1, 0)
    - Right delta = (1, 0)

    // SeededRng integration
    - GameEngine with same seed produces same encounter sequence
    - GameEngine with different seed produces different sequence

    // Backward compatibility
    - GameEngine::new(seed) returns valid engine
    - tick() still works with direction as u8
    - player_x/player_y return correct positions after movement
}
```

## Verification
```bash
cd engine && cargo test tests_phase_1c && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] `GameEngine` uses `GameState` internally
- [ ] SeededRng used for all randomness
- [ ] Constructor takes `seed: u64`
- [ ] Client passes `BigInt(Date.now())` as seed
- [ ] Game plays identically to before (no visible change)
- [ ] `./verify.sh` exits 0
