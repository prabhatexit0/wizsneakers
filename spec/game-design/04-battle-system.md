# Battle System

## Overview

Wizsneakers uses a **turn-based battle system** where equipped sneakers fight on behalf of the player. Each turn, both sides choose an action, and the order of execution is determined by the **Rarity** stat (speed equivalent). Battles are 1v1 (one active sneaker per side), with the ability to switch sneakers mid-battle.

## Battle Flow

```
┌─────────────────────────────────────┐
│         BATTLE START                │
│  (Encounter animation + intro)      │
└──────────────┬──────────────────────┘
               ▼
┌─────────────────────────────────────┐
│         ACTION SELECTION            │
│  Player chooses: Fight / Bag /      │
│  Sneakers / Run                     │
└──────────────┬──────────────────────┘
               ▼
┌─────────────────────────────────────┐
│         TURN RESOLUTION             │
│  1. Priority moves go first         │
│  2. Higher Rarity goes first        │
│  3. Ties broken randomly            │
│  4. Execute actions in order         │
└──────────────┬──────────────────────┘
               ▼
┌─────────────────────────────────────┐
│         END-OF-TURN EFFECTS         │
│  Status conditions tick down        │
│  Weather/field effects apply         │
└──────────────┬──────────────────────┘
               ▼
┌─────────────────────────────────────┐
│         CHECK WIN/LOSE              │
│  If either side's sneaker faints:   │
│  - Loser must switch or loses       │
│  - Winner gets XP / rewards         │
│  Else: return to ACTION SELECTION   │
└─────────────────────────────────────┘
```

## Action Types

### 1. Fight
Select one of up to 4 moves the active sneaker knows. Each move has a type, power, accuracy, PP (uses), and potential secondary effects.

### 2. Bag
Use an item from inventory:
- **Sole Sauce** — Heal 20 HP
- **Full Restore Spray** — Heal to full HP
- **Hype Potion** — Boost Hype (Attack) by 1 stage
- **Sneaker Case** — Attempt to catch a wild sneaker (wild battles only)
- Using an item consumes the player's turn.

### 3. Sneakers
Switch the active sneaker for another in the party. Switching consumes the player's turn, and the opponent attacks the incoming sneaker.

### 4. Run
Attempt to flee from a wild encounter. Success chance:
```
flee_chance = (player_rarity * 128 / opponent_rarity) + 30 * flee_attempts
if flee_chance > 255: guaranteed escape
```
Cannot flee from trainer/boss battles.

## Type System (Factions)

### The Four Factions

| Faction | Theme | Color | Playstyle |
|---------|-------|-------|-----------|
| **Retro** | Classic, vintage, OG | Red/White | Balanced, reliable stats |
| **Techwear** | Futuristic, performance | Blue/Black | High Drip (Sp.Atk), lower Comfort |
| **Skate** | Street, rugged, DIY | Green/Orange | High Hype (Atk), high Durability |
| **High-Fashion** | Luxury, avant-garde | Gold/Purple | High Rarity (Speed), glass cannon |

### Type Advantage Chart

```
             Defender
             Retro    Techwear   Skate    High-Fashion
Attacker  ┌─────────┬──────────┬─────────┬────────────┐
Retro     │  1.0x   │  0.5x    │  2.0x   │   1.0x     │
Techwear  │  2.0x   │  1.0x    │  0.5x   │   1.0x     │  (NEW)
Skate     │  0.5x   │  2.0x    │  1.0x   │   1.0x     │  (NEW)  
High-Fash │  1.0x   │  1.0x    │  1.0x   │   0.5x     │
          └─────────┴──────────┴─────────┴────────────┘

Super effective: 2.0x damage
Not very effective: 0.5x damage
Normal: 1.0x damage
```

**Mnemonic:**
- **Techwear beats Retro** — Innovation outpaces tradition
- **Retro beats Skate** — Classics never go out of style
- **Skate beats Techwear** — Street cred > tech specs
- **High-Fashion** — Neutral against others but weak to itself (fashion eats its own)

### Dual-Type Sneakers (Post-MVP)
Some sneakers can have two faction types. Damage multipliers stack:
- Double super effective: 4.0x
- Super effective + not very effective: 1.0x (cancels out)
- Double not very effective: 0.25x

## Damage Formula

```
damage = ((((2 * level / 5 + 2) * power * attack / defense) / 50) + 2)
         * STAB
         * type_effectiveness
         * critical
         * random_factor

Where:
- level: Attacker's level
- power: Move's base power
- attack: Attacker's Hype (physical) or Drip (special) stat
- defense: Defender's Comfort (physical) or Comfort (special) stat
- STAB: 1.5 if move type matches sneaker type, else 1.0
- type_effectiveness: 0.5, 1.0, or 2.0 (see chart above)
- critical: 1.5 on critical hit, else 1.0
- random_factor: Random float between 0.85 and 1.00
```

### Critical Hits
- Base crit rate: 1/16 (6.25%)
- High crit moves: 1/8 (12.5%)
- Crit ignores negative attack modifiers on attacker and positive defense modifiers on defender

### STAB (Same-Type Attack Bonus)
When a sneaker uses a move matching its own faction type, damage is multiplied by 1.5x. This encourages using moves that match your sneaker's type.

## Stat Stages

Stats can be modified during battle by moves and items. Each stat has stages from -6 to +6.

| Stage | Multiplier |
|-------|-----------|
| -6 | 2/8 (0.25x) |
| -5 | 2/7 (0.29x) |
| -4 | 2/6 (0.33x) |
| -3 | 2/5 (0.40x) |
| -2 | 2/4 (0.50x) |
| -1 | 2/3 (0.67x) |
| 0 | 2/2 (1.00x) |
| +1 | 3/2 (1.50x) |
| +2 | 4/2 (2.00x) |
| +3 | 5/2 (2.50x) |
| +4 | 6/2 (3.00x) |
| +5 | 7/2 (3.50x) |
| +6 | 8/2 (4.00x) |

