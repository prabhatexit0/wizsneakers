# Rust Core Specification

## Entry Point: GameEngine

The `GameEngine` is the single struct exposed to JavaScript via `wasm-bindgen`. All game logic is accessed through it.

```rust
#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
    battle: Option<BattleState>,
    rng: SeededRng,
    data: &'static GameData,  // Static reference to all game data
}

#[wasm_bindgen]
impl GameEngine {
    /// Initialize a new game
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> GameEngine;
    
    /// Load from a save file (JSON)
    pub fn load(save_json: &str) -> Result<GameEngine, JsValue>;
    
    /// Serialize current state for saving
    pub fn save(&self) -> String;
    
    /// Main tick — called every frame during overworld
    /// Returns JSON of the current render state
    pub fn tick(&mut self, dt_ms: f64, input: &str) -> String;
    
    /// Submit a battle action
    /// Returns JSON of battle turn results
    pub fn battle_action(&mut self, action: &str) -> String;
    
    /// Get current game mode
    pub fn mode(&self) -> String;  // "overworld" | "battle" | "dialogue" | "menu"
    
    /// Get full state snapshot (for UI)
    pub fn get_state(&self) -> String;
    
    /// Get battle state (if in battle)
    pub fn get_battle_state(&self) -> Option<String>;
    
    /// Interact with whatever is in front of the player
    pub fn interact(&mut self) -> String;
    
    /// Get player party summary (for menu)
    pub fn get_party(&self) -> String;
    
    /// Get inventory (for menu)
    pub fn get_inventory(&self) -> String;
    
    /// Get Sneakerdex data
    pub fn get_sneakerdex(&self) -> String;
}
```

## Core Models

### Faction & Type System

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Faction {
    Normal,
    Retro,
    Techwear,
    Skate,
    HighFashion,
}

impl Faction {
    /// Returns the damage multiplier when this faction attacks the defender
    pub fn effectiveness_against(&self, defender: Faction) -> f64 {
        match (self, defender) {
            (Faction::Retro, Faction::Skate) => 2.0,
            (Faction::Retro, Faction::Techwear) => 0.5,
            (Faction::Techwear, Faction::Retro) => 2.0,
            (Faction::Techwear, Faction::Skate) => 0.5,
            (Faction::Skate, Faction::Techwear) => 2.0,
            (Faction::Skate, Faction::Retro) => 0.5,
            (Faction::HighFashion, Faction::HighFashion) => 0.5,
            _ => 1.0,
        }
    }
}
```

### Stats

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stats {
    pub durability: u16,  // HP
    pub hype: u16,        // Attack
    pub comfort: u16,     // Defense
    pub drip: u16,        // Special Attack
    pub rarity: u16,      // Speed
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StatStages {
    pub hype: i8,     // -6 to +6
    pub comfort: i8,
    pub drip: i8,
    pub rarity: i8,
}

impl StatStages {
    pub fn multiplier(stage: i8) -> f64 {
        let stage = stage.clamp(-6, 6);
        if stage >= 0 {
            (2 + stage as u16) as f64 / 2.0
        } else {
            2.0 / (2 + (-stage) as u16) as f64
        }
    }
}
```

### Sneaker Models

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    Deadstock,       // +Hype, -Rarity
    Beat,            // +Comfort, -Drip
    Restored,        // +Drip, -Hype
    Custom,          // +Rarity, -Comfort
    Vintage,         // +Hype, -Comfort
    Prototype,       // +Drip, -Rarity
    PlayerExclusive, // +Rarity, -Hype
    Sample,          // +Comfort, -Drip
    GeneralRelease,  // Neutral
}

