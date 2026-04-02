use crate::models::faction::Faction;
use crate::models::moves::{MoveCategory, MoveData, MoveEffect, MoveTarget, StatusType};
use crate::models::stats::StatKind;

// ── Normal Moves (1-10) ──────────────────────────────────────────────────────

pub static LACE_UP: MoveData = MoveData {
    id: 1,
    name: "Lace Up",
    faction: Faction::Normal,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 20,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Hype, stages: 1 },
    description: "Tighten those laces. Time to get serious.",
};

pub static FLEX: MoveData = MoveData {
    id: 2,
    name: "Flex",
    faction: Faction::Normal,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Opponent, stat: StatKind::Comfort, stages: -1 },
    description: "Show off your kicks. Opponent's guard drops.",
};

pub static CAMP_OUT: MoveData = MoveData {
    id: 3,
    name: "Camp Out",
    faction: Faction::Normal,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 5,
    priority: 0,
    effect: MoveEffect::HealPercent { percent: 50 },
    description: "Set up camp outside the store. Rest and recover.",
};

pub static QUICK_STEP: MoveData = MoveData {
    id: 4,
    name: "Quick Step",
    faction: Faction::Normal,
    category: MoveCategory::Physical,
    power: Some(40),
    accuracy: 100,
    pp: 30,
    priority: 1,
    effect: MoveEffect::PriorityPlus,
    description: "A swift step that catches opponents off guard.",
};

pub static STOMP: MoveData = MoveData {
    id: 5,
    name: "Stomp",
    faction: Faction::Normal,
    category: MoveCategory::Physical,
    power: Some(65),
    accuracy: 100,
    pp: 20,
    priority: 0,
    effect: MoveEffect::FlinchChance { percent: 30 },
    description: "Bring the sole down hard.",
};

pub static DOUBLE_UP: MoveData = MoveData {
    id: 6,
    name: "Double Up",
    faction: Faction::Normal,
    category: MoveCategory::Physical,
    power: Some(35),
    accuracy: 90,
    pp: 15,
    priority: 0,
    effect: MoveEffect::MultiHit { times: 2 },
    description: "Buy two pairs. Hit twice.",
};

pub static DEADSTOCK_STRIKE: MoveData = MoveData {
    id: 7,
    name: "Deadstock Strike",
    faction: Faction::Normal,
    category: MoveCategory::Physical,
    power: Some(80),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "A pristine, powerful hit.",
};

pub static HYPE_TRAIN: MoveData = MoveData {
    id: 8,
    name: "Hype Train",
    faction: Faction::Normal,
    category: MoveCategory::Special,
    power: Some(90),
    accuracy: 85,
    pp: 10,
    priority: 0,
    effect: MoveEffect::None,
    description: "Ride the wave of hype for massive damage.",
};

pub static RESELL: MoveData = MoveData {
    id: 9,
    name: "Resell",
    faction: Faction::Normal,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 10,
    priority: 0,
    effect: MoveEffect::SwapStatChanges,
    description: "Flip the script. Their gains are yours now.",
};

pub static AUTHENTICATE: MoveData = MoveData {
    id: 10,
    name: "Authenticate",
    faction: Faction::Normal,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 5,
    priority: 0,
    effect: MoveEffect::RemoveBuffs,
    description: "Call out the fakes. Reset their buffs.",
};

// ── Retro Moves (11-18) ──────────────────────────────────────────────────────

pub static CREASE: MoveData = MoveData {
    id: 11,
    name: "Crease",
    faction: Faction::Retro,
    category: MoveCategory::Physical,
    power: Some(40),
    accuracy: 100,
    pp: 25,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Opponent, stat: StatKind::Comfort, stages: -1 },
    description: "Bend the toe box. Classic damage.",
};

pub static THROWBACK: MoveData = MoveData {
    id: 12,
    name: "Throwback",
    faction: Faction::Retro,
    category: MoveCategory::Physical,
    power: Some(60),
    accuracy: 95,
    pp: 20,
    priority: 0,
    effect: MoveEffect::None,
    description: "An old-school hit that still slaps.",
};

pub static VINTAGE_SLAM: MoveData = MoveData {
    id: 13,
    name: "Vintage Slam",
    faction: Faction::Retro,
    category: MoveCategory::Physical,
    power: Some(85),
    accuracy: 90,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "Channel the power of the OGs.",
};

