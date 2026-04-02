use crate::models::faction::Faction;
use crate::models::sneaker::{RarityTier, SneakerSpecies};
use crate::models::stats::Stats;

// ── Retro Faction ─────────────────────────────────────────────────────────────

static RETRO_RUNNER_LEARNSET: &[(u8, u16)] = &[
    (1, 5),   // Stomp
    (5, 11),  // Crease
    (9, 1),   // Lace Up
    (13, 12), // Throwback
];

pub static RETRO_RUNNER: SneakerSpecies = SneakerSpecies {
    id: 1,
    name: "Retro Runner",
    faction: Faction::Retro,
    base_stats: Stats { durability: 45, hype: 50, comfort: 40, drip: 35, rarity: 45 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 200,
    base_xp_yield: 64,
    ev_yield: Stats { durability: 0, hype: 1, comfort: 0, drip: 0, rarity: 0 },
    learnset: RETRO_RUNNER_LEARNSET,
    evolution: Some((16, 2)),
    description: "A classic runner that never goes out of style. Balanced and reliable.",
};

static RETRO_RUNNER_II_LEARNSET: &[(u8, u16)] = &[
    (16, 15), // Retro Wave
    (20, 13), // Vintage Slam
    (25, 16), // Classic Aura
    (30, 17), // OG Stamp
];

pub static RETRO_RUNNER_II: SneakerSpecies = SneakerSpecies {
    id: 2,
    name: "Retro Runner II",
    faction: Faction::Retro,
    base_stats: Stats { durability: 60, hype: 70, comfort: 55, drip: 50, rarity: 60 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 120,
    base_xp_yield: 142,
    ev_yield: Stats { durability: 0, hype: 2, comfort: 0, drip: 0, rarity: 0 },
    learnset: RETRO_RUNNER_II_LEARNSET,
    evolution: Some((32, 3)),
    description: "The sequel to a legend. Still rocking the classics, but harder.",
};

static RETRO_RUNNER_MAX_LEARNSET: &[(u8, u16)] = &[
    (32, 14), // Heritage Crush
    (36, 18), // Grail Beam
    (40, 7),  // Deadstock Strike
];

pub static RETRO_RUNNER_MAX: SneakerSpecies = SneakerSpecies {
    id: 3,
    name: "Retro Runner Max",
    faction: Faction::Retro,
    base_stats: Stats { durability: 80, hype: 90, comfort: 70, drip: 65, rarity: 80 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 45,
    base_xp_yield: 236,
    ev_yield: Stats { durability: 0, hype: 3, comfort: 0, drip: 0, rarity: 0 },
    learnset: RETRO_RUNNER_MAX_LEARNSET,
    evolution: None,
    description: "The pinnacle of retro design. Maximum power, maximum style.",
};

static CLASSIC_DUNK_LEARNSET: &[(u8, u16)] = &[
    (1, 5),   // Stomp
    (5, 11),  // Crease
    (9, 2),   // Flex
    (13, 12), // Throwback
];

pub static CLASSIC_DUNK: SneakerSpecies = SneakerSpecies {
    id: 4,
    name: "Classic Dunk",
    faction: Faction::Retro,
    base_stats: Stats { durability: 55, hype: 45, comfort: 50, drip: 30, rarity: 35 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 200,
    base_xp_yield: 56,
    ev_yield: Stats { durability: 1, hype: 0, comfort: 0, drip: 0, rarity: 0 },
    learnset: CLASSIC_DUNK_LEARNSET,
    evolution: Some((20, 5)),
    description: "A court classic with solid stats all around. Found on Route 1-2.",
};

static OG_FORCE_LEARNSET: &[(u8, u16)] = &[
    (20, 16), // Classic Aura
    (25, 17), // OG Stamp
    (30, 15), // Retro Wave
    (38, 13), // Vintage Slam
];

pub static OG_FORCE: SneakerSpecies = SneakerSpecies {
    id: 5,
    name: "OG Force",
    faction: Faction::Retro,
    base_stats: Stats { durability: 70, hype: 65, comfort: 70, drip: 45, rarity: 50 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 90,
    base_xp_yield: 158,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 2, drip: 0, rarity: 0 },
    learnset: OG_FORCE_LEARNSET,
    evolution: None,
    description: "The force of the originals. Tough, iconic, unstoppable.",
};

static VINTAGE_HIGH_TOP_LEARNSET: &[(u8, u16)] = &[
    (1, 12),  // Throwback
    (5, 2),   // Flex
    (10, 15), // Retro Wave
    (20, 16), // Classic Aura
    (30, 17), // OG Stamp
    (40, 18), // Grail Beam
];

pub static VINTAGE_HIGH_TOP: SneakerSpecies = SneakerSpecies {
    id: 6,
    name: "Vintage High-Top",
    faction: Faction::Retro,
    base_stats: Stats { durability: 65, hype: 80, comfort: 55, drip: 70, rarity: 85 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 60,
    base_xp_yield: 175,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 0, rarity: 2 },
    learnset: VINTAGE_HIGH_TOP_LEARNSET,
    evolution: None,
    description: "A rare find on Route 2. High-top style with surprisingly quick moves.",
};

static HERITAGE_COURT_LEARNSET: &[(u8, u16)] = &[
    (1, 13),  // Vintage Slam
    (10, 16), // Classic Aura
    (20, 17), // OG Stamp
    (30, 18), // Grail Beam
    (40, 43), // Vinyl Scratch (signature)
];

pub static HERITAGE_COURT: SneakerSpecies = SneakerSpecies {
    id: 7,
    name: "Heritage Court",
    faction: Faction::Retro,
    base_stats: Stats { durability: 85, hype: 90, comfort: 75, drip: 85, rarity: 90 },
    rarity_tier: RarityTier::Epic,
    base_catch_rate: 30,
    base_xp_yield: 220,
    ev_yield: Stats { durability: 0, hype: 2, comfort: 0, drip: 1, rarity: 0 },
    learnset: HERITAGE_COURT_LEARNSET,
    evolution: None,
    description: "A boss reward from DJ Throwback. The court's ultimate champion.",
};

static GENESIS_JORDAN_LEARNSET: &[(u8, u16)] = &[
    (1, 18),  // Grail Beam
    (10, 7),  // Deadstock Strike
    (20, 8),  // Hype Train
    (30, 14), // Heritage Crush
    (40, 48), // Genesis Aura
];

pub static GENESIS_JORDAN: SneakerSpecies = SneakerSpecies {
    id: 8,
    name: "Genesis Jordan",
    faction: Faction::Retro,
    base_stats: Stats { durability: 95, hype: 110, comfort: 85, drip: 95, rarity: 100 },
    rarity_tier: RarityTier::Legendary,
    base_catch_rate: 3,
    base_xp_yield: 270,
    ev_yield: Stats { durability: 0, hype: 3, comfort: 0, drip: 0, rarity: 0 },
    learnset: GENESIS_JORDAN_LEARNSET,
    evolution: None,
    description: "The first Genesis Grail. Its power transcends generations.",
};

// ── Techwear Faction ──────────────────────────────────────────────────────────

static TECH_TRAINER_LEARNSET: &[(u8, u16)] = &[
    (1, 4),   // Quick Step
    (5, 20),  // Shock Drop
    (9, 2),   // Flex
    (13, 21), // Bluetooth Blast
];

pub static TECH_TRAINER: SneakerSpecies = SneakerSpecies {
    id: 9,
    name: "Tech Trainer",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 40, hype: 35, comfort: 40, drip: 55, rarity: 45 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 200,
    base_xp_yield: 64,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 1, rarity: 0 },
    learnset: TECH_TRAINER_LEARNSET,
    evolution: Some((16, 10)),
    description: "Cutting-edge performance tech. Strong special attacks.",
};

static TECH_TRAINER_PRO_LEARNSET: &[(u8, u16)] = &[
    (16, 19), // Firmware Update
    (20, 23), // Neon Pulse
    (25, 22), // Data Mine
    (30, 25), // System Crash
];

pub static TECH_TRAINER_PRO: SneakerSpecies = SneakerSpecies {
    id: 10,
    name: "Tech Trainer Pro",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 55, hype: 50, comfort: 55, drip: 75, rarity: 60 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 120,
    base_xp_yield: 142,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 2, rarity: 0 },
    learnset: TECH_TRAINER_PRO_LEARNSET,
    evolution: Some((32, 11)),
    description: "Pro-grade technology with advanced firmware. Upgraded special power.",
};

static TECH_TRAINER_ULTRA_LEARNSET: &[(u8, u16)] = &[
    (32, 24), // Overclock
    (36, 26), // Quantum Leap
    (40, 8),  // Hype Train
];

pub static TECH_TRAINER_ULTRA: SneakerSpecies = SneakerSpecies {
    id: 11,
    name: "Tech Trainer Ultra",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 70, hype: 65, comfort: 70, drip: 100, rarity: 80 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 45,
    base_xp_yield: 236,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 3, rarity: 0 },
    learnset: TECH_TRAINER_ULTRA_LEARNSET,
    evolution: None,
    description: "The ultimate tech sneaker. Overclocked beyond factory specs.",
};

static FOAM_CELL_LEARNSET: &[(u8, u16)] = &[
    (1, 4),   // Quick Step
    (5, 20),  // Shock Drop
    (9, 2),   // Flex
    (13, 21), // Bluetooth Blast
];

pub static FOAM_CELL: SneakerSpecies = SneakerSpecies {
    id: 12,
    name: "Foam Cell",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 50, hype: 30, comfort: 55, drip: 45, rarity: 40 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 190,
    base_xp_yield: 58,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 1, drip: 0, rarity: 0 },
    learnset: FOAM_CELL_LEARNSET,
    evolution: Some((22, 13)),
    description: "A cushioned tech sneaker found on Route 4. Surprisingly resilient.",
};

