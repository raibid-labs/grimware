# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a cross-platform Tauri v2 application targeting macOS M3 (ARM64), Android (mobile), and Linux (NVIDIA DGX-Spark). It demonstrates a single codebase approach with Rust backend and vanilla JavaScript frontend.

## Common Commands

### Development
```bash
# Desktop development (hot reload)
npm run tauri:dev

# Android development (requires Android setup)
npm run tauri:android

# Frontend-only dev server
npm run dev
```

### Building
```bash
# Build desktop app (creates optimized binary)
npm run tauri:build

# Build Android APK
npm run tauri:android:build

# Build frontend only
npm run build
```

### Testing
```bash
# Check Android logs
adb logcat | grep Tauri

# List connected Android devices/emulators
adb devices
```

## Architecture

### Multi-Platform Entry Points
The application has **separate entry points** for desktop and mobile platforms:
- **Desktop**: `src-tauri/src/main.rs` - Entry point for macOS/Linux builds
- **Mobile**: `src-tauri/src/mobile.rs` - Entry point for Android/iOS builds (via `lib.rs`)
- **Shared**: `src-tauri/src/commands.rs` - Common Tauri command handlers used by both

The Rust crate is configured in `Cargo.toml` to build as:
- `staticlib` - For mobile (Android/iOS linking)
- `cdylib` - For dynamic linking scenarios
- `rlib` - For Rust library usage

### Platform Detection
Both entry points register the same command handlers but are compiled conditionally:
- Desktop uses `#[cfg(not(mobile))]`
- Mobile uses `#[cfg(mobile)]` and `#[tauri::mobile_entry_point]`

### IPC Communication Pattern
Frontend communicates with Rust backend via Tauri's type-safe IPC:

```javascript
// Frontend: src/main.js
import { invoke } from '@tauri-apps/api/core'
const result = await invoke('command_name', { param: value })
```

```rust
// Backend: src-tauri/src/commands.rs
#[tauri::command]
pub fn command_name(param: &str) -> String {
    // Implementation
}
```

All new commands must be:
1. Defined with `#[tauri::command]` in `commands.rs`
2. Registered in both `main.rs` and `mobile.rs` via `invoke_handler![]`

### Generated Android Code
The `src-tauri/gen/android/` directory contains auto-generated Android project files created by Tauri CLI. This includes:
- Gradle build scripts
- Kotlin/Java glue code (`MainActivity.kt`)
- Android manifest and resources
- Rust build integration

**Do not manually edit** files in `gen/android/` - regenerate via `tauri android init` if needed.

## Configuration Files

### tauri.conf.json
Central configuration for Tauri app:
- Build commands and dev server URL
- Window properties (size, title, resizable)
- Bundle targets and platform-specific settings
- Plugin configurations (shell access)
- Security policies (CSP)

### Cargo.toml Release Profile
Optimized for small binary size:
- `opt-level = "s"` - Size optimization
- `lto = true` - Link-time optimization
- `strip = true` - Strip symbols
- `codegen-units = 1` - Better optimization at compile-time cost

### Vite Configuration
- Dev server runs on port **1420** (must match `tauri.conf.json`)
- Ignores `src-tauri/` in watch mode
- Minification disabled in debug mode
- Build target: ES2021, Chrome 100+, Safari 13+

## Development Workflow

### Adding New Commands
1. Define command function in `src-tauri/src/commands.rs`
2. Add to `generate_handler![]` in both `src-tauri/src/main.rs` and `src-tauri/src/mobile.rs`
3. Call from frontend via `invoke('command_name', { args })`

### DevTools Access
Development builds automatically open DevTools on desktop (macOS/Linux) via the setup hook in `main.rs`. This is gated by `#[cfg(debug_assertions)]`.

### Platform-Specific Code
Use Rust's `cfg` attributes for conditional compilation:
- `#[cfg(mobile)]` - Android/iOS only
- `#[cfg(not(mobile))]` - Desktop only
- `#[cfg(target_os = "android")]` - Android specific
- `#[cfg(target_os = "macos")]` - macOS specific

## Important Notes

- The frontend uses **vanilla JavaScript** (no React/Vue) - keep it simple
- The `src-tauri/gen/` directory is auto-generated - don't manually edit
- Android builds require specific NDK/SDK setup (see docs/SETUP.md)
- The app bundle identifier is `com.raibid.tauri.hello` (in tauri.conf.json)
- Default window size is 800x600 (configurable in tauri.conf.json)
