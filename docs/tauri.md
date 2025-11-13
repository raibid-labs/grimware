# Tauri Cross-Platform Reference Implementation

## Overview

Production-ready Tauri v2 application demonstrating cross-platform development for desktop and mobile from a single codebase. Targets macOS M3 (ARM64), Android mobile, and Linux (NVIDIA DGX-Spark).

## Key Features

- **Single Codebase**: Write once, run on desktop and mobile
- **Native Performance**: Rust backend with platform-native UI
- **Small Binary Size**: 2-12MB depending on platform
- **Modern Frontend**: HTML5, CSS3, vanilla JavaScript
- **Type-Safe IPC**: Rust ↔ JavaScript communication
- **Platform Detection**: Automatic OS and architecture detection

## Quick Start

```bash
cd tauri-ref

# Install dependencies
npm install

# Desktop development (hot reload)
npm run tauri:dev

# Android development
npm run tauri:android

# Build for production
npm run tauri:build
```

## Architecture

### Multi-Platform Entry Points

**Desktop**: `src-tauri/src/main.rs`
- Entry point for macOS/Linux builds
- Opens DevTools in debug mode
- Uses window-based UI

**Mobile**: `src-tauri/src/mobile.rs` (via `lib.rs`)
- Entry point for Android/iOS builds
- Platform-specific initialization
- Uses native mobile UI components

**Shared**: `src-tauri/src/commands.rs`
- Common Tauri command handlers
- Used by both desktop and mobile

### Build Configuration

The Rust crate builds as:
- `staticlib` - For mobile (Android/iOS linking)
- `cdylib` - For dynamic linking
- `rlib` - For Rust library usage

## IPC Communication Pattern

Type-safe communication between frontend and backend:

**Frontend** (`src/main.js`):
```javascript
import { invoke } from '@tauri-apps/api/core'

const result = await invoke('greet', { name: 'World' })
console.log(result)
```

**Backend** (`src-tauri/src/commands.rs`):
```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Adding New Commands

1. Define in `src-tauri/src/commands.rs`:
```rust
#[tauri::command]
fn my_command(param: &str) -> String {
    // Implementation
}
```

2. Register in both entry points:

**main.rs**:
```rust
invoke_handler![commands::greet, commands::my_command]
```

**mobile.rs**:
```rust
invoke_handler![commands::greet, commands::my_command]
```

3. Call from frontend:
```javascript
const result = await invoke('my_command', { param: 'value' })
```

## Project Structure

```
tauri-ref/
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html         # Main UI
│   ├── style.css          # Styling
│   └── main.js            # Frontend logic
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs       # Desktop entry
│   │   ├── mobile.rs     # Mobile entry
│   │   ├── lib.rs        # Library exports
│   │   └── commands.rs   # Shared commands
│   ├── Cargo.toml        # Rust dependencies
│   ├── tauri.conf.json   # Tauri configuration
│   └── gen/              # Auto-generated (Android)
├── docs/
│   ├── SETUP.md          # Platform setup guide
│   ├── ARCHITECTURE.md   # Technical architecture
│   ├── PLATFORMS.md      # Platform-specific info
│   └── ANDROID_DEVICE.md # Android device testing
├── CLAUDE.md             # AI assistant config
└── package.json          # Node dependencies
```

## Platform Support

### macOS M3 (ARM64)
- Native Apple Silicon binary
- Uses WKWebView (native WebKit)
- ~2-3MB binary size
- Code signing supported
- DMG packaging

**Build**:
```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/dmg/
```

### Android (Mobile)
- API 24+ (Android 7.0+)
- ARM64-v8a primary target
- APK/AAB packaging
- ~8-12MB app size
- Google Play ready

**Setup**:
See [docs/SETUP.md](../tauri-ref/docs/SETUP.md) for Android SDK/NDK requirements.

**Build**:
```bash
npm run tauri:android:build
# Output: src-tauri/gen/android/app/build/outputs/apk/
```

**Testing**:
```bash
# List devices
adb devices

# Run on device/emulator
npm run tauri:android

# View logs
adb logcat | grep Tauri
```

### Linux (DGX-Spark)
- x86_64 architecture
- WebKit2GTK renderer
- DEB/AppImage packaging
- GPU acceleration support
- ~3-4MB binary size

**Build**:
```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/
```

## Configuration

### tauri.conf.json

Central configuration for:
- **Build**: Commands and dev server URL
- **Windows**: Size, title, resizable properties
- **Bundle**: Platform-specific settings
- **Plugins**: Shell access, file system, etc.
- **Security**: Content Security Policy (CSP)

### Cargo.toml Release Profile

Optimized for small binaries:
```toml
[profile.release]
opt-level = "s"      # Size optimization
lto = true           # Link-time optimization
strip = true         # Strip debug symbols
codegen-units = 1    # Better optimization
```

### Vite Configuration

- Dev server on port 1420 (matches tauri.conf.json)
- Ignores `src-tauri/` in watch mode
- Build target: ES2021, Chrome 100+, Safari 13+
- HMR (Hot Module Replacement) enabled

## Development Workflow

### Development Mode
```bash
# Frontend only (faster iteration)
npm run dev

