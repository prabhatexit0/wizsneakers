# WASM Interop Protocol

## Design Principles

1. **Minimize boundary crossings** — batch data, don't call WASM per-field
2. **JSON for complex data** — serde_json on Rust side, JSON.parse on JS side
3. **Primitives for hot paths** — use direct u32/f64 returns where possible
4. **One-way data flow** — JS sends actions, Rust returns state. JS never mutates game state directly.

## API Surface

### Initialization

```rust
// Rust side
#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> GameEngine;
    
    pub fn load_save(json: &str) -> Result<GameEngine, JsValue>;
    
    pub fn set_player_name(&mut self, name: &str);
    
    pub fn choose_starter(&mut self, choice: u8);  // 0, 1, or 2
}
```

```typescript
// JS side
import init, { GameEngine } from '../engine/pkg';

async function initializeGame(): Promise<GameEngine> {
  await init();  // Initialize WASM module
  const seed = BigInt(Date.now());
  return new GameEngine(seed);
}
```

### Frame Updates (Hot Path)

For the main game loop, we optimize for minimal allocation:

```rust
// Rust: Returns a compact JSON string, NOT a full state dump
#[wasm_bindgen]
impl GameEngine {
    /// Tick the overworld. Input is one of: "none","up","down","left","right","action","cancel","menu"
    /// Returns a JSON RenderDelta — only what changed since last frame
    pub fn tick(&mut self, dt_ms: f64, input: &str) -> String;
}
```

```typescript
// TypeScript: Tick return type
interface RenderDelta {
  player_x: number;
  player_y: number;
  player_facing: Direction;
  player_moving: boolean;
  player_move_progress: number;
  npc_updates?: { id: number; x: number; y: number; facing: Direction }[];
  map_changed?: number;  // New map ID if map transition happened
  encounter?: EncounterStart;
  dialogue?: DialogueEvent;
  event?: string;  // Event flag that just triggered
}
```

### Battle Actions

```rust
#[wasm_bindgen]
impl GameEngine {
    /// action JSON: {"type":"fight","move_index":0} | {"type":"bag","item_id":5}
    ///              | {"type":"switch","party_index":2} | {"type":"run"}
    /// Returns: JSON array of BattleTurnEvent[]
    pub fn battle_action(&mut self, action_json: &str) -> String;
    
    /// For move-learn prompts during battle
    /// slot: 0-3 to replace, 4 to skip learning
    pub fn battle_learn_move(&mut self, slot: u8) -> String;
    
    /// For evolution prompts
    pub fn battle_evolution_choice(&mut self, accept: bool) -> String;
}
```

### State Queries (Cold Path — Menu Opens, Etc.)

These are called infrequently (when opening menus) so JSON overhead is acceptable:

```rust
#[wasm_bindgen]
impl GameEngine {
    pub fn get_party_summary(&self) -> String;     // Vec<SneakerSummary>
    pub fn get_bag_contents(&self) -> String;      // Inventory
    pub fn get_sneakerdex(&self) -> String;        // Seen/caught per species
    pub fn get_player_info(&self) -> String;       // Name, money, stamps, play time
    pub fn get_current_map_id(&self) -> u16;       // For music/tileset loading
    pub fn get_mode(&self) -> String;              // Current GameMode
    pub fn export_save(&self) -> String;           // Full serialized state
}
```

## Map Data Loading

Maps are stored as JSON files in `public/maps/`. The Rust engine loads map data when transitioning:

```rust
#[wasm_bindgen]
impl GameEngine {
    /// Called by JS after fetching map JSON from public/maps/
    pub fn load_map_data(&mut self, map_json: &str) -> Result<(), JsValue>;
}
```

```typescript
// JS handles fetching, Rust handles parsing
async function onMapChange(engine: GameEngine, mapId: number) {
  const response = await fetch(`/maps/map_${mapId}.json`);
  const mapJson = await response.text();
  engine.load_map_data(mapJson);
  
  // Also load tileset images for rendering
  const tilesetIds = JSON.parse(engine.get_current_tilesets());
  await Promise.all(tilesetIds.map(loadTilesetImage));
}
```

## Performance Considerations

### Avoiding Allocation on Hot Path

The `tick()` function runs 60 times per second. To minimize GC pressure:

1. **Reuse String buffer** in Rust — write to a pre-allocated String, return a reference
2. **Keep RenderDelta small** — only send what changed (player pos is ~100 bytes of JSON)
3. **Avoid serde for simple returns** — for just position data, consider direct getters:

```rust
// Alternative hot-path API (no JSON)
#[wasm_bindgen]
impl GameEngine {
    pub fn player_x(&self) -> f32;
    pub fn player_y(&self) -> f32;
    pub fn player_facing(&self) -> u8;  // 0-3
    pub fn player_moving(&self) -> bool;
    pub fn has_pending_event(&self) -> bool;
    pub fn consume_event(&mut self) -> String;  // Only called when has_pending_event
}
```

### Benchmarks to Maintain

| Operation | Budget |
|-----------|--------|
| tick() with no state change | < 0.1ms |
| tick() with player movement | < 0.2ms |
| tick() with encounter check | < 0.3ms |
| battle_action() | < 1.0ms |
| JSON.parse of RenderDelta | < 0.1ms |
| JSON.parse of full state | < 2.0ms |
| WASM module init | < 500ms |
| Map load + parse | < 100ms |

## Error Handling

Rust functions that can fail return `Result<T, JsValue>`. On the JS side:

```typescript
try {
  const result = engine.battle_action(JSON.stringify(action));
  const events = JSON.parse(result) as BattleTurnEvent[];
  // process events
} catch (e) {
  // JsValue error from Rust
  console.error('Engine error:', e);
  // Show error UI, potentially auto-save and reload
}
```

## Build Configuration

### wasm-pack

```toml
# engine/Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
js-sys = "0.3"

[profile.release]
opt-level = "s"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Strip debug symbols
```

### Build Script

```bash
#!/bin/bash
# tools/build-wasm.sh
cd engine
wasm-pack build --target web --out-dir ../client/src/wasm-pkg
```

### Vite Configuration

```typescript
// client/vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  build: {
    target: 'esnext',
  },
});
```
