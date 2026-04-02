use crate::state::GameMode;
use crate::world::dialogue::DialogueData;
use crate::world::npc::NpcState;
use crate::world::map::EventDef;

pub enum InteractionResult {
    Dialogue(DialogueData),
    Sign(String),
    Shop(u16),
    Heal,
    SneakerBox,
}

/// Check what is in front of the player and return the interaction result.
/// - NPC in front → start dialogue
/// - Event tile in front (sign) → return sign text
/// - Nothing → None
pub fn interact(
    player_x: u16,
    player_y: u16,
    facing_dx: isize,
    facing_dy: isize,
    npcs: &[NpcState],
    events_defs: &[EventDef],
    dialogue_db: &std::collections::HashMap<String, DialogueData>,
) -> Option<InteractionResult> {
    let target_x = (player_x as isize + facing_dx) as u16;
    let target_y = (player_y as isize + facing_dy) as u16;

    // Check for NPC at target position
    for npc in npcs {
        if npc.x == target_x && npc.y == target_y {
            // Look up dialogue data
            if let Some(data) = dialogue_db.get(&npc.dialogue_id) {
                return Some(InteractionResult::Dialogue(data.clone()));
            } else {
                // Stub dialogue when no data found
                let stub = DialogueData {
                    id: npc.dialogue_id.clone(),
                    pages: vec![crate::world::dialogue::DialoguePage {
                        speaker: None,
                        text: "...".to_string(),
                        choices: None,
                    }],
                };
                return Some(InteractionResult::Dialogue(stub));
            }
        }
    }

    // Check for event at target position
    for event in events_defs {
        if event.x == target_x && event.y == target_y {
            match event.event_type.as_str() {
                "sign" => return Some(InteractionResult::Sign(event.data.clone())),
                "shop" => {
                    let shop_id = event.data.parse::<u16>().unwrap_or(0);
                    return Some(InteractionResult::Shop(shop_id));
                }
                "heal" => return Some(InteractionResult::Heal),
                "sneaker_box" => return Some(InteractionResult::SneakerBox),
                _ => {}
            }
        }
    }

    None
}

/// Check if the player's current position triggers any map events.
pub fn check_position_events(
    player_x: u16,
    player_y: u16,
    events_defs: &[EventDef],
) -> Vec<String> {
    let mut triggered = Vec::new();
    for event in events_defs {
        if event.x == player_x && event.y == player_y {
            triggered.push(event.id.clone());
        }
    }
    triggered
}
