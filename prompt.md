Act as an Expert Game Developer and Systems Architect specializing in WebAssembly. I am building a 2D top-down pixel art RPG called "Wizsneakers" (heavily inspired by classic Pokémon games, but centered around sneaker culture). 

I need you to write the foundational code and architecture for this game.

### Tech Stack
* **Frontend:** React, TypeScript, Vite.
* **Backend/Core Logic:** Rust compiled to WebAssembly (using `wasm-pack` and `wasm-bindgen`).
* **Rendering:** HTML5 Canvas API (managed by React) for the overworld, and standard React components for the UI/Battle menus.

### Game Concept & Mechanics
* **The World:** A 2D top-down tile-based grid map. The player moves a character around to explore and encounter rival "Hypebeasts".
* **The Goal:** Defeat rivals, collect rare sneakers, and find the ultimate "Genesis Grails."
* **Battle System:** Turn-based combat where equipped sneakers do the fighting. 
* **Sneaker Types (Factions):** Retro, Techwear, Skate, High-Fashion. (Include type advantages: e.g., Techwear beats Skate, Skate beats High-Fashion).
* **Sneaker Stats:** Hype (Attack), Comfort (Defense), Durability (HP), Drip (Special Attack), Rarity (Speed/Turn order).
* **Sample Moves:** Crease (Lowers Durability), Camp Out (Heals HP), Ankle Breaker (Critical hit).

### Architecture Requirements
1.  **Rust (WASM) Responsibilities:** Must act as the single source of truth. It should manage the `GameState` (player X/Y coordinates, current map grid, collision detection, inventory) and the `BattleEngine` (turn logic, damage math, stat tracking). 
2.  **React (TypeScript) Responsibilities:** Must handle the game loop (`requestAnimationFrame`), listen to keyboard inputs (WASD/Arrows), send those inputs to the Rust WASM module, and render the updated game state to the Canvas/DOM.
3.  **Interop:** Keep the boundary clean. Expose a central Rust struct (e.g., `GameEngine`) to JS. Use `serde` to pass state updates as JSON to React for easy UI rendering, or expose specific getter functions.

### What I need from you in your first response:
1.  **Setup Instructions:** The exact terminal commands to initialize this specific Vite + React-TS + Rust-WASM monorepo structure.
2.  **Rust Core (Models):** Write the initial Rust structs/enums for `Sneaker`, `Faction`, `Stats`, and `GameState`. 
3.  **Rust Logic (Movement):** Implement a simple Rust function to move the player on a grid and check for collisions/encounters.
4.  **React Integration (The Game Loop):** Provide the main `App.tsx` and a custom hook (e.g., `useGameLoop`) that instantiates the WASM module, captures keyboard events, ticks the Rust game engine, and renders a simple colored square representing the player on an HTML Canvas.

Please write clean, well-commented, production-ready code. Do not worry about actual pixel art assets right now; use colored blocks for rendering so I can get a moving prototype working immediately.