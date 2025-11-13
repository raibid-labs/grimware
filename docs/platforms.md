# Platform Support Guide

Comprehensive platform support matrix and platform-specific guidance for Grimware reference implementations.

## Platform Matrix

| Project | macOS | Linux | Windows | Android | iOS | Web/WASM |
|---------|-------|-------|---------|---------|-----|----------|
| **Bevy MCP** | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Bevy MCP Ratatui** | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Tauri** | ✅ M3 | ✅ DGX | ⚠️ | ✅ | ⚠️ | ❌ |
| **WebATUI** | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ |

Legend:
- ✅ Fully supported and tested
- ⚠️ Supported but not extensively tested
- ❌ Not supported

## macOS

### Supported Versions
- **macOS 11** (Big Sur) and later
- **Architecture**: Intel (x86_64) and Apple Silicon (ARM64)

### Apple Silicon Specific

**M1/M2/M3 Optimization**:
- Native ARM64 compilation (no Rosetta)
- Optimized for Metal graphics API
- Fast compile times with native toolchain

**Setup**:
```bash
# Verify ARM64 architecture
uname -m  # Should output: arm64

# Install Xcode Command Line Tools
xcode-select --install

# Verify Rust targets native ARM64
rustc --version --verbose | grep host
# Should show: aarch64-apple-darwin
```

### Project-Specific Notes

**Bevy MCP/Ratatui**:
- Native Metal rendering (fastest performance)
- CoreAudio for audio (when enabled)
- Full BRP support on localhost

**Tauri**:
- WKWebView (native WebKit)
- 2-3MB binary size
- DMG packaging for distribution
- Code signing via `tauri.conf.json`

**WebATUI**:
- iTerm2 recommended (best Unicode support)
- Kitty for GPU acceleration
- Alacritty for performance
- Full bacon/just tool support

## Linux

### Supported Distributions
- **Ubuntu** 20.04 LTS and later
- **Debian** 11 and later
- **Fedora** 35 and later
- **Arch Linux** (rolling)

### System Dependencies

**For Bevy Projects**:
```bash
# Ubuntu/Debian
sudo apt install \
  build-essential \
  pkg-config \
  libudev-dev \
  libasound2-dev

# Fedora
sudo dnf install \
  gcc \
  pkg-config \
  systemd-devel \
  alsa-lib-devel

# Arch
sudo pacman -S \
  base-devel \
  alsa-lib
```

**For Tauri**:
```bash
# Ubuntu/Debian
sudo apt install \
  libwebkit2gtk-4.0-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install \
  webkit2gtk4.0-devel \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel
```

### NVIDIA DGX-Spark Specific

**Configuration**:
- x86_64 architecture
- CUDA drivers installed
- High-performance GPU available

**Bevy Optimization**:
```bash
# Enable GPU acceleration
WINIT_UNIX_BACKEND=x11 cargo run --release
```

**Tauri on DGX**:
- WebKit2GTK renderer
- 3-4MB binary size
- DEB packaging for deployment
- GPU acceleration for WebGL

### Terminals

**Recommended**:
- Alacritty (GPU-accelerated, cross-platform)
- Kitty (feature-rich, ligatures)
- GNOME Terminal (native integration)

**Configuration**:
```bash
# Check terminal capabilities
echo $TERM  # Should be xterm-256color or better

# Test true color support
curl -s https://gist.githubusercontent.com/lifepillar/09a44b8cf0f9397465614e622979107f/raw/24-bit-color.sh | bash
```

## Windows

### Supported Versions
- **Windows 10** (1809+)
- **Windows 11**

### Prerequisites

**Visual Studio Build Tools**:
```powershell
# Download from:
# https://visualstudio.microsoft.com/downloads/

# Required components:
# - MSVC v143 - VS 2022 C++ build tools
# - Windows 10/11 SDK
```

**Rust Setup**:
```powershell
# Install via rustup-init.exe
# Select: x86_64-pc-windows-msvc

# Verify
rustc --version
```

### Project-Specific Notes

**Bevy MCP/Ratatui**:
- DirectX 12 rendering (Windows 10+)
- XAudio2 for audio
- Windows Terminal recommended

**Tauri**:
- WebView2 (built into Windows 11, install separately for Win 10)
- NSIS installer packaging
- Code signing via SignTool

**WebATUI**:
- Windows Terminal (best experience)
- ConEmu (alternative)
- Native crossterm support

### Terminal Setup

**Windows Terminal** (recommended):
```json
// settings.json
{
  "profiles": {
    "defaults": {
      "colorScheme": "One Half Dark",
      "fontFace": "Cascadia Code PL",
      "fontSize": 12
    }
  }
}
```

## Android

### Supported Devices
- **API Level 24+** (Android 7.0+)
- **Architecture**: ARM64-v8a (primary), ARMv7-a (legacy)

