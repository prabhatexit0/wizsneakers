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

function App() {
  const { engine, ready, error } = useWasm()
  const direction = useInput()
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [encounter, setEncounter] = useState(false)
  const [stepCount, setStepCount] = useState(0)

  const render = useCallback(() => {
    const eng = engine.current
    const canvas = canvasRef.current
    if (!eng || !canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const mapW = eng.map_width()
    const mapH = eng.map_height()
    const px = eng.player_x()
    const py = eng.player_y()

    // Calculate camera centered on player, clamped to map edges
    const camera = calculateCamera(px, py, mapW, mapH)
    const tileRange = getVisibleTileRange(camera)

    // Draw visible tiles only
    renderTiles(ctx, eng, camera, tileRange)

    // Draw player relative to camera
    const playerDrawX = px * TILE_PX - camera.x
    const playerDrawY = py * TILE_PX - camera.y
    ctx.fillStyle = '#ff6b6b'
    ctx.fillRect(playerDrawX + 6, playerDrawY + 6, TILE_PX - 12, TILE_PX - 12)
    ctx.strokeStyle = '#fff'
    ctx.lineWidth = 2
    ctx.strokeRect(playerDrawX + 6, playerDrawY + 6, TILE_PX - 12, TILE_PX - 12)
  }, [engine])

  // Game tick
  useGameLoop(
    useCallback(() => {
      const eng = engine.current
      if (!eng) return

      eng.tick(direction.current)

      if (eng.encounter_triggered()) {
        setEncounter(true)
        setTimeout(() => setEncounter(false), 1500)
      }

      setStepCount(eng.step_count())
      render()
    }, [engine, direction, render]),
    ready,
  )

  // Initial render
  useEffect(() => {
    if (ready) render()
  }, [ready, render])

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
        WASD / Arrow Keys to move — Steps: {stepCount}
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
