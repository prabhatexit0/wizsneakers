import { useEffect, useState } from 'react'
import { getSlotPreviews, getAutoSavePreview, loadFromSlot, loadAutoSave, formatPlayTime } from '../state/saveLoad'
import type { SavePreview } from '../state/saveLoad'

type TitleState = 'main' | 'continue' | 'new_game_name'

const MENU_STYLE: React.CSSProperties = {
  background: '#0a0a0a',
  minHeight: '100vh',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  justifyContent: 'center',
  color: '#e8e8e8',
  fontFamily: '"Courier New", monospace',
}

const TITLE_STYLE: React.CSSProperties = {
  fontSize: 56,
  fontWeight: 'bold',
  color: '#ff6b6b',
  letterSpacing: 8,
  marginBottom: 8,
  textShadow: '0 0 20px rgba(255,107,107,0.5)',
}

const SUBTITLE_STYLE: React.CSSProperties = {
  fontSize: 14,
  color: '#888',
  marginBottom: 48,
  letterSpacing: 4,
}

interface TitleScreenProps {
  onNewGame: (name: string) => void
  onContinue: (saveData: string) => void
}

function SlotPreview({ preview, label }: { preview: SavePreview | null; label: string }) {
  if (!preview) {
    return (
      <div style={{ padding: '12px 16px', color: '#444', fontSize: 13 }}>
        {label} — <em>Empty</em>
      </div>
    )
  }
  return (
    <div style={{ padding: '12px 16px', fontSize: 13 }}>
      <div style={{ color: '#ff6b6b', marginBottom: 2 }}>
        {label}: {preview.player_name}
      </div>
      <div style={{ color: '#aaa', fontSize: 11 }}>
        ⏱ {formatPlayTime(preview.play_time_ms)} &nbsp;|&nbsp;
        📍 {preview.location_name} &nbsp;|&nbsp;
        📦 Caught: {preview.sneakerdex_caught}/30
      </div>
    </div>
  )
}

