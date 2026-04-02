import { useEffect, useRef, useState, useCallback } from 'react'

export interface DialoguePage {
  speaker: string | null
  text: string
  choices?: Array<{
    text: string
    next_dialogue: string | null
    set_flag: string | null
    action: string | null
  }> | null
}

type TextSpeed = 'slow' | 'medium' | 'fast' | 'instant'

const TEXT_SPEED_MS: Record<TextSpeed, number> = {
  slow: 40,
  medium: 20,
  fast: 10,
  instant: 0,
}

interface DialogueBoxProps {
  page: DialoguePage
  onAdvance: () => void
  onChoice: (index: number) => void
  textSpeed?: TextSpeed
}

export function DialogueBox({ page, onAdvance, onChoice, textSpeed = 'medium' }: DialogueBoxProps) {
  const [displayedText, setDisplayedText] = useState('')
  const [isTyping, setIsTyping] = useState(true)
  const [selectedChoice, setSelectedChoice] = useState(0)
  const [blinkOn, setBlinkOn] = useState(true)
  const typingRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const charIndexRef = useRef(0)
  const fullTextRef = useRef(page.text)

  // Reset when page changes
  useEffect(() => {
    fullTextRef.current = page.text
    charIndexRef.current = 0
    setDisplayedText('')
    setIsTyping(true)
    setSelectedChoice(0)

    if (typingRef.current) {
      clearTimeout(typingRef.current)
    }

    const speedMs = TEXT_SPEED_MS[textSpeed]

    if (speedMs === 0) {
      // Instant mode
      setDisplayedText(page.text)
      setIsTyping(false)
      return
    }

    function typeNext() {
      const idx = charIndexRef.current
      const full = fullTextRef.current
      if (idx < full.length) {
        charIndexRef.current = idx + 1
        setDisplayedText(full.slice(0, idx + 1))
        typingRef.current = setTimeout(typeNext, speedMs)
      } else {
        setIsTyping(false)
      }
    }

    typingRef.current = setTimeout(typeNext, speedMs)

    return () => {
      if (typingRef.current) clearTimeout(typingRef.current)
    }
  }, [page.text, page.speaker, textSpeed])

  // Blinking indicator
  useEffect(() => {
    if (isTyping) return
    const interval = setInterval(() => {
      setBlinkOn(b => !b)
    }, 500)
    return () => clearInterval(interval)
  }, [isTyping])

  const handleAction = useCallback(() => {
    if (isTyping) {
      // Complete text immediately
      if (typingRef.current) clearTimeout(typingRef.current)
      setDisplayedText(fullTextRef.current)
      setIsTyping(false)
      return
    }

    const hasChoices = page.choices && page.choices.length > 0
    if (hasChoices) {
      onChoice(selectedChoice)
    } else {
      onAdvance()
    }
  }, [isTyping, page.choices, selectedChoice, onAdvance, onChoice])

  // Keyboard handler
  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      const hasChoices = page.choices && page.choices.length > 0

      if (e.code === 'KeyZ' || e.code === 'Enter' || e.code === 'Space') {
        e.preventDefault()
        handleAction()
        return
      }

      if (!isTyping && hasChoices) {
        const choiceCount = page.choices!.length
        if (e.code === 'ArrowUp' || e.code === 'KeyW') {
          e.preventDefault()
          setSelectedChoice(i => (i - 1 + choiceCount) % choiceCount)
        } else if (e.code === 'ArrowDown' || e.code === 'KeyS') {
          e.preventDefault()
          setSelectedChoice(i => (i + 1) % choiceCount)
        }
      }
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [handleAction, isTyping, page.choices])

  const hasChoices = !isTyping && page.choices && page.choices.length > 0

  return (
    <div
      style={{
        position: 'fixed',
        bottom: 0,
        left: '50%',
        transform: 'translateX(-50%)',
        width: 480,
        zIndex: 100,
        fontFamily: '"Courier New", monospace',
        imageRendering: 'pixelated',
      }}
    >
      {/* Main dialogue panel */}
      <div
        style={{
          background: 'rgba(10, 10, 20, 0.92)',
          border: '2px solid #e0c060',
          padding: '12px 16px',
          marginBottom: hasChoices ? 0 : 4,
          minHeight: 80,
          position: 'relative',
        }}
      >
        {/* Speaker name */}
        {page.speaker && (
          <div
            style={{
              position: 'absolute',
              top: -14,
              left: 12,
              background: 'rgba(10, 10, 20, 0.95)',
              border: '2px solid #e0c060',
              padding: '1px 8px',
              fontSize: 11,
              color: '#e0c060',
              fontWeight: 'bold',
              letterSpacing: 1,
              textTransform: 'uppercase',
            }}
          >
            {page.speaker}
          </div>
        )}

        {/* Dialogue text */}
        <div
          style={{
            fontSize: 14,
            color: '#f0f0e8',
            lineHeight: 1.6,
            minHeight: 44,
            whiteSpace: 'pre-wrap',
          }}
        >
          {displayedText}
          {/* Blinking cursor while typing */}
          {isTyping && <span style={{ color: '#e0c060' }}>▌</span>}
        </div>

        {/* Advance indicator — shown when not typing and no choices */}
        {!isTyping && !hasChoices && (
          <div
            style={{
              position: 'absolute',
              bottom: 8,
              right: 12,
              fontSize: 12,
              color: '#e0c060',
              opacity: blinkOn ? 1 : 0,
              transition: 'opacity 0.1s',
            }}
          >
            ▼
          </div>
        )}
      </div>

      {/* Choice panel */}
      {hasChoices && (
        <div
          style={{
            background: 'rgba(10, 10, 20, 0.95)',
            border: '2px solid #e0c060',
            borderTop: 'none',
            padding: '4px 0',
          }}
        >
          {page.choices!.map((choice, i) => (
            <div
              key={i}
              onClick={() => onChoice(i)}
              style={{
                padding: '6px 16px',
                fontSize: 13,
                color: i === selectedChoice ? '#e0c060' : '#c0c0b0',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                background: i === selectedChoice ? 'rgba(224, 192, 96, 0.1)' : 'transparent',
              }}
            >
              <span style={{ width: 12, fontSize: 10 }}>{i === selectedChoice ? '▶' : ''}</span>
              {choice.text}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
