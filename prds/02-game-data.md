# PRD 02 — Static Game Data (Phase 1B)

## Goal
Define all 30 sneaker species, 48 moves, and 37 items as static data in Rust. This is the content backbone — every battle, encounter, shop, and progression system references this data.

## Dependencies
- PRD 01 (model structs exist)

## Deliverables

### Files to Create

**`engine/src/data/mod.rs`**
- Re-exports for `sneakers`, `moves`, `items`
- `GameData` struct or module-level functions to look up data by ID:
  - `get_species(id: u16) -> &'static SneakerSpecies`
  - `get_move(id: u16) -> &'static MoveData`
  - `get_item(id: u16) -> &'static ItemData`
- Use arrays or `phf` or simple match statements — no heap allocation for static data

**`engine/src/data/sneakers.rs`**
All 30 sneaker species. Source: `spec/game-design/05-sneaker-system.md`

Retro Faction (8):
| ID | Name | DUR | HYP | CMF | DRP | RAR | Catch | XP Yield | Evolution |
|----|------|-----|-----|-----|-----|-----|-------|----------|-----------|
| 1 | Retro Runner | 45 | 50 | 40 | 35 | 45 | 200 | 64 | → #2 at Lv.16 |
| 2 | Retro Runner II | 60 | 70 | 55 | 50 | 60 | 120 | 142 | → #3 at Lv.32 |
| 3 | Retro Runner Max | 80 | 90 | 70 | 65 | 80 | 45 | 236 | None |
| 4 | Classic Dunk | 55 | 45 | 50 | 30 | 35 | 200 | 56 | → #5 at Lv.20 |
| 5 | OG Force | 70 | 65 | 70 | 45 | 50 | 90 | 158 | None |
| 6 | Vintage High-Top | 65 | 80 | 55 | 70 | 85 | 60 | 175 | None |
| 7 | Heritage Court | 85 | 90 | 75 | 85 | 90 | 30 | 220 | None |
| 8 | Genesis Jordan | 95 | 110 | 85 | 95 | 100 | 3 | 270 | None |

Techwear Faction (8):
| ID | Name | DUR | HYP | CMF | DRP | RAR | Catch | XP Yield | Evolution |
|----|------|-----|-----|-----|-----|-----|-------|----------|-----------|
| 9 | Tech Trainer | 40 | 35 | 40 | 55 | 45 | 200 | 64 | → #10 at Lv.16 |
| 10 | Tech Trainer Pro | 55 | 50 | 55 | 75 | 60 | 120 | 142 | → #11 at Lv.32 |
| 11 | Tech Trainer Ultra | 70 | 65 | 70 | 100 | 80 | 45 | 236 | None |
| 12 | Foam Cell | 50 | 30 | 55 | 45 | 40 | 190 | 58 | → #13 at Lv.22 |
| 13 | Boost Core | 65 | 50 | 65 | 75 | 55 | 85 | 162 | None |
| 14 | LED Lace | 55 | 60 | 50 | 90 | 100 | 60 | 175 | None |
| 15 | Quantum Sole | 80 | 75 | 80 | 105 | 90 | 30 | 220 | None |
| 16 | Genesis React | 90 | 85 | 90 | 115 | 105 | 3 | 270 | None |

Skate Faction (8):
| ID | Name | DUR | HYP | CMF | DRP | RAR | Catch | XP Yield | Evolution |
|----|------|-----|-----|-----|-----|-----|-------|----------|-----------|
| 17 | Skate Blazer | 50 | 55 | 45 | 30 | 35 | 200 | 64 | → #18 at Lv.16 |
| 18 | Skate Blazer Pro | 70 | 75 | 55 | 45 | 50 | 120 | 142 | → #19 at Lv.32 |
| 19 | Skate Blazer Elite | 90 | 100 | 70 | 55 | 70 | 45 | 236 | None |
| 20 | Grip Tape | 60 | 50 | 50 | 25 | 30 | 200 | 56 | → #21 at Lv.20 |
| 21 | Half-Pipe | 80 | 70 | 65 | 40 | 50 | 90 | 158 | None |
| 22 | Vulcanized | 70 | 85 | 60 | 65 | 80 | 60 | 175 | None |
| 23 | Board Destroyer | 95 | 105 | 75 | 70 | 80 | 30 | 220 | None |
| 24 | Genesis Kickflip | 100 | 115 | 85 | 80 | 100 | 3 | 270 | None |

