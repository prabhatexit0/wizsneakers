import type { Camera } from './camera'
import { TILE_PX, TILE_SIZE } from './camera'
import type { GameEngine } from '../wasm/wizsneakers_engine.js'
import { generateTileAtlas, getTileSrc } from './sprites'

let atlas: HTMLCanvasElement | null = null

export function renderTiles(
  ctx: CanvasRenderingContext2D,
  engine: GameEngine,
  camera: Camera,
  tileRange: { startX: number; startY: number; endX: number; endY: number },
): void {
  if (!atlas) atlas = generateTileAtlas()
  ctx.imageSmoothingEnabled = false

  const { startX, startY, endX, endY } = tileRange

  for (let y = startY; y < endY; y++) {
    for (let x = startX; x < endX; x++) {
      const tile = engine.get_tile(x, y)
      const drawX = x * TILE_PX - camera.x
      const drawY = y * TILE_PX - camera.y
      const { sx, sy } = getTileSrc(tile, x, y)

      ctx.drawImage(
        atlas, sx, sy, TILE_SIZE, TILE_SIZE,
        drawX, drawY, TILE_PX, TILE_PX,
      )
    }
  }
}