pub static HERITAGE_CRUSH: MoveData = MoveData {
    id: 14,
    name: "Heritage Crush",
    faction: Faction::Retro,
    category: MoveCategory::Physical,
    power: Some(120),
    accuracy: 80,
    pp: 5,
    priority: 0,
    effect: MoveEffect::Recoil { percent: 33 },
    description: "Devastating power, but it takes a toll.",
};

pub static RETRO_WAVE: MoveData = MoveData {
    id: 15,
    name: "Retro Wave",
    faction: Faction::Retro,
    category: MoveCategory::Special,
    power: Some(70),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "A wave of nostalgia washes over the opponent.",
};

pub static CLASSIC_AURA: MoveData = MoveData {
    id: 16,
    name: "Classic Aura",
    faction: Faction::Retro,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 10,
    priority: 0,
    effect: MoveEffect::MultiStatChange {
        target: MoveTarget::Self_,
        changes: &[(StatKind::Hype, 1), (StatKind::Comfort, 1)],
    },
    description: "The timeless energy of a true classic.",
};

pub static OG_STAMP: MoveData = MoveData {
    id: 17,
    name: "OG Stamp",
    faction: Faction::Retro,
    category: MoveCategory::Special,
    power: Some(95),
    accuracy: 90,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::Scuffed, chance: 20 },
    description: "Mark them with the seal of the originals.",
};

pub static GRAIL_BEAM: MoveData = MoveData {
    id: 18,
    name: "Grail Beam",
    faction: Faction::Retro,
    category: MoveCategory::Special,
    power: Some(130),
    accuracy: 85,
    pp: 5,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Drip, stages: -1 },
    description: "Unleash the power of a true grail. Draining.",
};

// ── Techwear Moves (19-26) ────────────────────────────────────────────────────

pub static FIRMWARE_UPDATE: MoveData = MoveData {
    id: 19,
    name: "Firmware Update",
    faction: Faction::Techwear,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Drip, stages: 2 },
    description: "Patch your sneaker's software for more power.",
};

pub static SHOCK_DROP: MoveData = MoveData {
    id: 20,
    name: "Shock Drop",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(45),
    accuracy: 100,
    pp: 25,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::Deflated, chance: 10 },
    description: "A sudden digital release. Zap!",
};

pub static BLUETOOTH_BLAST: MoveData = MoveData {
    id: 21,
    name: "Bluetooth Blast",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(65),
    accuracy: 95,
    pp: 20,
    priority: 0,
    effect: MoveEffect::None,
    description: "Wireless destruction from range.",
};

pub static DATA_MINE: MoveData = MoveData {
    id: 22,
    name: "Data Mine",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(50),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::DrainHp { percent: 50 },
    description: "Hack their systems. Drain their energy.",
};

pub static NEON_PULSE: MoveData = MoveData {
    id: 23,
    name: "Neon Pulse",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(80),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "A pulse of electric neon energy.",
};

pub static OVERCLOCK: MoveData = MoveData {
    id: 24,
    name: "Overclock",
    faction: Faction::Techwear,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 5,
    priority: 0,
    effect: MoveEffect::MultiStatChange {
        target: MoveTarget::Self_,
        changes: &[(StatKind::Drip, 1), (StatKind::Rarity, 1), (StatKind::Comfort, -1)],
    },
    description: "Push beyond limits. Faster, stronger, but fragile.",
};

pub static SYSTEM_CRASH: MoveData = MoveData {
    id: 25,
    name: "System Crash",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(100),
    accuracy: 85,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::SoldOut, chance: 30 },
    description: "Total system failure. May stun the opponent.",
};

pub static QUANTUM_LEAP: MoveData = MoveData {
    id: 26,
    name: "Quantum Leap",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(130),
    accuracy: 80,
    pp: 5,
    priority: 0,
    effect: MoveEffect::SkipNextTurn,
    description: "Teleport through dimensions. Needs recharge.",
};

// ── Skate Moves (27-34) ──────────────────────────────────────────────────────

pub static KICKFLIP: MoveData = MoveData {
    id: 27,
    name: "Kickflip",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(45),
    accuracy: 100,
    pp: 25,
    priority: 0,
    effect: MoveEffect::HighCrit,
    description: "A clean kickflip. Style and substance.",
};

