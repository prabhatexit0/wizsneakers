# PRD 16 — Kicksburg, Neon Springs & Routes 2-5 (Phase 10A)

## Goal
Build the next major chunk of world content: Route 2 through Kicksburg and Neon Springs, plus their connecting routes to Hypetown. This adds 4 routes, 2 towns with gyms, and roughly 2-3 hours of gameplay content. Includes Boss battles for DJ Throwback (Lace-Up), Ollie McFlip (Kicksburg), and Dr. Firmware (Neon Springs).

## Dependencies
- PRD 13 (world-starter, map transitions), PRD 11 (dialogue/NPC), PRD 09 (battle progression + AI)

## Deliverables

### Gym System (Engine)

**Create `engine/src/world/gym.rs`**
- `GymState` per town: locked/unlocked/cleared
- `GymBattle` struct: boss name, team (Vec of SneakerInstance), AI level, reward_money, stamp_id
- `attempt_gym_battle(&mut self, gym_id: &str) -> String` — initiates boss battle
- On victory: award Authentication Stamp, set event flag, award money
- Stamp effects: update obedience level cap (see spec/game-design/07-progression-economy.md)
- Gym puzzle state tracking (simple flag-based: puzzle_step_1_done, puzzle_step_2_done, etc.)

**Modify `engine/src/lib.rs`**
- Expose gym functions to WASM: `get_gym_state`, `attempt_gym_battle`, `get_stamps`
- Include stamp count in player info JSON

### Maps to Create

**`client/public/maps/laceup_interior_gym.json`** (10x12)
- Vinyl record maze puzzle: step on groove tiles in correct order to open path
- 2 trainer NPCs guarding the path
- DJ Throwback at the end

**`client/public/maps/route_2.json`** (60x20)
- Connects Lace-Up Village (south) → fork (north)
- Suburban streets transitioning to urban
- 2 tall grass patches with wild encounters: Lv.8-14
  - Classic Dunk Lv.8-10 (30%), Vintage High-Top Lv.9-11 (20%), Grip Tape Lv.8-10 (25%), Foam Cell Lv.9-11 (15%), Lace Viper Lv.10-12 (10%)
- 4 trainer NPCs (1-2 sneakers each, themed mixed)
- 2 item pickups (Insole Pad, Sneaker Case x3)
- Fork at north end: west path → Route 3 (Kicksburg), east path → Route 4 (Neon Springs)

**`client/public/maps/route_3.json`** (55x20)
- Connects fork (east) → Kicksburg (west)
- Skatepark terrain: half-pipes, rails, ramps as visual tiles
- Wild encounters Lv.14-18, Skate-types dominant:
  - Grip Tape Lv.14-16 (35%), Half-Pipe Lv.15-17 (25%), Vulcanized Lv.14-16 (20%), Sole Slasher Lv.15-17 (10%), LED Lace Lv.16-18 (10%)
- 4 trainer NPCs (Skater Kids, 2-3 sneakers each)
- 1 hidden item: TM Grind Rail location (ties to side quest)

**`client/public/maps/kicksburg.json`** (45x35)
- Skatepark town with gritty aesthetic
- Buildings: Sneaker Clinic, Thrasher's Board Shop, Kicksburg Ramp (gym)
- Southern connection from Route 3
- 8 NPCs with dialogue
- Secret area hint NPC (points to hidden rare sneaker spot on Route 3)

**`client/public/maps/kicksburg_interior_clinic.json`** (8x6)
- Nurse heals party, Sneaker Box terminal

**`client/public/maps/kicksburg_interior_shop.json`** (10x8)
- Thrasher's Board Shop inventory (from spec):
  - Sole Sauce (200), Insole Pad (500), Full Restore Spray (1500)
  - Grip Wax (1000), Skate Sole (2000), Grip Tape sneaker (3000)
  - Sneaker Case (200), Premium Case (600)

**`client/public/maps/kicksburg_interior_gym.json`** (12x10)
- Halfpipe platforming puzzle: navigate rail tiles to reach different platforms
- 3 trainer NPCs (Skate-themed, 2 sneakers each)
- Ollie McFlip at the end

**`client/public/maps/route_4.json`** (55x20)
- Connects fork (west) → Neon Springs (east)
- Tech corridor with lab buildings, clean-line tiles
- Wild encounters Lv.14-18, Techwear-types dominant:
  - Foam Cell Lv.14-16 (30%), LED Lace Lv.15-17 (25%), Boost Core Lv.14-16 (20%), Nano Weave Lv.16-18 (15%), Grip Tape Lv.15-17 (10%)
- 4 trainer NPCs (Tech Nerds, 2-3 sneakers each)
- 1 item pickup: Overclocking Chip

**`client/public/maps/neon_springs.json`** (45x35)
- Futuristic tech hub
- Buildings: Sneaker Clinic, ByteWear Boutique, Neon Lab (gym), Data Center (optional dungeon entrance)
- Connection from Route 4
- 8 NPCs with dialogue
- Data Center NPC hints at side quest

**`client/public/maps/neon_springs_interior_clinic.json`** (8x6)
**`client/public/maps/neon_springs_interior_shop.json`** (10x8)
- ByteWear Boutique inventory (from spec):
  - Sole Sauce (200), Insole Pad (500), Full Restore Spray (1500)
  - Overclocking Chip (1000), Nano-Fiber Sole (2000), Foam Cell sneaker (3000)
  - Sneaker Case (200), Premium Case (600)

**`client/public/maps/neon_springs_interior_gym.json`** (12x10)
- Circuit board puzzle: connect wire tiles to power the path
- 3 trainer NPCs (Techwear-themed)
- Dr. Firmware at the end

