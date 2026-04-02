import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'
import { PlayerCard } from './PlayerCard'
import { OptionsScreen } from './OptionsScreen'
import { InventoryScreen } from './InventoryScreen'
import { SneakerdexScreen } from './SneakerdexScreen'
import { SaveScreen } from './SaveScreen'

type Tab = 'party' | 'bag' | 'dex' | 'player' | 'options' | 'save'
const TABS: Tab[] = ['party', 'bag', 'dex', 'player', 'options', 'save']
const TAB_LABELS: Record<Tab, string> = {
  party: 'Party',
  bag: 'Bag',
  dex: 'Dex',
  player: 'Player',
  options: 'Options',
  save: 'Save',
}

interface PauseMenuProps {
  engine: GameEngine
  onClose: () => void
}

export function PauseMenu({ engine, onClose }: PauseMenuProps) {
  const [tab, setTab] = useState<Tab>('party')

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (e.code === 'Escape' || e.code === 'KeyX') {
        onClose()
        return
      }
      if (e.code === 'ArrowLeft') {
        setTab(t => {
          const i = TABS.indexOf(t)
          return TABS[Math.max(0, i - 1)]
        })
      }
      if (e.code === 'ArrowRight') {
        setTab(t => {
          const i = TABS.indexOf(t)
          return TABS[Math.min(TABS.length - 1, i + 1)]
        })
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [onClose])

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.85)',
        zIndex: 100,
        display: 'flex',
        flexDirection: 'column',
        fontFamily: '"Courier New", monospace',
        color: '#e8e8e8',
      }}
    >
      {/* Tab bar */}
      <div
        style={{
          display: 'flex',
          borderBottom: '2px solid #333',
          background: '#111',
        }}
      >
        {TABS.map(t => (
          <button
            key={t}
            onClick={() => setTab(t)}
            style={{
              flex: 1,
              padding: '12px 4px',
              background: tab === t ? '#1e1e1e' : 'transparent',
              color: tab === t ? '#ff6b6b' : '#666',
              border: 'none',
              borderBottom: tab === t ? '2px solid #ff6b6b' : '2px solid transparent',
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 13,
              fontWeight: tab === t ? 'bold' : 'normal',
            }}
          >
            {TAB_LABELS[t]}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div style={{ flex: 1, overflow: 'auto', padding: 16 }}>
        {tab === 'party' && <PartyTab engine={engine} />}
        {tab === 'bag' && <InventoryScreen engine={engine} />}
        {tab === 'dex' && <SneakerdexScreen engine={engine} />}
        {tab === 'player' && <PlayerCard engine={engine} />}
        {tab === 'options' && <OptionsScreen />}
        {tab === 'save' && <SaveScreen engine={engine} onSaved={() => {}} />}
      </div>

      <div
        style={{
          padding: '8px 16px',
          fontSize: 11,
          color: '#444',
          borderTop: '1px solid #222',
          background: '#111',
        }}
      >
        ◀▶ Switch Tab · X/Esc Close
      </div>
    </div>
  )
}

function PartyTab({ engine }: { engine: GameEngine }) {
  const partyJson = engine.get_party_state()
  const party = JSON.parse(partyJson) as Array<{
    uid: number
    name: string
    level: number
    current_hp: number
    max_hp: number
    faction: string
    status: string | null
    is_fainted: boolean
  }>

  if (party.length === 0) {
    return <div style={{ color: '#666', padding: 16 }}>No sneakers in party.</div>
  }

  return (
    <div>
      <div style={{ fontSize: 16, marginBottom: 16, color: '#ff6b6b' }}>Party</div>
      {party.map((snk, i) => {
        const hpPct = snk.max_hp > 0 ? snk.current_hp / snk.max_hp : 0
        const hpColor = hpPct > 0.5 ? '#27ae60' : hpPct > 0.2 ? '#f39c12' : '#e74c3c'
        return (
          <div
            key={snk.uid}
            style={{
              background: '#1a1a1a',
              border: '1px solid #333',
              borderRadius: 4,
              padding: '10px 14px',
              marginBottom: 8,
              display: 'flex',
              alignItems: 'center',
              gap: 12,
            }}
          >
            <div style={{ fontSize: 14, color: '#666', width: 20 }}>#{i + 1}</div>
            <div style={{ flex: 1 }}>
              <div style={{ fontSize: 14, fontWeight: 'bold', color: snk.is_fainted ? '#555' : '#e8e8e8' }}>
                {snk.name} <span style={{ fontSize: 11, color: '#888' }}>Lv.{snk.level}</span>
              </div>
              <div style={{ marginTop: 4 }}>
                <div
                  style={{
                    height: 6,
                    background: '#222',
                    borderRadius: 3,
                    overflow: 'hidden',
                    width: 160,
                  }}
                >
                  <div
                    style={{
                      height: '100%',
                      width: `${hpPct * 100}%`,
                      background: hpColor,
                      borderRadius: 3,
                    }}
                  />
                </div>
                <div style={{ fontSize: 10, color: '#888', marginTop: 2 }}>
                  {snk.current_hp}/{snk.max_hp} HP
                  {snk.status && <span style={{ color: '#e67e22', marginLeft: 8 }}>[{snk.status}]</span>}
                  {snk.is_fainted && <span style={{ color: '#e74c3c', marginLeft: 8 }}>FAINTED</span>}
                </div>
              </div>
            </div>
            <div style={{ fontSize: 11, color: '#666' }}>{snk.faction}</div>
          </div>
        )
      })}
    </div>
  )
}