### Prerequisites

**Android Studio**:
```bash
# Download from:
# https://developer.android.com/studio

# Required components:
# - Android SDK Platform 33+
# - Android SDK Build-Tools
# - Android SDK Platform-Tools
# - Android NDK (latest)
```

**Environment Setup**:
```bash
# Set ANDROID_HOME
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

# Add to PATH
export PATH="$ANDROID_HOME/platform-tools:$PATH"
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"

# Verify
adb version
```

### Tauri Android

**Build**:
```bash
cd tauri-ref

# Initialize Android project
npm run tauri android init

# Build APK
npm run tauri android build

# Install on device
adb install src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
```

**Testing**:
```bash
# List devices
adb devices

# View logs
adb logcat | grep Tauri

# Debug on device
npm run tauri android dev
```

**Performance**:
- ~8-12MB APK size
- ~650ms startup time
- ~50MB memory usage
- Native UI components

See [Tauri Android Guide](../tauri-ref/docs/ANDROID_DEVICE.md) for detailed instructions.

## Web/WASM

### Browser Support
- **Chrome/Edge** 90+
- **Firefox** 88+
- **Safari** 14+

### WebATUI Web Target

**Build**:
```bash
cd webatui-ref

# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Serve locally
python3 -m http.server 8000
# Open http://localhost:8000
```

**Optimization**:
```toml
[profile.release]
opt-level = "z"       # Size optimization
lto = true
codegen-units = 1

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz"]    # Aggressive size optimization
```

**Result**: ~200KB WASM (gzipped)

### Testing

**Node.js**:
```bash
wasm-pack test --node
```

**Browser** (requires ChromeDriver):
```bash
wasm-pack test --chrome --headless
```

## Terminal Compatibility

### Terminal Feature Matrix

| Terminal | Platform | True Color | Ligatures | GPU Accel | Unicode |
|----------|----------|------------|-----------|-----------|---------|
| Alacritty | All | ✅ | ❌ | ✅ | ✅ |
| Kitty | macOS/Linux | ✅ | ✅ | ✅ | ✅ |
| iTerm2 | macOS | ✅ | ✅ | ✅ | ✅ |
| WezTerm | All | ✅ | ✅ | ✅ | ✅ |
| GNOME Terminal | Linux | ✅ | ❌ | ❌ | ✅ |
| Windows Terminal | Windows | ✅ | ✅ | ⚠️ | ✅ |
| Terminal.app | macOS | ⚠️ | ❌ | ❌ | ✅ |

### Recommended by Use Case

**Performance**: Alacritty, Kitty
**Features**: WezTerm, iTerm2
**Native Integration**: GNOME Terminal, Windows Terminal, Terminal.app

## Cross-Platform Development Tips

### Code Organization

Use conditional compilation:
```rust
#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS code
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux code
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows code
}

#[cfg(mobile)]
fn mobile_specific() {
    // Android/iOS code
}
```

### Testing Strategy

1. **Primary platform**: Develop on your main platform
2. **CI/CD**: Test all platforms automatically
3. **Manual testing**: Test UI/UX on each platform
4. **Performance**: Profile on target hardware

### Build Scripts

```bash
#!/bin/bash
# build-all.sh

# Desktop
cargo build --release

# Android (if Tauri)
npm run tauri:android:build

# WASM (if WebATUI)
wasm-pack build --target web

echo "All platforms built successfully"
```

## Performance by Platform

### Bevy Projects

| Platform | Frame Time | Compile Time | Binary Size |
|----------|------------|--------------|-------------|
| macOS M3 | 8-12ms | 2-4 min | 15-20MB |
| Linux x86_64 | 10-14ms | 4-6 min | 18-22MB |
| Windows x86_64 | 12-16ms | 5-7 min | 18-22MB |

### Tauri Projects

| Platform | Startup Time | Binary Size | Memory |
|----------|--------------|-------------|--------|
| macOS M3 | ~150ms | 2-3MB | ~40MB |
| Linux x86_64 | ~200ms | 3-4MB | ~45MB |
| Windows x86_64 | ~250ms | 8-10MB | ~50MB |
| Android ARM64 | ~650ms | 8-12MB | ~50MB |

### WebATUI Projects

| Platform | Target | Bundle Size | Load Time |
|----------|--------|-------------|-----------|
| Native | Terminal | 8-12MB | <100ms |
| Web | WASM | ~200KB (gz) | ~300ms |

## Further Reading

- [Tauri Platform Guide](../tauri-ref/docs/PLATFORMS.md)
- [Tauri Android Setup](../tauri-ref/docs/ANDROID_DEVICE.md)
- [WebATUI WASM Setup](../webatui-ref/docs/WASM_TESTING_SETUP.md)
- [WebATUI Apple Silicon](../webatui-ref/docs/APPLE_SILICON_SETUP.md)