static BOOST_CORE_LEARNSET: &[(u8, u16)] = &[
    (22, 19), // Firmware Update
    (25, 23), // Neon Pulse
    (30, 22), // Data Mine
    (38, 25), // System Crash
];

pub static BOOST_CORE: SneakerSpecies = SneakerSpecies {
    id: 13,
    name: "Boost Core",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 65, hype: 50, comfort: 65, drip: 75, rarity: 55 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 85,
    base_xp_yield: 162,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 2, rarity: 0 },
    learnset: BOOST_CORE_LEARNSET,
    evolution: None,
    description: "Boosted energy returns with every step. Efficient and powerful.",
};

static LED_LACE_LEARNSET: &[(u8, u16)] = &[
    (1, 20),  // Shock Drop
    (5, 2),   // Flex
    (10, 21), // Bluetooth Blast
    (20, 23), // Neon Pulse
    (30, 25), // System Crash
    (40, 19), // Firmware Update
];

pub static LED_LACE: SneakerSpecies = SneakerSpecies {
    id: 14,
    name: "LED Lace",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 55, hype: 60, comfort: 50, drip: 90, rarity: 100 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 60,
    base_xp_yield: 175,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 0, rarity: 2 },
    learnset: LED_LACE_LEARNSET,
    evolution: None,
    description: "Glowing laces that pulse with neon energy. Found in Neon Springs.",
};

