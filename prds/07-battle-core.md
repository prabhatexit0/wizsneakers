# PRD 07 — Battle Core: Damage, Turns & Basic Flow (Phase 3A)

## Goal
Implement the core battle engine in Rust: damage formula, turn resolution, basic battle flow. When a wild encounter triggers, the game enters battle mode. A minimal debug-style battle overlay in React lets you fight (select moves) and see damage. Ugly but functional.

## Dependencies
- PRD 02 (move/sneaker data), PRD 06 (encounters trigger)

## Deliverables

### Files to Create

**`engine/src/battle/mod.rs`** — re-exports

**`engine/src/battle/types.rs`**
- `BattleState`:
  ```rust
  pub struct BattleState {
      pub kind: BattleKind,
      pub player_active: usize,      // index into player party
      pub opponent: BattleOpponent,
      pub opponent_active: usize,
      pub turn_number: u16,
      pub player_stages: StatStages,
      pub opponent_stages: StatStages,
      pub turn_log: Vec<BattleTurnEvent>,
      pub flee_attempts: u8,
      pub can_flee: bool,
      pub waiting_for: Option<BattlePrompt>,
  }
  ```
- `BattleKind`: `Wild, Trainer { id: u16, name: String }, Boss { id: u16, name: String }`
- `BattleOpponent`: `team: Vec<SneakerInstance>, items: Vec<(u16, u16)>, ai_level: AiLevel`
- `AiLevel`: `Random, Basic, Intermediate, Advanced, Expert`
- `BattlePrompt`: `MoveLearn { move_id: u16 }, Evolution { species_id: u16 }`
- `BattleAction`: `Fight { move_index: u8 }, Bag { item_id: u16 }, Switch { party_index: u8 }, Run`
- `BattleTurnEvent` enum — ALL variants per spec:
  `MoveUsed, Damage, StatChange, StatusApplied, StatusDamage, Healed, Fainted, SwitchedIn, ItemUsed, FleeAttempt, CaptureAttempt, XpGained, LevelUp, MoveLearnPrompt, EvolutionPrompt, BattleEnd, Message`
- `BattleSide`: `Player, Opponent`
- `Effectiveness`: `SuperEffective, Normal, NotVeryEffective`
- `BattleResult`: `PlayerWin, PlayerLose, PlayerFlee, PlayerCapture`

**`engine/src/battle/damage.rs`**
```rust
pub fn calculate_damage(
    attacker: &SneakerInstance,
    attacker_species: &SneakerSpecies,
    defender: &SneakerInstance,
    defender_species: &SneakerSpecies,
    move_data: &MoveData,
    attacker_stages: &StatStages,
    defender_stages: &StatStages,
    rng: &mut SeededRng,
) -> DamageResult

pub struct DamageResult {
    pub damage: u16,
    pub effectiveness: Effectiveness,
    pub is_critical: bool,
    pub type_multiplier: f64,
}
```
Formula: `((2*level/5+2) * power * attack/defense) / 50 + 2) * STAB * type_eff * crit * random`
- `attack`: Hype for Physical, Drip for Special (apply stat stage multiplier)
- `defense`: Comfort for both (apply stat stage multiplier)
- `STAB`: 1.5 if move faction matches attacker faction, else 1.0
- `type_eff`: `attacker_move_faction.effectiveness_against(defender_faction)`
- `crit`: base 1/16 chance → 1.5x multiplier. Crit ignores negative atk stages and positive def stages.
- `random`: 0.85 to 1.00

**`engine/src/battle/engine.rs`**
- `BattleEngine` methods (or functions taking `&mut BattleState`):
  - `new_wild(wild_sneaker: SneakerInstance) -> BattleState`
  - `submit_action(state: &mut BattleState, player_party: &mut Vec<SneakerInstance>, action: BattleAction, rng: &mut SeededRng) -> Vec<BattleTurnEvent>`
- Turn resolution for `Fight`:
  1. Determine order: compare priority of chosen moves, then compare Rarity stats, then RNG tiebreak
  2. Execute first action: check accuracy (roll 1-100 vs move accuracy), calculate damage, apply, check faint
  3. If opponent not fainted, execute second action with same logic
  4. End-of-turn: tick status conditions
  5. Check win/lose conditions

**`engine/src/battle/status.rs`** (stub)
- `apply_end_of_turn_status()` — Creased: lose 1/8 max HP. OnFire: lose 1/10 max HP. Others: decrement turn counter.
- `check_can_move()` — SoldOut: can't move. Hypnotized: 50% chance self-hit.
- (Full implementation in PRD 08)

### Files to Modify

**`engine/src/lib.rs`**
- Add `mod battle;`
- Add field: `battle: Option<BattleState>`
- When encounter triggers in `tick()`: create `BattleState::new_wild()` with the generated sneaker, set `mode = Battle`, store in `self.battle`
- New WASM methods:
  - `pub fn battle_action(&mut self, action_json: &str) -> String` — parse action JSON `{"type":"fight","move_index":0}`, call engine, return JSON array of events
  - `pub fn get_battle_state(&self) -> String` — serialize current battle state for UI (both sneakers' display info, HP, available moves)
  - `pub fn mode(&self) -> String` — return current GameMode as string
- When battle ends (PlayerWin/Flee): set `mode = Overworld`, clear `self.battle`

**`client/src/App.tsx`**
- Check `engine.mode()` each frame
- If mode is `"Battle"`: render a debug battle overlay instead of (or on top of) the overworld
- Debug battle overlay (temporary, replaced in PRD 10):
  - Show: "YOUR: [name] Lv.[x] HP:[current]/[max]" and "WILD: [name] Lv.[x] HP:[current]/[max]"
  - Show 4 move buttons (or keyboard 1-4)
  - Show "Run" button
  - Show battle log messages
  - After selecting a move, call `engine.battle_action()`, display events as text
  - When battle ends, return to overworld

**`client/src/hooks/useInput.ts`**
- In battle mode, number keys 1-4 select moves, R for run

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_3a {
    // Damage formula
    - Lv.10 Retro Runner (Hype 50 base) using Crease (40 power, Physical)
      vs Lv.8 Foam Cell (Comfort 55 base) → damage in expected range [8-12]
    - Same but with STAB (Retro move on Retro sneaker) → ~1.5x
    - Super effective (Retro vs Skate) → ~2x
    - Not very effective (Retro vs Techwear) → ~0.5x
    - Status move → 0 damage

    // Critical hits
    - With forced crit (test helper), damage is 1.5x
    - Crit ignores negative attack stage on attacker
    - Crit ignores positive defense stage on defender

    // Turn resolution
    - Higher Rarity goes first
    - Higher priority move goes first regardless of speed
    - Both sides take turns (unless one faints first)
    - Accuracy check: 100% accuracy always hits, 0% always misses

    // Battle flow
    - Create wild battle → both sides have HP
    - Fight until opponent HP = 0 → BattleEnd(PlayerWin)
    - Player HP = 0 → BattleEnd(PlayerLose)

    // WASM interop
    - battle_action with {"type":"fight","move_index":0} returns valid JSON events
    - get_battle_state returns JSON with both sneaker summaries
}
```

## Verification
```bash
cd engine && cargo test tests_phase_3a && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Wild encounters create battle state with generated sneaker
- [ ] Damage formula produces correct ranges
- [ ] STAB, type effectiveness, critical hits all work
- [ ] Turn order based on priority then speed
- [ ] Battle ends when HP reaches 0
- [ ] Debug battle overlay shows in React when in battle mode
- [ ] Player can select moves and see damage happen
- [ ] Run returns to overworld
- [ ] `./verify.sh` exits 0
