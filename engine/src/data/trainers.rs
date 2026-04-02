/// Trainer archetypes in the game world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// A sneaker slot on a trainer's team. `moves = None` uses the default
/// learnset for that species at the given level.
#[derive(Clone, Debug)]
pub struct TrainerSneaker {
    pub species_id: u16,
    pub level: u8,
    pub moves: Option<[u16; 4]>,
    pub held_item: Option<u16>,
}

/// Full trainer definition. Content populated in later PRDs.
#[derive(Clone, Debug)]
pub struct TrainerData {
    pub id: u16,
    pub name: &'static str,
    pub class: TrainerClass,
    pub team: &'static [TrainerSneaker],
    pub items: &'static [u16],
    pub ai_level: u8,
    pub reward_money: u32,
    pub sight_range: u8,
    pub pre_battle_dialogue: &'static str,
    pub post_battle_dialogue: &'static str,
    pub defeated_dialogue: &'static str,
}