static QUANTUM_SOLE_LEARNSET: &[(u8, u16)] = &[
    (1, 21),  // Bluetooth Blast
    (10, 19), // Firmware Update
    (20, 25), // System Crash
    (30, 26), // Quantum Leap
    (40, 44), // Debug Protocol (signature)
];

pub static QUANTUM_SOLE: SneakerSpecies = SneakerSpecies {
    id: 15,
    name: "Quantum Sole",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 80, hype: 75, comfort: 80, drip: 105, rarity: 90 },
    rarity_tier: RarityTier::Epic,
    base_catch_rate: 30,
    base_xp_yield: 220,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 1, drip: 2, rarity: 0 },
    learnset: QUANTUM_SOLE_LEARNSET,
    evolution: None,
    description: "A boss reward from Dr. Firmware. Technology beyond current understanding.",
};

static GENESIS_REACT_LEARNSET: &[(u8, u16)] = &[
    (1, 26),  // Quantum Leap
    (10, 8),  // Hype Train
    (20, 24), // Overclock
    (30, 25), // System Crash
    (40, 48), // Genesis Aura
];

pub static GENESIS_REACT: SneakerSpecies = SneakerSpecies {
    id: 16,
    name: "Genesis React",
    faction: Faction::Techwear,
    base_stats: Stats { durability: 90, hype: 85, comfort: 90, drip: 115, rarity: 105 },
    rarity_tier: RarityTier::Legendary,
    base_catch_rate: 3,
    base_xp_yield: 270,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 3, rarity: 0 },
    learnset: GENESIS_REACT_LEARNSET,
    evolution: None,
    description: "Genesis Grail #2. Reactive foam that responds to pure willpower.",
};

// ── Skate Faction ─────────────────────────────────────────────────────────────

static SKATE_BLAZER_LEARNSET: &[(u8, u16)] = &[
    (1, 5),   // Stomp
    (5, 27),  // Kickflip
    (9, 1),   // Lace Up
    (13, 28), // Ankle Breaker
];

