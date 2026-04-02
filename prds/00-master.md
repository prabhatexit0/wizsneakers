# Master PRD — Wizsneakers Autonomous Build Orchestrator

## Purpose
This document orchestrates the autonomous execution of all phase PRDs to build Wizsneakers from its current barebones state into a fully playable 2D RPG. Each PRD is self-contained with clear deliverables, test requirements, and verification gates.

## How to Execute

### For Claude Code
Execute each PRD in order. For each PRD:
1. Read the PRD file from `prds/`
2. Implement everything described in the PRD
3. Run the verification command specified in the PRD's `## Verification` section
4. If verification passes, move to the next PRD
5. If verification fails, fix the issues and re-verify before proceeding

### Execution Order (STRICT — do not skip or reorder)

| Order | PRD File | Phase | What It Delivers |
|-------|----------|-------|-----------------|
| 1 | `01-engine-models.md` | 1A | Rust module structure, core model structs |
| 2 | `02-game-data.md` | 1B | All 30 sneakers, 48 moves, 37 items as static data |
| 3 | `03-game-state.md` | 1C | GameState, PlayerState, SeededRng, refactored WASM API |
| 4 | `04-map-engine.md` | 2A | JSON map loading, tile layers, collision system |
| 5 | `05-camera-viewport.md` | 2B | 720×528 viewport, camera system, tile culling |
| 6 | `06-smooth-movement.md` | 2C | Smooth tile interpolation, 60fps game loop, input refactor |
| 7 | `07-battle-core.md` | 3A | Damage formula, turn resolution, basic battle flow |
| 8 | `08-battle-effects.md` | 3B | Stat stages, all move effects, switching, fleeing, statuses |
| 9 | `09-battle-progression.md` | 3C+3D | XP/leveling/evolution/capture + AI system |
| 10 | `10-battle-ui.md` | 4A+4B+4C | Full battle screen, animations, capture, level-up UI |
| 11 | `11-dialogue-npcs.md` | 6A+6B | Dialogue engine, NPC movement, trainer line-of-sight |
| 12 | `12-menus-save.md` | 7A-7D | Pause menu, inventory, shops, save/load, Sneakerdex |
| 13 | `13-world-starter.md` | 8A-8C | Map transitions, new-game flow, starter sequence, first content |

### Verification Gate
After EACH PRD, run:
```bash
./verify.sh
```
This must exit with code 0 before proceeding to the next PRD.

Additionally, each PRD specifies phase-specific Rust tests that must pass:
```bash
cd engine && cargo test <test_module_name>
```

## Testing Strategy

### Why This Approach Works for Autonomous Execution

**Rust unit tests are the primary verification mechanism** because:
- All game logic (damage, types, stats, AI, capture, XP) lives in Rust
- Rust tests are fast, deterministic, and don't need a browser
- A failing test means the game logic is wrong — period

**Build verification is the secondary gate** because:
- `wasm-pack build` catches Rust→WASM compilation issues
- `tsc --noEmit` catches TypeScript type errors (React + WASM interop)
- `vite build` catches bundling and import issues

**What we deliberately DON'T test** (and why):
- No browser/E2E tests — too flaky for autonomous runs, and the Rust tests cover the logic
- No visual regression — we're using colored blocks until Phase 5, so nothing to regress against
- No performance benchmarks yet — premature until we have real content

### Test Naming Convention
Each PRD specifies tests like:
```rust
#[cfg(test)]
mod tests_phase_1a { ... }
```
So you can verify a specific phase with:
```bash
cargo test tests_phase_1a
```

## Current State (Starting Point)
- `engine/src/lib.rs` — Monolithic 189-line file with basic GameEngine, hardcoded 20×15 map, u32 stats
- `client/src/App.tsx` — Full-map canvas render, colored blocks, 150ms tick limiter
- `client/src/hooks/` — useWasm (init), useInput (WASD→u8), useGameLoop (150ms tick)
- No: battle system, data models, maps, NPCs, dialogue, save, menus, audio

## Architecture Rules (Enforced Across All PRDs)
1. Rust/WASM is the **single source of truth**. React never mutates game state.
2. JSON serialization (serde) for complex data crossing the WASM boundary.
3. Direct primitive getters for hot-path data (player position in tick loop).
4. All randomness through SeededRng — never use `rand` crate or `Math.random()`.
5. Keep WASM binary <200KB gzipped at all times.
6. Every new Rust module must have associated unit tests.
