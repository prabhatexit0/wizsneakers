# PRD 17 — Hypetown, Boss Battles & Rival Arc (Phase 10B)

## Goal
Build Hypetown City Center (the largest map — the central hub), its two boss fights (Flex Queen + Shadow Broker), Routes 6-8, Grailheim, Resell Row, and the remaining Rival Flip encounters. This completes the main story through all 8 stamps before Pinnacle Tower.

## Dependencies
- PRD 16 (Kicksburg/Neon Springs/Routes 2-5), PRD 09 (AI system)

## Deliverables

### Maps to Create

**`client/public/maps/hypetown.json`** (60x50)
- Largest map in the game — bustling city center
- Buildings: Sneaker Clinic, The Mall, Convention Center (gym), Underground Market entrance, Sneaker Authentication Center, Player's apartment (locked until mid-story)
- Connections: from Route 5 (south), to Route 6 (northwest), to Route 7 (northeast)
- 12+ NPCs with varied dialogue
- Syndicate grunts visible near Underground entrance (scripted encounters)

**`client/public/maps/hypetown_interior_clinic.json`** (8x6)
**`client/public/maps/hypetown_interior_mall.json`** (15x12)
- The Mall: large shop with all items from spec
  - All specialty items at +50% markup
  - Grail Case (3000), Hype/Drip/Guard/Speed potions (1500 each)
  - Wide Lens held item (2000)

**`client/public/maps/hypetown_interior_gym.json`** (15x12)
- Convention Center: fashion runway puzzle — defeat models in themed rounds
- 4 trainer NPCs (High-Fashion themed, 3 sneakers each)
- Flex Queen at the end

**`client/public/maps/hypetown_underground.json`** (20x15)
- Underground market area — stealth and battle sections
- Syndicate Grunt encounters (4 grunts, 2 sneakers each)
- Shadow Broker at the end
- Dark/moody tiles, restricted areas

