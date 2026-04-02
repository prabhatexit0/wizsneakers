use crate::models::faction::Faction;
use crate::models::items::{ItemCategory, ItemData, ItemEffect};
use crate::models::moves::StatusType;
use crate::models::stats::StatKind;

// ── Heal Items (IDs 1-12) ─────────────────────────────────────────────────────

pub static SOLE_SAUCE: ItemData = ItemData {
    id: 1,
    name: "Sole Sauce",
    category: ItemCategory::HealItem,
    cost: 200,
    effect: ItemEffect::HealHp(20),
    description: "A basic conditioner for the sole. Restores 20 HP.",
};

pub static INSOLE_PAD: ItemData = ItemData {
    id: 2,
    name: "Insole Pad",
    category: ItemCategory::HealItem,
    cost: 500,
    effect: ItemEffect::HealHp(50),
    description: "A cushioned insole replacement. Restores 50 HP.",
};

pub static FULL_RESTORE_SPRAY: ItemData = ItemData {
    id: 3,
    name: "Full Restore Spray",
    category: ItemCategory::HealItem,
    cost: 1500,
    effect: ItemEffect::HealFull,
    description: "A premium spray that restores a sneaker to full HP.",
};

pub static MAX_REVIVE_LACE: ItemData = ItemData {
    id: 4,
    name: "Max Revive Lace",
    category: ItemCategory::HealItem,
    cost: 3000,
    effect: ItemEffect::ReviveFull,
    description: "New laces that bring a fainted sneaker back to full HP.",
};

pub static REVIVAL_THREAD: ItemData = ItemData {
    id: 5,
    name: "Revival Thread",
    category: ItemCategory::HealItem,
    cost: 1000,
    effect: ItemEffect::Revive(50),
    description: "Basic thread that revives a fainted sneaker to 50% HP.",
};

pub static CREASE_GUARD: ItemData = ItemData {
    id: 6,
    name: "Crease Guard",
    category: ItemCategory::HealItem,
    cost: 300,
    effect: ItemEffect::CureStatus(Some(StatusType::Creased)),
    description: "A plastic insert that removes the Creased status.",
};

pub static BUFF_SPRAY: ItemData = ItemData {
    id: 7,
    name: "Buff Spray",
    category: ItemCategory::HealItem,
    cost: 300,
    effect: ItemEffect::CureStatus(Some(StatusType::Scuffed)),
    description: "Polish spray that removes the Scuffed status.",
};

pub static SMELLING_SALTS: ItemData = ItemData {
    id: 8,
    name: "Smelling Salts",
    category: ItemCategory::HealItem,
    cost: 300,
    effect: ItemEffect::CureStatus(Some(StatusType::Hypnotized)),
    description: "Snaps a sneaker out of the Hypnotized status.",
};

pub static PUMP: ItemData = ItemData {
    id: 9,
    name: "Pump",
    category: ItemCategory::HealItem,
    cost: 300,
    effect: ItemEffect::CureStatus(Some(StatusType::Deflated)),
    description: "Re-inflates a Deflated sneaker back to fighting shape.",
};

pub static FULL_CLEANSE: ItemData = ItemData {
    id: 10,
    name: "Full Cleanse",
    category: ItemCategory::HealItem,
    cost: 800,
    effect: ItemEffect::CureAll,
    description: "A thorough clean that removes any status condition.",
};

pub static PP_RESTORE: ItemData = ItemData {
    id: 11,
    name: "PP Restore",
    category: ItemCategory::HealItem,
    cost: 400,
    effect: ItemEffect::RestorePp(10),
    description: "Restores 10 PP to one selected move.",
};

pub static PP_MAX: ItemData = ItemData {
    id: 12,
    name: "PP Max",
    category: ItemCategory::HealItem,
    cost: 1200,
    effect: ItemEffect::RestoreAllPp,
    description: "Restores all PP for all moves of one sneaker.",
};

// ── Battle Items (IDs 20-26) ──────────────────────────────────────────────────

pub static HYPE_POTION: ItemData = ItemData {
    id: 20,
    name: "Hype Potion",
    category: ItemCategory::BattleItem,
    cost: 1500,
    effect: ItemEffect::StatBoost(StatKind::Hype, 1),
    description: "Raises a sneaker's Hype by 1 stage in battle.",
};

pub static DRIP_POTION: ItemData = ItemData {
    id: 21,
    name: "Drip Potion",
    category: ItemCategory::BattleItem,
    cost: 1500,
    effect: ItemEffect::StatBoost(StatKind::Drip, 1),
    description: "Raises a sneaker's Drip by 1 stage in battle.",
};

pub static GUARD_SPRAY: ItemData = ItemData {
    id: 22,
    name: "Guard Spray",
    category: ItemCategory::BattleItem,
    cost: 1500,
    effect: ItemEffect::StatBoost(StatKind::Comfort, 1),
    description: "Raises a sneaker's Comfort by 1 stage in battle.",
};

