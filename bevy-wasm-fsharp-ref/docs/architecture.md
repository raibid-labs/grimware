# bevy-wasm-fsharp-ref architecture (sketch)

- `crates/app`: Bevy game application (native + WASM targets).
- `crates/logic-fsharp`: Rust crate that currently hard-codes combat logic; eventually
  this will be replaced by fsrs-generated Rust from F# sources.
- `fsharp/`: F# domain and game logic (`Domain.fs`, `GameLogic.fs`).

The goal is to prove out the pattern of:

F# game rules -> Fable+fsrs -> Rust crate -> Bevy systems.
