# Moves & Abilities

## Move Structure

Every move has the following properties:

```rust
pub struct Move {
    pub id: u16,
    pub name: String,
    pub faction: Faction,          // Or "Normal" for typeless
    pub category: MoveCategory,    // Physical, Special, Status
    pub power: Option<u8>,         // None for status moves
    pub accuracy: u8,              // 1-100 (percentage)
    pub pp: u8,                    // Uses before needing restoration
    pub priority: i8,              // -1 to +2 (higher goes first regardless of speed)
    pub effect: Option<MoveEffect>,
    pub description: String,
}

pub enum MoveCategory {
    Physical,  // Uses Hype (attack) vs Comfort (defense)
    Special,   // Uses Drip (sp.atk) vs Comfort (defense)
    Status,    // No direct damage, applies effects
}
```

## Complete Move List (MVP — 48 Moves)

### Normal-Type Moves (No faction — available to all)

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Description |
|---|------|-----|-------|-----|----|----|--------|-------------|
| 1 | **Lace Up** | Status | — | — | 20 | 0 | +1 Hype | "Tighten those laces. Time to get serious." |
| 2 | **Flex** | Status | — | 100 | 15 | 0 | -1 opponent Comfort | "Show off your kicks. Opponent's guard drops." |
| 3 | **Camp Out** | Status | — | — | 5 | 0 | Heal 50% max HP | "Set up camp outside the store. Rest and recover." |
| 4 | **Quick Step** | Physical | 40 | 100 | 30 | +1 | Always goes first | "A swift step that catches opponents off guard." |
| 5 | **Stomp** | Physical | 65 | 100 | 20 | 0 | 30% flinch | "Bring the sole down hard." |
| 6 | **Double Up** | Physical | 35 | 90 | 15 | 0 | Hits twice | "Buy two pairs. Hit twice." |
| 7 | **Deadstock Strike** | Physical | 80 | 100 | 15 | 0 | None | "A pristine, powerful hit." |
| 8 | **Hype Train** | Special | 90 | 85 | 10 | 0 | None | "Ride the wave of hype for massive damage." |
| 9 | **Resell** | Status | — | — | 10 | 0 | Swap stat changes with opponent | "Flip the script. Their gains are yours now." |
| 10 | **Authenticate** | Status | — | 100 | 5 | 0 | Remove opponent stat boosts | "Call out the fakes. Reset their buffs." |

### Retro Faction Moves

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Description |
|---|------|-----|-------|-----|----|----|--------|-------------|
| 11 | **Crease** | Physical | 40 | 100 | 25 | 0 | 10% lower opponent Comfort | "Bend the toe box. Classic damage." |
| 12 | **Throwback** | Physical | 60 | 95 | 20 | 0 | None | "An old-school hit that still slaps." |
| 13 | **Vintage Slam** | Physical | 85 | 90 | 15 | 0 | None | "Channel the power of the OGs." |
| 14 | **Heritage Crush** | Physical | 120 | 80 | 5 | 0 | User loses 33% recoil | "Devastating power, but it takes a toll." |
| 15 | **Retro Wave** | Special | 70 | 100 | 15 | 0 | None | "A wave of nostalgia washes over the opponent." |
| 16 | **Classic Aura** | Status | — | — | 10 | 0 | +1 Hype, +1 Comfort | "The timeless energy of a true classic." |
| 17 | **OG Stamp** | Special | 95 | 90 | 10 | 0 | 20% Scuffed status | "Mark them with the seal of the originals." |
| 18 | **Grail Beam** | Special | 130 | 85 | 5 | 0 | -1 Drip after use | "Unleash the power of a true grail. Draining." |

### Techwear Faction Moves

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Description |
|---|------|-----|-------|-----|----|----|--------|-------------|
| 19 | **Firmware Update** | Status | — | — | 10 | 0 | +2 Drip | "Patch your sneaker's software for more power." |
| 20 | **Shock Drop** | Special | 45 | 100 | 25 | 0 | 10% Deflated status | "A sudden digital release. Zap!" |
| 21 | **Bluetooth Blast** | Special | 65 | 95 | 20 | 0 | None | "Wireless destruction from range." |
| 22 | **Data Mine** | Special | 50 | 100 | 15 | 0 | Steals 50% of damage as HP | "Hack their systems. Drain their energy." |
| 23 | **Neon Pulse** | Special | 80 | 100 | 15 | 0 | None | "A pulse of electric neon energy." |
| 24 | **Overclock** | Status | — | — | 5 | 0 | +1 Drip, +1 Rarity, -1 Comfort | "Push beyond limits. Faster, stronger, but fragile." |
| 25 | **System Crash** | Special | 100 | 85 | 10 | 0 | 30% Sold Out status | "Total system failure. May stun the opponent." |
| 26 | **Quantum Leap** | Special | 130 | 80 | 5 | 0 | Skip next turn | "Teleport through dimensions. Needs recharge." |

### Skate Faction Moves

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Description |
|---|------|-----|-------|-----|----|----|--------|-------------|
| 27 | **Kickflip** | Physical | 45 | 100 | 25 | 0 | High crit rate (1/8) | "A clean kickflip. Style and substance." |
| 28 | **Ankle Breaker** | Physical | 70 | 90 | 15 | 0 | High crit rate (1/8) | "Cross 'em up so hard their ankles snap." |
| 29 | **Grind Rail** | Physical | 60 | 95 | 20 | 0 | +1 Rarity after | "Grind and gain momentum." |
| 30 | **Board Slide** | Physical | 80 | 100 | 15 | 0 | None | "Slide across with devastating force." |
| 31 | **Tre Flip** | Physical | 95 | 85 | 10 | 0 | None | "Triple the flip, triple the pain." |
| 32 | **Skater's Resolve** | Status | — | — | 10 | 0 | +2 Hype | "Get back up. Every time. Hit harder." |
| 33 | **Vulc Smash** | Physical | 110 | 85 | 10 | 0 | 20% Creased status | "A vulcanized sole slams down with authority." |
| 34 | **900 Spin** | Physical | 140 | 75 | 5 | 0 | Hypnotized if misses self | "The legendary 900. If you land it..." |

