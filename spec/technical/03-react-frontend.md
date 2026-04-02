# React Frontend Specification

## Component Hierarchy

```
<App>
  ├── <WasmProvider>              // Initializes and provides WASM engine
  │   ├── <TitleScreen />         // Start screen (New Game / Continue)
  │   │
  │   ├── <GameView>              // Main game container (when playing)
  │   │   ├── <GameCanvas />      // HTML5 Canvas for overworld rendering
  │   │   ├── <DialogueBox />     // NPC dialogue overlay
  │   │   ├── <TransitionOverlay /> // Fade in/out between maps
  │   │   │
  │   │   ├── <BattleScreen>      // Full-screen battle mode
  │   │   │   ├── <BattleCanvas />   // Battle background + sprites
  │   │   │   ├── <BattleHUD />      // HP bars, names, levels
  │   │   │   ├── <BattleMenu />     // Fight/Bag/Sneakers/Run
  │   │   │   ├── <MoveSelect />     // 4-move grid
  │   │   │   ├── <BattleLog />      // "It's super effective!"
  │   │   │   └── <BattleAnimations /> // Attack effect overlays
  │   │   │
  │   │   └── <PauseMenu>        // In-game menu overlay
  │   │       ├── <PartyScreen />     // View/reorder sneaker team
  │   │       ├── <BagScreen />       // Item inventory
  │   │       ├── <SneakerdexScreen /> // Collection tracker
  │   │       ├── <PlayerCard />      // Player stats/stamps
  │   │       ├── <OptionsScreen />   // Sound, text speed, controls
  │   │       └── <SaveScreen />      // Save game
  │   │
  │   └── <DebugOverlay />        // Dev-only: FPS, state viewer
  │
  └── <LoadingScreen />           // WASM loading indicator
```

## Key Hooks

### useWasm()

```typescript
interface UseWasmReturn {
  engine: GameEngine | null;
  loading: boolean;
  error: Error | null;
}

function useWasm(): UseWasmReturn {
  // 1. Dynamically import the WASM package
  // 2. Instantiate GameEngine
  // 3. Return engine reference for other hooks
}
```

### useGameLoop(engine, canvasRef)

```typescript
function useGameLoop(engine: GameEngine, canvasRef: RefObject<HTMLCanvasElement>) {
  // 1. Create requestAnimationFrame loop
  // 2. Calculate delta time
  // 3. Read buffered input
  // 4. Call engine.tick(dt, input)
  // 5. Parse returned JSON state
  // 6. Call render functions with new state
  // 7. Cleanup on unmount
  
  // Target: 60fps (16.67ms per frame)
  // If frame takes >20ms, skip rendering (keep ticking)
}
```

### useInput()

```typescript
interface InputState {
  up: boolean;
  down: boolean;
  left: boolean;
  right: boolean;
  action: boolean;   // Z / Enter / Space
  cancel: boolean;   // X / Backspace / Escape
  menu: boolean;     // Enter / Escape
  sprint: boolean;   // Shift
}

function useInput(): InputState {
  // 1. addEventListener for keydown/keyup
  // 2. Map WASD + Arrows + action keys
  // 3. Buffer input state (not event-driven — polled each frame)
  // 4. Handle key repeat prevention
  // 5. Return current frame's input state
  
  // Also support touch controls for mobile:
  // Virtual d-pad overlay (bottom-left)
  // Action/Cancel buttons (bottom-right)
}
```

### useAudio()

```typescript
interface UseAudioReturn {
  playBgm: (track: string) => void;
  stopBgm: () => void;
  playSfx: (sound: string) => void;
  setVolume: (bgm: number, sfx: number) => void;
  fadeOut: (durationMs: number) => void;
  fadeIn: (track: string, durationMs: number) => void;
}

function useAudio(): UseAudioReturn {
  // Uses Web Audio API for low-latency SFX
  // Uses HTMLAudioElement for BGM (streaming)
  // Handles browser autoplay policy (resume on first interaction)
}
```

## Canvas Rendering Pipeline

### Overworld Rendering (per frame)