pub static SKATE_BLAZER: SneakerSpecies = SneakerSpecies {
    id: 17,
    name: "Skate Blazer",
    faction: Faction::Skate,
    base_stats: Stats { durability: 50, hype: 55, comfort: 45, drip: 30, rarity: 35 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 200,
    base_xp_yield: 64,
    ev_yield: Stats { durability: 0, hype: 1, comfort: 0, drip: 0, rarity: 0 },
    learnset: SKATE_BLAZER_LEARNSET,
    evolution: Some((16, 18)),
    description: "Built for the streets. Hits hard and takes hits.",
};

static SKATE_BLAZER_PRO_LEARNSET: &[(u8, u16)] = &[
    (16, 29), // Grind Rail
    (20, 30), // Board Slide
    (25, 32), // Skater's Resolve
    (30, 31), // Tre Flip
];

pub static SKATE_BLAZER_PRO: SneakerSpecies = SneakerSpecies {
    id: 18,
    name: "Skate Blazer Pro",
    faction: Faction::Skate,
    base_stats: Stats { durability: 70, hype: 75, comfort: 55, drip: 45, rarity: 50 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 120,
    base_xp_yield: 142,
    ev_yield: Stats { durability: 0, hype: 2, comfort: 0, drip: 0, rarity: 0 },
    learnset: SKATE_BLAZER_PRO_LEARNSET,
    evolution: Some((32, 19)),
    description: "Pro-level street skating kicks. Momentum and power combined.",
};

static SKATE_BLAZER_ELITE_LEARNSET: &[(u8, u16)] = &[
    (32, 33), // Vulc Smash
    (36, 34), // 900 Spin
    (40, 7),  // Deadstock Strike
];

pub static SKATE_BLAZER_ELITE: SneakerSpecies = SneakerSpecies {
    id: 19,
    name: "Skate Blazer Elite",
    faction: Faction::Skate,
    base_stats: Stats { durability: 90, hype: 100, comfort: 70, drip: 55, rarity: 70 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 45,
    base_xp_yield: 236,
    ev_yield: Stats { durability: 0, hype: 3, comfort: 0, drip: 0, rarity: 0 },
    learnset: SKATE_BLAZER_ELITE_LEARNSET,
    evolution: None,
    description: "Elite street skating perfected. The premier skate weapon.",
};

static GRIP_TAPE_LEARNSET: &[(u8, u16)] = &[
    (1, 5),   // Stomp
    (5, 27),  // Kickflip
    (9, 2),   // Flex
    (13, 29), // Grind Rail
];

pub static GRIP_TAPE: SneakerSpecies = SneakerSpecies {
    id: 20,
    name: "Grip Tape",
    faction: Faction::Skate,
    base_stats: Stats { durability: 60, hype: 50, comfort: 50, drip: 25, rarity: 30 },
    rarity_tier: RarityTier::Common,
    base_catch_rate: 200,
    base_xp_yield: 56,
    ev_yield: Stats { durability: 1, hype: 0, comfort: 0, drip: 0, rarity: 0 },
    learnset: GRIP_TAPE_LEARNSET,
    evolution: Some((20, 21)),
    description: "Rough-and-tumble skate shoe from Route 3. Tough sole, tight grip.",
};

static HALF_PIPE_LEARNSET: &[(u8, u16)] = &[
    (20, 30), // Board Slide
    (25, 32), // Skater's Resolve
    (30, 31), // Tre Flip
    (38, 33), // Vulc Smash
];

pub static HALF_PIPE: SneakerSpecies = SneakerSpecies {
    id: 21,
    name: "Half-Pipe",
    faction: Faction::Skate,
    base_stats: Stats { durability: 80, hype: 70, comfort: 65, drip: 40, rarity: 50 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 90,
    base_xp_yield: 158,
    ev_yield: Stats { durability: 2, hype: 0, comfort: 0, drip: 0, rarity: 0 },
    learnset: HALF_PIPE_LEARNSET,
    evolution: None,
    description: "Born in the half-pipe. Takes hits and keeps skating.",
};

static VULCANIZED_LEARNSET: &[(u8, u16)] = &[
    (1, 27),  // Kickflip
    (5, 2),   // Flex
    (10, 28), // Ankle Breaker
    (20, 30), // Board Slide
    (30, 33), // Vulc Smash
    (40, 34), // 900 Spin
];

pub static VULCANIZED: SneakerSpecies = SneakerSpecies {
    id: 22,
    name: "Vulcanized",
    faction: Faction::Skate,
    base_stats: Stats { durability: 70, hype: 85, comfort: 60, drip: 65, rarity: 80 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 60,
    base_xp_yield: 175,
    ev_yield: Stats { durability: 0, hype: 2, comfort: 0, drip: 0, rarity: 0 },
    learnset: VULCANIZED_LEARNSET,
    evolution: None,
    description: "A Kicksburg rarity. Vulcanized rubber that hits with raw power.",
};

static BOARD_DESTROYER_LEARNSET: &[(u8, u16)] = &[
    (1, 30),  // Board Slide
    (10, 31), // Tre Flip
    (20, 33), // Vulc Smash
    (30, 34), // 900 Spin
    (40, 45), // 50-50 Grind (signature)
];

pub static BOARD_DESTROYER: SneakerSpecies = SneakerSpecies {
    id: 23,
    name: "Board Destroyer",
    faction: Faction::Skate,
    base_stats: Stats { durability: 95, hype: 105, comfort: 75, drip: 70, rarity: 80 },
    rarity_tier: RarityTier::Epic,
    base_catch_rate: 30,
    base_xp_yield: 220,
    ev_yield: Stats { durability: 1, hype: 2, comfort: 0, drip: 0, rarity: 0 },
    learnset: BOARD_DESTROYER_LEARNSET,
    evolution: None,
    description: "A boss reward from Ollie McFlip. Not the board — the sneaker.",
};

static GENESIS_KICKFLIP_LEARNSET: &[(u8, u16)] = &[
    (1, 34),  // 900 Spin
    (10, 7),  // Deadstock Strike
    (20, 8),  // Hype Train
    (30, 33), // Vulc Smash
    (40, 48), // Genesis Aura
];

pub static GENESIS_KICKFLIP: SneakerSpecies = SneakerSpecies {
    id: 24,
    name: "Genesis Kickflip",
    faction: Faction::Skate,
    base_stats: Stats { durability: 100, hype: 115, comfort: 85, drip: 80, rarity: 100 },
    rarity_tier: RarityTier::Legendary,
    base_catch_rate: 3,
    base_xp_yield: 270,
    ev_yield: Stats { durability: 0, hype: 3, comfort: 0, drip: 0, rarity: 0 },
    learnset: GENESIS_KICKFLIP_LEARNSET,
    evolution: None,
    description: "Genesis Grail #3. The perfect kickflip, crystallized forever.",
};

// ── High-Fashion Faction ──────────────────────────────────────────────────────

static RUNWAY_SLIP_LEARNSET: &[(u8, u16)] = &[
    (1, 35),  // Runway Strike
    (5, 2),   // Flex
    (10, 36), // Label Drop
    (20, 39), // Red Carpet
    (30, 40), // Fashion Police
];

pub static RUNWAY_SLIP: SneakerSpecies = SneakerSpecies {
    id: 25,
    name: "Runway Slip",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 45, hype: 55, comfort: 40, drip: 70, rarity: 90 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 120,
    base_xp_yield: 148,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 0, rarity: 2 },
    learnset: RUNWAY_SLIP_LEARNSET,
    evolution: None,
    description: "A sleek Hypetown encounter. Moves faster than the runway.",
};

static COUTURE_BOOT_LEARNSET: &[(u8, u16)] = &[
    (1, 35),  // Runway Strike
    (5, 36),  // Label Drop
    (10, 37), // Haute Beam
    (20, 40), // Fashion Police
    (30, 41), // Couture Cannon
];

pub static COUTURE_BOOT: SneakerSpecies = SneakerSpecies {
    id: 26,
    name: "Couture Boot",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 55, hype: 50, comfort: 55, drip: 80, rarity: 75 },
    rarity_tier: RarityTier::Uncommon,
    base_catch_rate: 120,
    base_xp_yield: 155,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 2, rarity: 0 },
    learnset: COUTURE_BOOT_LEARNSET,
    evolution: None,
    description: "Fashion-forward boot from Hypetown. Drip stat is unmatched.",
};

static AVANT_GARDE_LEARNSET: &[(u8, u16)] = &[
    (1, 36),  // Label Drop
    (5, 2),   // Flex
    (10, 37), // Haute Beam
    (20, 39), // Red Carpet
    (30, 41), // Couture Cannon
    (40, 38), // Price Tag
];

pub static AVANT_GARDE: SneakerSpecies = SneakerSpecies {
    id: 27,
    name: "Avant-Garde",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 50, hype: 60, comfort: 45, drip: 95, rarity: 110 },
    rarity_tier: RarityTier::Rare,
    base_catch_rate: 60,
    base_xp_yield: 175,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 1, rarity: 2 },
    learnset: AVANT_GARDE_LEARNSET,
    evolution: None,
    description: "An avant-garde design that defies expectations. Rare Hypetown find.",
};

