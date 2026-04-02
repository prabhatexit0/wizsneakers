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

interface BattleMoveInfo {
  name: string
  pp: number
  max_pp: number
  faction: string
}

interface BattleSneakerInfo {
  name: string
  level: number
  current_hp: number
  max_hp: number
  moves?: BattleMoveInfo[]
}

interface BattleStateUI {
  player: BattleSneakerInfo
  opponent: BattleSneakerInfo
}

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
  const [gameMode, setGameMode] = useState<string>('Overworld')
  const [battleState, setBattleState] = useState<BattleStateUI | null>(null)
  const [battleLog, setBattleLog] = useState<string[]>([])
  const battleLogRef = useRef<HTMLDivElement>(null)

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

  // Scroll battle log to bottom when it updates
  useEffect(() => {
    if (battleLogRef.current) {
      battleLogRef.current.scrollTop = battleLogRef.current.scrollHeight
    }
  }, [battleLog])

  // Game loop: 60fps, passes dt to engine
  useGameLoop(
    useCallback((dt: number) => {
      const eng = engine.current
      if (!eng) return

      const mode = eng.mode()
      setGameMode(mode)

      if (mode === 'Battle') {
        // Refresh battle state each frame
        try {
          const bsJson = eng.get_battle_state()
          if (bsJson && bsJson !== '{}') {
            const bs = JSON.parse(bsJson) as BattleStateUI
            setBattleState(bs)
          }
        } catch { /* ignore parse errors */ }
        // Still tick so time advances
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

  // Submit a battle action
  const submitBattleAction = useCallback((actionJson: string) => {
    const eng = engine.current
    if (!eng) return
    try {
      const eventsJson = eng.battle_action(actionJson)
      const events = JSON.parse(eventsJson) as Array<Record<string, unknown>>
      const logLines: string[] = []
      for (const ev of events) {
        const key = Object.keys(ev)[0]
        const val = ev[key] as Record<string, unknown>
        if (key === 'MoveUsed') {
          logLines.push(`[${val.side}] used move #${val.move_id}`)
        } else if (key === 'Damage') {
          const eff = val.effectiveness as string
          const crit = val.is_critical ? ' CRITICAL HIT!' : ''
          logLines.push(`[${val.side}] took ${val.amount} dmg (${eff})${crit}`)
        } else if (key === 'Fainted') {
          logLines.push(`[${val.side}] fainted!`)
        } else if (key === 'BattleEnd') {
          const result = (val.result as string)
          logLines.push(`Battle ended: ${result}`)
          setGameMode('Overworld')
          setBattleState(null)
          setBattleLog([])
        } else if (key === 'FleeAttempt') {
          logLines.push(val.success ? 'Escaped!' : 'Can\'t escape!')
        } else if (key === 'Message') {
          logLines.push(val.text as string)
        }
      }
      setBattleLog(prev => [...prev, ...logLines])
    } catch { /* ignore */ }
  }, [engine])

  // Battle keyboard input
  useEffect(() => {
    if (gameMode !== 'Battle') return
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      switch (e.code) {
        case 'Digit1': submitBattleAction('{"type":"fight","move_index":0}'); break
        case 'Digit2': submitBattleAction('{"type":"fight","move_index":1}'); break
        case 'Digit3': submitBattleAction('{"type":"fight","move_index":2}'); break
        case 'Digit4': submitBattleAction('{"type":"fight","move_index":3}'); break
        case 'KeyR':   submitBattleAction('{"type":"run"}'); break
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [gameMode, submitBattleAction])

  if (error) {
    return <div style={{ color: '#ff6b6b', padding: 40 }}>WASM Error: {error}</div>
  }

  if (!ready) {
    return <div style={{ padding: 40 }}>Loading engine...</div>
  }

  // ── Battle overlay ──────────────────────────────────────────────────────────
  if (gameMode === 'Battle' && battleState) {
    const { player, opponent } = battleState
    const moves = player.moves ?? []

    return (
      <div style={{ fontFamily: 'monospace', padding: 16, maxWidth: 600 }}>
        <h2 style={{ color: '#ffd700', margin: '0 0 12px' }}>⚔ BATTLE</h2>

        {/* HP displays */}
        <div style={{ display: 'flex', gap: 16, marginBottom: 12 }}>
          <div style={{
            flex: 1, border: '1px solid #555', padding: 10,
            background: '#1a1a2e', borderRadius: 4,
          }}>
            <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 4 }}>WILD</div>
            <div style={{ fontWeight: 'bold', marginBottom: 2 }}>
              {opponent.name} Lv.{opponent.level}
            </div>
            <div style={{ color: opponent.current_hp > opponent.max_hp * 0.5 ? '#4caf50' : opponent.current_hp > opponent.max_hp * 0.2 ? '#ff9800' : '#f44336' }}>
              HP: {opponent.current_hp}/{opponent.max_hp}
            </div>
            <div style={{ height: 6, background: '#333', borderRadius: 3, marginTop: 4 }}>
              <div style={{
                height: '100%',
                width: `${Math.max(0, (opponent.current_hp / opponent.max_hp) * 100)}%`,
                background: opponent.current_hp > opponent.max_hp * 0.5 ? '#4caf50' : opponent.current_hp > opponent.max_hp * 0.2 ? '#ff9800' : '#f44336',
                borderRadius: 3,
                transition: 'width 0.3s',
              }} />
            </div>
          </div>

          <div style={{
            flex: 1, border: '1px solid #555', padding: 10,
            background: '#1a2e1a', borderRadius: 4,
          }}>
            <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 4 }}>YOUR</div>
            <div style={{ fontWeight: 'bold', marginBottom: 2 }}>
              {player.name} Lv.{player.level}
            </div>
            <div style={{ color: player.current_hp > player.max_hp * 0.5 ? '#4caf50' : player.current_hp > player.max_hp * 0.2 ? '#ff9800' : '#f44336' }}>
              HP: {player.current_hp}/{player.max_hp}
            </div>
            <div style={{ height: 6, background: '#333', borderRadius: 3, marginTop: 4 }}>
              <div style={{
                height: '100%',
                width: `${Math.max(0, (player.current_hp / player.max_hp) * 100)}%`,
                background: player.current_hp > player.max_hp * 0.5 ? '#4caf50' : player.current_hp > player.max_hp * 0.2 ? '#ff9800' : '#f44336',
                borderRadius: 3,
                transition: 'width 0.3s',
              }} />
            </div>
          </div>
        </div>

        {/* Move buttons */}
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 6, marginBottom: 8 }}>
          {moves.map((mv, i) => (
            <button
              key={i}
              onClick={() => submitBattleAction(`{"type":"fight","move_index":${i}}`)}
              disabled={mv.pp === 0}
              style={{
                padding: '8px 12px',
                background: mv.pp === 0 ? '#333' : '#2a2a4a',
                color: mv.pp === 0 ? '#555' : '#e0e0e0',
                border: '1px solid #555',
                borderRadius: 4,
                cursor: mv.pp === 0 ? 'not-allowed' : 'pointer',
                textAlign: 'left',
                fontSize: 13,
              }}
            >
              <div style={{ fontWeight: 'bold' }}>[{i + 1}] {mv.name}</div>
              <div style={{ fontSize: 11, opacity: 0.7 }}>{mv.faction} · PP {mv.pp}/{mv.max_pp}</div>
            </button>
          ))}
          {/* Fill empty move slots */}
          {Array.from({ length: Math.max(0, 4 - moves.length) }).map((_, i) => (
            <div key={`empty-${i}`} style={{
              padding: '8px 12px',
              background: '#1a1a1a',
              border: '1px solid #333',
              borderRadius: 4,
              color: '#444',
              fontSize: 13,
            }}>
              —
            </div>
          ))}
        </div>

        {/* Run button */}
        <button
          onClick={() => submitBattleAction('{"type":"run"}')}
          style={{
            width: '100%', padding: '8px 12px', marginBottom: 8,
            background: '#3a2a2a', color: '#ff9999', border: '1px solid #844',
            borderRadius: 4, cursor: 'pointer', fontSize: 13,
          }}
        >
          [R] Run
        </button>

        {/* Battle log */}
        <div
          ref={battleLogRef}
          style={{
            border: '1px solid #333', padding: 8, height: 120,
            overflowY: 'auto', background: '#0a0a0a',
            fontSize: 12, lineHeight: 1.5,
          }}
        >
          {battleLog.length === 0 && (
            <div style={{ opacity: 0.5 }}>A wild {opponent.name} appeared!</div>
          )}
          {battleLog.map((line, i) => (
            <div key={i} style={{ opacity: 0.9 }}>{line}</div>
          ))}
        </div>

        <div style={{ marginTop: 8, fontSize: 11, opacity: 0.5 }}>
          Keys: 1-4 = Move, R = Run
        </div>
      </div>
    )
  }

  // ── Overworld ───────────────────────────────────────────────────────────────
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
