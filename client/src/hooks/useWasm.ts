import { useEffect, useRef, useState, useCallback } from 'react'

// Import the WASM init function and GameEngine class
import init, { GameEngine } from '../wasm/wizsneakers_engine.js'

export interface MapTransitionInfo {
  map: string
  x: number
  y: number
  type: 'walk' | 'fade' | 'warp'
  direction: string
}

export function useWasm() {
  const engineRef = useRef<GameEngine | null>(null)
  const [ready, setReady] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [mapId, setMapId] = useState<string>('boxfresh_town')
  const [pendingTransition, setPendingTransition] = useState<MapTransitionInfo | null>(null)

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

        // Load starting dialogue
        await loadDialogueForMap('boxfresh_town', engine)

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

  /**
   * Handle a map transition: fetch new map JSON, load it into the engine,
   * and load appropriate dialogue data.
   */
  const handleTransition = useCallback(async (transition: MapTransitionInfo) => {
    const engine = engineRef.current
    if (!engine) return

    try {
      const resp = await fetch(`/maps/${transition.map}.json`)
      if (!resp.ok) throw new Error(`Failed to fetch map: ${transition.map}`)
      const json = await resp.text()
      // load_map_data also applies the pending_transition position stored in the engine
      engine.load_map_data(json)
      await loadDialogueForMap(transition.map, engine)
      setMapId(transition.map)
      setPendingTransition(null)
    } catch (e) {
      console.error('Map transition failed:', e)
      setPendingTransition(null)
    }
  }, [])

  return { engine: engineRef, ready, error, mapId, pendingTransition, setPendingTransition, handleTransition }
}

/** Dialogue files to load per map. Extend as new maps are added. */
async function loadDialogueForMap(mapId: string, engine: GameEngine): Promise<void> {
  const dialogueFiles: string[] = getDialogueFilesForMap(mapId)
  for (const file of dialogueFiles) {
    try {
      const resp = await fetch(file)
      if (!resp.ok) continue
      const json = await resp.text()
      engine.load_dialogue_data(json)
    } catch {
      // Silently skip missing dialogue files
    }
  }
}

function getDialogueFilesForMap(mapId: string): string[] {
  const base: string[] = []

  if (mapId.startsWith('boxfresh')) {
    base.push('/data/dialogue/boxfresh_town.json')
    base.push('/data/dialogue/prof_sole.json')
  }

  if (mapId === 'route_1') {
    base.push('/data/dialogue/route_1_trainers.json')
  }

  if (mapId.startsWith('laceup')) {
    base.push('/data/dialogue/laceup_village.json')
  }

  return base
}
