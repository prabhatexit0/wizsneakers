import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'
import { FACTION_COLORS } from '../../types/game'
import type { Faction } from '../../types/game'

interface PartySneaker {
  uid: number
  name: string
  level: number
  current_hp: number
  max_hp: number
  faction: string
  status: string | null
  is_fainted: boolean
}

interface BoxSneaker {
  uid: number
  species_id: number
  level: number
  current_hp: number
  max_hp: number
  status: string | null
}

interface BoxData {
  sneakers: BoxSneaker[]
}

type SortKey = 'level' | 'name' | 'faction'
type ActivePanel = 'party' | 'box'

interface SneakerBoxScreenProps {
  engine: GameEngine
  onClose: () => void
}

export function SneakerBoxScreen({ engine, onClose }: SneakerBoxScreenProps) {
  const anyEng = engine as unknown as Record<string, (...args: unknown[]) => string>
  const [activePanel, setActivePanel] = useState<ActivePanel>('party')
  const [partyIdx, setPartyIdx] = useState(0)
  const [boxIdx, setBoxIdx] = useState(0)
  const [sortKey] = useState<SortKey>('level')
  const [message, setMessage] = useState<string | null>(null)
  const [, forceUpdate] = useState(0)

  const party = JSON.parse(anyEng['get_party']()) as PartySneaker[]

  const getBox = (): BoxSneaker[] => {
    try {
      const saveJson = anyEng['export_save']()
      const save = JSON.parse(saveJson) as { player: { sneaker_box: BoxData } }
      return save.player.sneaker_box.sneakers
    } catch {
      return []
    }
  }

  const box = getBox()

  const sortedBox = [...box].sort((a, b) => {
    if (sortKey === 'level') return b.level - a.level
    return 0
  })

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (message) { setMessage(null); return }

      if (e.code === 'ArrowLeft' || e.code === 'ArrowRight') {
        setActivePanel(p => p === 'party' ? 'box' : 'party')
        return
      }
      if (activePanel === 'party') {
        if (e.code === 'ArrowUp') setPartyIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setPartyIdx(i => Math.min(party.length - 1, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          // Deposit selected party member
          const result = JSON.parse(anyEng['deposit_sneaker'](partyIdx)) as { ok: boolean; error?: string }
          if (result.ok) {
            setMessage(`Deposited to box.`)
            setPartyIdx(i => Math.max(0, i - 1))
            forceUpdate(n => n + 1)
          } else {
            setMessage(result.error ?? 'Cannot deposit.')
          }
        }
      } else {
        if (e.code === 'ArrowUp') setBoxIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setBoxIdx(i => Math.min(sortedBox.length - 1, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          // Withdraw selected box sneaker
          const result = JSON.parse(anyEng['withdraw_sneaker'](boxIdx)) as { ok: boolean; error?: string }
          if (result.ok) {
            setMessage(`Added to party.`)
            setBoxIdx(i => Math.max(0, i - 1))
            forceUpdate(n => n + 1)
          } else {
            setMessage(result.error ?? 'Cannot withdraw.')
          }
        }
      }
      if (e.code === 'KeyX' || e.code === 'Escape') onClose()
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [activePanel, partyIdx, boxIdx, party.length, sortedBox.length, message, anyEng, onClose])

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.9)',
        zIndex: 200,
        display: 'flex',
        flexDirection: 'column',
        fontFamily: '"Courier New", monospace',
        color: '#e8e8e8',
        padding: 24,
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 16 }}>
        <div style={{ fontSize: 20, color: '#ff6b6b' }}>Sneaker Box</div>
        <div style={{ fontSize: 12, color: '#888' }}>
          Party: {party.length}/6 &nbsp;|&nbsp; Box: {box.length}/50
        </div>
      </div>

      {message && (
        <div style={{ background: '#1e2a1e', border: '1px solid #27ae60', borderRadius: 4, padding: '8px 12px', marginBottom: 12, fontSize: 13, color: '#27ae60' }}>
          {message}
        </div>
      )}

      <div style={{ display: 'flex', gap: 16, flex: 1 }}>
        {/* Party panel */}
        <div
          style={{
            flex: 1,
            border: `1px solid ${activePanel === 'party' ? '#ff6b6b' : '#333'}`,
            borderRadius: 4,
            padding: 12,
          }}
        >
          <div style={{ fontSize: 14, color: activePanel === 'party' ? '#ff6b6b' : '#888', marginBottom: 8 }}>
            Party ({party.length}/6)
          </div>
          {party.map((snk, i) => {
            const hpPct = snk.max_hp > 0 ? snk.current_hp / snk.max_hp : 0
            const color = FACTION_COLORS[snk.faction as Faction] ?? '#888'
            return (
              <div
                key={snk.uid}
                onClick={() => { setActivePanel('party'); setPartyIdx(i) }}
                style={{
                  background: partyIdx === i && activePanel === 'party' ? '#1e1e1e' : 'transparent',
                  border: `1px solid ${partyIdx === i && activePanel === 'party' ? '#444' : 'transparent'}`,
                  borderRadius: 3,
                  padding: '6px 8px',
                  marginBottom: 4,
                  cursor: 'pointer',
                  fontSize: 12,
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                  <span style={{ color }}>{snk.name}</span>
                  <span style={{ color: '#888' }}>Lv.{snk.level}</span>
                </div>
                <div style={{ height: 4, background: '#222', borderRadius: 2, marginTop: 3, overflow: 'hidden' }}>
                  <div style={{ width: `${hpPct * 100}%`, height: '100%', background: hpPct > 0.5 ? '#27ae60' : '#e74c3c', borderRadius: 2 }} />
                </div>
              </div>
            )
          })}
          {party.length === 0 && <div style={{ color: '#555', fontSize: 12 }}>Empty</div>}
          <div style={{ marginTop: 8, fontSize: 10, color: '#555' }}>Z to deposit</div>
        </div>

        {/* Box panel */}
        <div
          style={{
            flex: 2,
            border: `1px solid ${activePanel === 'box' ? '#ff6b6b' : '#333'}`,
            borderRadius: 4,
            padding: 12,
            overflow: 'auto',
          }}
        >
          <div style={{ fontSize: 14, color: activePanel === 'box' ? '#ff6b6b' : '#888', marginBottom: 8 }}>
            Box ({box.length}/50)
          </div>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(5, 1fr)', gap: 4 }}>
            {sortedBox.map((snk, i) => (
              <div
                key={snk.uid}
                onClick={() => { setActivePanel('box'); setBoxIdx(i) }}
                style={{
                  background: boxIdx === i && activePanel === 'box' ? '#1e1e1e' : '#111',
                  border: `1px solid ${boxIdx === i && activePanel === 'box' ? '#ff6b6b' : '#333'}`,
                  borderRadius: 3,
                  padding: '4px 6px',
                  cursor: 'pointer',
                  fontSize: 10,
                  textAlign: 'center',
                }}
              >
                <div style={{ fontSize: 16 }}>👟</div>
                <div style={{ color: '#aaa' }}>Lv.{snk.level}</div>
              </div>
            ))}
          </div>
          {sortedBox.length === 0 && <div style={{ color: '#555', fontSize: 12 }}>Box is empty.</div>}
          <div style={{ marginTop: 8, fontSize: 10, color: '#555' }}>Z to withdraw</div>
        </div>
      </div>

      <div style={{ marginTop: 12, fontSize: 11, color: '#555' }}>
        ◀▶ Switch panel · ↑↓ Select · Z Deposit/Withdraw · X/Esc Close
      </div>
    </div>
  )
}