# Full Tauri dev (with Rust backend)
npm run tauri:dev

# Android dev with hot reload
npm run tauri:android
```

### Building
```bash
# Build frontend
npm run build

# Build desktop app
npm run tauri:build

# Build Android APK
npm run tauri:android:build
```

### Testing
```bash
# Frontend tests (if configured)
npm test

# Rust tests
cd src-tauri && cargo test

# Android on emulator
adb devices
npm run tauri:android
adb logcat | grep Tauri
```

## Platform-Specific Code

Use Rust's `cfg` attributes for conditional compilation:

```rust
// Android/iOS only
#[cfg(mobile)]
fn mobile_specific_function() { }

// Desktop only
#[cfg(not(mobile))]
fn desktop_specific_function() { }

// Android specific
#[cfg(target_os = "android")]
fn android_specific_function() { }

// macOS specific
#[cfg(target_os = "macos")]
fn macos_specific_function() { }
```

## Performance Metrics

| Platform | Binary Size | Startup Time | Memory Usage |
|----------|-------------|--------------|--------------|
| macOS M3 | ~2-3MB | ~150ms | ~40MB |
| Android | ~8-12MB | ~650ms | ~50MB |
| Linux | ~3-4MB | ~200ms | ~45MB |

## Security

- **Minimal Permissions**: Only request what's needed
- **Type-Safe IPC**: Validated Rust ↔ JS communication
- **OS Sandboxing**: Platform-level security
- **CSP Configured**: Content Security Policy enforced
- **No Remote Code**: No `eval()` or dynamic code execution

## UI Demo Features

The reference app includes:
- Platform detection display (OS, architecture)
- Interactive greeting form with IPC demo
- Responsive design (desktop + mobile)
- Modern gradient UI with glassmorphism

## Generated Android Code

`src-tauri/gen/android/` contains auto-generated files:
- Gradle build scripts
- Kotlin glue code (`MainActivity.kt`)
- Android manifest and resources
- Rust build integration

**⚠️ Do not manually edit** - regenerate via `tauri android init` if needed.

## Best Practices

### Command Design
- Keep commands simple and focused
- Use Rust types for validation
- Return Results for error handling
- Avoid blocking operations in commands

### Frontend Design
- Keep frontend lightweight (vanilla JS preferred)
- Use Tauri APIs for native functionality
- Handle IPC errors gracefully
- Minimize bundle size

### Cross-Platform Development
- Test on all target platforms regularly
- Use feature flags for platform-specific code
- Document platform differences
- Consider mobile constraints (battery, network)

## Common Tasks

### Adding Dependencies

**Rust** (`src-tauri/Cargo.toml`):
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

**JavaScript** (`package.json`):
```bash
npm install <package-name>
```

### Accessing File System
```javascript
import { readTextFile } from '@tauri-apps/api/fs'

const contents = await readTextFile('path/to/file.txt')
```

### Opening URLs
```javascript
import { open } from '@tauri-apps/api/shell'

await open('https://example.com')
```

### System Dialogs
```rust
use tauri::api::dialog::blocking::message;

#[tauri::command]
fn show_message() {
    message(Some("Title"), "Message content");
}
```

## Troubleshooting

### Android Build Issues
- Ensure Android SDK/NDK are installed
- Check `ANDROID_HOME` environment variable
- Verify Java 11+ is available
- Run `tauri android sync` to update dependencies

### macOS Code Signing
- Configure signing in `tauri.conf.json`
- Use Developer ID for distribution
- Test unsigned builds first

### Linux Dependencies
```bash
# Install required system libraries
sudo apt install libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## Further Reading

- [Setup Guide](../tauri-ref/docs/SETUP.md) - Platform-specific installation
- [Architecture Details](../tauri-ref/docs/ARCHITECTURE.md) - Technical deep dive
- [Platform Info](../tauri-ref/docs/PLATFORMS.md) - Platform-specific details
- [Android Device Testing](../tauri-ref/docs/ANDROID_DEVICE.md) - Android setup
- [Tauri Documentation](https://tauri.app/)
- [Tauri Mobile Guide](https://tauri.app/v1/guides/building/android)
