import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'
import { saveToSlot, getSlotPreviews, formatPlayTime } from '../../state/saveLoad'
import type { SavePreview } from '../../state/saveLoad'

type SaveState = 'selecting' | 'confirming' | 'saved'

interface SaveScreenProps {
  engine: GameEngine
  onSaved: () => void
}

export function SaveScreen({ engine, onSaved }: SaveScreenProps) {
  const [slotIdx, setSlotIdx] = useState(0)
  const [saveState, setSaveState] = useState<SaveState>('selecting')
  const [previews, setPreviews] = useState<(SavePreview | null)[]>(() => getSlotPreviews())

  const slot = slotIdx + 1

  useEffect(() => {
    if (saveState !== 'saved') return
    const timer = setTimeout(() => {
      setSaveState('selecting')
      setPreviews(getSlotPreviews())
      onSaved()
    }, 1000)
    return () => clearTimeout(timer)
  }, [saveState, onSaved])

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (saveState === 'saved') return

      if (saveState === 'selecting') {
        if (e.code === 'ArrowUp') setSlotIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setSlotIdx(i => Math.min(2, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          setSaveState('confirming')
        }
      } else if (saveState === 'confirming') {
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          saveToSlot(slot, engine)
          setSaveState('saved')
        }
        if (e.code === 'KeyX' || e.code === 'Escape') {
          setSaveState('selecting')
        }
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [saveState, slotIdx, slot, engine])

  if (saveState === 'saved') {
    return (
      <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
        <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 16 }}>Save</div>
        <div
          style={{
            background: '#1e2a1e',
            border: '1px solid #27ae60',
            borderRadius: 4,
            padding: '16px 20px',
            fontSize: 16,
            color: '#27ae60',
            textAlign: 'center',
          }}
        >
          Game Saved!
        </div>
      </div>
    )
  }

  if (saveState === 'confirming') {
    const preview = previews[slotIdx]
    return (
      <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
        <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 16 }}>Save</div>
        <div
          style={{
            background: '#1a1a1a',
            border: '1px solid #f39c12',
            borderRadius: 4,
            padding: 16,
            marginBottom: 16,
          }}
        >
          {preview ? (
            <div>
              <div style={{ fontSize: 14, color: '#f39c12', marginBottom: 4 }}>
                ⚠ Overwrite existing save?
              </div>
              <div style={{ fontSize: 12, color: '#888' }}>
                {preview.player_name} · {formatPlayTime(preview.play_time_ms)}
              </div>
            </div>
          ) : (
            <div style={{ fontSize: 14 }}>Save to Slot {slot}?</div>
          )}
        </div>
        <div style={{ display: 'flex', gap: 12 }}>
          <button
            onClick={() => { saveToSlot(slot, engine); setSaveState('saved') }}
            style={{
              flex: 1,
              padding: '10px',
              background: '#27ae60',
              color: '#fff',
              border: 'none',
              borderRadius: 4,
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 14,
            }}
          >
            Z — Confirm
          </button>
          <button
            onClick={() => setSaveState('selecting')}
            style={{
              flex: 1,
              padding: '10px',
              background: '#1a1a1a',
              color: '#888',
              border: '1px solid #333',
              borderRadius: 4,
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 14,
            }}
          >
            X — Cancel
          </button>
        </div>
      </div>
    )
  }

  return (
    <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
      <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 16 }}>Save</div>
      <div style={{ fontSize: 13, color: '#888', marginBottom: 12 }}>Select a save slot:</div>

      {[0, 1, 2].map(i => {
        const preview = previews[i]
        return (
          <div
            key={i}
            onClick={() => setSlotIdx(i)}
            style={{
              background: slotIdx === i ? '#1e1e1e' : '#111',
              border: `1px solid ${slotIdx === i ? '#ff6b6b' : '#333'}`,
              borderRadius: 4,
              padding: '10px 14px',
              marginBottom: 8,
              cursor: 'pointer',
            }}
          >
            <div style={{ fontSize: 13, color: slotIdx === i ? '#ff6b6b' : '#aaa', marginBottom: 4 }}>
              {slotIdx === i ? '▶ ' : '  '}Slot {i + 1}
            </div>
            {preview ? (
              <div style={{ fontSize: 12, color: '#888' }}>
                {preview.player_name} · {formatPlayTime(preview.play_time_ms)} · Caught: {preview.sneakerdex_caught}/30
              </div>
            ) : (
              <div style={{ fontSize: 12, color: '#444', fontStyle: 'italic' }}>— Empty —</div>
            )}
          </div>
        )
      })}

      <div style={{ marginTop: 8, fontSize: 11, color: '#555' }}>↑↓ Select · Z to save</div>
    </div>
  )
}
