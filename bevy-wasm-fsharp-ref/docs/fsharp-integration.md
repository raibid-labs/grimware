# F# integration plan

1. Author game domain types and rules in F# under `fsharp/`.
2. Use `raibid-labs/fable` and `raibid-labs/fsrs` to transpile that F# code to Rust.
3. Generate Rust into a crate that replaces or extends `crates/logic-fsharp`.
4. Have `crates/app` depend on the generated crate and call its functions from Bevy systems.

For now, `crates/logic-fsharp` is a hand-written stand-in that mirrors the F#
signatures in `Domain.fs` / `GameLogic.fs`.
