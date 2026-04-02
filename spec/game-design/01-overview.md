# Game Overview & Vision

## Elevator Pitch

**Wizsneakers** is a 2D top-down pixel art RPG where you explore a sneaker-obsessed world, collect rare kicks, and battle rival "Hypebeasts" in turn-based combat. Think Pokemon meets sneaker culture — your equipped sneakers are your fighters, each with unique stats, types, and moves rooted in real sneakerhead terminology.

## Design Pillars

### 1. Collection is King
The drive to "catch 'em all" is replaced with the drive to "cop 'em all." Players should feel the rush of finding a rare sneaker in the wild — the same dopamine hit as hitting on a SNKRS drop. Every sneaker should feel unique and desirable.

### 2. Culture-First World Building
The world of Wizsneakers is built on real sneaker culture. Resellers, campouts, hype drops, grail hunting, authentication drama — these are gameplay mechanics, not just flavor text. Players who know sneaker culture should feel seen. Players who don't should learn to love it.

### 3. Strategic Depth with Approachable Surface
The battle system should be easy to pick up (choose a move, hit the opponent) but reward deep knowledge of type matchups, stat optimization, and sneaker team composition. Like Pokemon, the competitive ceiling should be sky-high.

### 4. Nostalgia with a Fresh Sole
The pixel art style and top-down RPG format evoke GBA-era Pokemon, but the theme, humor, and mechanics are distinctly modern. Memes, slang, and internet culture are woven into the fabric of the game.

## Target Audience

| Segment | Description | Draw |
|---------|-------------|------|
| **Sneakerheads** | Active sneaker collectors, ages 16-35 | See their culture gamified; collect virtual grails |
| **Pokemon Fans** | Nostalgic for GBA-era RPGs | Familiar loop with a fresh twist |
| **Casual RPG Players** | Enjoy story + collection games | Approachable combat, vibrant world |
| **Competitive Players** | Min-maxers, PvP enthusiasts | Deep stat system, team building meta |

## Platform & Distribution

- **Primary:** Web browser (desktop + mobile-responsive)
- **Technology:** React + TypeScript frontend, Rust compiled to WebAssembly
- **Distribution:** Hosted web app (playwizsneakers.com), potential Electron wrapper for desktop, potential PWA for mobile
- **Monetization (Future):** Cosmetic sneaker skins, premium sneaker colorways (no pay-to-win stats)

## Comparable Games

| Game | What We Share | What We Do Differently |
|------|--------------|----------------------|
| Pokemon (GBA) | Top-down RPG, creature collection, turn-based battles | Sneaker theme, web-native, modern humor |
| Coromon | Modern pixel art monster tamer | Sneaker culture lens, browser-based |
| Temtem | MMO monster tamer with competitive depth | Single-player focus first, lighter weight |
| Sneaker culture apps (GOAT, StockX) | Sneaker obsession, rarity tiers | It's a game, not a marketplace |

## Scope — Version 1.0 (MVP)

### In Scope
- 1 complete region with 5 cities/towns and connecting routes
- 30 collectible sneakers across 4 factions
- Turn-based battle system with type advantages
- 8 Rival Hypebeast boss battles (like Gym Leaders)
- Main storyline: Hunt for the Genesis Grails
- Inventory, sneaker box, and basic shop system
- NPC dialogue and world exploration
- Save/load system (localStorage)

### Out of Scope (Post-MVP)
- Multiplayer PvP and trading
- Additional regions
- Sneaker breeding/crafting
- Seasonal events and limited drops
- Leaderboards and ranked play
- Mobile-native app
- User-generated content / custom sneakers

## Success Metrics

| Metric | Target |
|--------|--------|
| Playthrough completion rate | >40% of players who start Chapter 1 finish the game |
| Average session length | 15-30 minutes |
| Sneaker collection rate | Average player collects 18/30 sneakers |
| Return rate | >50% of players return within 7 days |
| Performance | Stable 60fps on mid-range hardware |
