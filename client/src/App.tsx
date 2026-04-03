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
import { DialogueBox } from './components/DialogueBox'
import { TitleScreen } from './components/TitleScreen'
import { PauseMenu } from './components/menu/PauseMenu'
import { TransitionOverlay } from './components/TransitionOverlay'
import { NameEntryScreen } from './components/NameEntryScreen'
import type { DialoguePage } from './components/DialogueBox'
import type { MapTransitionInfo } from './hooks/useWasm'
import { autoSave } from './state/saveLoad'
import init, { GameEngine } from './wasm/wizsneakers_engine.js'

type Facing = 'up' | 'down' | 'left' | 'right'
type AppState = 'title' | 'name_entry' | 'game'

interface NpcRenderInfo {
  id: string
  x: number
  y: number
  facing: Facing
  is_trainer: boolean
  defeated: boolean
}

let charSheet: HTMLCanvasElement | null = null

function App() {
  const { engine, ready, error, pendingTransition, setPendingTransition, handleTransition } = useWasm()
  const activeTransitionRef = useRef<MapTransitionInfo | null>(null)
  const inputRef = useInput()
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [encounter, setEncounter] = useState(false)
  const [stepCount, setStepCount] = useState(0)
  const [gameMode, setGameMode] = useState<string>('Overworld')
  const [dialoguePage, setDialoguePage] = useState<DialoguePage | null>(null)
  const [trainerSpotted, setTrainerSpotted] = useState<string | null>(null)
  const [_npcsState, setNpcsState] = useState<NpcRenderInfo[]>([])
  const [appState, setAppState] = useState<AppState>('title')
  const [pauseOpen, setPauseOpen] = useState(false)
  const pauseOpenRef = useRef(false)

  // Keep ref in sync for game loop access
  useEffect(() => { pauseOpenRef.current = pauseOpen }, [pauseOpen])

  const render = useCallback((
    px: number, py: number,
    facing: Facing,
    moveProgress: number,
    mapW: number, mapH: number,
    walkFrame?: number,
    npcs?: NpcRenderInfo[],
    trainerSpottedId?: string | null,
  ) => {
    const eng = engine.current
    const canvas = canvasRef.current
    if (!eng || !canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    if (!charSheet) charSheet = generateCharSheet()

    const camera = calculateCamera(px, py, mapW, mapH, moveProgress, facing)
    const tileRange = getVisibleTileRange(camera)

    renderTiles(ctx, eng, camera, tileRange)

    if (npcs && npcs.length > 0) {
      for (const npc of npcs) {
        if (npc.defeated) continue
        const renderNX = npc.x * TILE_PX - camera.x
        const renderNY = npc.y * TILE_PX - camera.y
        ctx.fillStyle = npc.is_trainer ? '#ff9900' : '#ff7733'
        ctx.fillRect(
          renderNX + 2,
          renderNY + 2,
          TILE_SIZE * RENDER_SCALE - 4,
          TILE_SIZE * RENDER_SCALE - 4,
        )

        if (npc.is_trainer && trainerSpottedId === npc.id) {
          ctx.fillStyle = '#ffff00'
          ctx.font = `bold ${14 * RENDER_SCALE}px "Courier New", monospace`
          ctx.textAlign = 'center'
          ctx.fillText('!', renderNX + (TILE_SIZE * RENDER_SCALE) / 2, renderNY - 2)
        }
      }
    }

    const facingDelta: Record<Facing, [number, number]> = {
      up: [0, -1], down: [0, 1], left: [-1, 0], right: [1, 0],
    }
    const [dx, dy] = facingDelta[facing]
    const renderX = (px + dx * moveProgress) * TILE_PX - camera.x
    const renderY = (py + dy * moveProgress) * TILE_PX - camera.y

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

  // Game loop
  useGameLoop(
    useCallback((dt: number) => {
      const eng = engine.current
      if (!eng) return

      const mode = eng.mode()
      setGameMode(mode)

      if (pauseOpenRef.current) return

      if (mode === 'Battle') {
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
        npcs?: NpcRenderInfo[]
        trainer_spotted?: string | null
        dialogue_page?: DialoguePage | null
        transition?: MapTransitionInfo | null
      }

      // Handle map transition from engine
      if (state.transition && !activeTransitionRef.current) {
        activeTransitionRef.current = state.transition
        setPendingTransition(state.transition)
      }

      if (state.encounter) {
        setEncounter(true)
        setTimeout(() => setEncounter(false), 1500)
        // Autosave on new encounter
        try { autoSave(eng) } catch { /* ignore */ }
      }

      if (state.npcs) {
        setNpcsState(state.npcs)
      }

      setTrainerSpotted(state.trainer_spotted ?? null)

      if (state.mode === 'Dialogue' && state.dialogue_page) {
        setDialoguePage(state.dialogue_page)
      } else if (state.mode !== 'Dialogue') {
        // no-op
      }

      const steps = eng.step_count()
      setStepCount(steps)
      const walkFrame = steps % 2 === 0 ? 1 : 2
      render(
        state.player_x, state.player_y,
        state.facing, state.move_progress,
        state.map_width, state.map_height,
        walkFrame,
        state.npcs,
        state.trainer_spotted,
      )
    }, [engine, inputRef, render]),
    ready && appState === 'game',
  )

  // Initial render
  useEffect(() => {
    if (ready && appState === 'game') {
      const eng = engine.current
      if (!eng) return
      render(eng.player_x(), eng.player_y(), 'down', 0, eng.map_width(), eng.map_height())
    }
  }, [ready, render, engine, appState])

  // Escape key toggles pause menu in overworld
  useEffect(() => {
    if (appState !== 'game') return
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (e.code === 'Escape' && gameMode === 'Overworld') {
        setPauseOpen(p => !p)
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [appState, gameMode])

  if (error) {
    return <div style={{ color: '#ff6b6b', padding: 40 }}>WASM Error: {error}</div>
  }

  // Title screen — shown until player starts a game
  if (!ready) {
    return <div style={{ padding: 40, fontFamily: '"Courier New", monospace' }}>Loading engine...</div>
  }

  if (appState === 'name_entry') {
    return (
      <NameEntryScreen
        onConfirm={(name) => {
          const eng = engine.current
          if (!eng) return
          eng.set_player_name(name)
          setAppState('game')
        }}
      />
    )
  }

  if (appState === 'title') {
    return (
      <TitleScreen
        onNewGame={(_name) => {
          // Show name entry screen instead of using the title screen's name field
          setAppState('name_entry')
        }}
        onContinue={(saveData) => {
          // Load save using static method
          init().then(() => {
            try {
              const loaded = GameEngine.load_save(saveData)
              // Re-load map data
              fetch('/maps/boxfresh_town.json')
                .then(r => r.text())
                .then(json => {
                  loaded.load_map_data(json)
                  engine.current = loaded
                  setAppState('game')
                })
                .catch(() => {
                  engine.current = loaded
                  setAppState('game')
                })
            } catch {
              // If load fails, just start game with current engine
              setAppState('game')
            }
          }).catch(() => {
            setAppState('game')
          })
        }}
      />
    )
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
        <BattleScreen
          engine={eng}
          onBattleEnd={() => {
            setGameMode('Overworld')
          }}
        />
      </div>
    )
  }

  // ── Overworld (+ Dialogue overlay + Pause menu) ───────────────────────────────
  const eng = engine.current

  return (
    <div>
      <h1 style={{ fontSize: 24, marginBottom: 8, color: '#ff6b6b' }}>
        WIZSNEAKERS
      </h1>
      <p style={{ fontSize: 12, marginBottom: 12, opacity: 0.7 }}>
        WASD / Arrow Keys to move · Z/Enter=Action · X=Cancel · Esc=Menu · Shift=Sprint — Steps: {stepCount}
        {trainerSpotted && <span style={{ color: '#ffcc00', marginLeft: 8 }}>⚠ Trainer spotted you!</span>}
      </p>

      <div style={{ position: 'relative', display: 'inline-block' }}>
        <canvas
          ref={canvasRef}
          width={CANVAS_WIDTH}
          height={CANVAS_HEIGHT}
          style={{
            border: '2px solid #333',
            imageRendering: 'pixelated',
            width: CANVAS_WIDTH,
            height: CANVAS_HEIGHT,
            display: 'block',
          }}
        />

        {/* Dialogue overlay */}
        {gameMode === 'Dialogue' && dialoguePage && (
          <div
            style={{
              position: 'absolute',
              bottom: 0,
              left: 0,
              right: 0,
            }}
          >
            <DialogueBox
              page={dialoguePage}
              onAdvance={() => {
                const e = engine.current
                if (!e) return
                const result = JSON.parse(e.advance_dialogue())
                if (result.status === 'end') {
                  setGameMode('Overworld')
                  setDialoguePage(null)
                } else if (result.page) {
                  setDialoguePage(result.page)
                }
              }}
              onChoice={(index) => {
                const e = engine.current
                if (!e) return
                const result = JSON.parse(e.select_choice(index))
                if (result.status === 'end') {
                  setGameMode('Overworld')
                  setDialoguePage(null)
                } else if (result.page) {
                  setDialoguePage(result.page)
                }
              }}
            />
          </div>
        )}
      </div>

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

      {/* Pause menu overlay */}
      {pauseOpen && eng && (
        <PauseMenu
          engine={eng}
          onClose={() => setPauseOpen(false)}
        />
      )}

      {/* Map transition overlay */}
      {pendingTransition && (
        <TransitionOverlay
          type={pendingTransition.type}
          onComplete={() => {
            handleTransition(pendingTransition).then(() => {
              activeTransitionRef.current = null
            })
          }}
        />
      )}
    </div>
  )
}

export default App
