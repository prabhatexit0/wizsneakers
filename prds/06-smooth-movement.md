# PRD 06 — Smooth Movement & 60fps Game Loop (Phase 2C)

## Goal
Replace the 150ms tick-rate-limited movement with true 60fps rendering and smooth tile-to-tile interpolation. Movement feels fluid — the player slides between tiles over ~133ms (8 frames). The input system gets action/cancel/menu keys. The Rust tick now accepts delta time.

## Dependencies
- PRD 05 (camera system)

## Deliverables

### Files to Create

**`engine/src/world/movement.rs`**
- Constants: `WALK_FRAMES: f64 = 8.0` (8 frames at 60fps = 133ms), `SPRINT_FRAMES: f64 = 4.0`
- `InputAction` enum: `None, Up, Down, Left, Right, Action, Cancel, Menu`
- `fn parse_input(input: &str) -> InputAction` — maps "up"/"down"/"left"/"right"/"action"/"cancel"/"menu"/"none" to enum
- Movement logic on GameState/PlayerState:
  ```rust
  pub fn process_movement(&mut self, input: InputAction, dt_ms: f64, map: &MapData, rng: &mut SeededRng) -> Vec<GameEvent>
  ```
  - If player is `moving`: advance `move_progress` by `dt_ms / (1000.0 / 60.0) / WALK_FRAMES`
  - When `move_progress >= 1.0`: snap to target tile, set `moving = false`, call `on_step_complete()`
  - If player is NOT moving and direction input: set `facing`, check `can_move_to(target)`, if walkable start movement
  - Even if blocked, update `facing` direction
- `fn on_step_complete() -> Vec<GameEvent>` — check tile type at new position:
  - TallGrass → 15% encounter check → `GameEvent::WildEncounter(species_id, level)`
  - Door → `GameEvent::MapTransition(target_map, x, y)` (stubbed for now)
  - Warp → `GameEvent::Warp(target_map, x, y)` (stubbed for now)
- `GameEvent` enum: `WildEncounter { species_id: u16, level: u8 }, MapTransition { ... }, Warp { ... }, None`

### Files to Modify

**`engine/src/lib.rs`**
- Refactor `tick()` signature:
  ```rust
  pub fn tick(&mut self, dt_ms: f64, input: &str) -> String
  ```
  - Parse input string via `parse_input()`
  - Call `process_movement()` with delta time
  - Handle returned GameEvents (for now, just set encounter_triggered flag)
  - Return JSON with: `player_x, player_y, facing, moving, move_progress, map_width, map_height, encounter`
- Keep `player_x()`, `player_y()` getters working
- Add new getters: `player_facing() -> u8`, `player_moving() -> bool`, `player_move_progress() -> f32`
- Remove old u8-direction tick if no longer needed, OR keep backward compat

**`client/src/hooks/useInput.ts`**
- Expand to output string-based input: "up"/"down"/"left"/"right"/"action"/"cancel"/"menu"/"none"
- Add key mappings:
  - Action: Z, Enter, Space
  - Cancel: X, Backspace
  - Menu: Escape
  - Sprint: Shift (hold)
- Return a ref to current input string (polled each frame, not event-driven)
- Prevent key repeat (only register on first press, clear on release)

**`client/src/hooks/useGameLoop.ts`**
- Remove the 150ms tick rate limiter
- True 60fps rAF loop with delta time:
  ```typescript
  function frame(time: number) {
    const dt = lastTime ? time - lastTime : 16.67;
    lastTime = time;
    callbackRef.current(dt);
    rafId = requestAnimationFrame(frame);
  }
  ```
- Pass dt (in ms) to callback every frame

**`client/src/App.tsx`**
- Update game loop callback: call `engine.tick(dt, input)` with delta time and string input
- Parse returned JSON for player state including `move_progress` and `facing`
- Render player with interpolation:
  ```typescript
  // Player render position accounts for movement progress
  const dx = facing === 'right' ? 1 : facing === 'left' ? -1 : 0;
  const dy = facing === 'down' ? 1 : facing === 'up' ? -1 : 0;
  const renderX = (playerX + dx * moveProgress) * TILE_PX - camera.x;
  const renderY = (playerY + dy * moveProgress) * TILE_PX - camera.y;
  ```
  Wait — the player position from engine is the CURRENT tile (where they started moving from). During movement, interpolate toward the next tile.
- Update camera to account for move_progress (smooth camera tracking)
- Draw a facing indicator on the player square (small triangle showing direction)

**`client/src/rendering/camera.ts`**
- Update `calculateCamera()` to accept `moveProgress` and `facing` for smooth camera tracking during movement

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_2c {
    // Input parsing
    - parse_input("up") = InputAction::Up
    - parse_input("none") = InputAction::None
    - parse_input("action") = InputAction::Action
    - parse_input("garbage") = InputAction::None

    // Movement
    - Player at (5,5) facing right, input right: starts moving, move_progress > 0
    - After enough dt, player position changes to (6,5)
    - Player facing wall: facing changes but position unchanged
    - Player moving: additional directional input ignored until movement complete
    - Sprint mode: movement completes in fewer frames

    // Step events
    - Step onto tall grass with seeded RNG → deterministic encounter result
    - Step onto normal tile → no events
    - Step onto door tile → MapTransition event

    // Integration
    - Full movement cycle: start at (3,3), input "right", tick until move_progress >= 1.0, verify position is (4,3)
}
```

## Verification
```bash
cd engine && cargo test tests_phase_2c && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Player slides smoothly between tiles (no instant snapping)
- [ ] 60fps rendering (no artificial tick limiter)
- [ ] Camera tracks player smoothly during movement
- [ ] Action/Cancel/Menu keys registered
- [ ] Facing direction visible on player
- [ ] Holding direction = continuous walking
- [ ] Walking into wall = face direction but don't move
- [ ] Encounters trigger in tall grass
- [ ] `./verify.sh` exits 0
