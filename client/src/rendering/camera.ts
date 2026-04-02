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

export function calculateCamera(
  playerX: number,
  playerY: number,
  mapWidth: number,
  mapHeight: number,
): Camera {
  // Center camera on the player tile (player tile center = playerX * TILE_PX + TILE_PX/2)
  let camX = playerX * TILE_PX + TILE_PX / 2 - CANVAS_WIDTH / 2;
  let camY = playerY * TILE_PX + TILE_PX / 2 - CANVAS_HEIGHT / 2;

  // Clamp so camera never shows outside map bounds
  const maxCamX = mapWidth * TILE_PX - CANVAS_WIDTH;
  const maxCamY = mapHeight * TILE_PX - CANVAS_HEIGHT;

  camX = Math.max(0, Math.min(camX, maxCamX));
  camY = Math.max(0, Math.min(camY, maxCamY));

  return { x: camX, y: camY, width: CANVAS_WIDTH, height: CANVAS_HEIGHT };
}

export function getVisibleTileRange(camera: Camera): {
  startX: number;
  startY: number;
  endX: number;
  endY: number;
} {
  const startX = Math.floor(camera.x / TILE_PX);
  const startY = Math.floor(camera.y / TILE_PX);
  // Add 1 for partial tiles at the edges (buffer)
  const endX = Math.ceil((camera.x + camera.width) / TILE_PX);
  const endY = Math.ceil((camera.y + camera.height) / TILE_PX);

  return { startX, startY, endX, endY };
}
