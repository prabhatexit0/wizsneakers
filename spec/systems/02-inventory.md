# Inventory & Collection

## Bag System

### Bag Pockets

The player's bag is divided into 5 categorized pockets:

| Pocket | Max Unique Items | Description |
|--------|-----------------|-------------|
| **Heal Items** | 20 | HP restoration and status cures |
| **Battle Items** | 10 | Stat boosters and battle utilities |
| **Sneaker Cases** | 10 | Capture tools |
| **Key Items** | Unlimited | Story items, tools, stamps |
| **Held Items** | 15 | Equippable passive-effect items |

Items within each pocket stack (e.g., "Sole Sauce x15" = 1 slot).

### Complete Item List

#### Heal Items

| ID | Name | Cost | Effect |
|----|------|------|--------|
| 1 | Sole Sauce | 200 | Heal 20 HP |
| 2 | Insole Pad | 500 | Heal 50 HP |
| 3 | Full Restore Spray | 1,500 | Heal to full HP |
| 4 | Max Revive Lace | 3,000 | Revive fainted sneaker to full HP |
| 5 | Revival Thread | 1,000 | Revive fainted sneaker to 50% HP |
| 6 | Crease Guard | 300 | Cure Creased status |
| 7 | Buff Spray | 300 | Cure Scuffed status |
| 8 | Smelling Salts | 300 | Cure Hypnotized status |
| 9 | Pump | 300 | Cure Deflated status |
| 10 | Full Cleanse | 800 | Cure any status condition |
| 11 | PP Restore | 400 | Restore 10 PP to one move |
| 12 | PP Max | 1,200 | Restore all PP for one sneaker |

#### Battle Items (Usable in battle, consumed)

| ID | Name | Cost | Effect |
|----|------|------|--------|
| 20 | Hype Potion | 1,500 | +1 Hype stage |
| 21 | Drip Potion | 1,500 | +1 Drip stage |
| 22 | Guard Spray | 1,500 | +1 Comfort stage |
| 23 | Speed Lace | 1,500 | +1 Rarity stage |
| 24 | X-All | 5,000 | +1 to all stages |
| 25 | Crit Lens | 2,000 | Guaranteed crit on next move |
| 26 | Focus Sash | 3,000 | Survive one fatal hit at 1 HP |

#### Sneaker Cases

| ID | Name | Cost | Catch Multiplier |
|----|------|------|-----------------|
| 30 | Sneaker Case | 200 | 1.0x |
| 31 | Premium Case | 600 | 1.5x |
| 32 | Grail Case | 3,000 | 2.5x |
| 33 | Master Case | 50,000 | Guaranteed |
| 34 | Retro Case | 800 | 3.0x on Retro faction |
| 35 | Tech Case | 800 | 3.0x on Techwear faction |
| 36 | Skate Case | 800 | 3.0x on Skate faction |
| 37 | Fashion Case | 800 | 3.0x on High-Fashion faction |

#### Key Items

| ID | Name | Acquisition | Use |
|----|------|-------------|-----|
| 50 | Sneakerdex | Prof. Sole (start) | Track seen/caught sneakers |
| 51 | Town Map | Mom (start) | View world map |
| 52 | Escape Rope | Shop (300 DD) | Exit dungeon instantly |
| 53 | Repel | Shop (400 DD) | No encounters for 100 steps |
| 54 | Super Repel | Shop (700 DD) | No encounters for 200 steps |
| 55 | Authentication Stamp 1-8 | Bosses | Story progression, obedience |
| 56 | Syndicate Journal | Story (Ch.3) | Clue about Genesis Grails |
| 57 | Temple Key | Story (Ch.4) | Access Grailheim inner sanctum |
| 58 | Elevator Pass | Story (Ch.5) | Access Pinnacle Tower |
| 59 | Bicycle | Kicksburg (free, story) | 2x movement speed on routes |

#### Held Items

