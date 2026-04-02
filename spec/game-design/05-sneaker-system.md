# Sneaker System

## Overview

Sneakers are the core collectible and combat unit of Wizsneakers. Each sneaker belongs to a **Faction** (type), has **base stats**, learns **moves** as it levels up, and can be one of several **rarity tiers**. Players can carry up to 6 sneakers in their active party and store extras in the **Sneaker Box** (accessed at Sneaker Clinics).

## Stats

Every sneaker has 5 core stats:

| Stat | Abbreviation | Role | Analogy (Pokemon) |
|------|-------------|------|-------------------|
| **Hype** | HYP | Physical Attack power | Attack |
| **Comfort** | CMF | Physical + Special Defense | Defense + Sp.Def |
| **Durability** | DUR | Hit Points | HP |
| **Drip** | DRP | Special Attack power | Sp. Attack |
| **Rarity** | RAR | Turn order / Speed | Speed |

### Stat Calculation

```
stat = ((2 * base + iv + ev/4) * level / 100 + 5) * nature_modifier

For Durability (HP):
hp = ((2 * base + iv + ev/4) * level / 100 + level + 10)

Where:
- base: The sneaker species' base stat (fixed per sneaker type)
- iv: Individual Value, 0-31 (randomly assigned at encounter)
- ev: Effort Value, 0-252 per stat, max 510 total (gained from battles)
- level: Current level (1-50)
- nature_modifier: 0.9, 1.0, or 1.1 (based on sneaker's Condition)
```

### Conditions (Natures)

Each sneaker has a randomly assigned **Condition** that boosts one stat by 10% and reduces another by 10%.

| Condition | +10% | -10% | Flavor |
|-----------|------|------|--------|
| **Deadstock** | Hype | Rarity | "Untouched, pristine power" |
| **Beat** | Comfort | Drip | "Worn in, tough as nails" |
| **Restored** | Drip | Hype | "Brought back with care and finesse" |
| **Custom** | Rarity | Comfort | "One of a kind, flashy but fragile" |
| **Vintage** | Hype | Comfort | "Old school power, showing its age" |
| **Prototype** | Drip | Rarity | "Experimental tech, not yet optimized" |
| **Player Exclusive** | Rarity | Hype | "Fast and exclusive, not the strongest" |
| **Sample** | Comfort | Drip | "Sturdy build, basic features" |
| **GR (General Release)** | None | None | "Perfectly balanced" |

## Rarity Tiers

| Tier | Drop Rate | Base Stat Total Range | Icon | Examples |
|------|-----------|----------------------|------|----------|
| **Common** | 50% | 200-280 | Gray star | Mall walkers, basic trainers |
| **Uncommon** | 30% | 280-350 | Green star | Solid daily kicks |
| **Rare** | 15% | 350-420 | Blue star | Limited releases |
| **Epic** | 4% | 420-480 | Purple star | Collaboration sneakers |
| **Legendary** | 1% | 480-540 | Gold star | Genesis Grails, one-of-ones |

## Full Sneaker Roster (30 Sneakers for MVP)

### Retro Faction (8 sneakers)

| # | Name | Rarity | DUR | HYP | CMF | DRP | RAR | Total | Signature |
|---|------|--------|-----|-----|-----|-----|-----|-------|-----------|
| 001 | **Retro Runner** | Common | 45 | 50 | 40 | 35 | 45 | 215 | Starter option |
| 002 | **Retro Runner II** | Uncommon | 60 | 70 | 55 | 50 | 60 | 295 | Evolves from #001 at Lv.16 |
| 003 | **Retro Runner Max** | Rare | 80 | 90 | 70 | 65 | 80 | 385 | Evolves from #002 at Lv.32 |
| 004 | **Classic Dunk** | Common | 55 | 45 | 50 | 30 | 35 | 215 | Route 1-2 encounter |
| 005 | **OG Force** | Uncommon | 70 | 65 | 70 | 45 | 50 | 300 | Evolves from #004 at Lv.20 |
| 006 | **Vintage High-Top** | Rare | 65 | 80 | 55 | 70 | 85 | 355 | Route 2 rare encounter |
| 007 | **Heritage Court** | Epic | 85 | 90 | 75 | 85 | 90 | 425 | Boss reward (DJ Throwback) |
| 008 | **Genesis Jordan** | Legendary | 95 | 110 | 85 | 95 | 100 | 485 | Genesis Grail #1 |

### Techwear Faction (8 sneakers)

| # | Name | Rarity | DUR | HYP | CMF | DRP | RAR | Total | Signature |
|---|------|--------|-----|-----|-----|-----|-----|-------|-----------|
| 009 | **Tech Trainer** | Common | 40 | 35 | 40 | 55 | 45 | 215 | Starter option |
| 010 | **Tech Trainer Pro** | Uncommon | 55 | 50 | 55 | 75 | 60 | 295 | Evolves from #009 at Lv.16 |
| 011 | **Tech Trainer Ultra** | Rare | 70 | 65 | 70 | 100 | 80 | 385 | Evolves from #010 at Lv.32 |
| 012 | **Foam Cell** | Common | 50 | 30 | 55 | 45 | 40 | 220 | Route 4 encounter |
| 013 | **Boost Core** | Uncommon | 65 | 50 | 65 | 75 | 55 | 310 | Evolves from #012 at Lv.22 |
| 014 | **LED Lace** | Rare | 55 | 60 | 50 | 90 | 100 | 355 | Neon Springs rare encounter |
| 015 | **Quantum Sole** | Epic | 80 | 75 | 80 | 105 | 90 | 430 | Boss reward (Dr. Firmware) |
| 016 | **Genesis React** | Legendary | 90 | 85 | 90 | 115 | 105 | 485 | Genesis Grail #2 |

