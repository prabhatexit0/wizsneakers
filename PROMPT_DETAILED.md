# Wizsneakers — Master Build Prompt

> Use this prompt to build the game. Reference the `spec/` directory for full details on any system.

---

Act as an Expert Game Developer and Systems Architect specializing in WebAssembly, Rust, React, and 2D game development. I am building **Wizsneakers** — a 2D top-down pixel art RPG heavily inspired by classic GBA-era Pokemon games, but set in a world where sneaker culture IS the culture. Players explore a tile-based world, collect rare sneakers, and battle rival "Hypebeasts" in turn-based combat where equipped sneakers are the fighters.

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Frontend** | React 18+, TypeScript 5+, Vite 5+ | UI components, game loop, rendering |
| **Core Engine** | Rust (stable), compiled to WebAssembly | Game state, battle math, movement, AI — single source of truth |
| **WASM Tooling** | `wasm-pack`, `wasm-bindgen`, `serde` + `serde_json` | Rust ↔ JS interop |
| **Rendering** | HTML5 Canvas 2D API | Overworld map, sprites, animations |
| **UI Rendering** | React DOM components | Menus, battle UI, dialogue boxes, HUD |
| **Audio** | Web Audio API (SFX) + HTMLAudioElement (BGM) | Sound effects and music |
| **Persistence** | localStorage | Save/load (3 slots + autosave, ~32KB per save) |
| **Package Manager** | pnpm | Monorepo workspace |

## Architecture (Detailed spec: `spec/technical/01-architecture.md`)

**Monorepo structure:**
- `client/` — React + TypeScript frontend (Vite)
- `engine/` — Rust WASM core (wasm-pack)
- `spec/` — Game design specification documents
- `tools/` — Build scripts, map editor (future)

**Data flow — Overworld (every frame at 60fps):**
1. React `useGameLoop` hook fires via `requestAnimationFrame`
2. `useInput` hook reads buffered keyboard state (WASD/Arrows + action keys)
3. JS calls `engine.tick(delta_time, input_action)` across WASM boundary
4. Rust processes input → updates player position → checks collisions → checks encounters → updates NPC movement
5. Rust returns JSON `RenderDelta` (only what changed — player pos, NPC updates, events)
6. React renders updated state to Canvas (tile culling, sprite sorting by Y-depth, overlay layer)
7. React updates any DOM overlays (dialogue, menu)

**Data flow — Battle (on player action):**
1. Player selects action (Fight/Bag/Sneakers/Run) via React UI
2. JS calls `engine.battle_action(action_json)` across WASM boundary
3. Rust resolves full turn: priority, damage calc, status effects, AI response, win/lose check
4. Rust returns `Vec<BattleTurnEvent>` — a sequence of animation-ready events
5. React animates the sequence (attack effects, HP bar changes, messages, faint animations)
6. React prompts next action or ends battle

**Key architectural decisions:**
- Rust/WASM is the **single source of truth** for ALL game state. React never mutates game state directly.
- React-side state is UI-only (which menu is open, animation progress, text speed setting).
- JSON serialization for complex data (serde), direct getters for hot-path data (player position).
- Maps stored as JSON in `public/maps/`, fetched by JS, parsed by Rust.
- Seeded PRNG (xorshift64) in Rust for deterministic randomness.

## Game World (Detailed spec: `spec/game-design/03-world-design.md`)

**Region: Solecity Metropolitan Area** — 8 towns/cities, 8 connecting routes, and Pinnacle Tower (endgame).

**Tiles:** 16x16 pixels, rendered at 3x scale (48px on screen). Viewport: 15x11 tiles (720x528 canvas).

**4 tile layers per map:** Ground, Detail, Collision (not rendered), Overlay (drawn over player for depth).

**Tile types:** Walkable grass, tall grass (15% encounter rate per step), paths, solid walls, water (post-MVP), ledges (one-way), doors (map transition), NPC collision, signs, item pickups, warp tiles, scripted encounter zones.