```typescript
function renderOverworld(ctx: CanvasRenderingContext2D, state: OverworldState) {
  // 1. Calculate camera position (centered on player)
  const camera = calculateCamera(state.player, state.mapWidth, state.mapHeight);
  
  // 2. Render ground layer tiles (only visible tiles)
  renderTileLayer(ctx, state.map.ground, camera, tilesets);
  
  // 3. Render NPCs (sorted by Y for depth)
  for (const npc of sortByY(state.npcs)) {
    renderSprite(ctx, npc.sprite, npc.x, npc.y, npc.facing, npc.animFrame, camera);
  }
  
  // 4. Render player
  renderSprite(ctx, playerSprite, state.player.x, state.player.y, 
               state.player.facing, state.player.animFrame, camera);
  
  // 5. Render overlay layer (tree tops, roof overhangs — drawn over player)
  renderTileLayer(ctx, state.map.overlay, camera, tilesets);
  
  // 6. Render weather/particle effects (if any)
  renderWeatherEffects(ctx, state.weather, camera);
}
```

### Rendering Constants

```typescript
const TILE_SIZE = 16;          // Pixels per tile in sprite sheet
const RENDER_SCALE = 3;        // Scale up for pixel art (16px → 48px on screen)
const VIEWPORT_TILES_X = 15;   // Tiles visible horizontally (odd for centering)
const VIEWPORT_TILES_Y = 11;   // Tiles visible vertically
const CANVAS_WIDTH = TILE_SIZE * RENDER_SCALE * VIEWPORT_TILES_X;   // 720px
const CANVAS_HEIGHT = TILE_SIZE * RENDER_SCALE * VIEWPORT_TILES_Y;  // 528px
const PLAYER_MOVE_SPEED = 8;   // Frames to cross one tile (at 60fps = ~133ms)
const SPRINT_SPEED = 4;        // Frames to cross one tile when sprinting
```

## TypeScript Type Definitions (Mirroring Rust)

```typescript
// These types must stay in sync with Rust serialization output

type Faction = 'Normal' | 'Retro' | 'Techwear' | 'Skate' | 'HighFashion';
type GameMode = 'Overworld' | 'Battle' | 'Dialogue' | 'Menu' | 'Cutscene';
type Direction = 'Up' | 'Down' | 'Left' | 'Right';
type BattleSide = 'Player' | 'Opponent';

interface Stats {
  durability: number;
  hype: number;
  comfort: number;
  drip: number;
  rarity: number;
}

interface SneakerSummary {
  uid: number;
  species_id: number;
  name: string;         // display name (nickname or species)
  level: number;
  current_hp: number;
  max_hp: number;
  faction: Faction;
  rarity_tier: string;
  status: string | null;
  held_item: string | null;
}

interface OverworldState {
  player: {
    x: number;
    y: number;
    facing: Direction;
    moving: boolean;
    move_progress: number;  // 0.0 - 1.0
  };
  map_id: number;
  npcs: NpcState[];
  // Tile data sent once on map load, not every frame
}

interface BattleRenderState {
  player_sneaker: SneakerSummary;
  opponent_sneaker: SneakerSummary;
  player_stages: Record<string, number>;
  opponent_stages: Record<string, number>;
  turn_log: BattleTurnEvent[];
  can_flee: boolean;
  is_wild: boolean;
  available_moves: MoveDisplay[];
}

interface MoveDisplay {
  id: number;
  name: string;
  faction: Faction;
  category: string;
  power: number | null;
  accuracy: number;
  current_pp: number;
  max_pp: number;
}
```

## State Management

React-side state is minimal — the WASM engine is the source of truth. React only manages:

```typescript
// UI-only state (not in Rust)
interface UIState {
  currentScreen: 'title' | 'game' | 'loading';
  menuOpen: boolean;
  menuTab: 'party' | 'bag' | 'dex' | 'player' | 'options' | 'save';
  dialogueVisible: boolean;
  dialogueText: string;
  dialogueChoices: string[] | null;
  textSpeed: 'slow' | 'medium' | 'fast' | 'instant';
  battleAnimating: boolean;  // True while battle animations are playing
  transitioning: boolean;    // True during map transitions
}
```

Use React Context or Zustand for lightweight UI state. No Redux — the Rust engine IS the store.

## Mobile Responsiveness

```
Desktop (>768px):  Canvas at native resolution, keyboard controls
Tablet (768px):    Canvas scaled to fit, touch d-pad overlay
Mobile (<480px):   Canvas scaled to fit width, larger touch controls

Touch controls layout:
┌────────────────────────────┐
│                            │
│     [Game Canvas]          │
│                            │
│                            │
├────────────────────────────┤
│  [←][↑]        [B]  [A]   │
│  [↓][→]                   │
│              [Start]       │
└────────────────────────────┘
```