## Status Conditions

| Status | Effect | Duration | Cure |
|--------|--------|----------|------|
| **Creased** | Lose 1/8 max HP per turn | Until healed | Sneaker Clinic, Crease Guard item |
| **Scuffed** | Attack reduced by 50% | 1-4 turns | Wears off, Buff Spray item |
| **Sold Out** | Cannot use moves (stunned) | 1-2 turns | Wears off |
| **Hypnotized** | 50% chance to hit self each turn | 1-4 turns | Wears off, Smelling Salts item |
| **Deflated** | Speed reduced by 75% | Until healed | Sneaker Clinic, Pump item |
| **On Fire** | +50% Hype but lose 1/10 HP per turn | 3 turns | Wears off |

Only one major status can be active at a time (Creased, Scuffed, Sold Out, Hypnotized, Deflated). "On Fire" is a volatile status and can stack with one major status.

## Wild Sneaker Capture

When encountering wild sneakers, players can throw a **Sneaker Case** to attempt capture.

```
catch_rate = ((3 * max_hp - 2 * current_hp) * base_catch_rate * case_bonus) / (3 * max_hp)

case_bonus:
- Basic Sneaker Case: 1.0x
- Premium Sneaker Case: 1.5x  
- Grail Case: 2.5x
- Master Case: Guaranteed catch

Shake checks: 4 checks, each must pass:
shake_threshold = 1048560 / sqrt(sqrt(16711680 / catch_rate))
Each check: random(0, 65535) < shake_threshold
```

Visual: The sneaker case shakes 0-3 times before either catching or breaking free. More shakes = closer to catching.

## AI Behavior (Opponents)

### Wild Sneakers
- Choose moves randomly, weighted slightly toward super-effective moves.
- Never use items or switch.

### Regular Hypebeasts (Trainers)
- Use super-effective moves when available.
- Switch sneakers when current one is at type disadvantage and has <50% HP.
- Use healing items when HP < 25% (limited supply: 1-2 items).

### Boss Hypebeasts
- Optimal type matchup awareness.
- Will switch to counter player's active sneaker.
- Use healing items strategically (2-4 items).
- May use stat-boosting moves when safe.
- Each boss has a signature move/strategy.

### Elite Resellers + Champion
- Full competitive AI.
- Team synergy and coverage.
- Prediction-based switching.
- Optimal item usage.
- Each has 6 sneakers with competitive movesets.

## Experience & Leveling

```
xp_gained = (base_xp * opponent_level * trainer_bonus) / 7

trainer_bonus: 1.0 for wild, 1.5 for trainer battles

Level-up threshold (medium-slow growth):
xp_needed(level) = (6/5) * level^3 - 15 * level^2 + 100 * level - 140
```

### Level Cap: 50 (MVP)
- Starter sneakers begin at level 5
- Final boss's team peaks at level 50
- Postgame content: level 55-60

### Moves Learned on Level-Up
Each sneaker has a predefined move learn list. When a new move is available and the sneaker already knows 4 moves, the player chooses which to replace (or skip learning the new move).

## Battle Rewards

| Battle Type | Clout XP | $DD | Sneaker Drop |
|-------------|----------|-----|-------------|
| Wild (common) | Base | 50-150 | Catchable |
| Wild (rare) | 1.5x Base | 100-250 | Catchable |
| Trainer | 1.5x Base | 200-500 | No |
| Boss | 3x Base | 1000-3000 | Signature sneaker gift |
| Elite Reseller | 4x Base | 3000-5000 | No |
| Champion | 5x Base | 10000 | Legendary sneaker |

## Battle UI Layout

```
┌──────────────────────────────────────────────────┐
│  ┌─────────────────┐                             │
│  │ OPPONENT SNEAKER │    [Opponent Sprite]        │
│  │ "Air Retro 1"   │         ████                │
│  │ Lv. 15          │        ██████               │
│  │ HP: ████████░░  │         ████                │
│  └─────────────────┘                             │
│                                                  │
│                          ┌─────────────────┐     │
│    [Player Sprite]       │ YOUR SNEAKER     │     │
│         ████             │ "Skate Blazer"   │     │
│        ██████            │ Lv. 14           │     │
│         ████             │ HP: ██████████   │     │
│                          │ XP: ████░░░░░░   │     │
│                          └─────────────────┘     │
│                                                  │
│  ┌───────────────────────────────────────────┐   │
│  │  What will Skate Blazer do?               │   │
│  │  ┌──────────┐  ┌──────────┐               │   │
│  │  │  FIGHT   │  │   BAG    │               │   │
│  │  ├──────────┤  ├──────────┤               │   │
│  │  │ SNEAKERS │  │   RUN    │               │   │
│  │  └──────────┘  └──────────┘               │   │
│  └───────────────────────────────────────────┘   │
└──────────────────────────────────────────────────┘
```

When FIGHT is selected:
```
┌───────────────────────────────────────────┐
│  ┌─────────────────┐ ┌─────────────────┐  │
│  │ Ankle Breaker    │ │ Crease          │  │
│  │ Skate / PP 10/10 │ │ Retro / PP 15/15│  │
│  ├─────────────────┤ ├─────────────────┤  │
│  │ Camp Out         │ │ Hype Drop       │  │
│  │ Normal / PP 5/5  │ │ Techwear/ PP 8/8│  │
│  └─────────────────┘ └─────────────────┘  │
└───────────────────────────────────────────┘
```