pub static SPEED_LACE: ItemData = ItemData {
    id: 23,
    name: "Speed Lace",
    category: ItemCategory::BattleItem,
    cost: 1500,
    effect: ItemEffect::StatBoost(StatKind::Rarity, 1),
    description: "Raises a sneaker's Rarity by 1 stage in battle.",
};

pub static X_ALL: ItemData = ItemData {
    id: 24,
    name: "X-All",
    category: ItemCategory::BattleItem,
    cost: 5000,
    effect: ItemEffect::BoostAll,
    description: "Raises all of a sneaker's battle stats by 1 stage.",
};

pub static CRIT_LENS: ItemData = ItemData {
    id: 25,
    name: "Crit Lens",
    category: ItemCategory::BattleItem,
    cost: 2000,
    effect: ItemEffect::GuaranteedCrit,
    description: "Guarantees a critical hit on the next move used.",
};

pub static FOCUS_SASH: ItemData = ItemData {
    id: 26,
    name: "Focus Sash",
    category: ItemCategory::BattleItem,
    cost: 3000,
    effect: ItemEffect::SurviveFatalHit,
    description: "Allows a sneaker to survive one fatal hit at 1 HP.",
};

// ── Sneaker Cases (IDs 30-37) ─────────────────────────────────────────────────

pub static SNEAKER_CASE: ItemData = ItemData {
    id: 30,
    name: "Sneaker Case",
    category: ItemCategory::SneakerCase,
    cost: 200,
    effect: ItemEffect::CatchMultiplier(100),
    description: "A standard sneaker case. 1.0x catch rate.",
};

pub static PREMIUM_CASE: ItemData = ItemData {
    id: 31,
    name: "Premium Case",
    category: ItemCategory::SneakerCase,
    cost: 600,
    effect: ItemEffect::CatchMultiplier(150),
    description: "A higher-quality case. 1.5x catch rate.",
};

pub static GRAIL_CASE: ItemData = ItemData {
    id: 32,
    name: "Grail Case",
    category: ItemCategory::SneakerCase,
    cost: 3000,
    effect: ItemEffect::CatchMultiplier(250),
    description: "A premium grail case. 2.5x catch rate.",
};

pub static MASTER_CASE: ItemData = ItemData {
    id: 33,
    name: "Master Case",
    category: ItemCategory::SneakerCase,
    cost: 50000,
    effect: ItemEffect::GuaranteedCatch,
    description: "The ultimate sneaker case. Guaranteed catch.",
};

pub static RETRO_CASE: ItemData = ItemData {
    id: 34,
    name: "Retro Case",
    category: ItemCategory::SneakerCase,
    cost: 800,
    effect: ItemEffect::CatchMultiplierFaction(Faction::Retro, 300),
    description: "A specialized case. 3.0x catch rate for Retro faction sneakers.",
};

pub static TECH_CASE: ItemData = ItemData {
    id: 35,
    name: "Tech Case",
    category: ItemCategory::SneakerCase,
    cost: 800,
    effect: ItemEffect::CatchMultiplierFaction(Faction::Techwear, 300),
    description: "A specialized case. 3.0x catch rate for Techwear faction sneakers.",
};

pub static SKATE_CASE: ItemData = ItemData {
    id: 36,
    name: "Skate Case",
    category: ItemCategory::SneakerCase,
    cost: 800,
    effect: ItemEffect::CatchMultiplierFaction(Faction::Skate, 300),
    description: "A specialized case. 3.0x catch rate for Skate faction sneakers.",
};

pub static FASHION_CASE: ItemData = ItemData {
    id: 37,
    name: "Fashion Case",
    category: ItemCategory::SneakerCase,
    cost: 800,
    effect: ItemEffect::CatchMultiplierFaction(Faction::HighFashion, 300),
    description: "A specialized case. 3.0x catch rate for High-Fashion faction sneakers.",
};

// ── Key Items (IDs 50-59) ─────────────────────────────────────────────────────

pub static SNEAKERDEX: ItemData = ItemData {
    id: 50,
    name: "Sneakerdex",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Given by Prof. Sole. Tracks all seen and caught sneakers.",
};

pub static TOWN_MAP: ItemData = ItemData {
    id: 51,
    name: "Town Map",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "A detailed map of the world. Given by Mom at the start.",
};

pub static ESCAPE_ROPE: ItemData = ItemData {
    id: 52,
    name: "Escape Rope",
    category: ItemCategory::KeyItem,
    cost: 300,
    effect: ItemEffect::EscapeDungeon,
    description: "A long rope that instantly exits you from any dungeon.",
};

