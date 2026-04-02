# Map & Tile Engine

## Tile System

### Tile Size & Scale
- **Base tile**: 16x16 pixels in the spritesheet
- **Render scale**: 3x (each pixel becomes 3x3 on screen)
- **On-screen tile**: 48x48 pixels
- **Viewport**: 15 tiles wide x 11 tiles tall = 720x528 pixels

### Tile Layers

Each map has 4 layers, each a 2D array of tile IDs:

| Layer | Purpose | Render Order | Example |
|-------|---------|-------------|---------|
| **Ground** | Base terrain | First (bottom) | Grass, dirt, water, floor |
| **Detail** | Objects on ground | Second | Flowers, cracks, puddles |
| **Collision** | Walkability data | Not rendered | 0 = walkable, 1 = blocked |
| **Overlay** | Above-player elements | Last (top) | Tree canopy, roof overhangs, bridges |

### Tile IDs

Tile IDs map to positions in tileset sprite sheets:

```
tile_id → tileset_index = tile_id / tiles_per_tileset
           local_id = tile_id % tiles_per_tileset
           src_x = (local_id % tileset_columns) * TILE_SIZE
           src_y = (local_id / tileset_columns) * TILE_SIZE
```

Special tile IDs:
- `0` = empty/transparent (used in Detail and Overlay layers)
- `1-999` = base tileset
- `1000-1999` = buildings tileset
- `2000-2999` = interior tileset

## Collision System

### Collision Map

The collision layer is a simple 2D grid:

```rust
pub struct CollisionMap {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,  // 0 = walkable, 1 = solid, 2 = ledge, 3 = water, etc.
}

impl CollisionMap {
    pub fn is_walkable(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height { return false; }
        self.data[(y as usize) * (self.width as usize) + (x as usize)] == 0
    }
    
    pub fn tile_type(&self, x: u16, y: u16) -> TileType {
        // Returns the collision type for special handling
    }
}

pub enum TileType {
    Walkable,
    Solid,
    LedgeDown,     // Can walk off going down, not up
    LedgeLeft,
    LedgeRight,
    Water,          // Requires surf (post-MVP)
    TallGrass,      // Triggers encounter checks
    Door,           // Map transition
    Warp,           // Teleport
}
```

### NPC Collision

NPCs are dynamic obstacles. Before each move, check:
1. Is the target tile walkable? (collision map)
2. Is any NPC standing on the target tile? (NPC positions)

```rust
pub fn can_move_to(&self, x: u16, y: u16) -> bool {
    self.collision_map.is_walkable(x, y)
        && !self.npcs.iter().any(|npc| npc.x == x && npc.y == y)
}
```

## Player Movement

### Grid-Based Movement

The player moves tile-by-tile with smooth interpolation:

```rust
pub fn process_movement(&mut self, input: InputAction, dt: f64) {
    if self.player.moving {
        // Continue current movement
        self.player.move_progress += dt * self.move_speed();
        if self.player.move_progress >= 1.0 {
            // Snap to target tile
            self.player.x = self.player.target_x;
            self.player.y = self.player.target_y;
            self.player.moving = false;
            self.player.move_progress = 0.0;
            
            // Check for step events
            self.on_step_complete();
        }
    } else if let Some(direction) = input.direction() {
        // Start new movement
        self.player.facing = direction;
        let (dx, dy) = direction.delta();
        let target_x = (self.player.x as i32 + dx) as u16;
        let target_y = (self.player.y as i32 + dy) as u16;
        
        if self.can_move_to(target_x, target_y) {
            self.player.target_x = target_x;
            self.player.target_y = target_y;
            self.player.moving = true;
            self.player.move_progress = 0.0;
        }
        // Even if blocked, the player turns to face that direction
    }
}

fn move_speed(&self) -> f64 {
    if self.sprinting { 1.0 / SPRINT_FRAMES as f64 }
    else { 1.0 / WALK_FRAMES as f64 }
}
```

### Step Events

When the player completes a step onto a new tile:

```rust
fn on_step_complete(&mut self) {
    let tile = self.collision_map.tile_type(self.player.x, self.player.y);
    
    match tile {
        TileType::TallGrass => {
            if self.rng.chance(15) {  // 15% encounter rate
                self.trigger_wild_encounter();
            }
        }
        TileType::Door => {
            self.trigger_map_transition();
        }
        TileType::Warp => {
            self.trigger_warp();
        }
        _ => {}
    }
    
    // Check for scripted events at this position
    self.check_position_events(self.player.x, self.player.y);
}
```

## Camera System

```typescript
interface Camera {
  x: number;  // World pixel position of top-left corner
  y: number;
  width: number;   // Viewport width in pixels
  height: number;  // Viewport height in pixels
}

function calculateCamera(
  playerX: number,    // Player tile X
  playerY: number,    // Player tile Y
  moveProgress: number,
  facing: Direction,
  mapWidth: number,   // Map width in tiles
  mapHeight: number   // Map height in tiles
): Camera {
  const tilePixels = TILE_SIZE * RENDER_SCALE;
  
  // Player world position (with interpolation)
  let worldX = playerX * tilePixels;
  let worldY = playerY * tilePixels;
  if (moveProgress > 0) {
    const [dx, dy] = directionDelta(facing);
    worldX += dx * tilePixels * moveProgress;
    worldY += dy * tilePixels * moveProgress;
  }
  
  // Center camera on player
  let camX = worldX - CANVAS_WIDTH / 2 + tilePixels / 2;
  let camY = worldY - CANVAS_HEIGHT / 2 + tilePixels / 2;
  
  // Clamp to map bounds (no void at edges)
  const mapPixelW = mapWidth * tilePixels;
  const mapPixelH = mapHeight * tilePixels;
  camX = Math.max(0, Math.min(camX, mapPixelW - CANVAS_WIDTH));
  camY = Math.max(0, Math.min(camY, mapPixelH - CANVAS_HEIGHT));
  
  return { x: camX, y: camY, width: CANVAS_WIDTH, height: CANVAS_HEIGHT };
}
```