| ID | Name | Cost/Source | Effect |
|----|------|-------------|--------|
| 70 | Heritage Sole | Shop 2,000 | +10% Retro move power |
| 71 | Nano-Fiber Sole | Shop 2,000 | +10% Techwear move power |
| 72 | Skate Sole | Shop 2,000 | +10% Skate move power |
| 73 | Silk Insole | Hidden (Hypetown) | +10% High-Fashion move power |
| 74 | Snack Pack | Hidden (Route 5) | Heal 1/16 max HP per turn |
| 75 | Focus Band | Hidden (Grailheim) | 10% chance survive fatal hit at 1 HP |
| 76 | Quick Lace | Hidden (Neon Springs) | +1 priority to first move |
| 77 | Wide Lens | Shop 3,000 | +10% accuracy |
| 78 | Muscle Band | Hidden (Route 6) | +10% physical damage |
| 79 | Wise Glasses | Hidden (Route 7) | +10% special damage |
| 80 | Choice Band | Postgame | +50% Hype, locked to first move |
| 81 | Choice Specs | Postgame | +50% Drip, locked to first move |
| 82 | Choice Lace | Postgame | +50% Rarity, locked to first move |
| 83 | EV Band (Hype) | Postgame shop | Double Hype EVs gained |
| 84 | EV Band (All) | Postgame shop | Double all EVs gained |

## Sneaker Box

### Capacity
- **50 sneakers** (MVP)
- Organized by faction tabs: All / Retro / Techwear / Skate / High-Fashion
- Sortable by: Level, Rarity, Name, Caught Date, Faction

### Sneaker Box UI

```
┌──────────────────────────────────────────┐
│  SNEAKER BOX                   12/50     │
│  [All] [Retro] [Tech] [Skate] [HiFash]  │
│  ┌──────────────────────────────────────┐│
│  │ [Icon] Retro Runner    Lv.24  ★★    ││
│  │ [Icon] Classic Dunk    Lv.15  ★     ││
│  │ [Icon] Foam Cell       Lv.12  ★     ││
│  │ [Icon] OG Force        Lv.20  ★★    ││
│  │ [Icon] Grip Tape       Lv.18  ★     ││
│  │ ...                                  ││
│  └──────────────────────────────────────┘│
│  [A] Withdraw  [Y] Summary  [B] Back    │
│  Sort: [Level ▼]                         │
└──────────────────────────────────────────┘
```

### Operations
- **Deposit**: Move party sneaker to box (must keep at least 1 in party)
- **Withdraw**: Move box sneaker to party (party max 6)
- **Move item**: Transfer held item between sneakers
- **Release**: Permanently delete a sneaker (confirmation required, cannot release last sneaker)

## Sneakerdex

### Per-Species Data

```rust
pub struct SneakerdexEntry {
    pub species_id: u16,
    pub seen: bool,
    pub caught: bool,
    pub times_encountered: u32,
    pub times_caught: u32,
    pub first_seen_location: Option<u16>,
}
```

### Sneakerdex UI States

| State | Icon | Info Shown |
|-------|------|-----------|
| **Unseen** | `????` | Number only |
| **Seen** | Silhouette | Name, type, location hint |
| **Caught** | Full icon | Full stats, description, location, all forms |

### Completion Rewards

| Milestone | Reward |
|-----------|--------|
| See 10 sneakers | Sneaker Case x10 from Prof. Sole |
| Catch 15 sneakers | Premium Case x5 from Prof. Sole |
| See all 30 | Grail Case x3 from Prof. Sole |
| Catch all 30 | **Completion Certificate** + **Shiny Charm** (post-MVP: doubles rare encounter rate) |

## Item Pickup System

### Overworld Items
- Visible as glowing orbs on the map (item ball sprites)
- Walk over to pick up automatically
- Disappear after pickup, flagged in save data
- Contents defined per-map in the map JSON

### Hidden Items
- Not visible — player must interact with specific tiles
- Hinted by NPCs ("I heard someone lost something near the fountain...")
- Can use a "Dowsing App" key item (postgame) to detect nearby hidden items

### Post-Battle Drops
- Wild sneakers occasionally drop items on defeat (10% chance)
- Drop table varies by sneaker species
- E.g., Foam Cell drops "Foam Fragment" (sells for 500 DD)
