use serde::{Deserialize, Serialize};
use crate::models::sneaker::SneakerInstance;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    /// (item_id, quantity)
    pub heal_items: Vec<(u16, u16)>,
    pub battle_items: Vec<(u16, u16)>,
    pub sneaker_cases: Vec<(u16, u16)>,
    /// Key items only need an id (no quantity)
    pub key_items: Vec<u16>,
    pub held_items: Vec<(u16, u16)>,
}

impl Inventory {
    pub fn add_item(&mut self, item_id: u16, qty: u16, pocket: InventoryPocket) {
        let vec = self.pocket_mut(pocket);
        if let Some(slot) = vec.iter_mut().find(|(id, _)| *id == item_id) {
            slot.1 += qty;
        } else {
            vec.push((item_id, qty));
        }
    }

    /// Returns true if removal succeeded, false if item not found or insufficient qty.
    pub fn remove_item(&mut self, item_id: u16, qty: u16, pocket: InventoryPocket) -> bool {
        let vec = self.pocket_mut(pocket);
        if let Some(idx) = vec.iter().position(|(id, _)| *id == item_id) {
            if vec[idx].1 < qty {
                return false;
            }
            vec[idx].1 -= qty;
            if vec[idx].1 == 0 {
                vec.remove(idx);
            }
            true
        } else {
            false
        }
    }

    pub fn has_item(&self, item_id: u16, pocket: InventoryPocket) -> bool {
        self.item_count(item_id, pocket) > 0
    }

    pub fn item_count(&self, item_id: u16, pocket: InventoryPocket) -> u16 {
        let vec = self.pocket_ref(pocket);
        vec.iter().find(|(id, _)| *id == item_id).map(|(_, q)| *q).unwrap_or(0)
    }

    fn pocket_mut(&mut self, pocket: InventoryPocket) -> &mut Vec<(u16, u16)> {
        match pocket {
            InventoryPocket::HealItems => &mut self.heal_items,
            InventoryPocket::BattleItems => &mut self.battle_items,
            InventoryPocket::SneakerCases => &mut self.sneaker_cases,
            InventoryPocket::HeldItems => &mut self.held_items,
        }
    }

    fn pocket_ref(&self, pocket: InventoryPocket) -> &Vec<(u16, u16)> {
        match pocket {
            InventoryPocket::HealItems => &self.heal_items,
            InventoryPocket::BattleItems => &self.battle_items,
            InventoryPocket::SneakerCases => &self.sneaker_cases,
            InventoryPocket::HeldItems => &self.held_items,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryPocket {
    HealItems,
    BattleItems,
    SneakerCases,
    HeldItems,
}

pub const SNEAKER_BOX_MAX: usize = 50;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SneakerBox {
    pub sneakers: Vec<SneakerInstance>,
}

impl SneakerBox {
    pub fn deposit(&mut self, sneaker: SneakerInstance) -> bool {
        if self.is_full() {
            return false;
        }
        self.sneakers.push(sneaker);
        true
    }

    pub fn withdraw(&mut self, uid: u64) -> Option<SneakerInstance> {
        if let Some(idx) = self.sneakers.iter().position(|s| s.uid == uid) {
            Some(self.sneakers.remove(idx))
        } else {
            None
        }
    }

    pub fn is_full(&self) -> bool {
        self.sneakers.len() >= SNEAKER_BOX_MAX
    }

    pub fn count(&self) -> usize {
        self.sneakers.len()
    }
}
