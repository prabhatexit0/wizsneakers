import { useState, useEffect } from 'react'

interface Props {
  onConfirm: (name: string) => void
}

const DEFAULT_NAME = 'Red'
const MAX_NAME_LEN = 10

/**
 * NameEntryScreen — pixel-art styled name input for new game.
 * Type name, Enter to confirm, Backspace to delete.
 * Blank name defaults to "Red".
 */
export function NameEntryScreen({ onConfirm }: Props) {
  const [name, setName] = useState('')

  useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (e.repeat) return

      if (e.key === 'Enter') {
        const finalName = name.trim() || DEFAULT_NAME
        onConfirm(finalName)
        return
      }

      if (e.key === 'Backspace') {
        setName(prev => prev.slice(0, -1))
        return
      }

      // Accept printable ASCII characters
      if (e.key.length === 1 && name.length < MAX_NAME_LEN) {
        setName(prev => prev + e.key)
      }
    }

    window.addEventListener('keydown', handleKey)
    return () => window.removeEventListener('keydown', handleKey)
  }, [name, onConfirm])

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: '#0a0a1a',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        fontFamily: '"Courier New", monospace',
        color: '#e0e0e0',
        zIndex: 100,
      }}
    >
      {/* Pixel-art dialog box */}
      <div
        style={{
          border: '3px solid #e0e0e0',
          background: '#1a1a2e',
          padding: '32px 48px',
          minWidth: 320,
          textAlign: 'center',
          boxShadow: '4px 4px 0 #000',
        }}
      >
        <div style={{ fontSize: 12, marginBottom: 16, opacity: 0.7, letterSpacing: 2 }}>
          NEW GAME
        </div>
        <div style={{ fontSize: 16, marginBottom: 24 }}>
          What is your name?
        </div>

        {/* Name display box */}
        <div
          style={{
            border: '2px solid #e0e0e0',
            background: '#0a0a1a',
            padding: '8px 16px',
            fontSize: 20,
            letterSpacing: 4,
            minWidth: 200,
            minHeight: 36,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            marginBottom: 24,
            color: name ? '#ffffff' : '#555',
          }}
        >
          {name || DEFAULT_NAME}
          <span
            style={{
              display: 'inline-block',
              width: 2,
              height: 20,
              background: '#ffffff',
              marginLeft: 2,
              animation: 'blink 1s step-end infinite',
            }}
          />
        </div>

        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 8 }}>
          Type your name (max {MAX_NAME_LEN} characters)
        </div>
        <div style={{ fontSize: 11, opacity: 0.6 }}>
          Press <strong>ENTER</strong> to confirm
        </div>
      </div>

      <style>{`
        @keyframes blink {
          0%, 100% { opacity: 1; }
          50% { opacity: 0; }
        }
      `}</style>
    </div>
  )
}
