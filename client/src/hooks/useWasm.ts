import { useEffect, useRef, useState } from 'react'

// Import the WASM init function and GameEngine class
import init, { GameEngine } from '../wasm/wizsneakers_engine.js'

export function useWasm() {
  const engineRef = useRef<GameEngine | null>(null)
  const [ready, setReady] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false

    async function load() {
      try {
        await init()
        if (cancelled) return
        engineRef.current = new GameEngine()
        setReady(true)
      } catch (e) {
        if (!cancelled) setError(String(e))
      }
    }

    load()
    return () => { cancelled = true }
  }, [])

  return { engine: engineRef, ready, error }
}
