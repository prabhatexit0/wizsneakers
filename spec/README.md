# Wizsneakers - Game Design Specification

> A 2D top-down pixel art RPG where sneaker culture meets classic Pokemon-style gameplay.

## Document Index

### Game Design
- [Game Overview & Vision](game-design/01-overview.md) - High-level concept, pillars, target audience
- [Core Gameplay Loop](game-design/02-core-loop.md) - Minute-to-minute and session-level loops
- [World Design](game-design/03-world-design.md) - Regions, maps, cities, routes, landmarks
- [Battle System](game-design/04-battle-system.md) - Combat mechanics, type chart, damage formulas
- [Sneaker System](game-design/05-sneaker-system.md) - All sneakers, stats, evolutions, rarities
- [Moves & Abilities](game-design/06-moves-abilities.md) - Full move list, effects, animations
- [Progression & Economy](game-design/07-progression-economy.md) - XP, leveling, currency, shops
- [NPCs & Rivals](game-design/08-npcs-rivals.md) - Characters, dialogue, AI behavior
- [Multiplayer & Social](game-design/09-multiplayer-social.md) - Trading, PvP, leaderboards

### Technical
- [Architecture Overview](technical/01-architecture.md) - System diagram, Rust/WASM/React split
- [Rust Core Spec](technical/02-rust-core.md) - Data models, state management, battle engine
- [React Frontend Spec](technical/03-react-frontend.md) - Components, hooks, rendering pipeline
- [WASM Interop Protocol](technical/04-wasm-interop.md) - API boundary, serialization, performance
- [Map & Tile Engine](technical/05-map-tile-engine.md) - Tile format, collision, layers, camera
- [Save System](technical/06-save-system.md) - Persistence, slots, cloud sync
- [Performance Budget](technical/07-performance.md) - Target FPS, memory, load times

### Art
- [Art Style Guide](art/01-style-guide.md) - Pixel art specs, palette, tile sizes
- [Sneaker Visual Design](art/02-sneaker-visuals.md) - Sprite specs for all sneakers
- [UI/UX Design](art/03-ui-ux.md) - Menus, HUD, battle UI, inventory

### Audio
- [Sound Design](audio/01-sound-design.md) - SFX, music, ambient loops

### Narrative
- [Story & Lore](narrative/01-story-lore.md) - Main story, world lore, faction backstories
- [Dialogue System](narrative/02-dialogue-system.md) - Conversation trees, triggers, scripting

### Systems
- [Encounter System](systems/01-encounters.md) - Wild encounters, trainer battles, events
- [Inventory & Collection](systems/02-inventory.md) - Bag, sneaker box, key items
- [Achievement System](systems/03-achievements.md) - Badges, completionist goals, unlocks
