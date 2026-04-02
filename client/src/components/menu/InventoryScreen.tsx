import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'

type Pocket = 'heal' | 'battle' | 'cases' | 'held' | 'key'
const POCKETS: Pocket[] = ['heal', 'battle', 'cases', 'held', 'key']
const POCKET_LABELS: Record<Pocket, string> = {
  heal: 'Heal',
  battle: 'Battle',
  cases: 'Cases',
  held: 'Held',
  key: 'Key',
}

interface InventoryItem {
  id: number
  name: string
  qty?: number
  description: string
  category: string
}

interface InventoryData {
  heal: InventoryItem[]
  battle: InventoryItem[]
  cases: InventoryItem[]
  held: InventoryItem[]
  key_items: InventoryItem[]
}

interface PartySneaker {
  uid: number
  name: string
  level: number
  current_hp: number
  max_hp: number
  is_fainted: boolean
}

interface InventoryScreenProps {
  engine: GameEngine
}

export function InventoryScreen({ engine }: InventoryScreenProps) {
  const anyEng = engine as unknown as Record<string, (...args: unknown[]) => string>
  const [pocket, setPocket] = useState<Pocket>('heal')
  const [itemIdx, setItemIdx] = useState(0)
  const [targeting, setTargeting] = useState(false)
  const [targetIdx, setTargetIdx] = useState(0)
  const [message, setMessage] = useState<string | null>(null)
  const [, forceUpdate] = useState(0)

  const inv = JSON.parse(anyEng['get_inventory']()) as InventoryData
  const party = JSON.parse(anyEng['get_party']()) as PartySneaker[]

  const pocketItems: InventoryItem[] = pocket === 'key'
    ? inv.key_items
    : pocket === 'battle'
    ? inv.battle
    : pocket === 'cases'
    ? inv.cases
    : pocket === 'held'
    ? inv.held
    : inv.heal

  useEffect(() => { setItemIdx(0) }, [pocket])

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (message) { setMessage(null); return }

      if (targeting) {
        if (e.code === 'ArrowUp') setTargetIdx(i => Math.max(0, i - 1))
        if (e.code === 'ArrowDown') setTargetIdx(i => Math.min(party.length - 1, i + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          e.preventDefault()
          const item = pocketItems[itemIdx]
          if (item) {
            const result = JSON.parse(anyEng['use_item'](item.id, targetIdx)) as { ok: boolean; error?: string }
            if (result.ok) {
              setMessage(`Used ${item.name}!`)
              forceUpdate(n => n + 1)
            } else {
              setMessage(result.error ?? 'Cannot use item.')
            }
          }
          setTargeting(false)
        }
        if (e.code === 'KeyX' || e.code === 'Escape') setTargeting(false)
        return
      }

      if (e.code === 'ArrowLeft') {
        const i = POCKETS.indexOf(pocket)
        setPocket(POCKETS[Math.max(0, i - 1)])
      }
      if (e.code === 'ArrowRight') {
        const i = POCKETS.indexOf(pocket)
        setPocket(POCKETS[Math.min(POCKETS.length - 1, i + 1)])
      }
      if (e.code === 'ArrowUp') setItemIdx(i => Math.max(0, i - 1))
      if (e.code === 'ArrowDown') setItemIdx(i => Math.min(pocketItems.length - 1, i + 1))
      if (e.code === 'KeyZ' || e.code === 'Enter') {
        const item = pocketItems[itemIdx]
        if (!item) return
        if (pocket === 'heal') {
          setTargetIdx(0)
          setTargeting(true)
        } else if (pocket === 'key') {
          setMessage(`${item.name}: ${item.description}`)
        } else {
          setMessage('Cannot use this item outside of battle.')
        }
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [pocket, itemIdx, targeting, targetIdx, pocketItems, party.length, message, anyEng])

  const selectedItem = pocketItems[itemIdx]

  if (targeting) {
    return (
      <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
        <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 8 }}>Use on which sneaker?</div>
        <div style={{ fontSize: 13, color: '#888', marginBottom: 16 }}>
          Using: {selectedItem?.name}
        </div>
        {party.map((snk, i) => (
          <div
            key={snk.uid}
            style={{
              background: '#1a1a1a',
              border: `1px solid ${targetIdx === i ? '#ff6b6b' : '#333'}`,
              borderRadius: 4,
              padding: '10px 14px',
              marginBottom: 6,
              cursor: 'pointer',
              opacity: snk.is_fainted ? 0.5 : 1,
            }}
            onClick={() => { setTargetIdx(i) }}
          >
            <span style={{ color: targetIdx === i ? '#ff6b6b' : '#aaa' }}>
              {targetIdx === i ? '▶ ' : '  '}
            </span>
            {snk.name} Lv.{snk.level} &nbsp;
            <span style={{ fontSize: 11, color: '#888' }}>
              {snk.current_hp}/{snk.max_hp} HP
              {snk.is_fainted && <span style={{ color: '#e74c3c' }}> [FAINTED]</span>}
            </span>
          </div>
        ))}
        <div style={{ marginTop: 8, fontSize: 11, color: '#555' }}>Z/Enter to confirm · X/Esc to back</div>
      </div>
    )
  }

  return (
    <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
      <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 12 }}>Bag</div>

      {message && (
        <div
          style={{
            background: '#1e2a1e',
            border: '1px solid #27ae60',
            borderRadius: 4,
            padding: '8px 12px',
            marginBottom: 12,
            fontSize: 13,
            color: '#27ae60',
          }}
        >
          {message}
        </div>
      )}

      {/* Pocket tabs */}
      <div style={{ display: 'flex', gap: 4, marginBottom: 12 }}>
        {POCKETS.map(p => (
          <button
            key={p}
            onClick={() => setPocket(p)}
            style={{
              flex: 1,
              padding: '6px 2px',
              background: pocket === p ? '#ff6b6b' : '#1a1a1a',
              color: pocket === p ? '#fff' : '#666',
              border: '1px solid #333',
              borderRadius: 3,
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 11,
            }}
          >
            {POCKET_LABELS[p]}
          </button>
        ))}
      </div>

      <div style={{ display: 'flex', gap: 12 }}>
        {/* Item list */}
        <div style={{ flex: 1 }}>
          {pocketItems.length === 0 ? (
            <div style={{ color: '#555', fontSize: 13, padding: 8 }}>Empty</div>
          ) : (
            pocketItems.map((item, i) => (
              <div
                key={item.id}
                onClick={() => setItemIdx(i)}
                style={{
                  background: itemIdx === i ? '#1e1e1e' : 'transparent',
                  border: `1px solid ${itemIdx === i ? '#444' : 'transparent'}`,
                  borderRadius: 3,
                  padding: '6px 10px',
                  cursor: 'pointer',
                  fontSize: 13,
                  display: 'flex',
                  justifyContent: 'space-between',
                }}
              >
                <span style={{ color: itemIdx === i ? '#fff' : '#ccc' }}>
                  {itemIdx === i ? '▶ ' : '  '}{item.name}
                </span>
                {item.qty !== undefined && (
                  <span style={{ color: '#888' }}>×{item.qty}</span>
                )}
              </div>
            ))
          )}
        </div>

        {/* Description panel */}
        {selectedItem && (
          <div
            style={{
              width: 180,
              background: '#1a1a1a',
              border: '1px solid #333',
              borderRadius: 4,
              padding: 10,
              fontSize: 12,
              color: '#aaa',
              alignSelf: 'flex-start',
            }}
          >
            <div style={{ color: '#fff', marginBottom: 6, fontSize: 13, fontWeight: 'bold' }}>
              {selectedItem.name}
            </div>
            <div style={{ lineHeight: 1.5 }}>{selectedItem.description}</div>
            {pocket === 'heal' && (
              <div style={{ marginTop: 8, fontSize: 11, color: '#888' }}>
                Z to use on party member
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
