use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatKind {
    Durability,
    Hype,
    Comfort,
    Drip,
    Rarity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub durability: u16,
    pub hype: u16,
    pub comfort: u16,
    pub drip: u16,
    pub rarity: u16,
}

impl Stats {
    pub fn get(&self, kind: StatKind) -> u16 {
        match kind {
            StatKind::Durability => self.durability,
            StatKind::Hype => self.hype,
            StatKind::Comfort => self.comfort,
            StatKind::Drip => self.drip,
            StatKind::Rarity => self.rarity,
        }
    }

    pub fn zero() -> Self {
        Self { durability: 0, hype: 0, comfort: 0, drip: 0, rarity: 0 }
    }
}

/// In-battle stat stages for hype, comfort, drip, rarity (not durability)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StatStages {
    pub hype: i8,
    pub comfort: i8,
    pub drip: i8,
    pub rarity: i8,
}

impl StatStages {
    pub fn multiplier(stage: i8) -> f64 {
        let clamped = stage.max(-6).min(6);
        if clamped >= 0 {
            (2 + clamped as i32) as f64 / 2.0
        } else {
            2.0 / (2 + (-clamped) as i32) as f64
        }
    }

    pub fn get(&self, stat: StatKind) -> i8 {
        match stat {
            StatKind::Hype => self.hype,
            StatKind::Comfort => self.comfort,
            StatKind::Drip => self.drip,
            StatKind::Rarity => self.rarity,
            StatKind::Durability => 0,
        }
    }

    pub fn set_clamped(&mut self, stat: StatKind, value: i8) {
        let v = value.max(-6).min(6);
        match stat {
            StatKind::Hype => self.hype = v,
            StatKind::Comfort => self.comfort = v,
            StatKind::Drip => self.drip = v,
            StatKind::Rarity => self.rarity = v,
            StatKind::Durability => {}
        }
    }

    pub fn modify(&mut self, stat: StatKind, amount: i8) {
        let current = match stat {
            StatKind::Hype => &mut self.hype,
            StatKind::Comfort => &mut self.comfort,
            StatKind::Drip => &mut self.drip,
            StatKind::Rarity => &mut self.rarity,
            StatKind::Durability => return, // durability has no stage
        };
        *current = (*current + amount).max(-6).min(6);
    }
}

/// Sneaker nature — boosts one stat by 10%, reduces another by 10%
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    Deadstock,
    Beat,
    Restored,
    Custom,
    Vintage,
    Prototype,
    PlayerExclusive,
    Sample,
    GeneralRelease,
}

impl Condition {
    /// Returns 0.9, 1.0, or 1.1 for a given stat
    pub fn modifier(&self, stat: StatKind) -> f64 {
        // (boosted_stat, reduced_stat)
        let (up, down) = match self {
            Condition::Deadstock     => (StatKind::Hype,    StatKind::Rarity),
            Condition::Beat          => (StatKind::Comfort, StatKind::Drip),
            Condition::Restored      => (StatKind::Drip,    StatKind::Hype),
            Condition::Custom        => (StatKind::Rarity,  StatKind::Comfort),
            Condition::Vintage       => (StatKind::Hype,    StatKind::Comfort),
            Condition::Prototype     => (StatKind::Drip,    StatKind::Rarity),
            Condition::PlayerExclusive => (StatKind::Rarity, StatKind::Hype),
            Condition::Sample        => (StatKind::Comfort, StatKind::Drip),
            Condition::GeneralRelease => return 1.0,
        };
        if stat == up {
            1.1
        } else if stat == down {
            0.9
        } else {
            1.0
        }
    }
}