static MAISON_SOLE_LEARNSET: &[(u8, u16)] = &[
    (1, 37),  // Haute Beam
    (10, 40), // Fashion Police
    (20, 41), // Couture Cannon
    (30, 42), // Limited Edition
    (40, 46), // Vogue Strike (signature)
];

pub static MAISON_SOLE: SneakerSpecies = SneakerSpecies {
    id: 28,
    name: "Maison Sole",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 70, hype: 75, comfort: 65, drip: 105, rarity: 110 },
    rarity_tier: RarityTier::Epic,
    base_catch_rate: 30,
    base_xp_yield: 220,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 2, rarity: 1 },
    learnset: MAISON_SOLE_LEARNSET,
    evolution: None,
    description: "A boss reward from Flex Queen. Maison-crafted perfection.",
};

static TRIPLE_BLACK_LEARNSET: &[(u8, u16)] = &[
    (1, 40),  // Fashion Police
    (10, 41), // Couture Cannon
    (20, 42), // Limited Edition
    (30, 47), // Resell Markup (signature)
    (40, 8),  // Hype Train
];

pub static TRIPLE_BLACK: SneakerSpecies = SneakerSpecies {
    id: 29,
    name: "Triple Black",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 90, hype: 95, comfort: 85, drip: 80, rarity: 85 },
    rarity_tier: RarityTier::Epic,
    base_catch_rate: 25,
    base_xp_yield: 225,
    ev_yield: Stats { durability: 2, hype: 1, comfort: 0, drip: 0, rarity: 0 },
    learnset: TRIPLE_BLACK_LEARNSET,
    evolution: None,
    description: "All-black everything. Found on Resell Row for those who dare.",
};

