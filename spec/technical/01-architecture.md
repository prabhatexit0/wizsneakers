# Architecture Overview

## System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     BROWSER                                  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                 REACT (TypeScript)                     │   │
│  │                                                       │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │   │
│  │  │ Game Loop    │  │ Input Handler │  │ Audio Mgr  │  │   │
│  │  │ (rAF)       │  │ (Keyboard)   │  │            │  │   │
│  │  └──────┬──────┘  └──────┬───────┘  └────────────┘  │   │
│  │         │                │                            │   │
│  │         ▼                ▼                            │   │
│  │  ┌─────────────────────────────┐                     │   │
│  │  │    WASM Bridge (interop)     │                     │   │
│  │  │  - sendInput(action)         │                     │   │
│  │  │  - getState() → JSON         │                     │   │
│  │  │  - tick(dt)                  │                     │   │
│  │  └──────────────┬──────────────┘                     │   │
│  │                 │                                     │   │
│  │  ┌──────────────▼──────────────┐                     │   │
│  │  │                              │                     │   │
│  │  │     RUST / WASM MODULE       │                     │   │
│  │  │     (Single Source of Truth)  │                     │   │
│  │  │                              │                     │   │
│  │  │  ┌──────────┐ ┌──────────┐  │                     │   │
│  │  │  │GameState │ │BattleEng │  │                     │   │
│  │  │  │- player  │ │- turns   │  │                     │   │
│  │  │  │- map     │ │- damage  │  │                     │   │
│  │  │  │- npcs    │ │- AI      │  │                     │   │
│  │  │  │- inv     │ │- capture │  │                     │   │
│  │  │  └──────────┘ └──────────┘  │                     │   │
│  │  │  ┌──────────┐ ┌──────────┐  │                     │   │
│  │  │  │MapEngine │ │EventSys  │  │                     │   │
│  │  │  │- tiles   │ │- scripts │  │                     │   │
│  │  │  │- collide │ │- triggers│  │                     │   │
│  │  │  │- NPCs    │ │- flags   │  │                     │   │
│  │  │  └──────────┘ └──────────┘  │                     │   │
│  │  └──────────────────────────────┘                     │   │
│  │                                                       │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────────┐  │   │
│  │  │Canvas      │  │ React UI   │  │ Save/Load      │  │   │
│  │  │(Overworld) │  │(Menus,     │  │(localStorage)  │  │   │
│  │  │            │  │ Battle UI) │  │                │  │   │
│  │  └────────────┘  └────────────┘  └────────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure (Monorepo)

```
wizsneakers/
├── README.md
├── package.json              # Workspace root
├── spec/                     # This spec directory
│
├── client/                   # React + TypeScript frontend
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   ├── public/
│   │   ├── assets/
│   │   │   ├── sprites/      # Sneaker sprites, player, NPCs
│   │   │   ├── tilesets/     # Map tilesets
│   │   │   ├── ui/           # Menu backgrounds, buttons
│   │   │   └── audio/        # Music and SFX
│   │   └── maps/             # Map JSON files
│   └── src/
│       ├── main.tsx
│       ├── App.tsx
│       ├── wasm.ts           # WASM initialization and bridge
│       ├── types/
│       │   ├── game.ts       # TypeScript types mirroring Rust structs
│       │   ├── battle.ts
│       │   └── map.ts
│       ├── hooks/
│       │   ├── useGameLoop.ts
│       │   ├── useInput.ts
│       │   ├── useAudio.ts
│       │   └── useWasm.ts
│       ├── components/
│       │   ├── GameCanvas.tsx       # Canvas rendering
│       │   ├── OverworldRenderer.tsx
│       │   ├── BattleScreen.tsx
│       │   ├── BattleUI.tsx
│       │   ├── DialogueBox.tsx
│       │   ├── MainMenu.tsx
│       │   ├── PauseMenu.tsx
│       │   ├── InventoryScreen.tsx
│       │   ├── SneakerSummary.tsx
│       │   ├── ShopScreen.tsx
│       │   └── Sneakerdex.tsx
│       ├── rendering/
│       │   ├── camera.ts
│       │   ├── spritesheet.ts
│       │   ├── tileRenderer.ts
│       │   ├── animationManager.ts
│       │   └── battleAnimations.ts
│       ├── state/
│       │   ├── uiState.ts          # React-side UI state (which menu is open)
│       │   └── saveLoad.ts         # localStorage save/load
│       └── data/
│           ├── sneakerData.ts      # Static sneaker species data (mirror of Rust)
│           └── moveData.ts         # Static move data (for UI display)
│
├── engine/                   # Rust WASM core
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # WASM entry point, GameEngine struct
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   ├── game_state.rs     # Top-level game state
│   │   │   ├── player.rs         # Player data
│   │   │   └── flags.rs          # Event flags, story progress
│   │   ├── battle/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs         # Battle flow, turn resolution
│   │   │   ├── damage.rs         # Damage formula
│   │   │   ├── types.rs          # Type effectiveness chart
│   │   │   ├── ai.rs             # NPC battle AI
│   │   │   ├── capture.rs        # Wild sneaker capture logic
│   │   │   └── status.rs         # Status condition logic
│   │   ├── world/
│   │   │   ├── mod.rs
│   │   │   ├── map.rs            # Map loading, tile lookup
│   │   │   ├── movement.rs       # Player movement, collision
│   │   │   ├── npc.rs            # NPC movement and interaction
│   │   │   ├── encounters.rs     # Random encounter generation
│   │   │   └── events.rs         # Scripted event system
│   │   ├── data/
│   │   │   ├── mod.rs
│   │   │   ├── sneakers.rs       # All sneaker species definitions
│   │   │   ├── moves.rs          # All move definitions
│   │   │   ├── items.rs          # All item definitions
│   │   │   └── trainers.rs       # NPC trainer team definitions
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── sneaker.rs        # SneakerInstance, SneakerSpecies
│   │   │   ├── moves.rs          # Move, MoveCategory, MoveEffect
│   │   │   ├── stats.rs          # Stats, Condition, StatStages
│   │   │   ├── items.rs          # Item, ItemCategory
│   │   │   ├── faction.rs        # Faction enum, type chart
│   │   │   └── inventory.rs      # Bag, SneakerBox
│   │   └── util/
│   │       ├── mod.rs
│   │       └── rng.rs            # Seeded RNG for determinism
│   └── tests/
│       ├── battle_tests.rs
│       ├── movement_tests.rs
│       └── capture_tests.rs
│
└── tools/                    # Build tools and scripts
    ├── build-wasm.sh         # wasm-pack build script
    └── map-editor/           # Future: visual map editor
```

