import { useEffect, useRef, useState } from 'react'

// Import the WASM init function and GameEngine class
import init, { GameEngine } from '../wasm/wizsneakers_engine.js'

export function useWasm() {
  const engineRef = useRef<GameEngine | null>(null)
  const [ready, setReady] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [mapId, setMapId] = useState<string>('boxfresh_town')

  useEffect(() => {
    let cancelled = false

    async function load() {
      try {
        await init()
        if (cancelled) return

        const engine = new GameEngine(BigInt(Date.now()))

        // Load the starting map
        const resp = await fetch('/maps/boxfresh_town.json')
        if (!resp.ok) throw new Error(`Failed to fetch map: ${resp.status}`)
        const json = await resp.text()
        engine.load_map_data(json)

        if (cancelled) return
        engineRef.current = engine
        setMapId('boxfresh_town')
        setReady(true)
      } catch (e) {
        if (!cancelled) setError(String(e))
      }
    }

    load()
    return () => { cancelled = true }
  }, [])

  return { engine: engineRef, ready, error, mapId }
}