**`client/public/maps/hypetown_interior_apartment.json`** (10x8)
- Player's apartment: unlocked after Shadow Broker defeat
- Bed: heals party (like Mom's house)
- Sneaker Box terminal

**`client/public/maps/route_6.json`** (65x20)
- Mountain path: Hypetown → Grailheim
- Elevation changes represented by ledge tiles (one-way jumps down)
- Wild encounters Lv.28-34:
  - Heritage Court Lv.28-30 (20%), Couture Boot Lv.29-31 (15%), Triple Black Lv.30-32 (20%), Sole Slasher Lv.28-30 (15%), Avant-Garde Lv.30-32 (15%), Vintage High-Top Lv.31-34 (15%)
- 5 trainer NPCs (mixed high-level, 3 sneakers each)
- Hidden item: Muscle Band held item

**`client/public/maps/route_7.json`** (55x20)
- Market alleyways: Hypetown → Resell Row
- Shady alley tiles, market stalls as walls
- Wild encounters Lv.28-34 (same level range as Route 6, different distribution):
  - Maison Sole Lv.29-31 (20%), Runway Slip Lv.28-30 (15%), Nano Weave Lv.30-33 (20%), Boost Core Lv.28-30 (15%), Vulcanized Lv.30-32 (15%), Triple Black Lv.31-34 (15%)
- 5 trainer NPCs (Reseller types, 3 sneakers each)
- Hidden item: Wise Glasses held item

**`client/public/maps/grailheim.json`** (40x35)
- Remote mountain town — temple aesthetic
- Buildings: Sneaker Clinic, Vault of Soles (lore), Temple of the Grail (gym)
- Connection from Route 6, connection to Route 8
- 8 NPCs — monk/sage dialogue about sneaker lore and Genesis Grails
- Hidden cave entrance NPC hint

**`client/public/maps/grailheim_interior_clinic.json`** (8x6)
**`client/public/maps/grailheim_interior_gym.json`** (14x12)
- Temple puzzle: align sneaker symbols on stone tiles to open inner sanctum
- 4 trainer NPCs (balanced teams, 3 sneakers each)
- Grand Master Lace at the end

**`client/public/maps/resell_row.json`** (45x35)
- Shady market district
- Buildings: Sneaker Clinic, Black Market shop, The Warehouse (gym)
- Connection from Route 7, connection to Route 8
- 8 NPCs — reseller/hustle dialogue

**`client/public/maps/resell_row_interior_clinic.json`** (8x6)
**`client/public/maps/resell_row_interior_shop.json`** (10x8)
- Black Market inventory (from spec):
  - Master Case (50000), Hype Pill / Rare Candy (5000)
  - Mystery Box (10000), Counterfeit Check (2000)
  - Full Restore Spray (1500), Premium Case (600)

**`client/public/maps/resell_row_interior_gym.json`** (14x12)
- Warehouse navigation: dodge grunts, solve shipping container puzzle
- 4 trainer NPCs (rare sneakers, 3 each)
- King Markup at the end

**`client/public/maps/route_8.json`** (40x15)
- Skybridge connecting Grailheim/Resell Row → Pinnacle Tower
- No wild encounters — dramatic, cinematic route
- 2 final trainer NPCs (elite, 4 sneakers each)
- Visual: open sky tiles, bridge railing walls

### Boss Battles

**Boss 5: Flex Queen (Hypetown Convention Center)**
- Team: Runway Slip (Lv.27), Couture Boot (Lv.28), Avant-Garde (Lv.29), Maison Sole (Lv.30)
- AI: Speed-focused — Red Carpet buff, then sweeps with Haute Beam / Couture Cannon
- Items: Full Restore Spray x1, Drip Potion x1
- Reward: $2500, Fashion Stamp
- Event flags: `hypetown_gym_cleared`, `stamp_fashion`

**Boss 6: Shadow Broker (Hypetown Underground)**
- Team: Triple Black (Lv.31), plus one corrupted version of each faction (Lv.30-33)
- AI: Tricky — uses Resell to steal stat changes, unpredictable switching
- Items: Full Restore Spray x2
- Reward: $3000, Shadow Stamp
- Event flags: `hypetown_underground_cleared`, `stamp_shadow`
- Story trigger: reveals Counterfeit Carl's plan, unlocks player apartment

**Boss 7: Grand Master Lace (Grailheim Temple)**
- Team: One of each faction (Lv.35-37) + Heritage Court (Lv.38)
- AI: Balanced — reads player team, counters, uses Authenticate to remove buffs
- Items: Full Restore Spray x2, Full Cleanse x1
- Reward: $3500, Grail Stamp
- Event flags: `grailheim_gym_cleared`, `stamp_grail`

**Boss 8: King Markup (Resell Row Warehouse)**
- Team: All rare/epic sneakers (Lv.40-44), competitive movesets
- AI: Full competitive — switches aggressively, uses items perfectly
- Items: Full Restore Spray x3
- Reward: $4000, Hustle Stamp
- Event flags: `resell_row_gym_cleared`, `stamp_hustle`

### Rival Flip Encounters

**Encounter 3: Hypetown entrance (before gym)**
- Team: Counter-starter (Lv.28) + Classic Dunk (Lv.26) + Lace Viper (Lv.25)
- Pre-battle: "I've been flipping kicks all through Hypetown. My portfolio is STACKED."
- Post-battle: "How do you keep beating me? Maybe I need to rethink my strategy..."
- Event flag: `hypetown_rival_battled`

**Encounter 4: Resell Row (after player clears warehouse)**
- Team: Counter-starter (Lv.40) + 3 diverse sneakers (Lv.37-39)
- Context: Flip got scammed by a counterfeiter, humbled, joins forces with player
- Pre-battle: "I got burned by a fake seller... you were right about this whole collecting thing. But I still gotta know — am I better yet?"
- Post-battle: "Okay, I respect it now. It's not about the resale. Let's take these Syndicate punks down together."
- Event flag: `resell_row_rival_battled`

### Dialogue Data

Create dialogue files for each new area:
- **`client/public/data/dialogue/hypetown/`** — NPCs, mall, gym trainers, Syndicate grunts
- **`client/public/data/dialogue/grailheim/`** — Monk NPCs, lore, gym trainers
- **`client/public/data/dialogue/resell_row/`** — Shady NPCs, shop, gym trainers
- **`client/public/data/dialogue/route_6/`** through **`route_8/`**

### Trainer Data

**`client/public/data/trainers/`** — JSON files for each route and gym

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_10b {
    // Boss battles
    - Flex Queen team is correct (4 sneakers, Lv.27-30)
    - Shadow Broker team is correct (5 sneakers, Lv.30-33)
    - Grand Master Lace team is correct (5 sneakers, Lv.35-38)
    - King Markup team is correct (all rare/epic, Lv.40-44)
    - Each boss awards correct stamp on defeat
    - All 8 stamps collected → all sneakers obey

    // Rival encounters
    - Hypetown rival team has 3 sneakers, correct counter-starter
    - Resell Row rival team has 4 sneakers
    - Rival event flags set correctly

    // Map transitions
    - Hypetown → Route 6 → Grailheim
    - Hypetown → Route 7 → Resell Row
    - Grailheim → Route 8 → (Pinnacle area)
    - Resell Row → Route 8 → (Pinnacle area)

    // Shops
    - The Mall has correct items at +50% markup
    - Black Market has Master Case at 50000
}
```

## Verification
```bash
cd engine && cargo test tests_phase_10b && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Hypetown fully explorable — largest map, all buildings accessible
- [ ] The Mall shop with full inventory works
- [ ] Flex Queen and Shadow Broker beatable, award stamps 5-6
- [ ] Grailheim and Resell Row towns explorable with clinics, shops, gyms
- [ ] Grand Master Lace and King Markup beatable, award stamps 7-8
- [ ] Routes 6-8 with encounters and trainers
- [ ] Rival encounters 3 and 4 trigger correctly
- [ ] Black Market shop works with expensive items
- [ ] Player apartment unlocks after Shadow Broker
- [ ] Route 8 skybridge leads toward Pinnacle area
- [ ] `./verify.sh` exits 0
