# PRD 11 — Dialogue Engine & NPC System (Phase 6)

## Goal
Implement the dialogue system (typewriter text, choices, conditional branching) and the NPC system (movement patterns, trainer line-of-sight, interaction). The world becomes populated and interactive.

## Dependencies
- PRD 06 (smooth movement, action button), PRD 07 (battles for trainer encounters)

## Deliverables

### Files to Create

**`engine/src/world/dialogue.rs`**
- `DialogueData` (deserialize from JSON):
  ```rust
  pub struct DialogueData {
      pub id: String,
      pub pages: Vec<DialoguePage>,
  }
  pub struct DialoguePage {
      pub speaker: Option<String>,
      pub text: String,
      pub choices: Option<Vec<DialogueChoice>>,
  }
  pub struct DialogueChoice {
      pub text: String,
      pub next_dialogue: Option<String>,
      pub set_flag: Option<String>,
      pub action: Option<String>,  // "heal_party", "open_shop", etc.
  }
  ```
- `DialogueState` on GameEngine:
  ```rust
  pub struct DialogueState {
      pub active_dialogue_id: String,
      pub current_page: usize,
      pub waiting_for_choice: bool,
  }
  ```
- Template variable replacement: `{player_name}` → player's name, `{rival_name}` → "Flip"

**`engine/src/world/npc.rs`**
- `NpcState` struct:
  ```rust
  pub struct NpcState {
      pub id: String,
      pub x: u16, pub y: u16,
      pub facing: Direction,
      pub sprite: String,
      pub movement: NpcMovement,
      pub dialogue_id: String,
      pub is_trainer: bool,
      pub trainer_data: Option<TrainerNpcData>,
      pub defeated: bool,
      pub moving: bool,
      pub move_progress: f32,
      pub move_timer: f64,
  }
  ```
- `NpcMovement` enum: `Stationary, RandomWalk { radius: u8 }, Patrol { path: Vec<(u16, u16)> }, FacePlayer`
- `TrainerNpcData`: `trainer_id: u16, sight_range: u8`
- `tick_npcs(npcs: &mut Vec<NpcState>, player_pos: (u16, u16), map: &MapData, dt_ms: f64, rng: &mut SeededRng) -> Vec<GameEvent>`
  - RandomWalk: every 2-5 seconds (random), pick random adjacent walkable tile within radius, move
  - Patrol: follow path points in order, loop
  - FacePlayer: always face toward player position
  - NPC-to-NPC collision check
- `check_trainer_triggers(npcs: &[NpcState], player_pos: (u16, u16), map: &MapData) -> Option<String>`
  - For each non-defeated trainer NPC:
    - Check tiles in their facing direction up to `sight_range`
    - If any tile between trainer and player is a wall, line-of-sight blocked
    - If player is within line-of-sight, return trainer NPC ID
  - Returns the NPC ID that spotted the player (first one found)

**`engine/src/world/events.rs`**
- Event system functions:
  - `interact(engine: &mut GameEngine) -> Option<InteractionResult>` — check what's in front of player:
    - NPC → start dialogue, return dialogue data
    - Sign → return sign text
    - Nothing → None
  - `check_position_events(engine: &mut GameEngine) -> Vec<GameEvent>` — check if player stepped on event tile
- `InteractionResult`: `Dialogue(DialogueData), Shop(ShopId), Heal, SneakerBox`

**`client/src/components/DialogueBox.tsx`**
- Props: `dialogue: DialoguePage, onAdvance: () => void, onChoice: (index: number) => void`
- Bottom of screen overlay (per spec art/03-ui-ux.md):
  - Pixel-art bordered panel (2px border, semi-transparent dark bg)
  - Speaker name label (top-left of panel)
  - Typewriter text (character by character, speed configurable)
  - "▼" indicator when waiting for input (blinks)
  - Choice buttons when `choices` present (arrow key selection)
- Text speed: `slow` (40ms/char), `medium` (20ms/char), `fast` (10ms/char), `instant`
- Action key advances text (or completes current text if still typing)

**`client/public/data/dialogue/boxfresh_town.json`**
- Mom dialogue (multiple variants based on story progress)
- Tutorial NPC dialogues (explain controls, type matchups)
- 3-4 generic NPC dialogues

**`client/public/data/dialogue/prof_sole.json`**
- Starter selection dialogue (stub — full flow in PRD 13)

### Files to Modify

**`engine/src/lib.rs`**
- Add WASM methods:
  - `interact(&mut self) -> String` — calls event system, returns JSON (dialogue data or action type)
  - `advance_dialogue(&mut self) -> String` — advance to next page, returns next page or "end"
  - `select_choice(&mut self, index: u8) -> String` — select dialogue choice
- In `tick()`: call `tick_npcs()`, check trainer triggers
- When trainer spots player: emit exclamation event, walk trainer toward player, then start battle
- Include NPC positions/facings in tick return data

**`client/src/App.tsx`**
- When mode is "Dialogue": show `<DialogueBox>` overlay
- Action key → `engine.interact()` when in overworld
- Render NPCs as colored rectangles (orange, per spec prototype colors)
- Show "!" exclamation above trainer that spotted player

**`client/src/hooks/useInput.ts`**
- Ensure action key triggers interact in overworld mode

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_6 {
    // Dialogue
    - Template replacement: "{player_name}" → actual name
    - Multi-page dialogue advances correctly
    - Choices set flags

    // NPC movement
    - Stationary NPC never moves
    - RandomWalk NPC stays within radius
    - FacePlayer NPC faces player direction
    - NPC collision: two NPCs can't occupy same tile

    // Trainer line-of-sight
    - Trainer facing right, player 3 tiles right → detected
    - Trainer facing right, player 3 tiles left → not detected
    - Wall between trainer and player → not detected
    - Defeated trainer → not detected
    - Sight range respected (player at range+1 → not detected)

    // Interaction
    - Facing NPC and pressing interact → starts dialogue
    - Facing wall → no interaction
    - Facing sign → sign text returned
}
```

## Verification
```bash
cd engine && cargo test tests_phase_6 && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Action button triggers interaction when facing NPC
- [ ] Dialogue box shows with typewriter text effect
- [ ] Multi-page dialogue advances with action key
- [ ] Choices display and are selectable
- [ ] NPCs visible on map (colored rectangles)
- [ ] NPCs move per their pattern
- [ ] Trainers spot player within line-of-sight
- [ ] "!" exclamation shows when trainer spots player
- [ ] Trainer walks toward player and forces battle
- [ ] Template variables replaced in dialogue text
- [ ] `./verify.sh` exits 0
