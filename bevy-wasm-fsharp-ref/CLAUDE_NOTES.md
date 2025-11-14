# bevy-wasm-fsharp-ref – Claude Code Implementation Notes

## High-level purpose

This repo is a **reference integration** to prove out the F# → Rust → Bevy path:

- Game/logic code is authored in F#.
- F# is transpiled to Rust via `raibid-labs/fable` + `fsrs`.
- The resulting Rust code is compiled into a Bevy-based app that runs both:
  - Natively,
  - And as WASM in the browser.

This is not the final game – it’s a focused integration experiment and reference for future Grimware work.

## Repos / dependencies

- `raibid-labs/fable` – Fable fork with Rust backend.
- `raibid-labs/fsrs` – F# → Rust glue.
- `grimware/bevy-wasm-fsharp-ref` (this repo).

## Design goals

1. **Minimal Bevy app**
   - A tiny 2D or 3D scene that clearly demonstrates:
     - Game state,
     - Input handling,
     - A simple update loop (movement / combat ticks / timers).

2. **F#-authored game rules**
   - F# defines the “rules of the world”:
     - Entity stats/attributes.
     - Simple combat rules (e.g., player vs monster).
     - Ability cooldowns or simple buffs/debuffs.
   - F# logic gets transpiled into Rust modules that are called from Bevy systems.

3. **Dual target: native + WASM**
   - One codebase, two targets:
     - Native desktop (for debugging & iteration).
     - WebAssembly (browser) build to prove viability for deployment.

## Target structure

```text
bevy-wasm-fsharp-ref/
  Cargo.toml
  crates/
    app/             # bevy app crate
    logic-fsharp/    # (hand-written initially) crate that will be replaced by fsrs output
  fsharp/
    GameLogic.fs     # F# game logic (combat, abilities, etc.)
    Domain.fs        # Shared domain types (stats, damage, etc.)
  web/
    index.html       # simple page to load WASM
    wasm-bindgen.sh  # build script (or equivalent)
  docs/
    architecture.md
    fsharp-integration.md
  CLAUDE_NOTES.md
```

Initially, `crates/logic-fsharp` can be hand-written Rust that mirrors what we expect F# → Rust output to look like.

## Domain model

Keep it deliberately simple, but realistic enough:

- Types (Rust and F# mirrors):
  - `Character` (player, monster)
  - `Stats` (hp, attack, defense, speed)
  - `Ability` (name, cooldown, effect)
  - `CombatEvent` (DamageDealt, Death, Heal, etc.)

- F# responsibilities:
  - Given a `(attacker, defender)` and an `Ability`, compute:
    - Damage,
    - Resulting HP,
    - Generated `CombatEvent`s.
  - Define “AI” logic for the enemy (very simple, e.g. always use basic attack).

- Rust / Bevy responsibilities:
  - ECS components and systems.
  - Rendering sprites / shapes.
  - Input handling (keyboard/mouse).
  - Updating game state per frame and calling into F#-derived logic to resolve actions.

## Expectations for Claude Code – implementation steps

1. **Bevy app scaffold**
   - Create `crates/app` with:
     - Basic Bevy app.
     - A main system that spawns:
       - One player entity,
       - One monster entity.
   - Render simple shapes (e.g. rectangles/circles) or placeholder sprites.

2. **Domain crate**
   - Create a Rust crate `crates/logic-fsharp` exposing functions such as:
     - `fn compute_attack(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent`
   - For now, implement these functions directly in Rust, with a very simple combat rule.

3. **F# mirror (future fsrs target)**
   - Add `fsharp/Domain.fs` and `fsharp/GameLogic.fs` defining the same domain and logic signatures in F#.
   - Document in `docs/fsharp-integration.md`:
     - That the Rust `logic-fsharp` crate is a stand-in for the fsrs-generated output.
     - How we intend to replace it once fsrs integration is wired up.

4. **Native + WASM builds**
   - Add instructions and configuration for:
     - A native build (`cargo run -p app`).
     - A WASM build (likely using `wasm-bindgen`/`wasm-pack` + Bevy’s WASM support).
   - Provide a minimal `web/index.html` that loads the WASM bundle and shows the game.

5. **Proof-of-concept loop**
   - Implement a simple loop where:
     - The player can press a key (e.g., space) to attack the monster.
     - Each attack calls into `logic-fsharp` to compute damage.
     - When HP <= 0, show some simple state change (e.g., “You win!” text).

## Style and constraints

- Keep the Bevy setup minimal; this is about integration, not content.
- Make the domain types shared and clearly documented so it’s straightforward to align F# and Rust representations.
- Document all assumptions in `docs/architecture.md` so we can reuse the pattern in future Grimware games.
