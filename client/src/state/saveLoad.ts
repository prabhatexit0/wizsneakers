import type { GameEngine } from '../wasm/wizsneakers_engine.js'

export const SAVE_VERSION = 1

export interface SavePreview {
  player_name: string
  play_time_ms: number
  stamps_earned: number
  party_lead_species: number
  party_lead_level: number
  location_name: string
  sneakerdex_caught: number
}

export interface SaveFile {
  version: number
  slot: number
  timestamp: string
  checksum: string
  data: string
  preview: SavePreview
}

const SLOT_KEYS = ['wizsneakers_save_1', 'wizsneakers_save_2', 'wizsneakers_save_3']
const AUTOSAVE_KEY = 'wizsneakers_autosave'

export function calculateChecksum(data: string): string {
  let hash = 0
  for (let i = 0; i < data.length; i++) {
    const ch = data.charCodeAt(i)
    hash = ((hash << 5) - hash + ch) | 0
  }
  return (hash >>> 0).toString(16)
}

function buildPreview(eng: GameEngine): SavePreview {
  const anyEng = eng as unknown as Record<string, (...args: unknown[]) => string>
  const infoJson = anyEng['get_player_info']()
  const info = JSON.parse(infoJson) as {
    name: string
    play_time_ms: number
    stamps_earned: number
    sneakerdex_caught: number
  }
  const partyJson = anyEng['get_party']()
  const party = JSON.parse(partyJson) as Array<{
    species_id: number
    level: number
  }>
  return {
    player_name: info.name,
    play_time_ms: info.play_time_ms,
    stamps_earned: info.stamps_earned,
    party_lead_species: party[0]?.species_id ?? 0,
    party_lead_level: party[0]?.level ?? 0,
    location_name: 'Box Fresh Town',
    sneakerdex_caught: info.sneakerdex_caught,
  }
}

export function saveToSlot(slot: number, eng: GameEngine): void {
  const anyEng = eng as unknown as Record<string, (...args: unknown[]) => string>
  const data = anyEng['export_save']()
  const preview = buildPreview(eng)
  const save: SaveFile = {
    version: SAVE_VERSION,
    slot,
    timestamp: new Date().toISOString(),
    checksum: calculateChecksum(data),
    data,
    preview,
  }
  localStorage.setItem(SLOT_KEYS[slot - 1], JSON.stringify(save))
}

export function loadFromSlot(slot: number): SaveFile | null {
  const raw = localStorage.getItem(SLOT_KEYS[slot - 1])
  if (!raw) return null
  try {
    return JSON.parse(raw) as SaveFile
  } catch {
    return null
  }
}

export function getSlotPreviews(): (SavePreview | null)[] {
  return SLOT_KEYS.map((key) => {
    const raw = localStorage.getItem(key)
    if (!raw) return null
    try {
      const save = JSON.parse(raw) as SaveFile
      return save.preview
    } catch {
      return null
    }
  })
}

export function autoSave(eng: GameEngine): void {
  const anyEng = eng as unknown as Record<string, (...args: unknown[]) => string>
  const data = anyEng['export_save']()
  const preview = buildPreview(eng)
  const save: SaveFile = {
    version: SAVE_VERSION,
    slot: 0,
    timestamp: new Date().toISOString(),
    checksum: calculateChecksum(data),
    data,
    preview,
  }
  localStorage.setItem(AUTOSAVE_KEY, JSON.stringify(save))
}

export function getAutoSavePreview(): SavePreview | null {
  const raw = localStorage.getItem(AUTOSAVE_KEY)
  if (!raw) return null
  try {
    const save = JSON.parse(raw) as SaveFile
    return save.preview
  } catch {
    return null
  }
}

export function loadAutoSave(): SaveFile | null {
  const raw = localStorage.getItem(AUTOSAVE_KEY)
  if (!raw) return null
  try {
    return JSON.parse(raw) as SaveFile
  } catch {
    return null
  }
}

export function formatPlayTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000)
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}`
}
