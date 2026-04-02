import type { Camera } from './camera'
import { TILE_PX } from './camera'

// Import GameEngine type from WASM bindings
import type { GameEngine } from '../wasm/wizsneakers_engine.js'

const TILE_COLORS: Record<number, string> = {
  0: '#3a5a40', // floor
  1: '#4a4a5a', // wall
  2: '#5a7a3a', // tall_grass
  3: '#8b4513', // door
}

export function renderTiles(
  ctx: CanvasRenderingContext2D,
  engine: GameEngine,
  camera: Camera,
  tileRange: { startX: number; startY: number; endX: number; endY: number },
): void {
  const { startX, startY, endX, endY } = tileRange

  for (let y = startY; y < endY; y++) {
    for (let x = startX; x < endX; x++) {
      const tile = engine.get_tile(x, y)
      const drawX = x * TILE_PX - camera.x
      const drawY = y * TILE_PX - camera.y

      ctx.fillStyle = TILE_COLORS[tile] ?? '#000'
      ctx.fillRect(drawX, drawY, TILE_PX, TILE_PX)

      // Grid lines
      ctx.strokeStyle = 'rgba(0,0,0,0.15)'
      ctx.strokeRect(drawX, drawY, TILE_PX, TILE_PX)
    }
  }
}
