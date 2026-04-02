# PRD 13 — World Building, Starter Sequence & First Content (Phase 8)

## Goal
Implement map transitions, build the new-game flow (wake up → Prof Sole → choose starter → battle rival Flip → Route 1), and create the first 15-30 minutes of playable content through Lace-Up Village. This is where the game becomes a GAME.

## Dependencies
- PRD 12 (save/load, shops, menus), PRD 11 (dialogue, NPCs, trainers)

## Deliverables

### Map Transitions

**Modify `engine/src/world/map.rs`:**
- `MapConnections` handling: when player walks off a map edge, check the connection in that direction
- `trigger_map_transition(direction, current_map, player_pos) -> MapTransition`
  ```rust
  pub struct MapTransition {
      pub target_map: String,
      pub target_x: u16,
      pub target_y: u16,
      pub transition_type: TransitionType,
  }
  pub enum TransitionType { Walk, Fade, Warp }
  ```
- Edge transitions: player position maps to same X (north/south) or same Y (east/west) on new map
- Door/Warp transitions: defined in map event data with explicit target coordinates

**Modify `engine/src/lib.rs`:**
- When transition pending, include in tick return: `{"transition": {"map": "route_1", "x": 5, "y": 14, "type": "walk"}}`
- After JS loads new map and calls `load_map_data()`: update player position, load NPCs from new map

**Create `client/src/components/TransitionOverlay.tsx`:**
- `Fade`: black overlay fades in (150ms) → hold (100ms) → fade out (150ms)
- `Walk`: no overlay, just load new map (camera follows player off edge)
- `Warp`: same as fade but with white flash

**Modify `client/src/hooks/useWasm.ts`:**
- On map transition event from engine: fetch new map JSON → `engine.load_map_data()` → continue

### New Game Flow

**Create `client/src/components/NameEntryScreen.tsx`:**
- Text input for player name (max 10 characters)
- Keyboard: type name, Enter to confirm, Backspace to delete
- Default name "Red" if left blank
- Styled as pixel-art dialog box

**Modify `engine/src/lib.rs`:**
- `choose_starter(&mut self, choice: u8)` — choice 0=Retro Runner, 1=Tech Trainer, 2=Skate Blazer
  - Creates starter SneakerInstance at Lv.5 with appropriate moves
  - Sets rival Flip's team to counter-type starter
  - Sets event flag "has_starter"
  - Gives player starting items: $500 DD, 5x Sole Sauce, 5x Sneaker Case
  - Initializes Sneakerdex with starter marked as caught

**New Game Sequence (scripted via dialogue + events):**
1. Name entry → `engine.set_player_name(name)`
2. Fade in: player's room in Boxfresh Town
3. Mom dialogue: "Good morning! Prof Sole wants to see you at the lab."
4. Player walks to Prof Sole's lab (free movement in Boxfresh Town)
5. Enter lab → interact with Prof Sole → exposition dialogue
6. Starter selection: 3 sneaker descriptions shown, player picks one
7. Rival Flip enters, picks counter-type, challenges you
8. First battle: Flip with Lv.7 counter-starter (Basic AI, no items)
9. Win or lose: story proceeds. Prof Sole gives Sneakerdex.
10. Prof Sole: "Go explore Route 1! Catalog every sneaker you find."
11. Player is free to explore

### Maps to Create

**`client/public/maps/boxfresh_town.json`** (update from PRD 04)
- 30×20, with proper layout:
  - Player's house (south area)
  - Prof Sole's lab (north-east)
  - Mom NPC inside player's house (heals party)
  - 3-4 tutorial NPCs
  - North exit → Route 1
  - No wild encounters

**`client/public/maps/boxfresh_interior_player.json`**
- 10×8 interior: player's room + living area
- Mom NPC: heals party, dialogue varies by story progress
- Door → exits to Boxfresh Town

**`client/public/maps/boxfresh_interior_lab.json`**
- 12×10 interior: lab with 3 starter pedestals
- Prof Sole NPC: starter selection event
- Table with 3 sneakers (interactable)
- After starter chosen: Flip NPC spawns for rival battle