export function TitleScreen({ onNewGame, onContinue }: TitleScreenProps) {
  const [state, setState] = useState<TitleState>('main')
  const [menuIdx, setMenuIdx] = useState(0)
  const [slotIdx, setSlotIdx] = useState(0)
  const [playerName, setPlayerName] = useState('')
  const [slotPreviews, setSlotPreviews] = useState<(SavePreview | null)[]>([null, null, null])
  const [autosave, setAutosave] = useState<SavePreview | null>(null)

  const hasSave = slotPreviews.some(Boolean) || autosave !== null

  useEffect(() => {
    setSlotPreviews(getSlotPreviews())
    setAutosave(getAutoSavePreview())
  }, [])

  const mainItems = hasSave
    ? ['New Game', 'Continue', 'Options']
    : ['New Game', 'Options']

  // All save slots for display: autosave + 3 manual
  const allSlots: { label: string; preview: SavePreview | null; isAuto: boolean; slotNum: number }[] = [
    { label: 'Autosave', preview: autosave, isAuto: true, slotNum: 0 },
    { label: 'Slot 1', preview: slotPreviews[0], isAuto: false, slotNum: 1 },
    { label: 'Slot 2', preview: slotPreviews[1], isAuto: false, slotNum: 2 },
    { label: 'Slot 3', preview: slotPreviews[2], isAuto: false, slotNum: 3 },
  ]
  const loadableSlots = allSlots.filter(s => s.preview !== null)

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return

      if (state === 'new_game_name') return // handled by input

      if (state === 'main') {
        if (e.code === 'ArrowUp') setMenuIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setMenuIdx(i => Math.min(mainItems.length - 1, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          const item = mainItems[menuIdx]
          if (item === 'New Game') {
            setState('new_game_name')
          } else if (item === 'Continue') {
            setState('continue')
            setSlotIdx(0)
          }
        }
      } else if (state === 'continue') {
        if (e.code === 'ArrowUp') setSlotIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setSlotIdx(i => Math.min(loadableSlots.length - 1, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          const slot = loadableSlots[slotIdx]
          if (slot) {
            const save = slot.isAuto ? loadAutoSave() : loadFromSlot(slot.slotNum)
            if (save) onContinue(save.data)
          }
        }
        if (e.code === 'KeyX' || e.code === 'Escape') {
          setState('main')
        }
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [state, menuIdx, slotIdx, mainItems, loadableSlots, onContinue])

  if (state === 'new_game_name') {
    return (
      <div style={MENU_STYLE}>
        <div style={TITLE_STYLE}>WIZSNEAKERS</div>
        <div style={{ fontSize: 18, marginBottom: 24 }}>Enter your name:</div>
        <input
          autoFocus
          value={playerName}
          onChange={e => setPlayerName(e.target.value.slice(0, 12))}
          onKeyDown={e => {
            if (e.key === 'Enter' && playerName.trim()) {
              onNewGame(playerName.trim())
            }
            if (e.key === 'Escape') {
              setState('main')
            }
          }}
          style={{
            background: '#1a1a1a',
            border: '2px solid #ff6b6b',
            color: '#fff',
            padding: '8px 16px',
            fontSize: 20,
            fontFamily: '"Courier New", monospace',
            outline: 'none',
            marginBottom: 16,
            textAlign: 'center',
          }}
          maxLength={12}
          placeholder="Player"
        />
        <div style={{ fontSize: 12, color: '#666' }}>Press Enter to confirm · Esc to go back</div>
      </div>
    )
  }

  if (state === 'continue') {
    return (
      <div style={MENU_STYLE}>
        <div style={TITLE_STYLE}>WIZSNEAKERS</div>
        <div style={{ fontSize: 16, marginBottom: 24, color: '#aaa' }}>Select Save File</div>
        <div style={{ border: '1px solid #333', minWidth: 320 }}>
          {loadableSlots.length === 0 ? (
            <div style={{ padding: 16, color: '#555' }}>No save files found.</div>
          ) : (
            loadableSlots.map((slot, i) => (
              <div
                key={slot.label}
                style={{
                  borderBottom: i < loadableSlots.length - 1 ? '1px solid #222' : 'none',
                  background: slotIdx === i ? '#1e1e1e' : 'transparent',
                  cursor: 'pointer',
                }}
                onClick={() => {
                  setSlotIdx(i)
                  const save = slot.isAuto ? loadAutoSave() : loadFromSlot(slot.slotNum)
                  if (save) onContinue(save.data)
                }}
              >
                <SlotPreview preview={slot.preview} label={`${slotIdx === i ? '▶ ' : '  '}${slot.label}`} />
              </div>
            ))
          )}
        </div>
        <div style={{ marginTop: 16, fontSize: 12, color: '#555' }}>Z/Enter to load · X/Esc to back</div>
      </div>
    )
  }

  return (
    <div style={MENU_STYLE}>
      <div style={TITLE_STYLE}>WIZSNEAKERS</div>
      <div style={SUBTITLE_STYLE}>SNEAKER CULTURE MEETS CREATURE COLLECTION</div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8, minWidth: 200 }}>
        {mainItems.map((item, i) => (
          <div
            key={item}
            onClick={() => {
              setMenuIdx(i)
              if (item === 'New Game') setState('new_game_name')
              else if (item === 'Continue') { setState('continue'); setSlotIdx(0) }
            }}
            style={{
              padding: '10px 24px',
              fontSize: 18,
              cursor: 'pointer',
              color: menuIdx === i ? '#ff6b6b' : '#888',
              fontWeight: menuIdx === i ? 'bold' : 'normal',
            }}
          >
            {menuIdx === i ? '▶ ' : '  '}{item}
          </div>
        ))}
      </div>
      <div style={{ marginTop: 48, fontSize: 11, color: '#333' }}>
        Arrow Keys to navigate · Z/Enter to select
      </div>
    </div>
  )
}
