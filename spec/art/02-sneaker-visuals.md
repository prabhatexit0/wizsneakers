# Sneaker Visual Design

## Design Philosophy

Each sneaker should feel like a character, not just a piece of footwear. They should have:
- **Silhouette identity**: Recognizable even as a tiny 16x16 icon
- **Faction clarity**: Color scheme and shape language instantly signal faction
- **Rarity progression**: Higher rarity = more visual detail, glow effects, unique features

## Faction Visual Language

### Retro
- **Shape**: Classic silhouettes — chunky soles, high-tops, clean lines
- **Colors**: Red/white/black, vintage cream accents
- **Materials**: Leather, canvas, rubber
- **Details**: Visible stitching, classic swoosh/stripe equivalents, clean laces
- **Aura (battle)**: Warm sepia glow

### Techwear
- **Shape**: Sleek, aerodynamic, angular
- **Colors**: Black/blue/neon cyan accents
- **Materials**: Mesh, foam, reactive soles
- **Details**: LED strips, digital displays, no visible laces (auto-lace), vents
- **Aura (battle)**: Electric blue pulse

### Skate
- **Shape**: Low-profile, flat sole, sturdy
- **Colors**: Green/orange/earth tones, graffiti accents
- **Materials**: Suede, vulcanized rubber, reinforced toe
- **Details**: Scuff marks (intentional style), sticker patches, grip tape texture on sole
- **Aura (battle)**: Green flame / urban grit particles

### High-Fashion
- **Shape**: Avant-garde, exaggerated proportions (oversized soles, unusual angles)
- **Colors**: Gold/purple/black, monochrome with metallic accents
- **Materials**: Exotic (crystal, chrome, holographic)
- **Details**: Designer logos, gems, unusual closures (zippers, straps)
- **Aura (battle)**: Purple/gold sparkle

## Per-Sneaker Visual Notes

### Starters

| Sneaker | Visual Description |
|---------|-------------------|
| Retro Runner | Classic running shoe. White base, red swoosh, gum sole. Clean and simple. |
| Retro Runner II | Sleeker profile, leather quality upgrade, subtle gold accents on swoosh. |
| Retro Runner Max | Full leather, visible air unit in sole, chrome accents, slight aura glow. |
| Tech Trainer | Black mesh upper, blue foam sole, small LED on heel tab. |
| Tech Trainer Pro | Reactive knit upper, holographic heel counter, pulsing blue sole. |
| Tech Trainer Ultra | Full black with neon circuit patterns, floating sole effect, HUD visor on tongue. |
| Skate Blazer | Green suede, white sole, orange laces. Intentionally simple. |
| Skate Blazer Pro | Reinforced toe cap, wider sole, graffiti-style pattern on quarter panel. |
| Skate Blazer Elite | Metal-reinforced, glowing grip tape sole, chain laces, graffiti tags animate. |

### Legendaries (Genesis Grails)

| Sneaker | Visual Description |
|---------|-------------------|
| Genesis Jordan | Glowing red/black, wings logo pulses with light, sole has visible energy. Ancient artifact energy. |
| Genesis React | Holographic mesh that shifts colors, floating sole particles, eye-shaped heel counter. |
| Genesis Kickflip | Board-shaped sole, eternal flame on heel, wheels embedded in sole that spin during idle. |
| Genesis Couture | Crystal-encrusted, golden glow, changes colorway every few idle frames, crown emblem. |

## Battle Sprite Specifications

### Front Sprite (Opponent View) — 64x64
- Sneaker faces slightly left, angled 3/4 view
- Shows the "personality" side — tongue, laces, logo visible
- Slight upward tilt to look imposing
- Rarity glow: Legendaries have a 2-frame animated glow aura

### Back Sprite (Player View) — 64x64
- Sneaker faces away, slight right angle
- Shows heel counter, sole, back detail
- Lower in frame (player perspective looking "up" at battle)

### Attack Animations
- **Physical move**: Sneaker lunges forward 8px, quick return (3 frames)
- **Special move**: Sneaker glows with faction color, projectile fires (4 frames)
- **Status move**: Sneaker pulses/spins in place (3 frames)
- **Hit reaction**: Target sneaker flashes white 3 times, shakes (4 frames)
- **Faint**: Sneaker tilts, falls off screen downward (3 frames)

### Battle Backgrounds

| Location Type | Description |
|---------------|-------------|
| Grass route | Green field, sky backdrop, trees in distance |
| Urban route | Concrete ground, city skyline |
| Gym (Retro) | Vinyl record floor, vintage posters on walls |
| Gym (Skate) | Halfpipe, graffiti walls |
| Gym (Tech) | Lab floor, holographic grid walls |
| Gym (Fashion) | Runway floor, spotlights |
| Cave/Underground | Dark stone, dim lighting |
| Pinnacle Tower | Glass floor, clouds visible below |
| Boss (generic) | Elevated platform, crowd cheering (pixel silhouettes) |

## Overworld Sneaker Appearances

When a wild encounter starts, or when viewing sneakers in the overworld (e.g., in shops):
- Use the 32x32 icon version
- Displayed on a pedestal/display stand in shops
- Shown emerging from tall grass in encounter transitions

## Evolution Visual

When a sneaker evolves:
1. Screen flashes white (500ms)
2. Current sprite dissolves into particles (1s)
3. Particles swirl and reform into new sprite (1.5s)
4. New sprite appears with a burst of faction-colored light (500ms)
5. New name displays
