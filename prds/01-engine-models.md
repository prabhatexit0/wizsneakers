# PRD 01 — Engine Models & Module Structure (Phase 1A)

## Goal
Restructure the monolithic `engine/src/lib.rs` into a modular Rust codebase. Create all core data model structs/enums that the entire game will build on. The client must still work identically after this refactor.

## Context
Currently `lib.rs` is a single 189-line file with `Faction`, `Stats`, `Sneaker`, and `GameEngine` all inlined. Stats use `u32` (spec says `u16`). There are no IVs, EVs, conditions, move slots, items, or inventory. This PRD creates the model layer that everything else depends on.

## Deliverables

### Files to Create

**`engine/src/models/mod.rs`**
- Re-export all model modules

**`engine/src/models/faction.rs`**
- `Faction` enum: `Normal`, `Retro`, `Techwear`, `Skate`, `HighFashion`
- Derive: `Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize`
- `wasm_bindgen` on the enum for JS interop
- Method: `effectiveness_against(&self, defender: Faction) -> f64`
  - Retro vs Skate: 2.0, Retro vs Techwear: 0.5
  - Techwear vs Retro: 2.0, Techwear vs Skate: 0.5
  - Skate vs Techwear: 2.0, Skate vs Retro: 0.5
  - HighFashion vs HighFashion: 0.5
  - All others: 1.0

**`engine/src/models/stats.rs`**
- `Stats` struct with `u16` fields: `durability`, `hype`, `comfort`, `drip`, `rarity`
- Implement `get(&self, kind: StatKind) -> u16`
- `StatKind` enum: `Durability, Hype, Comfort, Drip, Rarity`
- `StatStages` struct with `i8` fields (hype, comfort, drip, rarity — NOT durability)
  - Default all to 0
  - `fn multiplier(stage: i8) -> f64` — clamp to -6..+6, positive: `(2+stage)/2`, negative: `2/(2+abs(stage))`
  - `fn modify(&mut self, stat: StatKind, amount: i8)` — clamp result to -6..+6
- `Condition` enum (9 natures): `Deadstock, Beat, Restored, Custom, Vintage, Prototype, PlayerExclusive, Sample, GeneralRelease`
  - `fn modifier(&self, stat: StatKind) -> f64` — returns 0.9, 1.0, or 1.1 per spec table

**`engine/src/models/sneaker.rs`**
- `RarityTier` enum: `Common, Uncommon, Rare, Epic, Legendary`
- `SneakerSpecies` struct: `id: u16, name: &'static str, faction: Faction, base_stats: Stats, rarity_tier: RarityTier, base_catch_rate: u8, base_xp_yield: u16, ev_yield: Stats, learnset: &'static [(u8, u16)], evolution: Option<(u8, u16)>, description: &'static str`
- `SneakerInstance` struct: `uid: u64, species_id: u16, nickname: Option<String>, level: u8, xp: u32, current_hp: u16, max_hp: u16, ivs: Stats, evs: Stats, condition: Condition, moves: [Option<MoveSlot>; 4], status: Option<StatusCondition>, held_item: Option<u16>, friendship: u8, caught_location: u16, original_trainer: String`
- `StatusCondition` enum: `Creased, Scuffed { turns_left: u8 }, SoldOut { turns_left: u8 }, Hypnotized { turns_left: u8 }, Deflated, OnFire { turns_left: u8 }`
- Implement on `SneakerInstance`:
  - `calc_stat(&self, species: &SneakerSpecies, stat: StatKind) -> u16` — HP: `(2*base+iv+ev/4)*level/100+level+10`, others: `((2*base+iv+ev/4)*level/100+5)*condition_mod`
  - `calc_max_hp(&self, species: &SneakerSpecies) -> u16`
  - `is_fainted(&self) -> bool`
  - `display_name(&self, species: &SneakerSpecies) -> &str` — nickname or species name

**`engine/src/models/moves.rs`**
- `MoveCategory` enum: `Physical, Special, Status`
- `MoveEffect` enum covering all effects needed for 48 moves:
  - `None`
  - `StatChange { target: MoveTarget, stat: StatKind, stages: i8 }`
  - `MultiStatChange { target: MoveTarget, changes: &'static [(StatKind, i8)] }`
  - `StatusInflict { status: StatusType, chance: u8 }` (chance as percent, 0 = guaranteed)
  - `Recoil { percent: u8 }`
  - `DrainHp { percent: u8 }`
  - `HealPercent { percent: u8 }`
  - `MultiHit { times: u8 }`
  - `HighCrit`
  - `AlwaysCritOnSuperEffective`
  - `FlinchChance { percent: u8 }`
  - `SkipNextTurn`
  - `SwapStatChanges`
  - `RemoveBuffs`
  - `PowerEqualsLevel`
  - `PercentCurrentHp { percent: u8 }`
  - `HealPercentDamage { percent: u8 }`
  - `SelfStatusOnMiss { status: StatusType }`
  - `PriorityPlus`
  - `RemoveStatusDealDamage`
