# PRD 08 — Battle Effects: Stats, Statuses, Switching & Fleeing (Phase 3B)

## Goal
Complete all battle mechanics: stat stage modifications, all 48 move effects, status conditions with full behavior, sneaker switching, and the flee formula.

## Dependencies
- PRD 07 (battle core)

## Deliverables

### Files to Modify

**`engine/src/battle/engine.rs`**
- **Switching**: `BattleAction::Switch { party_index }` — switching consumes the player's turn. Opponent attacks the incoming sneaker. Reset stat stages for the switched-out sneaker. Emit `SwitchedIn` event.
- **Fleeing**: `BattleAction::Run` — formula: `flee_chance = (player_rarity * 128 / opponent_rarity) + 30 * flee_attempts`. If `flee_chance > 255`: guaranteed. Roll random 0-255. Emit `FleeAttempt { success }`. If success: end battle with `PlayerFlee`. Increment flee_attempts on failure.
- **Move effects** — implement ALL `MoveEffect` variants in the damage/effect pipeline:
  - `StatChange`: Apply stage change to target. Emit `StatChange` event. Clamp to -6/+6. If already at cap, emit message "X's stat won't go any higher/lower!"
  - `MultiStatChange`: Apply multiple stage changes (e.g., Classic Aura: +1 Hype, +1 Comfort)
  - `StatusInflict`: Roll `chance` percent. If hit and target has no major status (or it's OnFire as volatile), apply status. Emit `StatusApplied`.
  - `Recoil`: After dealing damage, attacker takes `percent` of damage dealt as self-damage. Emit `Damage` on attacker.
  - `DrainHp`: Heal attacker by `percent` of damage dealt. Emit `Healed`.
  - `HealPercent`: Heal user by `percent` of max HP (Camp Out = 50%).
  - `MultiHit`: Execute the move `times` times. Each hit rolls damage separately.
  - `HighCrit`: Crit rate is 1/8 instead of 1/16.
  - `AlwaysCritOnSuperEffective`: If type effectiveness > 1.0, force crit (Vogue Strike).
  - `FlinchChance`: Target has `percent` chance to skip their move this turn (only works if user moves first).
  - `SkipNextTurn`: After using the move, user must skip the next turn (Quantum Leap).
  - `SwapStatChanges`: Swap all stat stages between player and opponent (Resell).
  - `RemoveBuffs`: Set all opponent's positive stat stages to 0 (Authenticate).
  - `PowerEqualsLevel`: Move power = `attacker.level * 1.5` (Price Tag).
  - `PercentCurrentHp`: Damage = `percent%` of target's current HP (Resell Markup = 50%).
  - `HealPercentDamage`: Heal attacker by `percent%` of damage dealt (Genesis Aura = 25%).
  - `SelfStatusOnMiss`: If the move misses, inflict status on self (900 Spin → Hypnotized).
  - `PriorityPlus`: This move has +1 priority (Quick Step — already handled by move data priority field).
  - `RemoveStatusDealDamage`: Remove target's status condition AND deal damage (Debug Protocol).

**`engine/src/battle/status.rs`** (full implementation)
- **Creased**: End of turn → lose 1/8 max HP. Persists until healed.
- **Scuffed**: Attack (Hype) reduced by 50% (apply in damage calculation). Lasts 1-4 turns (random on application). Decrements each turn, clears at 0.
- **SoldOut**: Cannot use moves. Lasts 1-2 turns. Decrements each turn.
- **Hypnotized**: Each turn, 50% chance to hit self instead (deal self damage with power=40 typeless). Lasts 1-4 turns.
- **Deflated**: Speed (Rarity) reduced by 75% (apply in turn order calculation). Persists until healed.
- **OnFire**: Hype increased by 50% (apply in damage calc). Lose 1/10 max HP per turn. Fixed 3 turns.
- Rule: Only one major status (Creased/Scuffed/SoldOut/Hypnotized/Deflated) at a time. OnFire is volatile and can stack with one major status.

**`engine/src/battle/damage.rs`**
- Apply Scuffed (halve attack) in damage calc when attacker has Scuffed
- Apply OnFire (+50% Hype) in damage calc when attacker has OnFire
- Apply stat stages with `StatStages::multiplier()` to attack and defense

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_3b {
    // Stat stages
    - +1 Hype stage → 1.5x attack in damage
    - -2 Comfort stage on defender → 2.0x damage
    - +6 stage → 4.0x multiplier

    // Status conditions
    - Creased: 1/8 max HP DOT per turn
    - Scuffed: physical damage halved
    - SoldOut: turn skipped
    - OnFire: +50% Hype AND DOT
    - Deflated: speed quartered for turn order
    - Hypnotized: over 100 turns, ~50% self-hit rate

    // Status rules
    - Cannot apply Scuffed when already Creased (both major)
    - Can apply OnFire when Creased (OnFire is volatile)
    - Status duration decrements correctly

    // Switching
    - Switch resets stat stages for outgoing sneaker
    - Opponent attacks incoming sneaker
    - Switched sneaker becomes active

    // Fleeing
    - With very high Rarity vs very low → always flee
    - With very low Rarity → flee_attempts increase chance
    - flee_attempts > 3 → guaranteed flee (255+ threshold)

    // Move effects (sample)
    - Camp Out heals 50% max HP
    - Heritage Crush deals recoil (33% of damage dealt)
    - Data Mine heals 50% of damage dealt
    - Authenticate removes opponent stat boosts
    - Resell swaps stat stages
    - Double Up hits twice
    - Kickflip has higher crit rate (~12.5% over many trials)
    - Price Tag damage scales with level
}
```

## Verification
```bash
cd engine && cargo test tests_phase_3b && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] All 48 move effects implemented and functional
- [ ] All 6 status conditions work correctly
- [ ] Stat stages modify damage and speed correctly
- [ ] Switching works with proper event sequence
- [ ] Flee formula works correctly
- [ ] Status rules enforced (one major + one volatile max)
- [ ] `./verify.sh` exits 0