## Tech Stack Details

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Frontend Framework | React 18+ | UI components, state management |
| Language (Frontend) | TypeScript 5+ | Type safety |
| Bundler | Vite 5+ | Fast dev server, WASM support |
| Core Logic | Rust (stable) | Game engine, battle math, state |
| WASM Tooling | wasm-pack + wasm-bindgen | Rust → WASM compilation |
| Serialization | serde + serde_json | Rust ↔ JS data exchange |
| Rendering | HTML5 Canvas 2D | Overworld, sprites, animations |
| UI Rendering | React DOM | Menus, battle UI, dialogue |
| Audio | Web Audio API | Music and sound effects |
| Persistence | localStorage | Save/load game state |
| Package Manager | pnpm | Monorepo workspace management |

## Data Flow

### Overworld Tick (Every Frame — 16.67ms at 60fps)

```
1. React: useGameLoop fires via requestAnimationFrame
2. React: useInput reads buffered keyboard state
3. React → WASM: engine.tick(delta_time, input_action)
4. Rust: Process input → update player position → check collisions → check encounters → update NPCs
5. Rust → React: Return serialized GameState (or diff)
6. React: Render updated state to Canvas
7. React: Update any UI overlays (dialogue, menu)
```

### Battle Tick (On Player Action)

```
1. React: Player selects action (Fight/Bag/Sneakers/Run)
2. React → WASM: engine.battle_action(action)
3. Rust: Resolve turn → calculate damage → apply effects → check faint → AI responds
4. Rust → React: Return BattleState with turn log
5. React: Animate the turn sequence (attacks, HP bars, messages)
6. React: Wait for animations → prompt next action (or end battle)
```

## Key Architectural Decisions

### Why Rust/WASM for game logic?
- **Determinism**: Identical state calculations regardless of JS engine quirks
- **Performance**: Damage formulas, pathfinding, encounter rolls are CPU-hot
- **Integrity**: Harder to tamper with game state in the browser (not impossible, but harder than plain JS)
- **Portability**: The Rust engine could be reused for a native client or server-side validation

### Why React for rendering?
- **Battle UI**: Complex menus with animations are natural in React
- **Hot Reload**: Fast iteration on UI during development
- **Canvas for Overworld**: React manages the canvas element, but raw Canvas 2D API draws the game world for performance
- **Hybrid**: Overworld = Canvas, Menus/Battle UI = React DOM components overlaid

### Why not a full game engine (Bevy, Macroquad)?
- **Web-first**: We want browser deployment with no plugins
- **UI flexibility**: React's component model is better for complex menus than any Rust UI framework
- **Team skills**: More developers know React than Bevy
- **Scope**: A 2D tile-based RPG doesn't need a full game engine's complexity
