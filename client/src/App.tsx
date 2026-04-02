import { useCallback, useEffect, useRef, useState } from 'react'
import { useWasm } from './hooks/useWasm'
import { useInput } from './hooks/useInput'
import { useGameLoop } from './hooks/useGameLoop'
import {
  calculateCamera,
  getVisibleTileRange,
  CANVAS_WIDTH,
  CANVAS_HEIGHT,
  TILE_PX,
} from './rendering/camera'
import { renderTiles } from './rendering/tileRenderer'

type Facing = 'up' | 'down' | 'left' | 'right'

/** Draw a small triangle on the player square indicating facing direction */
function drawFacingIndicator(
  ctx: CanvasRenderingContext2D,
  drawX: number,
  drawY: number,
  facing: Facing,
) {
  const cx = drawX + TILE_PX / 2
  const cy = drawY + TILE_PX / 2
  const r = TILE_PX / 6

  ctx.fillStyle = '#fff'
  ctx.beginPath()
  switch (facing) {
    case 'up':
      ctx.moveTo(cx, drawY + 4)
      ctx.lineTo(cx - r, drawY + 4 + r * 1.5)
      ctx.lineTo(cx + r, drawY + 4 + r * 1.5)
      break
    case 'down':
      ctx.moveTo(cx, drawY + TILE_PX - 4)
      ctx.lineTo(cx - r, drawY + TILE_PX - 4 - r * 1.5)
      ctx.lineTo(cx + r, drawY + TILE_PX - 4 - r * 1.5)
      break
    case 'left':
      ctx.moveTo(drawX + 4, cy)
      ctx.lineTo(drawX + 4 + r * 1.5, cy - r)
      ctx.lineTo(drawX + 4 + r * 1.5, cy + r)
      break
    case 'right':
      ctx.moveTo(drawX + TILE_PX - 4, cy)
      ctx.lineTo(drawX + TILE_PX - 4 - r * 1.5, cy - r)
      ctx.lineTo(drawX + TILE_PX - 4 - r * 1.5, cy + r)
      break
  }
  ctx.closePath()
  ctx.fill()
}

function App() {
  const { engine, ready, error } = useWasm()
  const inputRef = useInput()
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [encounter, setEncounter] = useState(false)
  const [stepCount, setStepCount] = useState(0)

  const render = useCallback((
    px: number, py: number,
    facing: Facing,
    moveProgress: number,
    mapW: number, mapH: number,
  ) => {
    const eng = engine.current
    const canvas = canvasRef.current
    if (!eng || !canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Camera tracks the interpolated player position
    const camera = calculateCamera(px, py, mapW, mapH, moveProgress, facing)
    const tileRange = getVisibleTileRange(camera)

    renderTiles(ctx, eng, camera, tileRange)

    // Interpolated render position
    const facingDelta: Record<Facing, [number, number]> = {
      up: [0, -1], down: [0, 1], left: [-1, 0], right: [1, 0],
    }
    const [dx, dy] = facingDelta[facing]
    const renderX = (px + dx * moveProgress) * TILE_PX - camera.x
    const renderY = (py + dy * moveProgress) * TILE_PX - camera.y

    ctx.fillStyle = '#ff6b6b'
    ctx.fillRect(renderX + 6, renderY + 6, TILE_PX - 12, TILE_PX - 12)
    ctx.strokeStyle = '#fff'
    ctx.lineWidth = 2
    ctx.strokeRect(renderX + 6, renderY + 6, TILE_PX - 12, TILE_PX - 12)

    drawFacingIndicator(ctx, renderX, renderY, facing)
  }, [engine])

  // Game loop: 60fps, passes dt to engine
  useGameLoop(
    useCallback((dt: number) => {
      const eng = engine.current
      if (!eng) return

      const json = eng.tick(dt, inputRef.current)
      const state = JSON.parse(json) as {
        player_x: number
        player_y: number
        facing: Facing
        moving: boolean
        move_progress: number
        map_width: number
        map_height: number
        encounter: boolean
      }

      if (state.encounter) {
        setEncounter(true)
        setTimeout(() => setEncounter(false), 1500)
      }

      setStepCount(eng.step_count())
      render(
        state.player_x, state.player_y,
        state.facing, state.move_progress,
        state.map_width, state.map_height,
      )
    }, [engine, inputRef, render]),
    ready,
  )

  // Initial render
  useEffect(() => {
    if (ready) {
      const eng = engine.current
      if (!eng) return
      render(eng.player_x(), eng.player_y(), 'down', 0, eng.map_width(), eng.map_height())
    }
  }, [ready, render, engine])

  if (error) {
    return <div style={{ color: '#ff6b6b', padding: 40 }}>WASM Error: {error}</div>
  }

  if (!ready) {
    return <div style={{ padding: 40 }}>Loading engine...</div>
  }

  return (
    <div>
      <h1 style={{ fontSize: 24, marginBottom: 8, color: '#ff6b6b' }}>
        WIZSNEAKERS
      </h1>
      <p style={{ fontSize: 12, marginBottom: 12, opacity: 0.7 }}>
        WASD / Arrow Keys to move · Z/Enter=Action · X=Cancel · Esc=Menu · Shift=Sprint — Steps: {stepCount}
      </p>

      <canvas
        ref={canvasRef}
        width={CANVAS_WIDTH}
        height={CANVAS_HEIGHT}
        style={{
          border: '2px solid #333',
          imageRendering: 'pixelated',
          width: CANVAS_WIDTH,
          height: CANVAS_HEIGHT,
        }}
      />

      {encounter && (
        <div
          style={{
            position: 'fixed',
            top: 0, left: 0, right: 0, bottom: 0,
            background: 'rgba(0,0,0,0.8)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 32,
            color: '#ffd700',
            zIndex: 10,
          }}
        >
          Wild Sneaker Encountered!
        </div>
      )}

      <div style={{ marginTop: 12, fontSize: 11, opacity: 0.5 }}>
        <span style={{ color: '#4a4a5a' }}>■</span> Wall{' '}
        <span style={{ color: '#3a5a40' }}>■</span> Floor{' '}
        <span style={{ color: '#5a7a3a' }}>■</span> Tall Grass{' '}
        <span style={{ color: '#ff6b6b' }}>■</span> Player
      </div>
    </div>
  )
}

export default App