- `MoveTarget` enum: `Self_, Opponent`
- `StatusType` enum: `Creased, Scuffed, SoldOut, Hypnotized, Deflated, OnFire`
- `MoveData` struct: `id: u16, name: &'static str, faction: Faction, category: MoveCategory, power: Option<u8>, accuracy: u8, pp: u8, priority: i8, effect: MoveEffect, description: &'static str`
- `MoveSlot` struct: `move_id: u16, current_pp: u8, max_pp: u8`

**`engine/src/models/items.rs`**
- `ItemCategory` enum: `HealItem, BattleItem, SneakerCase, KeyItem, HeldItem`
- `ItemEffect` enum: `HealHp(u16), HealFull, Revive(u8), ReviveFull, CureStatus(Option<StatusType>), CureAll, RestorePp(u8), RestoreAllPp, StatBoost(StatKind, i8), BoostAll, GuaranteedCrit, SurviveFatalHit, CatchMultiplier(u16), CatchMultiplierFaction(Faction, u16), GuaranteedCatch, LevelUp, Repel(u16), EscapeDungeon, None`
  - Catch multipliers stored as x100 (150 = 1.5x) to avoid floats
- `ItemData` struct: `id: u16, name: &'static str, category: ItemCategory, cost: u32, effect: ItemEffect, description: &'static str`

**`engine/src/models/inventory.rs`**
- `Inventory` struct with 5 pockets: `heal_items: Vec<(u16, u16)>` (item_id, qty), `battle_items`, `sneaker_cases`, `key_items: Vec<u16>`, `held_items: Vec<(u16, u16)>`
- Methods: `add_item()`, `remove_item()`, `has_item()`, `item_count()`
- `SneakerBox` struct: `sneakers: Vec<SneakerInstance>` (max 50)
- Methods: `deposit()`, `withdraw()`, `is_full()`, `count()`

**`engine/src/util/mod.rs`** — re-export

**`engine/src/util/rng.rs`**
- `SeededRng` struct with `state: u64`
- Xorshift64 implementation:
  - `new(seed: u64) -> Self`
  - `next_u64(&mut self) -> u64`
  - `next_f64(&mut self) -> f64` — 0.0 to 1.0
  - `range(&mut self, min: u32, max: u32) -> u32` — inclusive min, exclusive max
  - `chance(&mut self, percent: u8) -> bool`

### Files to Modify

**`engine/src/lib.rs`**
- Add `mod models; mod util;`
- Keep existing `GameEngine` working with same API (don't break the client)
- Remove the old `Faction`, `Stats`, `Sneaker` definitions — use the ones from `models/`
- The existing `tick()`, `player_x()`, `get_tile()`, `state_json()` must continue to work

**`engine/Cargo.toml`**
- Add `js-sys = "0.3"` to dependencies

## Tests Required

Create `engine/src/models/tests.rs` (or inline `#[cfg(test)]` modules) with:

```rust
#[cfg(test)]
mod tests_phase_1a {
    // Type effectiveness
    - Retro vs Skate = 2.0
    - Techwear vs Retro = 2.0
    - Skate vs Techwear = 2.0
    - HighFashion vs HighFashion = 0.5
    - Normal vs anything = 1.0
    - Retro vs Retro = 1.0

    // Stat stages
    - multiplier(0) = 1.0
    - multiplier(1) = 1.5
    - multiplier(-1) ≈ 0.667
    - multiplier(6) = 4.0
    - multiplier(-6) = 0.25
    - clamping: modify(+3) from +5 results in +6

    // Conditions
    - Deadstock: Hype +10% (1.1), Rarity -10% (0.9)
    - GeneralRelease: all 1.0
    - Beat: Comfort +10%, Drip -10%

    // Stat calculation
    - Known input: base=50, iv=15, ev=0, level=10, condition=1.0 → expected stat
    - HP formula at level 5 with starter stats
    - HP formula at level 50

    // RNG
    - Same seed produces same sequence
    - range() stays within bounds over 1000 calls
    - chance(100) always true, chance(0) always false

    // Inventory
    - add_item increments qty
    - remove_item decrements qty
    - remove_item returns false when qty=0
    - SneakerBox respects 50 cap
}
```

## Verification
```bash
cd engine && cargo test tests_phase_1a && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] All model structs/enums compile and are importable from `engine::models`
- [ ] `lib.rs` imports from models, no duplicate definitions
- [ ] Client renders and plays identically (no visible change)
- [ ] All `tests_phase_1a` tests pass
- [ ] `./verify.sh` exits with code 0