pub static REPEL: ItemData = ItemData {
    id: 53,
    name: "Repel",
    category: ItemCategory::KeyItem,
    cost: 400,
    effect: ItemEffect::Repel(100),
    description: "Keeps wild sneakers away for 100 steps.",
};

pub static SUPER_REPEL: ItemData = ItemData {
    id: 54,
    name: "Super Repel",
    category: ItemCategory::KeyItem,
    cost: 700,
    effect: ItemEffect::Repel(200),
    description: "Keeps wild sneakers away for 200 steps.",
};

pub static AUTHENTICATION_STAMP: ItemData = ItemData {
    id: 55,
    name: "Authentication Stamp",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "A badge of authenticity from a Syndicate Boss. Required for progression.",
};

pub static SYNDICATE_JOURNAL: ItemData = ItemData {
    id: 56,
    name: "Syndicate Journal",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "A journal found in Chapter 3 with clues about the Genesis Grails.",
};

pub static TEMPLE_KEY: ItemData = ItemData {
    id: 57,
    name: "Temple Key",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "A key that opens the inner sanctum of Grailheim.",
};

pub static ELEVATOR_PASS: ItemData = ItemData {
    id: 58,
    name: "Elevator Pass",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Grants access to Pinnacle Tower's upper floors.",
};

pub static BICYCLE: ItemData = ItemData {
    id: 59,
    name: "Bicycle",
    category: ItemCategory::KeyItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "A bicycle that doubles movement speed on routes.",
};

// ── Held Items (IDs 70-84) ────────────────────────────────────────────────────

pub static HERITAGE_SOLE: ItemData = ItemData {
    id: 70,
    name: "Heritage Sole",
    category: ItemCategory::HeldItem,
    cost: 2000,
    effect: ItemEffect::None,
    description: "Boosts the power of Retro faction moves by 10%.",
};

pub static NANO_FIBER_SOLE: ItemData = ItemData {
    id: 71,
    name: "Nano-Fiber Sole",
    category: ItemCategory::HeldItem,
    cost: 2000,
    effect: ItemEffect::None,
    description: "Boosts the power of Techwear faction moves by 10%.",
};

pub static SKATE_SOLE: ItemData = ItemData {
    id: 72,
    name: "Skate Sole",
    category: ItemCategory::HeldItem,
    cost: 2000,
    effect: ItemEffect::None,
    description: "Boosts the power of Skate faction moves by 10%.",
};

pub static SILK_INSOLE: ItemData = ItemData {
    id: 73,
    name: "Silk Insole",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Boosts the power of High-Fashion faction moves by 10%. Hidden in Hypetown.",
};

pub static SNACK_PACK: ItemData = ItemData {
    id: 74,
    name: "Snack Pack",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Restores 1/16 of max HP at the end of each turn. Hidden on Route 5.",
};

pub static FOCUS_BAND: ItemData = ItemData {
    id: 75,
    name: "Focus Band",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::SurviveFatalHit,
    description: "10% chance to survive a fatal hit at 1 HP. Hidden in Grailheim.",
};

pub static QUICK_LACE: ItemData = ItemData {
    id: 76,
    name: "Quick Lace",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Grants +1 priority to the first move used each battle. Hidden in Neon Springs.",
};

pub static WIDE_LENS: ItemData = ItemData {
    id: 77,
    name: "Wide Lens",
    category: ItemCategory::HeldItem,
    cost: 3000,
    effect: ItemEffect::None,
    description: "Raises the accuracy of all moves by 10%.",
};

pub static MUSCLE_BAND: ItemData = ItemData {
    id: 78,
    name: "Muscle Band",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Boosts the power of physical moves by 10%. Hidden on Route 6.",
};

pub static WISE_GLASSES: ItemData = ItemData {
    id: 79,
    name: "Wise Glasses",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Boosts the power of special moves by 10%. Hidden on Route 7.",
};

pub static CHOICE_BAND: ItemData = ItemData {
    id: 80,
    name: "Choice Band",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Raises Hype by 50% but locks the holder to its first move. Postgame.",
};

pub static CHOICE_SPECS: ItemData = ItemData {
    id: 81,
    name: "Choice Specs",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Raises Drip by 50% but locks the holder to its first move. Postgame.",
};

pub static CHOICE_LACE: ItemData = ItemData {
    id: 82,
    name: "Choice Lace",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Raises Rarity by 50% but locks the holder to its first move. Postgame.",
};

pub static EV_BAND_HYPE: ItemData = ItemData {
    id: 83,
    name: "EV Band (Hype)",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Doubles Hype EVs gained from battle. Postgame shop.",
};

pub static EV_BAND_ALL: ItemData = ItemData {
    id: 84,
    name: "EV Band (All)",
    category: ItemCategory::HeldItem,
    cost: 0,
    effect: ItemEffect::None,
    description: "Doubles all EVs gained from battle. Postgame shop.",
};