static GENESIS_COUTURE_LEARNSET: &[(u8, u16)] = &[
    (1, 42),  // Limited Edition
    (10, 8),  // Hype Train
    (20, 41), // Couture Cannon
    (30, 40), // Fashion Police
    (40, 48), // Genesis Aura
];

pub static GENESIS_COUTURE: SneakerSpecies = SneakerSpecies {
    id: 30,
    name: "Genesis Couture",
    faction: Faction::HighFashion,
    base_stats: Stats { durability: 85, hype: 95, comfort: 80, drip: 120, rarity: 110 },
    rarity_tier: RarityTier::Legendary,
    base_catch_rate: 3,
    base_xp_yield: 270,
    ev_yield: Stats { durability: 0, hype: 0, comfort: 0, drip: 3, rarity: 0 },
    learnset: GENESIS_COUTURE_LEARNSET,
    evolution: None,
    description: "Genesis Grail #4. Where streetwear meets haute couture, forever.",
};

/// All 30 species in ID order (index 0 = ID 1).
pub static ALL_SPECIES: [&SneakerSpecies; 30] = [
    &RETRO_RUNNER,
    &RETRO_RUNNER_II,
    &RETRO_RUNNER_MAX,
    &CLASSIC_DUNK,
    &OG_FORCE,
    &VINTAGE_HIGH_TOP,
    &HERITAGE_COURT,
    &GENESIS_JORDAN,
    &TECH_TRAINER,
    &TECH_TRAINER_PRO,
    &TECH_TRAINER_ULTRA,
    &FOAM_CELL,
    &BOOST_CORE,
    &LED_LACE,
    &QUANTUM_SOLE,
    &GENESIS_REACT,
    &SKATE_BLAZER,
    &SKATE_BLAZER_PRO,
    &SKATE_BLAZER_ELITE,
    &GRIP_TAPE,
    &HALF_PIPE,
    &VULCANIZED,
    &BOARD_DESTROYER,
    &GENESIS_KICKFLIP,
    &RUNWAY_SLIP,
    &COUTURE_BOOT,
    &AVANT_GARDE,
    &MAISON_SOLE,
    &TRIPLE_BLACK,
    &GENESIS_COUTURE,
];
