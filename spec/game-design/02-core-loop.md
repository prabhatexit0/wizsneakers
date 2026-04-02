# Core Gameplay Loop

## Micro Loop (Moment-to-Moment, 30 seconds - 2 minutes)

```
EXPLORE → ENCOUNTER → BATTLE → REWARD → EXPLORE
```

1. **Explore**: Walk around the tile-based world. Talk to NPCs, find items, enter buildings.
2. **Encounter**: Step into tall grass / certain zones to trigger wild sneaker encounters or get challenged by rival Hypebeasts.
3. **Battle**: Turn-based combat. Choose moves, switch sneakers, use items.
4. **Reward**: Gain Clout XP, earn Drip Dollars ($DD), potentially acquire a new sneaker.
5. **Return to Explore**: Use rewards to heal, upgrade, and push further.

## Meso Loop (Session-Level, 15-45 minutes)

```
ARRIVE IN AREA → EXPLORE & COLLECT → CHALLENGE BOSS → UNLOCK NEW AREA
```

1. **Arrive**: Enter a new city, route, or zone. Get a feel for the local vibe and NPCs.
2. **Explore & Collect**: Battle wild sneakers, find hidden items, complete side quests, shop for gear.
3. **Challenge**: Take on the area's Hypebeast Boss (equivalent to a Gym Leader). Must have strong enough team.
4. **Progress**: Defeating the boss grants a Badge (Authentication Stamp), unlocking the path to the next area and new sneaker encounters.

## Macro Loop (Full Game, 10-20 hours)

```
START JOURNEY → BUILD COLLECTION → CONQUER BOSSES → FACE FINAL CHALLENGE → ENDGAME
```

1. **Chapter 1 — Fresh Out the Box**: Player gets their starter sneaker, learns mechanics, defeats first 2 bosses.
2. **Chapter 2 — Building the Rotation**: Player has 8-12 sneakers, understands type matchups, defeats bosses 3-5.
3. **Chapter 3 — Chasing Grails**: Story intensifies around Genesis Grails. Player faces bosses 6-8.
4. **Chapter 4 — The Drop**: Final confrontation. Player faces the Elite Resellers and the Champion.
5. **Postgame — Completionist**: Collect remaining sneakers, face optional superbosses, unlock secret areas.

## Reward Cadence

| Interval | Reward Type | Example |
|----------|------------|---------|
| Every 30 seconds | Small dopamine hit | Discovering a new tile, NPC quip, item pickup |
| Every 2-5 minutes | Battle victory | Clout XP, $DD, potential sneaker drop |
| Every 15-30 minutes | Major milestone | New sneaker acquired, new area unlocked |
| Every 1-2 hours | Boss victory | Authentication Stamp, story progression, rare sneaker |
| Every 5+ hours | Chapter completion | Major story beat, legendary encounter unlocked |

## Player Agency & Decision Points

### Team Composition
Players carry up to 6 sneakers at a time. Choosing the right mix of factions and moves for upcoming challenges is a core strategic decision.

### Resource Management
- **Drip Dollars ($DD)**: Spent on healing items, stat boosters, and shop sneakers. Earned from battles.
- **Sneaker Box**: Can store up to 50 sneakers. Must choose what to keep in active rotation.
- **Items**: Limited bag space forces choices about what consumables to carry.

### Exploration vs. Grinding
Players can push forward on the main path or grind encounters for XP and rare sneaker drops. The game should be beatable without excessive grinding if the player plays strategically, but grinding should feel rewarding for those who want to over-prepare.

## State Transitions

```
┌─────────────┐
│  OVERWORLD   │──── Enter Building ────►┌──────────┐
│  (Exploring) │                         │ INTERIOR  │
│              │◄─── Exit Building ──────│ (Shop/NPC)│
└──────┬───────┘                         └──────────┘
       │
       │ Random/Scripted Encounter
       ▼
┌─────────────┐
│   BATTLE     │──── Victory ────►  Reward Screen ──► Back to Overworld
│  (Turn-based)│
│              │──── Defeat  ────►  Blackout ──► Respawn at Last Sneaker Clinic
│              │──── Flee    ────►  Back to Overworld (no reward)
└──────┬───────┘
       │
       │ Open Menu (anytime in Overworld)
       ▼
┌─────────────┐
│    MENU      │  Sneaker Team / Bag / Save / Sneakerdex / Options
│  (Pause)     │
└─────────────┘
```

## Pacing Guidelines

- **Routes between cities**: 3-8 minutes of walking + encounters
- **Cities/Towns**: 5-15 minutes of exploration, shopping, NPC interaction
- **Boss battles**: 3-7 minutes of strategic combat
- **Story cutscenes**: Never longer than 2 minutes. Players can skip dialogue.
- **Difficulty curve**: Gradual. First boss should be beatable with starter sneaker at level 8-10. Final boss expects a well-built team at level 45-50.