pub static ANKLE_BREAKER: MoveData = MoveData {
    id: 28,
    name: "Ankle Breaker",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(70),
    accuracy: 90,
    pp: 15,
    priority: 0,
    effect: MoveEffect::HighCrit,
    description: "Cross 'em up so hard their ankles snap.",
};

pub static GRIND_RAIL: MoveData = MoveData {
    id: 29,
    name: "Grind Rail",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(60),
    accuracy: 95,
    pp: 20,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Rarity, stages: 1 },
    description: "Grind and gain momentum.",
};

pub static BOARD_SLIDE: MoveData = MoveData {
    id: 30,
    name: "Board Slide",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(80),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "Slide across with devastating force.",
};

pub static TRE_FLIP: MoveData = MoveData {
    id: 31,
    name: "Tre Flip",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(95),
    accuracy: 85,
    pp: 10,
    priority: 0,
    effect: MoveEffect::None,
    description: "Triple the flip, triple the pain.",
};

pub static SKATERS_RESOLVE: MoveData = MoveData {
    id: 32,
    name: "Skater's Resolve",
    faction: Faction::Skate,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Hype, stages: 2 },
    description: "Get back up. Every time. Hit harder.",
};

pub static VULC_SMASH: MoveData = MoveData {
    id: 33,
    name: "Vulc Smash",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(110),
    accuracy: 85,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::Creased, chance: 20 },
    description: "A vulcanized sole slams down with authority.",
};

pub static NINE_HUNDRED_SPIN: MoveData = MoveData {
    id: 34,
    name: "900 Spin",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(140),
    accuracy: 75,
    pp: 5,
    priority: 0,
    effect: MoveEffect::SelfStatusOnMiss { status: StatusType::Hypnotized },
    description: "The legendary 900. If you land it...",
};

// ── High-Fashion Moves (35-42) ────────────────────────────────────────────────

pub static RUNWAY_STRIKE: MoveData = MoveData {
    id: 35,
    name: "Runway Strike",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(45),
    accuracy: 100,
    pp: 25,
    priority: 0,
    effect: MoveEffect::None,
    description: "Strike a pose, then strike your foe.",
};

pub static LABEL_DROP: MoveData = MoveData {
    id: 36,
    name: "Label Drop",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(65),
    accuracy: 95,
    pp: 20,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Opponent, stat: StatKind::Comfort, stages: -1 },
    description: "Name-drop your way to dominance.",
};

pub static HAUTE_BEAM: MoveData = MoveData {
    id: 37,
    name: "Haute Beam",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(80),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::None,
    description: "A beam of pure haute couture energy.",
};

pub static PRICE_TAG: MoveData = MoveData {
    id: 38,
    name: "Price Tag",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(50),
    accuracy: 100,
    pp: 15,
    priority: 0,
    effect: MoveEffect::PowerEqualsLevel,
    description: "More expensive = more powerful.",
};

pub static RED_CARPET: MoveData = MoveData {
    id: 39,
    name: "Red Carpet",
    faction: Faction::HighFashion,
    category: MoveCategory::Status,
    power: None,
    accuracy: 100,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Self_, stat: StatKind::Rarity, stages: 2 },
    description: "Roll out the red carpet. Maximum speed.",
};

pub static FASHION_POLICE: MoveData = MoveData {
    id: 40,
    name: "Fashion Police",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(70),
    accuracy: 90,
    pp: 15,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::Scuffed, chance: 30 },
    description: "You're under arrest for crimes against fashion.",
};

pub static COUTURE_CANNON: MoveData = MoveData {
    id: 41,
    name: "Couture Cannon",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(110),
    accuracy: 85,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatChange { target: MoveTarget::Opponent, stat: StatKind::Drip, stages: -1 },
    description: "Blast them with concentrated luxury.",
};

pub static LIMITED_EDITION: MoveData = MoveData {
    id: 42,
    name: "Limited Edition",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(150),
    accuracy: 70,
    pp: 3,
    priority: 0,
    effect: MoveEffect::None,
    description: "Rare. Exclusive. Devastating.",
};

// ── Signature Moves (43-48) ──────────────────────────────────────────────────