**`client/public/maps/route_5.json`** (70x15)
- Connects Neon Springs → Hypetown
- Underground tunnels: dark tiles, limited visibility theme
- Wild encounters Lv.19-24, rare encounters more common:
  - Boost Core Lv.19-21 (25%), Nano Weave Lv.20-22 (20%), Triple Black Lv.21-23 (15%), Sole Slasher Lv.19-21 (15%), Avant-Garde Lv.22-24 (10%), LED Lace Lv.20-22 (15%)
- 5 trainer NPCs (mix of types, 2-3 sneakers)
- Hidden item: Leftovers (Snack Pack) held item
- Long route — serves as level grind area before Hypetown

### Boss Battles

**Boss 2: DJ Throwback (Lace-Up)**
- Team: Classic Dunk (Lv.12), Vintage High-Top (Lv.14)
- AI: Moderate — uses Classic Aura to buff, then attacks
- Items: Sole Sauce x1
- Reward: $1000, Retro Stamp
- Event flags: `laceup_gym_cleared`, `stamp_retro`

**Boss 3: Ollie McFlip (Kicksburg)**
- Team: Grip Tape (Lv.17), Half-Pipe (Lv.18), Vulcanized (Lv.19)
- AI: Aggressive — spams high-crit moves, uses Skater's Resolve
- Items: Sole Sauce x2
- Reward: $1500, Street Stamp
- Event flags: `kicksburg_gym_cleared`, `stamp_street`

**Boss 4: Dr. Firmware (Neon Springs)**
- Team: Foam Cell (Lv.22), LED Lace (Lv.23), Boost Core (Lv.24)
- AI: Strategic — leads with Firmware Update buff, uses Data Mine for sustain
- Items: Insole Pad x2
- Reward: $2000, Tech Stamp
- Event flags: `neon_springs_gym_cleared`, `stamp_tech`

### Rival Flip — Second Encounter

- Location: Kicksburg entrance (scripted, before gym)
- Team: Counter-starter (Lv.18) + Classic Dunk (Lv.16)
- AI: Moderate
- Reward: $500
- Pre-battle: "You again? I've been grinding Route 2 all week. Check out my new pickup!"
- Post-battle (win): "Whatever, I'm still ahead on resale value..."
- Event flag: `kicksburg_rival_battled`

### Trainer Data

Create JSON files (or embed in Rust) for all route/gym trainers with:
- Name, trainer type, team (species + level), AI type, reward money
- Pre-battle and post-battle dialogue

**`client/public/data/trainers/route_2.json`** — 4 trainers
**`client/public/data/trainers/route_3.json`** — 4 trainers
**`client/public/data/trainers/route_4.json`** — 4 trainers
**`client/public/data/trainers/route_5.json`** — 5 trainers
**`client/public/data/trainers/laceup_gym.json`** — 2 trainers + DJ Throwback
**`client/public/data/trainers/kicksburg_gym.json`** — 3 trainers + Ollie McFlip
**`client/public/data/trainers/neon_springs_gym.json`** — 3 trainers + Dr. Firmware

### Dialogue Data

**`client/public/data/dialogue/kicksburg/`** — Town NPCs, Thrasher shop, gym trainers
**`client/public/data/dialogue/neon_springs/`** — Town NPCs, ByteWear shop, gym trainers
**`client/public/data/dialogue/route_2/`** — Trainer dialogue
**`client/public/data/dialogue/route_3/`** — Trainer dialogue
**`client/public/data/dialogue/route_4/`** — Trainer dialogue
**`client/public/data/dialogue/route_5/`** — Trainer dialogue

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_10a {
    // Gym system
    - attempt_gym_battle returns valid battle state
    - Defeating DJ Throwback awards Retro Stamp
    - Defeating Ollie McFlip awards Street Stamp
    - Defeating Dr. Firmware awards Tech Stamp
    - Stamp count increments correctly
    - Obedience cap updates with stamps (Retro → Lv.20, Street → Lv.25, Tech → Lv.30)
    - Can't re-fight cleared gym

    // Map transitions
    - Lace-Up → Route 2 transition works
    - Route 2 fork → Route 3 or Route 4
    - Route 3 → Kicksburg
    - Route 4 → Neon Springs
    - Route 5 → (towards Hypetown, may dead-end for now)
    - Neon Springs → Route 5

    // Wild encounters
    - Route 2 encounters are Lv.8-14
    - Route 3 encounters skew Skate-type
    - Route 4 encounters skew Techwear-type
    - Route 5 encounters Lv.19-24

    // Rival
    - Kicksburg rival has counter-starter Lv.18 + Classic Dunk Lv.16
    - Rival battle sets event flag
}
```

## Verification
```bash
cd engine && cargo test tests_phase_10a && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Route 2 explorable with wild encounters and trainers
- [ ] Route 2 forks to Route 3 (Kicksburg) and Route 4 (Neon Springs)
- [ ] Kicksburg town fully explorable — clinic, shop, gym
- [ ] Neon Springs town fully explorable — clinic, shop, gym
- [ ] Route 5 connects Neon Springs toward Hypetown
- [ ] Lace-Up gym beatable (DJ Throwback), awards Retro Stamp
- [ ] Kicksburg gym beatable (Ollie McFlip), awards Street Stamp
- [ ] Neon Springs gym beatable (Dr. Firmware), awards Tech Stamp
- [ ] Rival Flip encounter at Kicksburg works
- [ ] All shops have correct inventory per spec
- [ ] Obedience system works with stamps
- [ ] `./verify.sh` exits 0
