import { useEffect, useRef, useState } from 'react'

interface BattleLogProps {
  messages: string[]
  isAnimating: boolean
  onAdvance?: () => void
}

const TYPEWRITER_SPEED_MS = 18

export function BattleLog({ messages, isAnimating, onAdvance }: BattleLogProps) {
  const scrollRef = useRef<HTMLDivElement>(null)
  const [lastMessageFull, setLastMessageFull] = useState('')
  const [displayedChars, setDisplayedChars] = useState(0)
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const lastMessage = messages[messages.length - 1] ?? ''

  // When a new message arrives, start typewriter
  useEffect(() => {
    if (!lastMessage) return
    setLastMessageFull(lastMessage)
    setDisplayedChars(0)

    if (timerRef.current) clearInterval(timerRef.current)
    timerRef.current = setInterval(() => {
      setDisplayedChars(prev => {
        if (prev >= lastMessage.length) {
          if (timerRef.current) clearInterval(timerRef.current)
          return prev
        }
        return prev + 1
      })
    }, TYPEWRITER_SPEED_MS)

    return () => {
      if (timerRef.current) clearInterval(timerRef.current)
    }
  }, [lastMessage])

  // Skip typewriter on Z/Enter
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.code === 'KeyZ' || e.code === 'Enter' || e.code === 'Space') {
        if (displayedChars < lastMessageFull.length) {
          // Skip to end of typewriter
          setDisplayedChars(lastMessageFull.length)
          if (timerRef.current) clearInterval(timerRef.current)
        } else if (!isAnimating && onAdvance) {
          onAdvance()
        }
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [displayedChars, lastMessageFull, isAnimating, onAdvance])

  // Auto-scroll to bottom
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
    }
  }, [messages, displayedChars])

  const prevMessages = messages.slice(0, -1)
  const isLastComplete = displayedChars >= lastMessageFull.length

  return (
    <div
      style={{
        background: '#0a0d0a',
        border: '2px solid #2a3a2a',
        borderRadius: 6,
        padding: '10px 12px',
        fontFamily: 'monospace',
        fontSize: 13,
        lineHeight: 1.6,
        minHeight: 80,
        maxHeight: 120,
        overflowY: 'auto',
      }}
      ref={scrollRef}
    >
      {messages.length === 0 && (
        <span style={{ color: '#666' }}>Battle start!</span>
      )}
      {prevMessages.map((msg, i) => (
        <div key={i} style={{ color: '#888', opacity: 0.8 }}>
          {msg}
        </div>
      ))}
      {lastMessage && (
        <div style={{ color: '#e0e0e0' }}>
          {lastMessageFull.slice(0, displayedChars)}
          {isLastComplete && !isAnimating && (
            <span
              style={{
                display: 'inline-block',
                marginLeft: 4,
                animation: 'blink 1s step-end infinite',
                color: '#3498db',
              }}
            >
              ▼
            </span>
          )}
        </div>
      )}
      <style>{`
        @keyframes blink {
          0%, 100% { opacity: 1; }
          50% { opacity: 0; }
        }
      `}</style>
    </div>
  )
}
