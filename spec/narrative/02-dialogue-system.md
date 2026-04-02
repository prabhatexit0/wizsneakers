# Dialogue System

## Overview

Dialogue is driven by a simple scripting format stored in JSON. The Rust engine processes dialogue triggers and state, while React renders the text and choices.

## Dialogue Data Format

```json
{
  "id": "prof_sole_starter",
  "speaker": "Prof. Sole",
  "portrait": "prof_sole_happy",
  "pages": [
    {
      "text": "Welcome, {player_name}! I've been expecting you.",
      "auto_advance": false
    },
    {
      "text": "In this world, sneakers aren't just footwear... they're partners!",
      "auto_advance": false
    },
    {
      "text": "I have three special pairs here. Each one is unique. Which speaks to you?",
      "choices": [
        {
          "text": "Retro Runner (Retro)",
          "result": { "set_flag": "chose_retro", "trigger": "give_starter_retro" }
        },
        {
          "text": "Tech Trainer (Techwear)",
          "result": { "set_flag": "chose_techwear", "trigger": "give_starter_tech" }
        },
        {
          "text": "Skate Blazer (Skate)",
          "result": { "set_flag": "chose_skate", "trigger": "give_starter_skate" }
        }
      ]
    }
  ],
  "conditions": {
    "requires": ["entered_lab"],
    "excludes": ["has_starter"]
  }
}
```

## Template Variables

| Variable | Replaced With |
|----------|--------------|
| `{player_name}` | Player's chosen name |
| `{rival_name}` | "Flip" (or future customizable) |
| `{lead_sneaker}` | Name of player's first party sneaker |
| `{money}` | Current $DD amount |
| `{stamp_count}` | Number of Authentication Stamps |
| `{dex_count}` | Number of caught sneakers |

## Dialogue Triggers

### Position Triggers
NPC dialogue triggers when the player presses the action button while facing an NPC.

### Step Triggers
Some dialogue fires automatically when stepping on a specific tile (cutscenes).

### Conditional Dialogue
NPCs can have multiple dialogue options based on game state:

```json
{
  "npc_id": "museum_guide",
  "dialogues": [
    {
      "condition": { "flag_not_set": "has_retro_stamp" },
      "dialogue_id": "museum_guide_before_stamp"
    },
    {
      "condition": { "flag_set": "has_retro_stamp" },
      "dialogue_id": "museum_guide_after_stamp"
    }
  ]
}
```

## Dialogue Processing (Rust)

```rust
pub struct DialogueState {
    pub active_dialogue_id: Option<String>,
    pub current_page: usize,
    pub waiting_for_input: bool,
    pub waiting_for_choice: bool,
    pub choices: Vec<DialogueChoice>,
}

impl GameEngine {
    pub fn start_dialogue(&mut self, dialogue_id: &str) -> DialogueOutput {
        // Load dialogue data
        // Check conditions
        // Process template variables
        // Return first page
    }
    
    pub fn advance_dialogue(&mut self) -> DialogueOutput {
        // Move to next page or end dialogue
    }
    
    pub fn select_choice(&mut self, index: usize) -> DialogueOutput {
        // Process choice result (set flags, trigger events)
        // Continue or end dialogue
    }
}

pub struct DialogueOutput {
    pub speaker: String,
    pub portrait: String,
    pub text: String,
    pub choices: Option<Vec<String>>,
    pub finished: bool,
}
```

## Writing Guidelines

### Tone
- **Casual and modern**: Characters talk like real people, not formal NPCs
- **Sneaker slang is natural**: "cop," "grail," "fire," "deadstock," "L," "W"
- **Humor**: Light jokes, memes, self-aware genre humor
- **No cringe**: Avoid trying too hard. If a joke doesn't land naturally, cut it.

### Length
- Most NPC dialogue: 1-3 pages (under 50 words per page)
- Story dialogues: 3-8 pages max
- Boss pre-battle: 2-3 pages
- Boss post-battle: 1-2 pages
- Never more than 2 minutes of dialogue without gameplay

### Sample NPC Dialogues

**Random Trainer (Route 2)**:
> "You think those kicks are fire? Let me show you what REAL heat looks like!"

**Sneaker Clinic Nurse**:
> "Welcome to the Sneaker Clinic! We'll have your kicks looking fresh in no time. ... Your sneakers have been fully restored!"

**Kid in Boxfresh Town**:
> "My mom said I can't have my own sneakers until I'm older. That's SO unfair."

**Lore NPC (Grailheim)**:
> "The Cobbler walked these halls centuries ago. They say he could hear the soul in every sole."

**Shop Keeper**:
> "Welcome! Take a look around. Everything's authentic, guaranteed. Unlike SOME places I could mention..."

**Syndicate Grunt**:
> "Mind your own business, kid. Unless you want your sneakers to have an... accident."
