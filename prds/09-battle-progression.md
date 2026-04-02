# PRD 09 — Battle Progression: XP, Capture & AI (Phase 3C+3D)

## Goal
Complete the battle system with XP/leveling/move-learning/evolution, wild sneaker capture, and the AI system. After this PRD, the battle engine is 100% complete.

## Dependencies
- PRD 08 (full move effects and statuses)

## Deliverables

### Files to Create

**`engine/src/battle/capture.rs`**
- `attempt_capture(target: &SneakerInstance, species: &SneakerSpecies, case_item: &ItemData, rng: &mut SeededRng) -> CaptureResult`
- `CaptureResult`: `{ shakes: u8, success: bool }`
- Formula:
  ```
  catch_rate = ((3 * max_hp - 2 * current_hp) * base_catch_rate * case_bonus) / (3 * max_hp)
  // case_bonus from ItemEffect::CatchMultiplier (stored as x100, so 150 = 1.5x)
  // ItemEffect::GuaranteedCatch → return CaptureResult { shakes: 4, success: true }
  // Faction-specific cases: CatchMultiplierFaction → 3.0x if faction matches
  ```
- 4 shake checks:
  ```
  shake_threshold = 1048560 / sqrt(sqrt(16711680 / catch_rate))
  For each shake: rng.range(0, 65536) < shake_threshold → pass
  shakes = number of passed checks before failure (or 4 = caught)
  ```

**`engine/src/battle/ai.rs`**
- `choose_action(state: &BattleState, opponent_team: &[SneakerInstance], ai_level: AiLevel, rng: &mut SeededRng) -> BattleAction`
- **Random** (wild sneakers): Pick random move from available (non-zero PP).
- **Basic** (route trainers):
  - If any move is super-effective, prefer it (70% chance to pick SE move)
  - Otherwise random from available moves
  - Never switches or uses items
- **Intermediate** (early bosses):
  - Prefer super-effective moves (80%)
  - Switch if current sneaker is at type disadvantage AND has <50% HP AND has a better matchup on bench
  - Use healing item when HP <25% (if has items)
- **Advanced** (late bosses):
  - Always pick highest-damage move considering type effectiveness
  - Switch aggressively to counter player's active sneaker
  - Use stat boosting moves when at >75% HP and no boosts yet
  - Heal at <30% HP
- **Expert** (Elite + Champion):
  - Same as Advanced plus:
  - Predict switches (if player at type disadvantage, assume they'll switch)
  - Use coverage moves to hit likely switch-ins
  - Optimal item timing

### Files to Modify

**`engine/src/models/sneaker.rs`**
- `add_xp(&mut self, amount: u32, species: &SneakerSpecies) -> XpResult`
  ```rust
  pub struct XpResult {
      pub leveled_up: bool,
      pub new_level: u8,
      pub new_moves: Vec<u16>,  // move IDs available to learn
      pub can_evolve: Option<u16>,  // target species ID
  }
  ```
- XP formula: `xp_needed(level) = (6 * level^3 / 5) - 15 * level^2 + 100 * level - 140`
- On level-up: recalculate max_hp, heal by the HP increase amount
- `check_evolution(&self, species: &SneakerSpecies) -> Option<u16>` — if species has evolution and level >= evolution_level
- `evolve(&mut self, new_species_id: u16, new_species: &SneakerSpecies)` — update species_id, recalculate stats

**`engine/src/battle/engine.rs`**
- After opponent faints (PlayerWin):
  1. Calculate XP: `base_xp * opponent_level / 7 * trainer_bonus` (1.0 wild, 1.5 trainer)
  2. Distribute to all non-fainted party members that participated
  3. Call `add_xp()` for each recipient
  4. Emit `XpGained { amount }`, `LevelUp { new_level }` if leveled
  5. If new moves available at new level: emit `MoveLearnPrompt { move_id }`, set `waiting_for = Some(MoveLearn)`
  6. If can evolve: emit `EvolutionPrompt { species_id }`, set `waiting_for = Some(Evolution)`
  7. Award money: generate $DD based on opponent level and battle type
- **Bag action in battle**:
  - Heal items: apply to specified party member
  - Battle items (Hype Potion etc): apply stat boost
  - Sneaker Cases: call `attempt_capture()`, emit `CaptureAttempt { shakes, success }`
    - If success: add sneaker to party (if <6) or box, emit `BattleEnd(PlayerCapture)`
    - If fail: opponent gets a free attack

**`engine/src/lib.rs`**
- Add WASM methods:
  - `battle_learn_move(&mut self, slot: u8) -> String` — slot 0-3 to replace, 4 to skip. Returns remaining events.
  - `battle_evolution_choice(&mut self, accept: bool) -> String` — accept or cancel evolution.
- Update `battle_action()` to handle Bag items and capture

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_3c {
    // XP formula
    - xp_needed(5) = 135 (starter level)
    - xp_needed(10) ≈ 1000
    - xp_needed(50) ≈ 125000

    // XP gain
    - Defeat Lv.5 Common (base_xp=56) → xp = 56*5/7 = 40
    - With trainer bonus: 40 * 1.5 = 60

    // Level-up
    - At Lv.5 with 135 XP needed, gaining 200 XP → level up to 6
    - Max HP increases on level up
    - New moves available at correct levels

    // Evolution
    - Retro Runner at Lv.16 → can evolve to Retro Runner II (species 2)
    - After evolution, species_id changes, stats recalculate
    - Evolution can be cancelled

    // Capture
    - Full HP, basic case, high catch rate → fewer shakes (harder to catch)
    - 1 HP, grail case, high catch rate → almost guaranteed
    - Master Case → always success with 4 shakes
    - Faction case on matching faction → 3.0x bonus
    - Over 1000 attempts at known rates, success rate matches expected

    // AI levels
    - Random AI: all moves chosen (over 100 rounds, no move has 0% usage)
    - Basic AI: super-effective move chosen >50% of the time when available
    - Intermediate AI: switches when at disadvantage with low HP

    // Money
    - Wild battle awards money based on opponent level
    - Money added to player state after battle
}
```

## Verification
```bash
cd engine && cargo test tests_phase_3c && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] XP awarded after battle victory
- [ ] Level-up triggers with correct thresholds
- [ ] Move learning prompt at correct levels
- [ ] Evolution triggers at correct levels, can be accepted/cancelled
- [ ] Capture formula produces expected shake counts
- [ ] All 5 AI levels make reasonable decisions
- [ ] Items usable in battle (heals, cases, stat boosters)
- [ ] Money awarded after battles
- [ ] Battle engine is COMPLETE — all spec mechanics implemented
- [ ] `./verify.sh` exits 0
