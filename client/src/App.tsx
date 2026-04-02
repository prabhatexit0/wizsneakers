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
  TILE_SIZE,
  RENDER_SCALE,
} from './rendering/camera'
import { renderTiles } from './rendering/tileRenderer'
import { generateCharSheet, getCharSrc, CHAR_W, CHAR_H } from './rendering/sprites'
import { BattleScreen } from './components/battle/BattleScreen'

type Facing = 'up' | 'down' | 'left' | 'right'

let charSheet: HTMLCanvasElement | null = null

function App() {
  const { engine, ready, error } = useWasm()
  const inputRef = useInput()
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [encounter, setEncounter] = useState(false)
  const [stepCount, setStepCount] = useState(0)
  const [gameMode, setGameMode] = useState<string>('Overworld')

  const render = useCallback((
    px: number, py: number,
    facing: Facing,
    moveProgress: number,
    mapW: number, mapH: number,
    walkFrame?: number,
  ) => {
    const eng = engine.current
    const canvas = canvasRef.current
    if (!eng || !canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    if (!charSheet) charSheet = generateCharSheet()

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

    // Draw character sprite
    const isMoving = moveProgress > 0
    const frame = isMoving ? (walkFrame ?? 1) : 0
    const { sx, sy } = getCharSrc(facing, frame)
    const yOffset = (CHAR_H - TILE_SIZE) * RENDER_SCALE
    ctx.imageSmoothingEnabled = false
    ctx.drawImage(
      charSheet, sx, sy, CHAR_W, CHAR_H,
      renderX, renderY - yOffset, CHAR_W * RENDER_SCALE, CHAR_H * RENDER_SCALE,
    )
  }, [engine])

  // Game loop: 60fps, passes dt to engine
  useGameLoop(
    useCallback((dt: number) => {
      const eng = engine.current
      if (!eng) return

      const mode = eng.mode()
      setGameMode(mode)

      if (mode === 'Battle') {
        // Just tick time — BattleScreen manages its own state
        eng.tick(dt, 'none')
        return
      }

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
        mode: string
      }

      if (state.encounter) {
        setEncounter(true)
        setTimeout(() => setEncounter(false), 1500)
      }

      const steps = eng.step_count()
      setStepCount(steps)
      const walkFrame = steps % 2 === 0 ? 1 : 2
      render(
        state.player_x, state.player_y,
        state.facing, state.move_progress,
        state.map_width, state.map_height,
        walkFrame,
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

  // ── Battle screen ────────────────────────────────────────────────────────────
  if (gameMode === 'Battle') {
    const eng = engine.current
    if (!eng) return null
    return (
      <div
        style={{
          minHeight: '100vh',
          background: '#0a0a0a',
          display: 'flex',
          alignItems: 'flex-start',
          justifyContent: 'center',
          padding: 24,
        }}
      >
        {/* Encounter transition flash */}
        <BattleScreen
          engine={eng}
          onBattleEnd={() => {
            setGameMode('Overworld')
          }}
        />
      </div>
    )
  }

  // ── Overworld ────────────────────────────────────────────────────────────────
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
            background: 'rgba(255,255,255,0.9)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 32,
            color: '#1a1a1a',
            zIndex: 10,
            animation: 'encounterFlash 1.5s ease-out forwards',
          }}
        >
          Wild Sneaker Encountered!
        </div>
      )}

      <div style={{ marginTop: 8, fontSize: 11, opacity: 0.4 }}>
        Steps: {stepCount}
      </div>

      <style>{`
        @keyframes encounterFlash {
          0%   { opacity: 1; }
          60%  { opacity: 0.9; }
          100% { opacity: 0; }
        }
      `}</style>
    </div>
  )
}

export default App