### Skate Faction (8 sneakers)

| # | Name | Rarity | DUR | HYP | CMF | DRP | RAR | Total | Signature |
|---|------|--------|-----|-----|-----|-----|-----|-------|-----------|
| 017 | **Skate Blazer** | Common | 50 | 55 | 45 | 30 | 35 | 215 | Starter option |
| 018 | **Skate Blazer Pro** | Uncommon | 70 | 75 | 55 | 45 | 50 | 295 | Evolves from #017 at Lv.16 |
| 019 | **Skate Blazer Elite** | Rare | 90 | 100 | 70 | 55 | 70 | 385 | Evolves from #018 at Lv.32 |
| 020 | **Grip Tape** | Common | 60 | 50 | 50 | 25 | 30 | 215 | Route 3 encounter |
| 021 | **Half-Pipe** | Uncommon | 80 | 70 | 65 | 40 | 50 | 305 | Evolves from #020 at Lv.20 |
| 022 | **Vulcanized** | Rare | 70 | 85 | 60 | 65 | 80 | 360 | Kicksburg rare encounter |
| 023 | **Board Destroyer** | Epic | 95 | 105 | 75 | 70 | 80 | 425 | Boss reward (Ollie McFlip) |
| 024 | **Genesis Kickflip** | Legendary | 100 | 115 | 85 | 80 | 100 | 480 | Genesis Grail #3 |

### High-Fashion Faction (6 sneakers)

| # | Name | Rarity | DUR | HYP | CMF | DRP | RAR | Total | Signature |
|---|------|--------|-----|-----|-----|-----|-----|-------|-----------|
| 025 | **Runway Slip** | Uncommon | 45 | 55 | 40 | 70 | 90 | 300 | Hypetown encounter |
| 026 | **Couture Boot** | Uncommon | 55 | 50 | 55 | 80 | 75 | 315 | Hypetown encounter |
| 027 | **Avant-Garde** | Rare | 50 | 60 | 45 | 95 | 110 | 360 | Hypetown rare encounter |
| 028 | **Maison Sole** | Epic | 70 | 75 | 65 | 105 | 110 | 425 | Boss reward (Flex Queen) |
| 029 | **Triple Black** | Epic | 90 | 95 | 85 | 80 | 85 | 435 | Resell Row rare |
| 030 | **Genesis Couture** | Legendary | 85 | 95 | 80 | 120 | 110 | 490 | Genesis Grail #4 |

## Evolution System

Sneakers "evolve" when they reach a certain level. Evolution is framed as "upgrading" — your sneaker gets a fresh colorway, better materials, and improved stats.

- Evolution is automatic on level-up (player can cancel if desired)
- Evolution triggers a special animation
- Evolved sneakers learn different moves than their pre-evolutions
- Some sneakers don't evolve (legendaries, epics that aren't part of a line)

### Evolution Lines

```
Retro Runner (Lv.5) → Retro Runner II (Lv.16) → Retro Runner Max (Lv.32)
Tech Trainer (Lv.5) → Tech Trainer Pro (Lv.16) → Tech Trainer Ultra (Lv.32)
Skate Blazer (Lv.5) → Skate Blazer Pro (Lv.16) → Skate Blazer Elite (Lv.32)
Classic Dunk (Lv.3) → OG Force (Lv.20)
Foam Cell (Lv.3) → Boost Core (Lv.22)
Grip Tape (Lv.3) → Half-Pipe (Lv.20)
```

## Starter Selection

At the beginning of the game, Professor Sole offers the player a choice of three starter sneakers:

1. **Retro Runner** (Retro) — "A classic that never goes out of style. Balanced and reliable."
2. **Tech Trainer** (Techwear) — "Cutting-edge performance tech. Strong special attacks."
3. **Skate Blazer** (Skate) — "Built for the streets. Hits hard and takes hits."

Each starts at Level 5 with 2 moves.

## Sneaker Data Structure (Rust)

```rust
pub struct SneakerInstance {
    pub species_id: u16,          // References the SneakerSpecies
    pub nickname: Option<String>, // Player-assigned name
    pub level: u8,
    pub current_hp: u16,
    pub xp: u32,
    pub ivs: Stats,              // 0-31 each, set on encounter
    pub evs: Stats,              // 0-252 each, gained from battles
    pub condition: Condition,    // Nature equivalent
    pub moves: [Option<MoveSlot>; 4],
    pub status: Option<StatusCondition>,
    pub held_item: Option<ItemId>,
    pub friendship: u8,          // 0-255
    pub caught_location: MapId,
    pub original_trainer: String,
}

pub struct SneakerSpecies {
    pub id: u16,
    pub name: String,
    pub faction: Faction,
    pub base_stats: Stats,
    pub rarity_tier: RarityTier,
    pub base_catch_rate: u8,     // 0-255
    pub base_xp_yield: u16,
    pub ev_yield: Stats,         // Which EVs defeating this gives
    pub learnset: Vec<(u8, MoveId)>, // (level, move) pairs
    pub evolution: Option<Evolution>,
    pub description: String,
    pub height: f32,             // In cm
    pub weight: f32,             // In grams
}
```

## Sneaker Box

- Accessible at any Sneaker Clinic terminal
- Stores up to 50 sneakers (expandable post-MVP)
- Organized by faction tabs + search/sort
- Can swap party members freely
- Deposited sneakers retain all stats, moves, items
