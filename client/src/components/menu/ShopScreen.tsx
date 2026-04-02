import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'

interface ShopItem {
  id: number
  name: string
  qty?: number
  description: string
  cost: number
  sell_price: number
}

interface ShopInventory {
  heal: ShopItem[]
  battle: ShopItem[]
  cases: ShopItem[]
  held: ShopItem[]
  key_items: ShopItem[]
}

interface ShopScreenProps {
  engine: GameEngine
  shopId: number
  onClose: () => void
}

// Default shop stock — in a full game this would come from shop data
const DEFAULT_SHOP_STOCK: number[] = [1, 2, 3, 5, 6, 7, 8, 9, 10, 30, 31]

type ShopTab = 'buy' | 'sell'

export function ShopScreen({ engine, onClose }: ShopScreenProps) {
  const anyEng = engine as unknown as Record<string, (...args: unknown[]) => string | number>
  const [tab, setTab] = useState<ShopTab>('buy')
  const [itemIdx, setItemIdx] = useState(0)
  const [qty, setQty] = useState(1)
  const [message, setMessage] = useState<string | null>(null)
  const [money, setMoney] = useState<number>(() => {
    const info = JSON.parse(anyEng['get_player_info']()) as { money: number }
    return info.money
  })
  const [, forceUpdate] = useState(0)

  const refreshMoney = () => {
    const info = JSON.parse(anyEng['get_player_info']()) as { money: number }
    setMoney(info.money)
  }

  const inv = JSON.parse(anyEng['get_inventory']()) as ShopInventory
  const allPlayerItems: ShopItem[] = [
    ...inv.heal,
    ...inv.battle,
    ...inv.cases,
    ...inv.held,
  ]

  const buyItems: ShopItem[] = DEFAULT_SHOP_STOCK.map(id => {
    try {
      // We'll use a dummy item structure based on what we know from data
      return { id, name: `Item #${id}`, qty: undefined, description: '', cost: 200, sell_price: 100 }
    } catch {
      return null
    }
  }).filter(Boolean) as ShopItem[]

  // Enrich buy items from the bag data if player has them (else from known data)
  // In practice the item data comes from the WASM module after buy; use player inv as reference

  const sellItems = allPlayerItems.filter(i => (i.qty ?? 0) > 0)

  const displayItems = tab === 'buy' ? buyItems : sellItems
  const selectedItem = displayItems[itemIdx]

  useEffect(() => { setItemIdx(0); setQty(1); setMessage(null) }, [tab])

  const doAction = () => {
    if (!selectedItem) return
    if (tab === 'buy') {
      const result = JSON.parse(anyEng['buy_item'](selectedItem.id, qty) as string) as { ok: boolean; money?: number; error?: string }
      if (result.ok) {
        setMessage(`You bought ${qty}x ${selectedItem.name}!`)
        setMoney(result.money ?? money)
        forceUpdate(n => n + 1)
      } else {
        setMessage(result.error ?? 'Cannot buy.')
      }
    } else {
      const result = JSON.parse(anyEng['sell_item'](selectedItem.id, qty) as string) as { ok: boolean; money?: number; error?: string }
      if (result.ok) {
        setMessage(`You sold ${qty}x ${selectedItem.name}!`)
        setMoney(result.money ?? money)
        forceUpdate(n => n + 1)
      } else {
        setMessage(result.error ?? 'Cannot sell.')
      }
    }
    refreshMoney()
  }

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (message) { setMessage(null); return }

      if (e.code === 'ArrowLeft' || e.code === 'ArrowRight') {
        setTab(t => t === 'buy' ? 'sell' : 'buy')
        return
      }
      if (e.code === 'ArrowUp') setItemIdx(i => Math.max(0, i - 1))
      if (e.code === 'ArrowDown') setItemIdx(i => Math.min(displayItems.length - 1, i + 1))
      if (e.code === 'KeyQ') setQty(q => Math.max(1, q - 1))
      if (e.code === 'KeyE') setQty(q => Math.min(99, q + 1))
      if (e.code === 'KeyZ' || e.code === 'Enter') doAction()
      if (e.code === 'KeyX' || e.code === 'Escape') onClose()
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [tab, itemIdx, qty, displayItems, selectedItem, message, onClose])

  const total = selectedItem
    ? (tab === 'buy' ? selectedItem.cost : selectedItem.sell_price) * qty
    : 0

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
        <div style={{ fontSize: 20, color: '#ff6b6b' }}>Shop</div>
        <div style={{ fontSize: 16, color: '#f39c12' }}>${money.toLocaleString()}</div>
      </div>

      {/* Tabs */}
      <div style={{ display: 'flex', gap: 0, marginBottom: 16 }}>
        {(['buy', 'sell'] as ShopTab[]).map(t => (
          <button
            key={t}
            onClick={() => setTab(t)}
            style={{
              flex: 1,
              padding: '8px',
              background: tab === t ? '#ff6b6b' : '#1a1a1a',
              color: tab === t ? '#fff' : '#666',
              border: '1px solid #333',
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 14,
              textTransform: 'capitalize',
            }}
          >
            {t}
          </button>
        ))}
      </div>

      {message && (
        <div style={{ background: '#1e2a1e', border: '1px solid #27ae60', borderRadius: 4, padding: '8px 12px', marginBottom: 12, fontSize: 13, color: '#27ae60' }}>
          {message}
        </div>
      )}

      {/* Item list */}
      <div style={{ flex: 1, overflow: 'auto', marginBottom: 12 }}>
        {displayItems.length === 0 ? (
          <div style={{ color: '#555', padding: 8 }}>Nothing available.</div>
        ) : (
          displayItems.map((item, i) => (
            <div
              key={item.id}
              onClick={() => setItemIdx(i)}
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                padding: '8px 10px',
                background: itemIdx === i ? '#1e1e1e' : 'transparent',
                cursor: 'pointer',
                fontSize: 13,
                borderRadius: 3,
              }}
            >
              <span style={{ color: itemIdx === i ? '#fff' : '#ccc' }}>
                {itemIdx === i ? '▶ ' : '  '}{item.name}
                {item.qty !== undefined && <span style={{ color: '#888' }}> ×{item.qty}</span>}
              </span>
              <span style={{ color: '#f39c12' }}>
                ${tab === 'buy' ? item.cost : item.sell_price}
              </span>
            </div>
          ))
        )}
      </div>

      {/* Quantity + total */}
      {selectedItem && (
        <div
          style={{
            background: '#1a1a1a',
            border: '1px solid #333',
            borderRadius: 4,
            padding: 12,
            marginBottom: 12,
          }}
        >
          <div style={{ fontSize: 12, color: '#aaa', marginBottom: 6 }}>{selectedItem.description}</div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <span style={{ fontSize: 13 }}>Qty:</span>
            <span style={{ fontSize: 13, color: '#888' }}>Q/E to adjust</span>
            <span style={{ fontSize: 16, fontWeight: 'bold' }}>{qty}</span>
            <span style={{ marginLeft: 'auto', fontSize: 14, color: '#f39c12' }}>
              Total: ${total.toLocaleString()}
            </span>
          </div>
        </div>
      )}

      <div style={{ fontSize: 11, color: '#555' }}>
        ◀▶ Switch tab · ↑↓ Select · Q/E Quantity · Z Confirm · X/Esc Close
      </div>
    </div>
  )
}
