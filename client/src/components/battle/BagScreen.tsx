import { useEffect, useState } from 'react'
import type { BagItem, BagItems, PartySneaker } from '../../types/battle'

type Tab = 'heal' | 'battle' | 'cases'
const TAB_LABELS: Record<Tab, string> = {
  heal: 'Heal',
  battle: 'Battle',
  cases: 'Cases',
}

interface BagScreenProps {
  bagItems: BagItems
  party: PartySneaker[]
  isWild: boolean
  onUseItem: (itemId: number, targetIndex?: number) => void
  onCancel: () => void
}

export function BagScreen({ bagItems, party, isWild, onUseItem, onCancel }: BagScreenProps) {
  const [tab, setTab] = useState<Tab>('heal')
  const [itemIdx, setItemIdx] = useState(0)
  const [targeting, setTargeting] = useState(false)
  const [targetIdx, setTargetIdx] = useState(0)

  const tabs: Tab[] = isWild ? ['heal', 'battle', 'cases'] : ['heal', 'battle']
  const items: BagItem[] = bagItems[tab]

  // Reset item index when tab changes
  useEffect(() => setItemIdx(0), [tab])

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return

      if (targeting) {
        if (e.code === 'ArrowUp') setTargetIdx(p => Math.max(0, p - 1))
        if (e.code === 'ArrowDown') setTargetIdx(p => Math.min(party.length - 1, p + 1))
        if (e.code === 'KeyZ' || e.code === 'Enter') {
          e.preventDefault()
          const item = items[itemIdx]
          if (item) onUseItem(item.id, targetIdx)
          setTargeting(false)
        }
        if (e.code === 'KeyX' || e.code === 'Escape') setTargeting(false)
        return
      }

      // Tab navigation
      if (e.code === 'ArrowLeft') {
        const i = tabs.indexOf(tab)
        if (i > 0) setTab(tabs[i - 1])
      }
      if (e.code === 'ArrowRight') {
        const i = tabs.indexOf(tab)
        if (i < tabs.length - 1) setTab(tabs[i + 1])
      }

      if (e.code === 'ArrowUp') setItemIdx(p => Math.max(0, p - 1))
      if (e.code === 'ArrowDown') setItemIdx(p => Math.min(items.length - 1, p + 1))

      if (e.code === 'KeyZ' || e.code === 'Enter') {
        e.preventDefault()
        const item = items[itemIdx]
        if (!item) return
        // Cases and battle items use directly (no target needed)
        if (tab === 'cases' || tab === 'battle') {
          onUseItem(item.id)
        } else {
          // Heal items need a target
          setTargetIdx(0)
          setTargeting(true)
        }
      }
      if (e.code === 'KeyX' || e.code === 'Escape') onCancel()
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [targeting, tab, tabs, items, itemIdx, targetIdx, party, onUseItem, onCancel])

  return (
    <div style={{ fontFamily: 'monospace', color: '#ddd' }}>
      {targeting ? (
        // Target selection
        <div>
          <div style={{ fontSize: 13, color: '#aaa', marginBottom: 8 }}>
            Use on which sneaker?
          </div>
          {party.map((snk, i) => (
            <div
              key={snk.uid}
              onClick={() => {
                const item = items[itemIdx]
                if (item) onUseItem(item.id, i)
                setTargeting(false)
              }}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                padding: '8px 10px',
                background: targetIdx === i ? '#1e2e3e' : '#111',
                border: targetIdx === i ? '1px solid #3498db' : '1px solid #2a2a2a',
                borderRadius: 6,
                marginBottom: 4,
                cursor: snk.is_fainted ? 'not-allowed' : 'pointer',
                opacity: snk.is_fainted ? 0.4 : 1,
              }}
            >
              <span style={{ fontWeight: 'bold' }}>{snk.name}</span>
              <span style={{ color: '#888', fontSize: 12 }}>Lv.{snk.level}</span>
              <span style={{ color: snk.current_hp > snk.max_hp * 0.5 ? '#2ecc71' : '#e74c3c', fontSize: 12 }}>
                {snk.current_hp}/{snk.max_hp} HP
              </span>
              {snk.is_fainted && (
                <span style={{ fontSize: 10, color: '#e74c3c', marginLeft: 'auto' }}>FAINTED</span>
              )}
            </div>
          ))}
          <div style={{ fontSize: 11, color: '#555', marginTop: 6 }}>X — Back</div>
        </div>
      ) : (
        // Bag view
        <>
          {/* Tabs */}
          <div style={{ display: 'flex', gap: 4, marginBottom: 8 }}>
            {tabs.map(t => (
              <button
                key={t}
                onClick={() => setTab(t)}
                style={{
                  padding: '4px 12px',
                  background: tab === t ? '#1e3a1e' : '#111',
                  border: tab === t ? '1px solid #2ecc71' : '1px solid #333',
                  color: tab === t ? '#2ecc71' : '#777',
                  borderRadius: 4,
                  fontFamily: 'monospace',
                  fontSize: 12,
                  cursor: 'pointer',
                }}
              >
                {TAB_LABELS[t]}
              </button>
            ))}
          </div>

          {/* Item list */}
          <div style={{ maxHeight: 180, overflowY: 'auto' }}>
            {items.length === 0 ? (
              <div style={{ color: '#444', fontSize: 13, padding: 8 }}>No items</div>
            ) : (
              items.map((item, i) => (
                <div
                  key={item.id}
                  onClick={() => {
                    setItemIdx(i)
                    if (tab === 'cases' || tab === 'battle') {
                      onUseItem(item.id)
                    } else {
                      setTargetIdx(0)
                      setTargeting(true)
                    }
                  }}
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    padding: '7px 10px',
                    background: itemIdx === i ? '#1e2e3e' : '#111',
                    border: itemIdx === i ? '1px solid #3498db' : '1px solid #1a1a1a',
                    borderRadius: 4,
                    marginBottom: 3,
                    cursor: 'pointer',
                    fontSize: 13,
                  }}
                >
                  <span>{item.name}</span>
                  <span style={{ color: '#888' }}>×{item.qty}</span>
                </div>
              ))
            )}
          </div>

          {/* Description */}
          {items[itemIdx] && (
            <div
              style={{
                marginTop: 8,
                padding: '6px 10px',
                background: '#0a0a0a',
                border: '1px solid #1a1a1a',
                borderRadius: 4,
                fontSize: 11,
                color: '#888',
              }}
            >
              {items[itemIdx].description}
            </div>
          )}

          <div style={{ fontSize: 11, color: '#555', marginTop: 6 }}>
            ← → Tabs · ↑↓ Navigate · Z Use · X Back
          </div>
        </>
      )}
    </div>
  )
}