impl Condition {
    pub fn modifier(&self, stat: StatKind) -> f64 {
        // Returns 0.9, 1.0, or 1.1 based on condition + stat
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RarityTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SneakerSpecies {
    pub id: u16,
    pub name: &'static str,
    pub faction: Faction,
    pub base_stats: Stats,
    pub rarity_tier: RarityTier,
    pub base_catch_rate: u8,
    pub base_xp_yield: u16,
    pub ev_yield: Stats,
    pub learnset: Vec<(u8, u16)>,  // (level, move_id)
    pub evolution: Option<(u8, u16)>,  // (level, target_species_id)
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SneakerInstance {
    pub uid: u64,             // Unique instance ID
    pub species_id: u16,
    pub nickname: Option<String>,
    pub level: u8,
    pub xp: u32,
    pub current_hp: u16,
    pub max_hp: u16,          // Cached, recalculated on level up
    pub ivs: Stats,           // 0-31 per stat
    pub evs: Stats,           // 0-252 per stat, max 510 total
    pub condition: Condition,
    pub moves: [Option<MoveSlot>; 4],
    pub status: Option<StatusCondition>,
    pub held_item: Option<u16>,
    pub friendship: u8,
    pub caught_location: u16,
    pub original_trainer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSlot {
    pub move_id: u16,
    pub current_pp: u8,
    pub max_pp: u8,
}

impl SneakerInstance {
    /// Calculate effective stat at current level
    pub fn calc_stat(&self, stat: StatKind) -> u16 {
        let species = GameData::get_species(self.species_id);
        let base = species.base_stats.get(stat);
        let iv = self.ivs.get(stat);
        let ev = self.evs.get(stat);
        
        if stat == StatKind::Durability {
            ((2 * base + iv + ev / 4) as u32 * self.level as u32 / 100
                + self.level as u32 + 10) as u16
        } else {
            let raw = ((2 * base + iv + ev / 4) as u32 * self.level as u32 / 100 + 5) as f64;
            (raw * self.condition.modifier(stat)) as u16
        }
    }
    
    /// Generate a new wild sneaker with random IVs and condition
    pub fn generate_wild(species_id: u16, level: u8, rng: &mut SeededRng) -> Self;
    
    /// Check and apply evolution if eligible
    pub fn check_evolution(&self) -> Option<u16>;
    
    /// Add XP and return (leveled_up, new_moves_available)
    pub fn add_xp(&mut self, amount: u32) -> (bool, Vec<u16>);
    
    /// Is this sneaker fainted?
    pub fn is_fainted(&self) -> bool { self.current_hp == 0 }
    
    /// Get display name (nickname or species name)
    pub fn display_name(&self) -> &str;
}
```

### GameState

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub mode: GameMode,
    pub player: PlayerState,
    pub current_map: u16,
    pub event_flags: HashSet<String>,
    pub story_progress: u8,       // Chapter/checkpoint tracker
    pub play_time_ms: u64,
    pub authentication_stamps: [bool; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Overworld,
    Battle,
    Dialogue,
    Menu,
    Cutscene,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub x: u16,                   // Tile X position
    pub y: u16,                   // Tile Y position  
    pub facing: Direction,
    pub party: Vec<SneakerInstance>,  // Up to 6
    pub sneaker_box: Vec<SneakerInstance>,  // Up to 50
    pub bag: Inventory,
    pub money: u32,               // Drip Dollars
    pub sneakerdex: SneakerdexData,
    pub moving: bool,             // Currently in movement animation
    pub move_progress: f32,       // 0.0 to 1.0 for smooth tile transition
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub heal_items: Vec<(u16, u16)>,    // (item_id, quantity)
    pub battle_items: Vec<(u16, u16)>,
    pub sneaker_cases: Vec<(u16, u16)>,
    pub key_items: Vec<u16>,
    pub held_items: Vec<(u16, u16)>,
}
```

### BattleState

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    pub kind: BattleKind,
    pub player_active: usize,      // Index into player's party
    pub opponent: BattleOpponent,
    pub opponent_active: usize,
    pub turn_number: u16,
    pub player_stages: StatStages,
    pub opponent_stages: StatStages,
    pub weather: Option<Weather>,
    pub turn_log: Vec<BattleTurnEvent>,
    pub flee_attempts: u8,
    pub can_flee: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleKind {
    Wild,
    Trainer { trainer_id: u16, trainer_name: String },
    Boss { boss_id: u16, boss_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleOpponent {
    pub team: Vec<SneakerInstance>,
    pub items: Vec<(u16, u16)>,     // AI's usable items
    pub ai_level: AiLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AiLevel {
    Random,        // Wild sneakers
    Basic,         // Route trainers
    Intermediate,  // City trainers, early bosses
    Advanced,      // Late bosses
    Expert,        // Elite Resellers, Champion
}

/// Events that happen during a battle turn, used for animation sequencing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleTurnEvent {
    MoveUsed { attacker: BattleSide, move_id: u16 },
    Damage { target: BattleSide, amount: u16, effectiveness: Effectiveness },
    StatChange { target: BattleSide, stat: StatKind, stages: i8 },
    StatusApplied { target: BattleSide, status: StatusCondition },
    StatusDamage { target: BattleSide, amount: u16 },
    Healed { target: BattleSide, amount: u16 },
    Fainted { side: BattleSide },
    SwitchedIn { side: BattleSide, sneaker_name: String },
    ItemUsed { side: BattleSide, item_id: u16 },
    FleeAttempt { success: bool },
    CaptureAttempt { shakes: u8, success: bool },
    XpGained { amount: u32 },
    LevelUp { new_level: u8 },
    MoveLearnPrompt { move_id: u16 },
    EvolutionPrompt { new_species_id: u16 },
    BattleEnd { result: BattleResult },
    Message { text: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BattleSide { Player, Opponent }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Effectiveness { SuperEffective, Normal, NotVeryEffective }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BattleResult { PlayerWin, PlayerLose, PlayerFlee, PlayerCapture }
```

## RNG

All randomness uses a seeded PRNG (xorshift64) to ensure deterministic behavior when needed (e.g., replays, testing).

```rust
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self;
    pub fn next_u64(&mut self) -> u64;
    pub fn next_f64(&mut self) -> f64;          // 0.0 to 1.0
    pub fn range(&mut self, min: u32, max: u32) -> u32;
    pub fn chance(&mut self, percent: u8) -> bool;
}
```

## Testing Strategy

### Unit Tests (Rust)
- Damage formula with known inputs → expected outputs
- Type effectiveness chart completeness
- Stat calculation at various levels
- Capture rate formula
- XP/leveling thresholds
- Movement + collision detection
- AI decision-making given specific board states

### Integration Tests
- Full battle simulation (player vs AI) from start to finish
- Encounter generation over 1000 steps (verify rates)
- Save → load → verify state identity
- Evolution triggers at correct levels
