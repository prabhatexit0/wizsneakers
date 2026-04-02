# Performance Budget

## Targets

| Metric | Target | Hard Limit |
|--------|--------|-----------|
| Frame rate | 60fps | Never below 30fps |
| Frame budget | 16.67ms | 33ms max |
| WASM tick (overworld) | <0.5ms | <1ms |
| WASM battle_action | <2ms | <5ms |
| Canvas render (overworld) | <8ms | <12ms |
| React UI re-render | <4ms | <8ms |
| Initial load (WASM + assets) | <3s | <5s |
| Map transition | <200ms | <500ms |
| WASM module size (gzipped) | <200KB | <500KB |
| Total asset bundle | <5MB | <10MB |
| Memory usage | <100MB | <200MB |
| localStorage usage | <200KB | <500KB |

## Frame Budget Breakdown (16.67ms)

```
┌────────────────────────────────────────┐
│ Input polling          │  0.1ms        │
│ WASM tick()            │  0.3ms        │
│ JSON parse             │  0.1ms        │
│ Canvas clear           │  0.1ms        │
│ Tile rendering         │  4.0ms        │
│ Sprite rendering       │  2.0ms        │
│ Overlay rendering      │  1.0ms        │
│ React UI update        │  2.0ms        │
│ Browser compositing    │  2.0ms        │
│ ─────────────────────  │ ──────        │
│ TOTAL                  │ ~11.6ms       │
│ Headroom               │ ~5.0ms        │
└────────────────────────────────────────┘
```

## Optimization Strategies

### Rendering
- Only redraw tiles that are in the viewport (tile culling)
- Cache tileset images as pre-loaded `Image` objects
- Use `OffscreenCanvas` for background layer (render once per map, scroll)
- Batch sprite draws to minimize Canvas state changes
- Use `requestAnimationFrame` — never `setInterval`
- Skip rendering if state didn't change (but keep ticking)

### WASM
- Use `opt-level = "s"` for small binary size
- Enable LTO and single codegen unit
- Minimize allocations in tick() — reuse buffers
- Consider direct getters instead of JSON for hot-path data
- Use `wee_alloc` for smaller allocator if needed

### Assets
- Sprite sheets (atlas) instead of individual images
- Compress PNGs with `optipng` / `pngquant`
- Lazy-load map data (only load current + adjacent maps)
- Audio: OGG Vorbis for music (~128kbps), short WAV for SFX
- Use Vite's asset hashing for cache busting

### React
- Memoize battle UI components (HP bars, move lists)
- Don't re-render GameCanvas component — only the canvas context changes
- Use `React.memo` on menu components
- Avoid prop drilling — use context for engine reference
- Battle animations: CSS transitions where possible, Canvas for complex effects

## Profiling Plan

### Dev Tools
- Chrome DevTools Performance tab for frame analysis
- `performance.mark()` / `performance.measure()` around key operations
- Custom FPS counter overlay (dev mode only)

### Key Measurements
```typescript
// Wrap engine calls with timing
const t0 = performance.now();
const result = engine.tick(dt, input);
const tickTime = performance.now() - t0;

if (tickTime > 1.0) {
  console.warn(`Slow tick: ${tickTime.toFixed(2)}ms`);
}
```

### Load Testing
- Walk 10,000 steps in automated test — verify no memory leak
- Run 100 consecutive battles — verify no FPS degradation
- Open/close menus 1,000 times — verify no React memory leak

## Browser Compatibility

| Browser | Minimum Version | Notes |
|---------|----------------|-------|
| Chrome | 89+ | Full WASM support |
| Firefox | 89+ | Full WASM support |
| Safari | 15+ | WASM + BigInt support |
| Edge | 89+ | Chromium-based, same as Chrome |
| Mobile Chrome | 89+ | Touch controls needed |
| Mobile Safari | 15+ | May need performance tuning |

### Required Web APIs
- WebAssembly (core requirement)
- Canvas 2D
- Web Audio API
- localStorage
- requestAnimationFrame
- KeyboardEvent
- Touch Events (mobile)
- Pointer Events (mobile, fallback)
