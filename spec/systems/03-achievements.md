# Achievement System

## Authentication Stamps (Main Progression)

The 8 Authentication Stamps are the primary progression markers. See [Progression & Economy](../game-design/07-progression-economy.md) for details.

## Achievement Badges

Optional achievements that reward exploration and mastery. Displayed on the Player Card.

### Collection Badges

| Badge | Requirement | Reward |
|-------|-------------|--------|
| **First Cop** | Catch your first wild sneaker | — |
| **Starter Squad** | Have 6 sneakers in your party | 500 $DD |
| **Dozen Fresh** | Catch 12 different sneaker species | Premium Case x3 |
| **Gotta Cop 'Em All** | Complete the Sneakerdex (30/30) | Completion Certificate, Shiny Charm |
| **Faction Fan** | Catch all sneakers of one faction | Faction-themed held item |
| **Full Set** | Own one of each rarity tier | 5,000 $DD |
| **Genesis Hunter** | Obtain all 4 Genesis Grails | Access to Secret Area |

### Battle Badges

| Badge | Requirement | Reward |
|-------|-------------|--------|
| **First W** | Win your first battle | — |
| **Win Streak 10** | Win 10 battles in a row | 1,000 $DD |
| **Win Streak 25** | Win 25 battles in a row | 5,000 $DD |
| **Type Master** | Win a battle using type advantage | — |
| **Underdog** | Win a battle with a sneaker 10+ levels lower | 2,000 $DD |
| **Sweep** | Defeat a trainer's full team with one sneaker | 1,000 $DD |
| **No Items** | Defeat a boss without using items | Special title: "Purist" |
| **Crit King** | Land 50 critical hits total | Wide Lens |

### Exploration Badges

| Badge | Requirement | Reward |
|-------|-------------|--------|
| **Tourist** | Visit all 8 towns/cities | Town Map upgrade |
| **Hiker** | Walk 10,000 total steps | Repel x10 |
| **Marathon** | Walk 50,000 total steps | Super Repel x10 |
| **Hidden Gem** | Find a hidden item | — |
| **Treasure Hunter** | Find 20 hidden items | Dowsing App |
| **Shopaholic** | Spend 100,000 $DD total | 10% shop discount |
| **Side Hustler** | Complete all side quests | 10,000 $DD |

### Special Badges

| Badge | Requirement | Reward |
|-------|-------------|--------|
| **Hall of Fame** | Defeat the Champion | Postgame content unlocked |
| **Rematch Ready** | Defeat all bosses in postgame rematches | Golden Sneaker Trophy (cosmetic) |
| **Speedrunner** | Beat the game in under 4 hours (play time) | Special title: "Speed Lace" |
| **Pacifist Run** | Reach Lace-Up Village with only starter (no catches) | 2,000 $DD |
| **Fashionista** | Have a full party of High-Fashion sneakers | Special title: "Drip Lord" |

## Player Titles

Earned from achievements, displayed on Player Card and in multiplayer (post-MVP).

| Title | Source |
|-------|--------|
| Rookie | Default |
| Collector | Catch 15 sneakers |
| Purist | No Items badge |
| Drip Lord | Fashionista badge |
| Speed Lace | Speedrunner badge |
| Champion | Beat the game |
| Grail Keeper | Obtain all Genesis Grails |
| Legend | 100% all achievements |

## Tracking (Rust Side)

```rust
pub struct AchievementTracker {
    pub unlocked: HashSet<String>,    // Achievement IDs
    pub stats: PlayerStats,           // Running counters
}

pub struct PlayerStats {
    pub total_steps: u64,
    pub total_battles_won: u32,
    pub total_battles_lost: u32,
    pub current_win_streak: u32,
    pub best_win_streak: u32,
    pub total_sneakers_caught: u32,
    pub total_money_earned: u64,
    pub total_money_spent: u64,
    pub total_crits_landed: u32,
    pub total_items_found: u32,
    pub total_trainers_defeated: u32,
    pub bosses_defeated_no_items: HashSet<u16>,
}
```

After each relevant action, the engine checks if any new achievements are unlocked:

```rust
fn check_achievements(&mut self) -> Vec<String> {
    let mut newly_unlocked = Vec::new();
    
    if self.stats.total_steps >= 10000 && !self.unlocked.contains("hiker") {
        self.unlocked.insert("hiker".to_string());
        newly_unlocked.push("hiker".to_string());
    }
    // ... etc
    
    newly_unlocked
}
```

New achievements trigger a non-intrusive toast notification in the UI.
