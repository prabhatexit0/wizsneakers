# PRD 05 — Camera & Viewport System (Phase 2B)

## Goal
Replace the full-map canvas render with a proper 720×528 viewport. Camera centers on player and clamps to map edges. Only visible tiles render (culling). The game feels like looking through a window at a larger world.

## Dependencies
- PRD 04 (maps load from JSON, variable sizes)

## Deliverables

### Files to Create

**`client/src/rendering/camera.ts`**
```typescript
export interface Camera {
  x: number;  // world pixel X of viewport top-left
  y: number;
  width: number;   // always CANVAS_WIDTH (720)
  height: number;  // always CANVAS_HEIGHT (528)
}

export const TILE_SIZE = 16;
export const RENDER_SCALE = 3;
export const TILE_PX = TILE_SIZE * RENDER_SCALE;  // 48
export const VIEWPORT_TILES_X = 15;
export const VIEWPORT_TILES_Y = 11;
export const CANVAS_WIDTH = TILE_PX * VIEWPORT_TILES_X;   // 720
export const CANVAS_HEIGHT = TILE_PX * VIEWPORT_TILES_Y;  // 528

export function calculateCamera(
  playerX: number, playerY: number,
  mapWidth: number, mapHeight: number
): Camera;
// Center on player tile, clamp so camera never shows outside map bounds

export function getVisibleTileRange(camera: Camera): {
  startX: number; startY: number;
  endX: number; endY: number;
};
// Returns tile indices that are visible — used for culling
```

**`client/src/rendering/tileRenderer.ts`**
```typescript
export function renderTiles(
  ctx: CanvasRenderingContext2D,
  engine: GameEngine,
  camera: Camera,
  tileRange: { startX: number; startY: number; endX: number; endY: number }
): void;
// For each visible tile, draw colored rectangle based on tile type
// Colors: floor=#3a5a40, wall=#4a4a5a, tall_grass=#5a7a3a, door=#8b4513
// Apply camera offset: drawX = tileX * TILE_PX - camera.x
```

### Files to Modify

**`client/src/App.tsx`**
- Set canvas to fixed `720×528` (CANVAS_WIDTH × CANVAS_HEIGHT)
- In render function:
  1. Calculate camera from player position and map dimensions
  2. Get visible tile range
  3. Call `renderTiles()` for visible tiles only
  4. Draw player relative to camera position
- Remove the old full-map iteration
- Add `image-rendering: pixelated` CSS to canvas

## Tests Required

No Rust tests needed (this is client-only). Verification through build + visual inspection.

The camera math can be verified by inspection:
- On a 30×20 map, player at center → camera shows middle tiles
- Player at (0,0) → camera at (0,0), no negative coords
- Player at (29,19) → camera clamped so right/bottom edge is map edge

## Verification
```bash
./verify.sh
```

## Acceptance Criteria
- [ ] Canvas is fixed at 720×528 pixels
- [ ] Camera centers on player
- [ ] Camera clamps to map edges (no void visible)
- [ ] Only visible tiles render (15×11 + 1 buffer = ~176 tiles max per frame)
- [ ] Player draws at correct position relative to camera
- [ ] Grid lines visible on tiles
- [ ] `./verify.sh` exits 0