High-Fashion Faction (6):
| ID | Name | DUR | HYP | CMF | DRP | RAR | Catch | XP Yield | Evolution |
|----|------|-----|-----|-----|-----|-----|-------|----------|-----------|
| 25 | Runway Slip | 45 | 55 | 40 | 70 | 90 | 120 | 148 | None |
| 26 | Couture Boot | 55 | 50 | 55 | 80 | 75 | 120 | 155 | None |
| 27 | Avant-Garde | 50 | 60 | 45 | 95 | 110 | 60 | 175 | None |
| 28 | Maison Sole | 70 | 75 | 65 | 105 | 110 | 30 | 220 | None |
| 29 | Triple Black | 90 | 95 | 85 | 80 | 85 | 25 | 225 | None |
| 30 | Genesis Couture | 85 | 95 | 80 | 120 | 110 | 3 | 270 | None |

Each species must include:
- Learnset (level→move_id pairs) from `spec/game-design/06-moves-abilities.md`
- EV yields (1-3 points in the species' strongest stat)
- Description string

**`engine/src/data/moves.rs`**
All 48 moves. Source: `spec/game-design/06-moves-abilities.md`

Organize into sections:
- Normal (IDs 1-10): Lace Up, Flex, Camp Out, Quick Step, Stomp, Double Up, Deadstock Strike, Hype Train, Resell, Authenticate
- Retro (IDs 11-18): Crease, Throwback, Vintage Slam, Heritage Crush, Retro Wave, Classic Aura, OG Stamp, Grail Beam
- Techwear (IDs 19-26): Firmware Update, Shock Drop, Bluetooth Blast, Data Mine, Neon Pulse, Overclock, System Crash, Quantum Leap
- Skate (IDs 27-34): Kickflip, Ankle Breaker, Grind Rail, Board Slide, Tre Flip, Skater's Resolve, Vulc Smash, 900 Spin
- High-Fashion (IDs 35-42): Runway Strike, Label Drop, Haute Beam, Price Tag, Red Carpet, Fashion Police, Couture Cannon, Limited Edition
- Signature (IDs 43-48): Vinyl Scratch, Debug Protocol, 50-50 Grind, Vogue Strike, Resell Markup, Genesis Aura

Each move must have correct: power, accuracy, PP, priority, category, faction, effect, description.

**`engine/src/data/items.rs`**
All 37 items. Source: `spec/systems/02-inventory.md` and `spec/game-design/07-progression-economy.md`

- Heal Items (IDs 1-12): Sole Sauce through PP Max
- Battle Items (IDs 20-26): Hype Potion through Focus Sash
- Sneaker Cases (IDs 30-37): Sneaker Case through Fashion Case
- Key Items (IDs 50-59): Sneakerdex through Bicycle
- Held Items (IDs 70-84): Heritage Sole through EV Band (All)

**`engine/src/data/trainers.rs`**
- `TrainerClass` enum: `Hypebeast, SkaterKid, TechNerd, FashionStudent, Reseller, Collector, SyndicateGrunt, Boss, EliteReseller, Champion`
- `TrainerData` struct: `id, name, class, team: Vec<TrainerSneaker>, items, ai_level, reward_money, sight_range, pre/post/defeated_dialogue`
- `TrainerSneaker` struct: `species_id, level, moves: Option<[u16; 4]>, held_item`
- No trainer definitions yet (just the structure) — content comes in later PRDs

### Files to Modify

**`engine/src/lib.rs`**
- Add `mod data;`
- No other changes needed (data is referenced by later modules)

**`engine/Cargo.toml`**
- Consider if `once_cell` or `lazy_static` is needed. With Rust 2024 edition, `std::sync::LazyLock` should be available. Prefer stdlib over external deps.

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_1b {
    // Sneaker data integrity
    - All 30 species have unique IDs (1-30)
    - All species have non-zero base stats
    - Starter species (1, 9, 17) have base stat total of 215
    - Legendary species have base stat total ≥ 480
    - All evolution targets reference valid species IDs
    - Learnsets are sorted by level ascending

    // Move data integrity
    - All 48 moves have unique IDs (1-48)
    - Physical/Special moves have Some(power), Status moves have None
    - All accuracy values are 1-100
    - All PP values are > 0
    - Quick Step (ID 4) has priority = +1
    - All other standard moves have priority = 0

    // Item data integrity
    - All items have unique IDs
    - Sneaker Case catch multipliers: Basic=100, Premium=150, Grail=250, Master=0(guaranteed)
    - Heal items have positive HP values
    - All items have a category assigned

    // Data lookups
    - get_species(1) returns Retro Runner
    - get_move(1) returns Lace Up
    - get_item(1) returns Sole Sauce
    - get_species(999) panics or returns error (out of range)
}
```

## Verification
```bash
cd engine && cargo test tests_phase_1b && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] All 30 sneaker species defined with correct stats from spec
- [ ] All 48 moves defined with correct properties from spec
- [ ] All 37 items defined with correct properties from spec
- [ ] Lookup functions work by ID
- [ ] Learnsets match spec tables for at least the 3 starter lines
- [ ] All `tests_phase_1b` pass
- [ ] `./verify.sh` exits 0
