# Chapter 1: Getting Started

## What You'll Learn

In this chapter, you'll set up your development environment, run the reference implementation, and understand the project structure. By the end, you'll have a working game running both natively and in your browser.

**Time Required**: 30 minutes

## Prerequisites

Before starting, ensure you have:

- A computer running macOS, Linux, or Windows
- Internet connection for downloading tools
- Basic command line familiarity
- 2GB free disk space

## Step 1: Install Development Tools

### 1.1 Install Rust

Rust is our primary implementation language.

```bash
# Install rustup (Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then:
source $HOME/.cargo/env

# Verify installation
rustc --version
# Expected: rustc 1.75.0 or later
```

### 1.2 Install F# (Optional for Tutorial)

F# is used for domain modeling. While not required to run the game, it's needed to modify the domain model.

**macOS/Linux:**
```bash
# Install .NET SDK
curl -L https://dot.net/v1/dotnet-install.sh | bash

# Add to PATH
export PATH="$HOME/.dotnet:$PATH"

# Verify
dotnet --version
# Expected: 8.0.0 or later
```

**Windows:**
Download and install from [https://dotnet.microsoft.com/download](https://dotnet.microsoft.com/download)

### 1.3 Install Additional Tools

```bash
# Install wasm-pack for WebAssembly builds
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install basic-http-server for testing WASM locally
cargo install basic-http-server

# Install just (command runner) - optional but recommended
cargo install just
```

## Step 2: Clone and Explore the Repository

### 2.1 Clone the Project

```bash
# Clone the repository
git clone https://github.com/raibid-labs/grimware.git
cd grimware/bevy-wasm-fsharp-ref

# Verify you're in the right place
ls
# Expected: Cargo.toml, src/, fsharp/, crates/, etc.
```

### 2.2 Understand the Project Structure

```
bevy-wasm-fsharp-ref/
├── Cargo.toml           # Root Rust configuration
├── src/                 # Main game source
│   ├── main.rs         # Entry point
│   ├── combat/         # Combat systems
│   ├── visual/         # Rendering systems
│   └── wasm.rs         # Web-specific code
├── fsharp/             # F# domain model (reference)
│   ├── Domain.fs       # Core types
│   ├── Combat.fs       # Combat logic
│   └── Game.fsx        # Interactive script
├── crates/             # Rust sub-projects
│   └── logic-fsharp/   # F# logic ported to Rust
│       ├── src/lib.rs  # Core logic implementation
│       └── tests/      # Logic tests
├── assets/             # Game assets
│   └── fonts/          # UI fonts
└── docs/               # Documentation
    └── tutorial/       # This tutorial!
```

**Key Directories:**

- **`src/`**: The game implementation using Bevy
- **`fsharp/`**: Domain model in F# (our "source of truth")
- **`crates/logic-fsharp/`**: F# logic translated to Rust
- **`assets/`**: Game resources (fonts, eventually sprites)

## Step 3: Build and Run

### 3.1 Run Native Version

Let's run the game on your desktop:

```bash
# Build and run (debug mode)
cargo run

# Or with just (if installed)
just run
```

**Expected Output:**
```
Compiling bevy-wasm-fsharp-ref v0.1.0
Finished dev [unoptimized + debuginfo] target(s)
Running `target/debug/bevy-wasm-fsharp-ref`

[Game window opens showing combat scene]
```

### 3.2 Understanding the Game Window

When the game runs, you'll see:

```
┌─────────────────────────────────┐
│  COMBAT ARENA                   │
│                                 │
│  Hero              Slime        │
│  HP: 100/100      HP: 30/30    │
│                                 │
│  [A] Attack  [D] Defend  [R] Run│
│                                 │
│  Combat Log:                    │
│  > Battle begins!               │
│  > Hero's turn...               │
└─────────────────────────────────┘
```

**Controls:**
- **A**: Attack the enemy
- **D**: Defend (reduces damage)
- **R**: Run from battle (not implemented)
- **ESC**: Exit game

### 3.3 Build for WebAssembly

Now let's run the game in a browser:

```bash
# Build WASM version
just build-wasm

# Or manually:
wasm-pack build --target web --out-dir wasm/pkg

# Serve locally
just serve-wasm

# Or manually:
cd wasm && basic-http-server --addr 127.0.0.1:8080
```

Open your browser to [http://localhost:8080](http://localhost:8080)

**Expected:** The same game running in your browser!

## Step 4: Verify Everything Works

### 4.1 Run Tests

```bash
# Run all tests
cargo test

# Expected output:
running 12 tests
test logic::tests::test_calculate_damage ... ok
test logic::tests::test_apply_damage ... ok
test logic::tests::test_is_alive ... ok
[... more tests ...]
test result: ok. 12 passed; 0 failed
```

### 4.2 Check F# Domain (Optional)

If you have F# installed:

```bash
# Navigate to F# directory
cd fsharp

# Run interactive script
dotnet fsi Game.fsx

# Expected output:
Hero attacks Slime for 8 damage!
Slime HP: 22/30
Slime attacks Hero for 3 damage!
Hero HP: 97/100
```

## Step 5: Make Your First Change

Let's make a simple modification to verify your setup:

### 5.1 Change the Hero's Name

Edit `crates/logic-fsharp/src/lib.rs`:

```rust
// Find this function (around line 45)
pub fn create_hero() -> Character {
    Character {
        name: "Hero".to_string(),  // Change this!
        hp: 100,
        stats: Stats {
            hp: 100,
            attack: 10,
            defense: 5,
        },
    }
}

// Change to:
pub fn create_hero() -> Character {
    Character {
        name: "Mighty Warrior".to_string(),  // Your custom name!
        hp: 100,
        stats: Stats {
            hp: 100,
            attack: 10,
            defense: 5,
        },
    }
}
```

### 5.2 Rebuild and Run

```bash
# Rebuild and run
cargo run

# The game should now show "Mighty Warrior" instead of "Hero"
```

Congratulations! You've successfully modified the game.

## Common Issues and Solutions

### Issue: "rustc: command not found"

**Solution:** Rust isn't in your PATH. Run:
```bash
source $HOME/.cargo/env
```

### Issue: Game window doesn't appear

**Solution:** Ensure you have graphics drivers installed:
- **Linux**: Install `libx11-dev`, `libasound2-dev`
- **macOS**: Should work out of the box
- **Windows**: Update graphics drivers

### Issue: WASM build fails

**Solution:** Ensure wasm-pack is installed:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Issue: Slow compilation

**Solution:** Enable dynamic linking for faster development builds:
```bash
cargo run --features bevy/dynamic_linking
```

## What You've Accomplished

✅ Installed all required development tools
✅ Cloned and explored the project structure
✅ Built and ran the game natively
✅ Built and ran the game in a browser
✅ Made your first code change
✅ Verified tests pass

## Key Takeaways

1. **Three Languages, One Game**: F# defines the domain, Rust implements it, WebAssembly deploys it
2. **Separation of Concerns**: Logic is separate from presentation
3. **Cross-Platform by Default**: Same code runs native and web
4. **Type Safety Throughout**: F# and Rust both provide strong typing

## Exercises

Before moving on, try these exercises:

### Exercise 1: Modify Monster Stats

Change the slime's stats in `crates/logic-fsharp/src/lib.rs`:
- Give it more HP (try 50)
- Increase its attack power
- Run the game and observe the changes

### Exercise 2: Add a New Monster Type

Create a new function `create_goblin()` with different stats:
- HP: 40
- Attack: 8
- Defense: 3

### Exercise 3: Explore the Combat Log

Run the game and observe the combat log. Try to identify:
- How damage is calculated
- When defense applies
- What happens when HP reaches 0

## Next Steps

Now that you have a working development environment and understand the project structure, you're ready to learn about F# and why it's perfect for domain modeling.

In the next chapter, we'll explore F# fundamentals and see how functional programming makes game logic more reliable and easier to reason about.

[Next: Understanding F# →](02-understanding-fsharp.md)

[← Back to Tutorial Index](README.md)