### High-Fashion Faction Moves

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Description |
|---|------|-----|-------|-----|----|----|--------|-------------|
| 35 | **Runway Strike** | Special | 45 | 100 | 25 | 0 | None | "Strike a pose, then strike your foe." |
| 36 | **Label Drop** | Special | 65 | 95 | 20 | 0 | -1 opponent Comfort | "Name-drop your way to dominance." |
| 37 | **Haute Beam** | Special | 80 | 100 | 15 | 0 | None | "A beam of pure haute couture energy." |
| 38 | **Price Tag** | Special | 50 | 100 | 15 | 0 | Power = user level * 1.5 | "More expensive = more powerful." |
| 39 | **Red Carpet** | Status | — | — | 10 | 0 | +2 Rarity | "Roll out the red carpet. Maximum speed." |
| 40 | **Fashion Police** | Special | 70 | 90 | 15 | 0 | 30% Scuffed status | "You're under arrest for crimes against fashion." |
| 41 | **Couture Cannon** | Special | 110 | 85 | 10 | 0 | -1 opponent Drip | "Blast them with concentrated luxury." |
| 42 | **Limited Edition** | Special | 150 | 70 | 3 | 0 | None | "Rare. Exclusive. Devastating." |

### Signature Boss Moves (Learned only by specific sneakers)

| # | Name | Cat | Power | Acc | PP | Pri | Effect | Owner |
|---|------|-----|-------|-----|----|----|--------|-------|
| 43 | **Vinyl Scratch** | Physical | 90 | 95 | 10 | 0 | 50% Scuffed | Heritage Court |
| 44 | **Debug Protocol** | Special | 90 | 95 | 10 | 0 | Removes target status + deals damage | Quantum Sole |
| 45 | **50-50 Grind** | Physical | 90 | 95 | 10 | 0 | +1 Hype, +1 Rarity | Board Destroyer |
| 46 | **Vogue Strike** | Special | 90 | 95 | 10 | 0 | Always crits on super-effective | Maison Sole |
| 47 | **Resell Markup** | Special | — | 100 | 5 | 0 | Deals damage = 50% of target's current HP | Triple Black |
| 48 | **Genesis Aura** | Special | 100 | 100 | 5 | 0 | Heals 25% of damage dealt | All Genesis Grails |

## Move Learning by Level (Starter Examples)

### Retro Runner Line
| Level | Retro Runner | Retro Runner II | Retro Runner Max |
|-------|-------------|-----------------|------------------|
| 1 | Stomp | — | — |
| 5 | Crease | — | — |
| 9 | Lace Up | — | — |
| 13 | Throwback | — | — |
| 16 | *Evolves* | Retro Wave | — |
| 20 | — | Vintage Slam | — |
| 25 | — | Classic Aura | — |
| 30 | — | OG Stamp | — |
| 32 | — | *Evolves* | Heritage Crush |
| 36 | — | — | Grail Beam |
| 40 | — | — | Deadstock Strike |

### Tech Trainer Line
| Level | Tech Trainer | Tech Trainer Pro | Tech Trainer Ultra |
|-------|-------------|-----------------|-------------------|
| 1 | Quick Step | — | — |
| 5 | Shock Drop | — | — |
| 9 | Flex | — | — |
| 13 | Bluetooth Blast | — | — |
| 16 | *Evolves* | Firmware Update | — |
| 20 | — | Neon Pulse | — |
| 25 | — | Data Mine | — |
| 30 | — | System Crash | — |
| 32 | — | *Evolves* | Overclock |
| 36 | — | — | Quantum Leap |
| 40 | — | — | Hype Train |

### Skate Blazer Line
| Level | Skate Blazer | Skate Blazer Pro | Skate Blazer Elite |
|-------|-------------|-----------------|-------------------|
| 1 | Stomp | — | — |
| 5 | Kickflip | — | — |
| 9 | Lace Up | — | — |
| 13 | Ankle Breaker | — | — |
| 16 | *Evolves* | Grind Rail | — |
| 20 | — | Board Slide | — |
| 25 | — | Skater's Resolve | — |
| 30 | — | Tre Flip | — |
| 32 | — | *Evolves* | Vulc Smash |
| 36 | — | — | 900 Spin |
| 40 | — | — | Deadstock Strike |

## Passive Abilities (Post-MVP Consideration)

Each sneaker species could have a passive ability that provides a small persistent bonus:

| Ability | Effect | Example Sneakers |
|---------|--------|-----------------|
| **Box Fresh** | Immune to Creased status | Retro Runner line |
| **Auto-Lace** | +1 Rarity at battle start | Tech Trainer line |
| **Grip Soles** | Cannot have Rarity lowered | Skate Blazer line |
| **Head Turner** | -1 opponent Hype at battle start | Runway Slip, Couture Boot |
| **Sole Survivor** | +50% Hype when last sneaker standing | Half-Pipe, Vulcanized |
| **Hype Beast** | Moves with <60 power get +50% power | Grip Tape, Foam Cell |
| **Limited Run** | Super-effective moves do 1.25x instead of 1.0x (no weakness, enhanced strength) | All Legendary sneakers |