### Tile Culling

Only render tiles visible in the viewport:

```typescript
function getVisibleTileRange(camera: Camera) {
  const tilePixels = TILE_SIZE * RENDER_SCALE;
  return {
    startX: Math.floor(camera.x / tilePixels),
    startY: Math.floor(camera.y / tilePixels),
    endX: Math.ceil((camera.x + camera.width) / tilePixels),
    endY: Math.ceil((camera.y + camera.height) / tilePixels),
  };
}
```

## Map Transitions

### Connected Maps

Maps connect at edges. When the player walks off one map's boundary:

```rust
fn trigger_map_transition(&mut self) {
    let current_map = &self.maps[self.current_map_id];
    let direction = self.player.facing;
    
    if let Some(next_map_id) = current_map.connections.get(&direction) {
        // Calculate entry position on new map
        let (new_x, new_y) = match direction {
            Direction::Up => (self.player.x, new_map.height - 1),
            Direction::Down => (self.player.x, 0),
            Direction::Left => (new_map.width - 1, self.player.y),
            Direction::Right => (0, self.player.y),
        };
        
        self.pending_transition = Some(MapTransition {
            target_map: *next_map_id,
            target_x: new_x,
            target_y: new_y,
            transition_type: TransitionType::Walk,
        });
    }
}
```

### Door/Warp Transitions

Doors and warps are defined in the map event data:

```json
{
  "type": "warp",
  "x": 10,
  "y": 5,
  "target_map": 3,
  "target_x": 7,
  "target_y": 12,
  "transition": "fade"
}
```

### Transition Animations

| Type | Duration | Visual |
|------|----------|--------|
| Walk (edge) | 500ms | Scroll to new map |
| Door | 300ms | Fade to black, fade in |
| Warp | 600ms | Swirl effect, fade |
| Cave entrance | 400ms | Fade to black, longer |

## NPC Movement

### Movement Patterns

```rust
pub enum NpcMovement {
    Stationary,                    // Never moves
    RandomWalk { radius: u8 },     // Wanders within radius of spawn
    Patrol { path: Vec<(u16, u16)> },  // Walks a fixed path, loops
    FacePlayer,                    // Always faces player when nearby
    FollowPlayer { distance: u8 }, // Story companion (e.g., rival walks with you)
}
```

### NPC Tick

```rust
fn tick_npcs(&mut self, dt: f64) {
    for npc in &mut self.npcs {
        match npc.movement {
            NpcMovement::RandomWalk { radius } => {
                npc.move_timer -= dt;
                if npc.move_timer <= 0.0 && !npc.moving {
                    // Pick random adjacent walkable tile within radius
                    let dir = self.rng.range(0, 4);
                    // ... attempt move
                    npc.move_timer = self.rng.range(2000, 5000) as f64; // 2-5 seconds
                }
            }
            // ... other patterns
        }
        
        // Interpolate movement (same as player)
        if npc.moving {
            npc.move_progress += dt * NPC_MOVE_SPEED;
            if npc.move_progress >= 1.0 {
                npc.x = npc.target_x;
                npc.y = npc.target_y;
                npc.moving = false;
            }
        }
    }
}
```

## Encounter Zones

### Tall Grass Encounters

```rust
fn check_wild_encounter(&mut self) -> Option<BattleState> {
    let encounter_table = &self.current_map().wild_encounters;
    if encounter_table.is_empty() { return None; }
    
    // 15% chance per step in tall grass
    if !self.rng.chance(15) { return None; }
    
    // Weighted random selection from encounter table
    let total_weight: u32 = encounter_table.iter().map(|e| e.weight).sum();
    let roll = self.rng.range(0, total_weight);
    
    let mut accumulated = 0;
    for entry in encounter_table {
        accumulated += entry.weight;
        if roll < accumulated {
            let level = self.rng.range(entry.level_min, entry.level_max + 1) as u8;
            let sneaker = SneakerInstance::generate_wild(entry.species_id, level, &mut self.rng);
            return Some(BattleState::new_wild(sneaker));
        }
    }
    None
}
```

### Trainer Line of Sight

Trainers challenge the player when they walk into their line of sight:

```rust
fn check_trainer_triggers(&self) -> Option<u16> {
    for trainer in &self.current_map().trainers {
        if trainer.defeated { continue; }
        
        // Check if player is in trainer's line of sight
        let sight_range = trainer.sight_range; // Usually 3-5 tiles
        let (dx, dy) = trainer.facing.delta();
        
        for i in 1..=sight_range {
            let check_x = (trainer.x as i32 + dx * i as i32) as u16;
            let check_y = (trainer.y as i32 + dy * i as i32) as u16;
            
            // Check for walls blocking line of sight
            if !self.collision_map.is_walkable(check_x, check_y) { break; }
            
            if check_x == self.player.x && check_y == self.player.y {
                return Some(trainer.id);
            }
        }
    }
    None
}
```