**`client/public/maps/route_1.json`** (update)
- 50×15 with:
  - Southern connection to Boxfresh Town
  - Northern connection to Lace-Up Village
  - 2 tall grass patches (encounters active)
  - 3 trainer NPCs positioned along route
  - 2 item pickups (Sole Sauce, Sneaker Case)
  - Wild encounters: Classic Dunk Lv.3-5 (40%), Grip Tape Lv.3-5 (35%), Foam Cell Lv.4-6 (15%), wild Retro Runner Lv.4-6 (10%)

**`client/public/maps/laceup_village.json`**
- 40×30 with:
  - Southern connection from Route 1
  - Sneaker Clinic building
  - Retro Rick's Vintage Shop
  - Lace-Up Gym (entrance only — gym interior is later PRD)
  - 6-8 NPCs with dialogue
  - Museum building (lore NPC about type matchups)

**`client/public/maps/laceup_interior_clinic.json`**
- 8×6 interior: Sneaker Clinic nurse (heals party), Sneaker Box terminal

**`client/public/maps/laceup_interior_shop.json`**
- 10×8 interior: Retro Rick's shop
- Shop inventory: Sole Sauce (200), Insole Pad (500), Sneaker Case (200), Premium Case (600), Crease Guard (300)

### Trainer Data

**`client/public/data/trainers/route_1.json`** (or embedded in Rust)
Define 3 route trainers:
1. "Hypebeast Jake": 1 sneaker — Classic Dunk Lv.4. Basic AI. Reward: $200
2. "Skater Kid Sam": 1 sneaker — Grip Tape Lv.5. Basic AI. Reward: $250
3. "Tech Nerd Mia": 1 sneaker — Foam Cell Lv.5. Basic AI. Reward: $250

### Dialogue Data

**`client/public/data/dialogue/boxfresh/`** — Mom, tutorial NPCs, generic townspeople
**`client/public/data/dialogue/route_1/`** — Trainer pre/post battle dialogue
**`client/public/data/dialogue/laceup/`** — Village NPCs, Retro Rick, Clinic nurse, museum NPC
**`client/public/data/dialogue/prof_sole.json`** — Full starter selection flow, post-game congratulations

### Rival Flip — First Battle

- Team: counter-type starter at Lv.7
  - If player chose Retro Runner → Flip has Tech Trainer Lv.7
  - If player chose Tech Trainer → Flip has Skate Blazer Lv.7
  - If player chose Skate Blazer → Flip has Retro Runner Lv.7
- AI: Basic (prefers SE moves)
- Reward: $300, event flag "route1_rival_battled"
- Pre-battle dialogue: "I already know my kicks are fresher than yours. Let's prove it!"
- Post-battle (win): "Okay okay, you got lucky. But I'm going to be WAY ahead of you next time."
- Post-battle (lose): story still proceeds (unbeatable story-wise)

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_8 {
    // Map transitions
    - Walk off north edge → transition to connected map
    - Walk off edge with no connection → blocked
    - Door tile → fade transition to target map/position
    - Player position correct after transition

    // Starter selection
    - choose_starter(0) → party has Retro Runner Lv.5 with Stomp + Crease
    - choose_starter(1) → party has Tech Trainer Lv.5 with Quick Step + Shock Drop
    - choose_starter(2) → party has Skate Blazer Lv.5 with Stomp + Kickflip
    - After choose_starter: player has $500, 5 Sole Sauce, 5 Sneaker Cases
    - Sneakerdex has starter marked as caught

    // Rival team
    - Player chose Retro → Flip has Techwear
    - Player chose Techwear → Flip has Skate
    - Player chose Skate → Flip has Retro
    - Flip's starter is Lv.7

    // Integration
    - New game → choose starter → load Boxfresh Town → walk to Route 1 → encounter works
    - Save after getting starter → load → party intact
}
```

## Verification
```bash
cd engine && cargo test tests_phase_8 && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Map transitions work (edge walk, doors)
- [ ] Transition overlay (fade) plays
- [ ] New game sequence: name → starter → rival battle → free exploration
- [ ] Boxfresh Town fully explorable with NPCs
- [ ] Route 1 has wild encounters and trainers
- [ ] Lace-Up Village accessible with clinic and shop
- [ ] Clinic heals party
- [ ] Shop buy/sell functional
- [ ] Trainers spot player and force battle
- [ ] 15-30 minutes of playable content
- [ ] Save/load works at any point in the sequence
- [ ] `./verify.sh` exits 0
