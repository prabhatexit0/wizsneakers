import { useCallback, useEffect, useRef, useState } from 'react'
import { useWasm } from './hooks/useWasm'
import { useInput } from './hooks/useInput'
import { useGameLoop } from './hooks/useGameLoop'

const TILE_SIZE = 48 // 16px tiles at 3x scale
const TILE_COLORS: Record<number, string> = {
  0: '#3a5a40', // floor — dark green
  1: '#4a4a5a', // wall — grey
  2: '#5a7a3a', // tall grass — lighter green
}

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

    const w = eng.map_width()
    const h = eng.map_height()

    canvas.width = w * TILE_SIZE
    canvas.height = h * TILE_SIZE

    // Draw tiles
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w; x++) {
        const tile = eng.get_tile(x, y)
        ctx.fillStyle = TILE_COLORS[tile] ?? '#000'
        ctx.fillRect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE)

        // Grid lines
        ctx.strokeStyle = 'rgba(0,0,0,0.15)'
        ctx.strokeRect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE)
      }
    }

    // Draw player
    const px = eng.player_x()
    const py = eng.player_y()
    ctx.fillStyle = '#ff6b6b'
    ctx.fillRect(
      px * TILE_SIZE + 6,
      py * TILE_SIZE + 6,
      TILE_SIZE - 12,
      TILE_SIZE - 12,
    )
    // Player outline
    ctx.strokeStyle = '#fff'
    ctx.lineWidth = 2
    ctx.strokeRect(
      px * TILE_SIZE + 6,
      py * TILE_SIZE + 6,
      TILE_SIZE - 12,
      TILE_SIZE - 12,
    )
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
        style={{
          border: '2px solid #333',
          imageRendering: 'pixelated',
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
