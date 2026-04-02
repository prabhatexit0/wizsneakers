# PRD 15 — Placeholder Sprites & Tileset Pipeline (Phase 9B)

## Goal
Replace colored blocks with structured placeholder sprites. Each sneaker, NPC type, and tile type gets a distinct, recognizable colored sprite so the game is visually parseable. Set up the spritesheet pipeline so future real art can be dropped in without code changes.

## Dependencies
- PRD 14 (UI component library), PRD 13 (maps and NPCs exist)

## Deliverables

### Spritesheet System

**Create `client/src/rendering/spritesheetGenerator.ts`**
- Programmatic generation of placeholder sprites using Canvas 2D
- Each sprite is 16x16 pixels (matches tile size)
- Sprites are drawn onto a spritesheet canvas, then exported as ImageBitmap
- Registry maps sprite IDs to spritesheet coordinates

**Create `client/src/rendering/sneakerSprites.ts`**
- Generate a 16x16 placeholder sprite per sneaker species (30 total)
- Each sprite: faction-colored body (main fill), darker outline, 2-letter abbreviation in center
  - Retro = red tones, Techwear = blue tones, Skate = green tones, HighFashion = purple tones
- Rarity indicated by border style: Common=solid, Uncommon=dashed, Rare=double, Epic=gold, Legendary=rainbow shimmer
- Battle sprites are 48x48 (3x upscale) for the battle screen

**Create `client/src/rendering/npcSprites.ts`**
- NPC sprite generator: colored humanoid silhouettes (16x16)
- Types with distinct colors:
  - Player: bright blue
  - Rival Flip: orange
  - Trainers: red
  - Shop NPCs: green
  - Clinic NPCs: pink/white
  - Syndicate Grunts: black/purple
  - Prof Sole: white (lab coat)
  - Mom: yellow
  - Generic townspeople: gray with slight variation
- 4-frame directional sprites (down, up, left, right) — simple palette shifts per direction

**Create `client/src/rendering/tileSprites.ts`**
- Tile type sprites (16x16):
  - Grass: green fill with small darker green dots
  - Tall Grass: darker green with wavy lines (indicates encounters)
  - Path/Road: light gray with subtle grid
  - Wall/Building: dark gray brick pattern
  - Water: blue with wave pattern
  - Door: brown rectangle with handle dot
  - Ledge: brown with downward arrow
  - Sign: brown post with white rectangle
  - Item Ball: yellow circle with white highlight
  - Interior floor: light wood brown
  - Interior wall: cream/off-white
- Each tile type maps to its tile code from the map JSON

### Rendering Integration

**Modify `client/src/rendering/tileRenderer.ts`**
- Replace colored rectangles with tile sprites from tileSprites.ts
- Look up tile code → sprite from registry → drawImage from spritesheet
- Fallback to colored rectangle if sprite not found (backwards compat)

**Modify `client/src/rendering/sprites.ts`**
- Integrate sneaker and NPC sprite systems
- `getSneakerSprite(speciesId, size: 'tile' | 'battle')` → returns canvas region
- `getNpcSprite(npcType, direction)` → returns canvas region
- Player character uses npcSprites with type='player'

**Modify `client/src/App.tsx` (or relevant rendering hook)**
- Initialize spritesheet on WASM load (generate all sprites once)
- Pass sprite registry to rendering functions
- NPC rendering: draw NPC sprites at their map positions facing correct direction

### Battle Screen Sprites

**Modify `client/src/components/battle/BattleScreen.tsx`** (if exists)
- Replace colored rectangle placeholders with 48x48 sneaker sprites
- Player's sneaker on left (facing right), opponent on right (facing left)
- Simple idle animation: 2-frame bob (translate Y ±2px, 500ms interval)

### Asset Loading Architecture

**Create `client/src/rendering/assetLoader.ts`**
- Central asset registry that future PRDs can extend
- `registerSpritesheet(name, image)` — add a real spritesheet when available
- `getSprite(name, frame?)` → ImageBitmap region
- Priority: real asset > generated placeholder
- This allows dropping in real pixel art later with zero code changes — just register the spritesheet

## Tests Required

No Rust tests (client-only). Verification through:
1. `tsc --noEmit` passes
2. `vite build` succeeds
3. Visual: overworld shows distinct tile types and colored NPC sprites instead of plain blocks

## Verification
```bash
./verify.sh
```

## Acceptance Criteria
- [ ] Overworld renders with distinct tile sprites (grass, path, wall, water, tall grass, doors)
- [ ] Player character is a blue humanoid sprite, not a colored block
- [ ] NPCs render with type-appropriate colored sprites
- [ ] NPC sprites face the correct direction
- [ ] Battle screen shows 48x48 sneaker sprites with faction colors
- [ ] Each of 30 sneaker species has a visually distinct placeholder
- [ ] Asset loader supports hot-swapping with real sprites later
- [ ] No rendering regressions — everything that worked before still works
- [ ] `./verify.sh` exits 0