**Camera:** Centered on player, clamped to map edges. Smooth interpolation during tile-to-tile movement.

**Map data format:** JSON with layers as 2D arrays of tile IDs, plus NPC definitions, event triggers, wild encounter tables, and map connection data. (Full schema in `spec/game-design/03-world-design.md`)

## Sneaker System (Detailed spec: `spec/game-design/05-sneaker-system.md`)

**30 sneakers** across 4 factions + Normal type. Each sneaker is a species with base stats, and each instance has random IVs (0-31), earned EVs (0-252), a Condition (nature), and up to 4 moves.

### Factions (Types)

| Faction | Theme | Advantage Over | Weak To |
|---------|-------|---------------|---------|
| **Retro** | Classic, vintage | Skate (2x) | Techwear (2x) |
| **Techwear** | Futuristic, performance | Retro (2x) | Skate (2x) |
| **Skate** | Street, rugged | Techwear (2x) | Retro (2x) |
| **High-Fashion** | Luxury, avant-garde | — | Self (0.5x) |

STAB (Same-Type Attack Bonus): 1.5x when move faction matches sneaker faction.

### 5 Core Stats

| Stat | Code | Role |
|------|------|------|
| **Durability** | DUR | Hit Points |
| **Hype** | HYP | Physical Attack |
| **Comfort** | CMF | Defense (physical + special) |
| **Drip** | DRP | Special Attack |
| **Rarity** | RAR | Speed / Turn order |

**Stat formula:** `stat = ((2 * base + iv + ev/4) * level / 100 + 5) * nature_modifier`
**HP formula:** `hp = ((2 * base + iv + ev/4) * level / 100 + level + 10)`

### Rarity Tiers
Common (50% drop), Uncommon (30%), Rare (15%), Epic (4%), Legendary (1%).

### Evolution
Three starter lines evolve at Lv.16 and Lv.32. Other species have single-stage evolutions at Lv.20-22.

### Starters (Player chooses 1 at game start)
1. **Retro Runner** — Balanced stats, Retro type
2. **Tech Trainer** — High Drip, Techwear type
3. **Skate Blazer** — High Hype, Skate type

## Battle System (Detailed spec: `spec/game-design/04-battle-system.md`)

**Turn-based, 1v1.** Each turn: both sides choose action → resolve by Rarity (speed) → apply effects → check faint → next turn.

**Actions:** Fight (use a move), Bag (use item), Sneakers (switch active), Run (wild only).

**Damage formula:**
```
damage = ((((2 * level / 5 + 2) * power * attack / defense) / 50) + 2)
         * STAB * type_effectiveness * critical * random(0.85, 1.00)
```

**Critical hits:** Base 1/16 chance, 1.5x multiplier. Some moves have high-crit rate (1/8).

**Stat stages:** -6 to +6, multiplier from 0.25x to 4.0x. Modified by status moves and items.

**6 Status conditions:** Creased (DOT), Scuffed (-50% attack), Sold Out (stunned), Hypnotized (50% self-hit), Deflated (-75% speed), On Fire (+50% Hype but DOT). One major + one volatile max.

**Capture (wild):** Sneaker Cases with multipliers (1x, 1.5x, 2.5x, guaranteed). Formula accounts for HP %, catch rate, and case bonus. 4 shake checks for animation.

**AI levels:** Random (wild), Basic (trainers), Intermediate (early bosses), Advanced (late bosses), Expert (Elite + Champion). Higher AI uses type advantage, switching, items, and prediction.

**48 moves total** across Normal + 4 factions. Physical, Special, and Status categories. Each move has power, accuracy, PP, priority, and optional effects. (Full list: `spec/game-design/06-moves-abilities.md`)

## Story (Detailed spec: `spec/narrative/01-story-lore.md`)

5 chapters + postgame. Player journeys from Boxfresh Town to Pinnacle Tower, collecting 8 Authentication Stamps, battling the Sole Syndicate criminal organization, and hunting the 4 legendary Genesis Grails.

