export interface Camera {
  x: number; // world pixel X of viewport top-left
  y: number;
  width: number;  // always CANVAS_WIDTH (720)
  height: number; // always CANVAS_HEIGHT (528)
}

export const TILE_SIZE = 16;
export const RENDER_SCALE = 3;
export const TILE_PX = TILE_SIZE * RENDER_SCALE; // 48
export const VIEWPORT_TILES_X = 15;
export const VIEWPORT_TILES_Y = 11;
export const CANVAS_WIDTH = TILE_PX * VIEWPORT_TILES_X;  // 720
export const CANVAS_HEIGHT = TILE_PX * VIEWPORT_TILES_Y; // 528

/** Facing-direction deltas (matches Rust Direction enum) */
const FACING_DELTA: Record<string, [number, number]> = {
  up:    [0, -1],
  down:  [0,  1],
  left:  [-1, 0],
  right: [1,  0],
}

/**
 * Calculate the camera position centred on the player.
 *
 * During movement the camera tracks the player's interpolated world position
 * so it slides smoothly alongside the player sprite.
 *
 * @param playerX      Tile column (integer, source tile)
 * @param playerY      Tile row (integer, source tile)
 * @param mapWidth     Map width in tiles
 * @param mapHeight    Map height in tiles
 * @param moveProgress 0–1 animation progress (0 = at source tile, 1 = at target)
 * @param facing       "up" | "down" | "left" | "right"
 */
export function calculateCamera(
  playerX: number,
  playerY: number,
  mapWidth: number,
  mapHeight: number,
  moveProgress = 0,
  facing = 'down',
): Camera {
  const [dx, dy] = FACING_DELTA[facing] ?? [0, 0]

  // Interpolated world pixel position of the player's center
  const worldPx = (playerX + dx * moveProgress) * TILE_PX + TILE_PX / 2
  const worldPy = (playerY + dy * moveProgress) * TILE_PX + TILE_PX / 2

  let camX = worldPx - CANVAS_WIDTH / 2
  let camY = worldPy - CANVAS_HEIGHT / 2

  // Clamp so camera never shows outside map bounds
  const maxCamX = mapWidth * TILE_PX - CANVAS_WIDTH
  const maxCamY = mapHeight * TILE_PX - CANVAS_HEIGHT

  camX = Math.max(0, Math.min(camX, maxCamX))
  camY = Math.max(0, Math.min(camY, maxCamY))

  return { x: camX, y: camY, width: CANVAS_WIDTH, height: CANVAS_HEIGHT }
}

export function getVisibleTileRange(camera: Camera): {
  startX: number;
  startY: number;
  endX: number;
  endY: number;
} {
  const startX = Math.floor(camera.x / TILE_PX)
  const startY = Math.floor(camera.y / TILE_PX)
  // Add 1 for partial tiles at the edges (buffer)
  const endX = Math.ceil((camera.x + camera.width) / TILE_PX)
  const endY = Math.ceil((camera.y + camera.height) / TILE_PX)

  return { startX, startY, endX, endY }
}