pub static VINYL_SCRATCH: MoveData = MoveData {
    id: 43,
    name: "Vinyl Scratch",
    faction: Faction::Retro,
    category: MoveCategory::Physical,
    power: Some(90),
    accuracy: 95,
    pp: 10,
    priority: 0,
    effect: MoveEffect::StatusInflict { status: StatusType::Scuffed, chance: 50 },
    description: "The DJ's signature move. Scratch your way to victory.",
};

pub static DEBUG_PROTOCOL: MoveData = MoveData {
    id: 44,
    name: "Debug Protocol",
    faction: Faction::Techwear,
    category: MoveCategory::Special,
    power: Some(90),
    accuracy: 95,
    pp: 10,
    priority: 0,
    effect: MoveEffect::RemoveStatusDealDamage,
    description: "Removes target status conditions while dealing damage.",
};

pub static FIFTY_FIFTY_GRIND: MoveData = MoveData {
    id: 45,
    name: "50-50 Grind",
    faction: Faction::Skate,
    category: MoveCategory::Physical,
    power: Some(90),
    accuracy: 95,
    pp: 10,
    priority: 0,
    effect: MoveEffect::MultiStatChange {
        target: MoveTarget::Self_,
        changes: &[(StatKind::Hype, 1), (StatKind::Rarity, 1)],
    },
    description: "Lock into the grind. Gain power and speed.",
};

pub static VOGUE_STRIKE: MoveData = MoveData {
    id: 46,
    name: "Vogue Strike",
    faction: Faction::HighFashion,
    category: MoveCategory::Special,
    power: Some(90),
    accuracy: 95,
    pp: 10,
    priority: 0,
    effect: MoveEffect::AlwaysCritOnSuperEffective,
    description: "Always crits when it's super-effective.",
};

pub static RESELL_MARKUP: MoveData = MoveData {
    id: 47,
    name: "Resell Markup",
    faction: Faction::Normal,
    category: MoveCategory::Special,
    power: None,
    accuracy: 100,
    pp: 5,
    priority: 0,
    effect: MoveEffect::PercentCurrentHp { percent: 50 },
    description: "Deals damage equal to 50% of the target's current HP.",
};

pub static GENESIS_AURA: MoveData = MoveData {
    id: 48,
    name: "Genesis Aura",
    faction: Faction::Normal,
    category: MoveCategory::Special,
    power: Some(100),
    accuracy: 100,
    pp: 5,
    priority: 0,
    effect: MoveEffect::HealPercentDamage { percent: 25 },
    description: "Heals 25% of damage dealt. The power of a true grail.",
};

/// All 48 moves in ID order (index 0 = ID 1).
pub static ALL_MOVES: [&MoveData; 48] = [
    &LACE_UP,
    &FLEX,
    &CAMP_OUT,
    &QUICK_STEP,
    &STOMP,
    &DOUBLE_UP,
    &DEADSTOCK_STRIKE,
    &HYPE_TRAIN,
    &RESELL,
    &AUTHENTICATE,
    &CREASE,
    &THROWBACK,
    &VINTAGE_SLAM,
    &HERITAGE_CRUSH,
    &RETRO_WAVE,
    &CLASSIC_AURA,
    &OG_STAMP,
    &GRAIL_BEAM,
    &FIRMWARE_UPDATE,
    &SHOCK_DROP,
    &BLUETOOTH_BLAST,
    &DATA_MINE,
    &NEON_PULSE,
    &OVERCLOCK,
    &SYSTEM_CRASH,
    &QUANTUM_LEAP,
    &KICKFLIP,
    &ANKLE_BREAKER,
    &GRIND_RAIL,
    &BOARD_SLIDE,
    &TRE_FLIP,
    &SKATERS_RESOLVE,
    &VULC_SMASH,
    &NINE_HUNDRED_SPIN,
    &RUNWAY_STRIKE,
    &LABEL_DROP,
    &HAUTE_BEAM,
    &PRICE_TAG,
    &RED_CARPET,
    &FASHION_POLICE,
    &COUTURE_CANNON,
    &LIMITED_EDITION,
    &VINYL_SCRATCH,
    &DEBUG_PROTOCOL,
    &FIFTY_FIFTY_GRIND,
    &VOGUE_STRIKE,
    &RESELL_MARKUP,
    &GENESIS_AURA,
];
