# Art Style Guide

## Visual Identity

Wizsneakers uses a **GBA-era pixel art style** with a modern color palette. Think Pokemon FireRed/LeafGreen fidelity but with a streetwear-inspired color scheme — bolder, more saturated, with neon accents.

## Technical Specifications

### Tile & Sprite Sizes

| Element | Size (pixels) | Notes |
|---------|--------------|-------|
| Base tile | 16x16 | Standard ground/wall tiles |
| Player sprite | 16x24 | 16 wide, 24 tall (head extends above tile) |
| NPC sprite | 16x24 | Same as player |
| Sneaker battle sprite (front) | 64x64 | Shown on opponent side |
| Sneaker battle sprite (back) | 64x64 | Shown on player side |
| Sneaker icon (menu/party) | 32x32 | Used in UI |
| Sneaker mini icon | 16x16 | Used in party bar, Sneakerdex |
| Item icon | 16x16 | Bag screen |
| Battle background | 240x160 | Full battle scene backdrop |

### Render Scale
- All sprites are rendered at **3x scale** (nearest-neighbor, no anti-aliasing)
- `image-rendering: pixelated` on canvas CSS
- Final canvas: 720x528 pixels (15 tiles x 11 tiles at 48px each)

### Animation Frames

| Animation | Frames | Speed |
|-----------|--------|-------|
| Player walk (per direction) | 4 frames | 8 frames/cycle at 60fps |
| Player idle | 2 frames | 30 frames/cycle (slow breathe) |
| NPC walk | 4 frames | 8 frames/cycle |
| Sneaker battle idle | 2 frames | 20 frames/cycle |
| Sneaker attack | 3-4 frames | 4 frames/anim |
| Sneaker faint | 3 frames | 6 frames/anim |
| Tall grass sway | 3 frames | 15 frames/cycle |
| Water shimmer | 4 frames | 20 frames/cycle |

## Color Palette

### Primary Palette (UI & World)

```
Background Dark:    #1a1a2e
Background Medium:  #16213e
Background Light:   #0f3460
Accent Primary:     #e94560  (Red — hot drops, danger)
Accent Secondary:   #533483  (Purple — rare, special)
Text Light:         #f5f5f5
Text Dark:          #2d2d2d
Text Gold:          #ffd700
```

### Faction Colors

```
Retro:         Primary #c0392b  Secondary #e74c3c  Accent #f5f5dc
Techwear:      Primary #2980b9  Secondary #3498db  Accent #00d4ff
Skate:         Primary #27ae60  Secondary #2ecc71  Accent #f39c12
High-Fashion:  Primary #8e44ad  Secondary #9b59b6  Accent #f1c40f
```

### Rarity Colors

```
Common:     #95a5a6  (Gray)
Uncommon:   #2ecc71  (Green)
Rare:       #3498db  (Blue)
Epic:       #9b59b6  (Purple)
Legendary:  #f39c12  (Gold)
```

### HP Bar Colors

```
High (>50%):   #2ecc71 (Green)
Medium (25-50%): #f39c12 (Yellow/Orange)
Low (<25%):    #e74c3c (Red)
```

## Sprite Sheet Layout

### Player Sprite Sheet (96x96 pixels)

```
Row 0: Walk Down  — 4 frames × 16x24
Row 1: Walk Up    — 4 frames × 16x24
Row 2: Walk Left  — 4 frames × 16x24
Row 3: Walk Right — 4 frames × 16x24
```

### NPC Sprite Sheets
Same layout as player. Each NPC type gets their own sheet.

### Sneaker Battle Sprites

Each sneaker gets a 128x64 sheet:
```
[Front 64x64] [Back 64x64]
```

Sneakers should be drawn as stylized, slightly anthropomorphic shoes — they have personality. Think Nike Dunks with attitude, not just a shoe sitting there.

## World Tileset Guidelines

### Outdoor Tiles
- Grass: Soft green with subtle texture variation (2-3 grass tile variants to avoid repetition)
- Paths: Lighter stone/concrete, clear contrast with grass
- Tall grass: Taller, more vibrant green with small movement sprites
- Trees: 2 tiles tall (trunk on ground layer, canopy on overlay)
- Fences, benches, trash cans, street lights — urban props

### Indoor Tiles
- Clean floors (different per building type)
- Shelves, counters, machines for shops
- Sneaker displays (like Pokeball shelves but sneaker boxes)
- Healing machine at Sneaker Clinics (futuristic sneaker cleaning booth)

### Building Exteriors
- Sneaker Clinic: Clean white, neon cross replaced with a sneaker silhouette
- Shops: Colorful storefronts with faction-themed awnings
- Player's House: Modest suburban home
- Gyms: Larger buildings themed to their faction

## UI Design Tokens

### Fonts
- **Dialogue/Menus**: Use a clean pixel font (8x8 base, like "Press Start 2P" or custom)
- **Numbers/Stats**: Monospace pixel font for alignment
- **Title/Logo**: Custom stylized logo (hand-drawn pixel art)

### UI Panels
- Rounded-rectangle borders (2px pixel border)
- Semi-transparent dark backgrounds (#1a1a2e at 85% opacity)
- Consistent 4px padding inside panels
- Selection cursor: animated arrow or highlight

### Battle UI
- Player sneaker HP bar: bottom-right
- Opponent sneaker HP bar: top-left
- Move selection: 2x2 grid at bottom
- Battle log: text box at bottom, scrolls messages

## Prototype Visuals (Pre-Art)

For initial development before pixel art is ready:

```
Player:     Bright blue 16x24 rectangle
NPC:        Orange 16x24 rectangle  
Grass:      #4a7c4f solid tile
Tall Grass: #2d5a2d solid tile with "T" marker
Path:       #c4a882 solid tile
Wall:       #555555 solid tile
Water:      #4488cc solid tile
Door:       #8B4513 solid tile
```

Sneakers in battle: Colored circles with faction initial (R/T/S/H), sized 64x64.
