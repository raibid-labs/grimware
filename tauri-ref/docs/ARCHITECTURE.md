# Tauri Hello World - Architecture

## Overview

This is a cross-platform application built with Tauri v2, targeting three distinct platforms:
- **macOS M3 (ARM64)** - Desktop
- **Android** - Mobile
- **Linux (NVIDIA DGX-Spark)** - High-performance computing

## Technology Stack

### Frontend
- **HTML5/CSS3** - Modern web standards
- **Vanilla JavaScript (ES Modules)** - No framework overhead
- **Vite** - Build tool and dev server

### Backend
- **Rust** - Systems programming language
- **Tauri v2** - Application framework
- **tauri-plugin-shell** - System integration

## Architecture Layers

```
┌─────────────────────────────────────┐
│         Frontend (WebView)          │
│  ┌──────────────────────────────┐  │
│  │   HTML + CSS + JavaScript    │  │
│  │   (Vite bundled)             │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
              ↕ (IPC)
┌─────────────────────────────────────┐
│      Tauri Core (Rust)              │
│  ┌──────────────────────────────┐  │
│  │  Command Handlers            │  │
│  │  - greet()                   │  │
│  │  - get_platform_info()       │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
              ↕
┌─────────────────────────────────────┐
│      Platform Layer                 │
│  ┌────────┬────────┬────────────┐  │
│  │ macOS  │Android │   Linux    │  │
│  │ ARM64  │ Mobile │ DGX-Spark  │  │
│  └────────┴────────┴────────────┘  │
└─────────────────────────────────────┘
```

## Project Structure

```
tauri-ref/
├── src/                      # Frontend source
│   ├── index.html           # Main HTML
│   ├── style.css            # Styles
│   └── main.js              # JavaScript logic
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Desktop entry point
│   │   └── lib.rs           # Mobile library
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   └── build.rs             # Build script
├── docs/                     # Documentation
│   ├── SETUP.md             # Setup guide
│   └── ARCHITECTURE.md      # This file
├── package.json             # Node dependencies
└── vite.config.js           # Vite configuration
```

## IPC Communication

### Command Pattern

Frontend calls Rust backend via Tauri's IPC:

```javascript
// Frontend (JavaScript)
import { invoke } from '@tauri-apps/api/core'
const result = await invoke('greet', { name: 'World' })
```

```rust
// Backend (Rust)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Available Commands

1. **greet(name: string)**: Returns personalized greeting
2. **get_platform_info()**: Returns OS and architecture information

## Platform-Specific Builds

### macOS M3 (ARM64)
- **Target**: `aarch64-apple-darwin`
- **Binary Type**: Native macOS app bundle
- **Window Manager**: Cocoa (via Tauri)
- **WebView**: WKWebView (Apple's native)

### Android
- **Target**: `aarch64-linux-android` (primary), `armv7-linux-androideabi`
- **Binary Type**: APK/AAB
- **Activity**: TauriActivity (JNI bridge)
- **WebView**: Android System WebView

### Linux (DGX-Spark)
- **Target**: `x86_64-unknown-linux-gnu`
- **Binary Type**: DEB, AppImage, or binary
- **Window Manager**: GTK
- **WebView**: WebKit2GTK-4.1

## Build Profiles

### Development
- **Optimizations**: Minimal (faster builds)
- **Debug Info**: Full
- **DevTools**: Enabled

### Release
- **Optimizations**: Maximum (`opt-level = "s"`, LTO, strip)
- **Debug Info**: None
- **DevTools**: Disabled
- **Binary Size**: Minimized

## Security

- **CSP**: Configured in tauri.conf.json
- **IPC**: Type-safe with Rust validation
- **Capabilities**: Minimal permissions (shell:allow-open only)
- **Sandboxing**: OS-level isolation

## Performance Considerations

### Binary Size
- Strip symbols in release mode
- LTO (Link Time Optimization) enabled
- Code unit optimization (`codegen-units = 1`)

### Runtime
- Native performance via Rust
- WebView uses system renderer
- Minimal JavaScript overhead
- Efficient IPC via serialization

## Cross-Platform Compatibility

### Shared Code
- Rust core logic (src-tauri/src/main.rs)
- Frontend UI (src/)
- Configuration (tauri.conf.json)

### Platform-Specific
- Android: lib.rs, gen/android/
- macOS: Bundle creation, code signing
- Linux: System dependencies, packaging

## Future Enhancements

- [ ] Add native database (SQLite)
- [ ] Implement file system access
- [ ] Add notification support
- [ ] System tray integration
- [ ] Auto-updater
- [ ] Analytics/telemetry
- [ ] Internationalization (i18n)
