# Save System

## Overview

The game saves to `localStorage` as a JSON blob. The Rust engine handles serialization/deserialization of the full `GameState`. Players get 3 save slots.

## Save Data Structure

```typescript
interface SaveFile {
  version: number;        // Schema version for migration
  slot: number;           // 1, 2, or 3
  timestamp: string;      // ISO 8601
  checksum: string;       // Simple integrity check
  data: string;           // JSON-serialized GameState from Rust
  preview: {              // Quick-access info for save slot UI
    player_name: string;
    play_time_ms: number;
    stamps_earned: number;
    party_lead_species: number;
    party_lead_level: number;
    location_name: string;
    sneakerdex_count: number;
  };
}
```

## localStorage Keys

```
wizsneakers_save_1  → SaveFile JSON
wizsneakers_save_2  → SaveFile JSON
wizsneakers_save_3  → SaveFile JSON
wizsneakers_settings → { musicVolume, sfxVolume, textSpeed, controls }
```

## Save/Load Flow

### Saving
```
1. Player opens menu → Save
2. Player selects slot (1/2/3)
3. Confirm overwrite if slot occupied
4. JS calls engine.export_save() → JSON string
5. JS wraps in SaveFile with preview data and timestamp
6. JS writes to localStorage
7. Show "Game Saved!" confirmation
```

### Loading
```
1. Title screen → Continue
2. Show 3 save slots with preview data
3. Player selects slot
4. JS reads SaveFile from localStorage
5. Verify checksum
6. JS calls GameEngine.load_save(data) → new engine instance
7. JS loads the appropriate map assets
8. Resume gameplay
```

### Auto-Save
- Auto-save triggers on:
  - Entering a new town/city
  - After defeating a boss
  - After a significant story event
- Auto-save uses a hidden 4th slot (`wizsneakers_autosave`)
- Player can load auto-save from title screen

## Data Size Estimates

| Component | Estimated Size |
|-----------|---------------|
| Player state (position, name, money) | ~200 bytes |
| Party (6 sneakers, full detail) | ~3 KB |
| Sneaker Box (50 sneakers) | ~25 KB |
| Inventory | ~1 KB |
| Event flags (100 flags) | ~2 KB |
| Sneakerdex | ~500 bytes |
| **Total per save** | **~32 KB** |
| **All 4 saves + settings** | **~130 KB** |

localStorage limit is typically 5-10 MB, so we're well within bounds.

## Migration

When the save schema version doesn't match the current game version:

```rust
pub fn migrate_save(json: &str) -> Result<GameState, MigrationError> {
    let raw: serde_json::Value = serde_json::from_str(json)?;
    let version = raw["version"].as_u64().unwrap_or(0);
    
    match version {
        1 => migrate_v1_to_v2(raw),
        2 => serde_json::from_value(raw),  // Current version
        _ => Err(MigrationError::UnsupportedVersion),
    }
}
```

## Integrity Check

Simple checksum to catch corruption (not anti-cheat):

```typescript
function calculateChecksum(data: string): string {
  let hash = 0;
  for (let i = 0; i < data.length; i++) {
    const char = data.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash |= 0;
  }
  return hash.toString(16);
}
```

## Cloud Sync (Post-MVP)

Future: sync saves to a backend so players can continue across devices.
- Use a simple REST API with user auth
- Upload save JSON on save, download on load
- Conflict resolution: show both saves, let player choose
