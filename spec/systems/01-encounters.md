# Encounter System

## Wild Encounters

### Trigger Conditions
- Player completes a step onto a **Tall Grass** tile
- Base encounter rate: **15% per step**
- Modified by:
  - Repel item active: 0% (blocks encounters for 100 steps)
  - Sprint: No change (encounter checks per tile, not per time)

### Encounter Table Structure

Each map zone has a weighted encounter table:

```rust
pub struct EncounterTable {
    pub entries: Vec<EncounterEntry>,
}

pub struct EncounterEntry {
    pub species_id: u16,
    pub level_min: u8,
    pub level_max: u8,
    pub weight: u32,        // Relative probability
    pub time_of_day: Option<TimeOfDay>,  // Post-MVP: day/night encounters
}
```

### Example: Route 1 Encounter Table

| Sneaker | Level Range | Weight | Effective Rate |
|---------|-------------|--------|---------------|
| Classic Dunk | 3-5 | 40 | 40% |
| Grip Tape | 3-5 | 35 | 35% |
| Retro Runner (wild) | 4-6 | 15 | 15% |
| Foam Cell | 4-6 | 10 | 10% |

### Encounter Level Scaling

Wild sneaker levels are determined by the map's level range. Within that range, levels follow a weighted distribution:
- 60% chance: middle of range
- 25% chance: low end
- 15% chance: high end

### Rare Encounter Mechanics

Some encounters are designated rare (Epic/Legendary sneakers):
- **Shiny grass**: Certain tiles have a golden sparkle — these have a separate, rarer encounter table
- **Chain encounters**: Defeating 5+ of the same species in a row increases rare encounter chance by 2x (stacks up to 5x at 25 chain)

## Trainer Encounters

### Line of Sight

Trainers standing on the map will challenge the player when they enter their line of sight.

```
Trainer facing right, sight range 4:

[T] → [1] [2] [3] [4]

If player enters tiles 1-4, battle triggers.
Blocked by walls/obstacles between trainer and player.
```

### Trainer Challenge Sequence

1. Trainer spots player (exclamation mark animation over trainer, 500ms)
2. Trainer walks toward player (or player walks to trainer if closer)
3. Pre-battle dialogue (1-2 lines)
4. Battle begins
5. Post-battle dialogue (1 line)
6. Trainer is marked as defeated (won't challenge again)

### Trainer Data

```rust
pub struct TrainerData {
    pub id: u16,
    pub name: String,
    pub class: TrainerClass,    // "Hypebeast", "Skater Kid", etc.
    pub team: Vec<TrainerSneaker>,
    pub items: Vec<(u16, u16)>,  // Items the AI can use
    pub ai_level: AiLevel,
    pub reward_money: u32,
    pub sight_range: u8,
    pub pre_battle_dialogue: String,
    pub post_battle_dialogue: String,
    pub defeated_dialogue: String,  // When talked to after defeat
}

pub struct TrainerSneaker {
    pub species_id: u16,
    pub level: u8,
    pub moves: Option<[u16; 4]>,  // Custom moveset, or None for default
    pub held_item: Option<u16>,
}

pub enum TrainerClass {
    Hypebeast,
    SkaterKid,
    TechNerd,
    FashionStudent,
    Reseller,
    Collector,
    SyndicateGrunt,
    Boss,
    EliteReseller,
    Champion,
}
```

## Boss Encounters

### Boss Battle Flow

```
1. Player enters gym / boss area
2. Gym puzzle (navigate maze, defeat trainers)
3. Reach boss NPC
4. Extended pre-battle dialogue (2-4 pages)
5. Battle starts (special boss BGM)
6. If player loses: respawn at Sneaker Clinic, gym puzzle resets, boss is re-fightable
7. If player wins:
   a. Post-battle dialogue
   b. Authentication Stamp awarded (animation + SFX)
   c. Boss gives a reward (TM, rare sneaker, or items)
   d. Boss is marked as defeated (can be rematched in postgame at higher levels)
```

### Boss Difficulty Scaling

| Boss # | Team Size | Item Count | AI Level | Recommended Player Level |
|--------|-----------|------------|----------|------------------------|
| 1 (Flip) | 1 | 0 | Basic | 7 |
| 2 (DJ Throwback) | 2 | 1 | Basic | 13 |
| 3 (Ollie McFlip) | 3 | 2 | Intermediate | 19 |
| 4 (Dr. Firmware) | 3 | 2 | Intermediate | 24 |
| 5 (Flex Queen) | 4 | 2 | Intermediate | 29 |
| 6 (Shadow Broker) | 4 | 2 | Advanced | 33 |
| 7 (Grand Master) | 5 | 3 | Advanced | 37 |
| 8 (King Markup) | 5 | 3 | Advanced | 43 |
| Elite (each) | 6 | 2 | Expert | 48 |
| Champion | 6 | 3 | Expert | 52 |

## Scripted Encounters

One-time story encounters that trigger at specific map positions:

```json
{
  "id": "route1_rival_battle",
  "x": 25,
  "y": 7,
  "trigger": "step",
  "condition": "!route1_rival_battled",
  "sequence": [
    { "type": "lock_player" },
    { "type": "spawn_npc", "npc": "flip", "x": 25, "y": 4, "facing": "down" },
    { "type": "npc_walk_to", "npc": "flip", "x": 25, "y": 6 },
    { "type": "dialogue", "id": "flip_route1_challenge" },
    { "type": "battle", "trainer_id": "flip_battle_1" },
    { "type": "dialogue", "id": "flip_route1_aftermath" },
    { "type": "set_flag", "flag": "route1_rival_battled" },
    { "type": "npc_walk_to", "npc": "flip", "x": 25, "y": 0 },
    { "type": "despawn_npc", "npc": "flip" },
    { "type": "unlock_player" }
  ]
}
```

## Encounter Transition Animation

### Wild Encounter
1. Screen flash (100ms white)
2. Horizontal bar wipe (bars slide in from alternating sides, 400ms)
3. Battle scene fades in (200ms)
4. Wild sneaker slides in from right (300ms)
5. Player sneaker slides in from left (300ms)
6. "A wild [Sneaker Name] appeared!" message

### Trainer Encounter
1. Exclamation mark over trainer (500ms)
2. Trainer walks to player
3. Screen flash (same as wild)
4. Battle scene with trainer portrait visible
5. "[Trainer Class] [Name] wants to battle!" message

### Boss Encounter
1. Dramatic screen shake (200ms)
2. Slow fade to black (500ms)
3. Boss silhouette reveal (500ms)
4. Full boss art reveal (500ms)
5. Boss battle BGM starts
6. Battle scene with boss portrait