**Key characters:** Professor Sole (mentor), Flip (rival — reseller turned authentic collector), Counterfeit Carl (villain), The Collector (Champion).

**8 bosses** + 4 Elite Resellers + Champion. Each boss has a themed gym with a puzzle, unique AI personality, and signature sneaker reward.

## Progression & Economy (Detailed spec: `spec/game-design/07-progression-economy.md`)

- **Clout XP** from battles → level up sneakers (cap: 50)
- **Drip Dollars ($DD)** — universal currency for shops and items
- **Authentication Stamps** — 8 bosses, unlock obedience thresholds and new areas
- **Sneakerdex** — track seen/caught (30 species)
- **Achievement badges** — optional goals with rewards (see `spec/systems/03-achievements.md`)

## What I Need Built

### Phase 1: Foundation (Playable Prototype)
1. **Project setup** — Vite + React-TS + Rust-WASM monorepo with wasm-pack, working build pipeline
2. **Rust core models** — All structs/enums: `Sneaker`, `SneakerInstance`, `Faction`, `Stats`, `Move`, `GameState`, `BattleState`, `Inventory`, `Map`
3. **Rust movement** — Grid-based player movement with collision detection, smooth tile interpolation, tall grass encounter triggers
4. **Rust battle engine** — Full damage formula, type effectiveness, turn resolution, stat stages, status conditions, capture mechanics, AI (at least Basic level)
5. **React game loop** — `useGameLoop` (rAF), `useInput` (keyboard), `useWasm` (engine init), Canvas rendering with camera, tile culling
6. **React battle UI** — Battle screen with HP bars, move selection, item use, switching, capture, battle log messages
7. **Prototype rendering** — Colored rectangles for tiles, colored shapes for sneakers (no pixel art yet)

### Phase 2: Content
8. Map data for all towns and routes (JSON format per spec)
9. All 30 sneaker species with stats, learnsets, evolutions
10. All 48 moves with effects
11. All shop inventories and item data
12. NPC and trainer definitions
13. Dialogue scripts for main story + NPCs

### Phase 3: Polish
14. Pixel art assets (tilesets, sprites, battle backgrounds)
15. Audio (BGM + SFX per `spec/audio/01-sound-design.md`)
16. Save/load system (localStorage, 3 slots)
17. Menu system (pause menu, party screen, bag, Sneakerdex)
18. Battle animations and encounter transitions
19. Achievement tracking

### Phase 4: Post-MVP
20. Multiplayer (trading, PvP via WebSocket)
21. Additional regions, sneakers, moves
22. Sneaker breeding/crafting
23. Seasonal events

## Constraints

- **Performance:** 60fps target. WASM tick < 0.5ms, Canvas render < 8ms. Total frame budget: 16.67ms.
- **Bundle size:** WASM module < 200KB gzipped. Total assets < 5MB initial load.
- **Browser support:** Chrome/Firefox/Safari/Edge 89+. Must work without plugins.
- **No pixel art yet:** Use colored blocks/shapes for everything until art is ready.
- **Clean WASM boundary:** Expose one `GameEngine` struct to JS. JSON for complex data, direct getters for hot-path. Minimize allocations in `tick()`.

## Reference

Full specifications for every system are in the `spec/` directory:
- `spec/README.md` — Document index
- `spec/game-design/` — Gameplay, world, battles, sneakers, moves, economy, NPCs, multiplayer
- `spec/technical/` — Architecture, Rust core, React frontend, WASM interop, map engine, save system, performance
- `spec/art/` — Style guide, sneaker visuals, UI/UX wireframes
- `spec/audio/` — Sound design, music list, SFX list
- `spec/narrative/` — Story chapters, lore, dialogue system
- `spec/systems/` — Encounters, inventory, achievements

Write clean, well-commented, production-ready code. Start with Phase 1.
