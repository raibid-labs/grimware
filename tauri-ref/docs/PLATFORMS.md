# Platform-Specific Information

## macOS M3 (ARM64)

### Overview
- **Architecture**: ARM64 (Apple Silicon)
- **Minimum OS**: macOS 10.13 High Sierra
- **Recommended**: macOS 13+ (Ventura or later)
- **Chip**: Apple M3 (3nm process)

### Build Configuration

```toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]
```

### Development

```bash
# Run development server
npm run tauri:dev

# Build for macOS
npm run tauri:build
```

### Output Artifacts
- **DMG**: `src-tauri/target/release/bundle/dmg/Tauri Hello World_0.1.0_aarch64.dmg`
- **App Bundle**: `src-tauri/target/release/bundle/macos/Tauri Hello World.app`

### Code Signing

For distribution, you'll need an Apple Developer account:

```bash
# Sign the app
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" "Tauri Hello World.app"

# Notarize for Gatekeeper
xcrun notarytool submit "Tauri Hello World.dmg" --apple-id "email" --password "app-password" --team-id "TEAM_ID"
```

### Performance
- **Startup Time**: ~100-200ms
- **Memory Usage**: ~30-50MB base
- **Binary Size**: ~2-3MB (stripped)

---

## Android

### Overview
- **Minimum SDK**: API 24 (Android 7.0 Nougat)
- **Target SDK**: API 34 (Android 14)
- **Architectures**: ARM64-v8a, ARMv7, x86, x86_64

### Prerequisites

1. Install Android targets:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

2. Set environment variables:
```bash
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393
```

3. Initialize Android project:
```bash
npm run tauri android init
```

### Development

```bash
# Run on emulator/device
npm run tauri:android

# Build APK
npm run tauri:android:build

# Build for release (signed)
npm run tauri android build -- --release
```

### Output Artifacts
- **Debug APK**: `src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk`
- **Release APK**: `src-tauri/gen/android/app/build/outputs/apk/release/app-release.apk`
- **AAB** (for Play Store): `src-tauri/gen/android/app/build/outputs/bundle/release/app-release.aab`

### Configuration

Edit `src-tauri/gen/android/app/build.gradle.kts`:

```kotlin
android {
    namespace = "com.raibid.tauri.hello"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.raibid.tauri.hello"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0"
    }
}
```

### Permissions

Android-specific permissions in `AndroidManifest.xml`:
- `INTERNET` - Network access
- `READ_EXTERNAL_STORAGE` - File access (if needed)
- `WRITE_EXTERNAL_STORAGE` - File writing (if needed)

### Performance
- **APK Size**: ~8-12MB (ARM64, release)
- **Startup Time**: ~500-800ms
- **Memory Usage**: ~40-60MB

### Testing

```bash
# List connected devices
adb devices

# Install APK
adb install src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk

# View logs
adb logcat | grep Tauri
```

---

## Linux (NVIDIA DGX-Spark)

### Overview
- **OS**: Ubuntu 22.04 LTS (typical for DGX systems)
- **Architecture**: x86_64
- **GPU**: NVIDIA A100 or H100 (Hopper/Ampere)
- **CUDA**: 12.x

### System Requirements

```bash
# Install system dependencies
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf
```

### Development

```bash
# Run development server
npm run tauri:dev

# Build for Linux
npm run tauri:build
```

### Output Artifacts
- **DEB Package**: `src-tauri/target/release/bundle/deb/tauri-hello-world_0.1.0_amd64.deb`
- **AppImage**: `src-tauri/target/release/bundle/appimage/tauri-hello-world_0.1.0_amd64.AppImage`
- **Binary**: `src-tauri/target/release/tauri-hello-world`

### Installation

```bash
# DEB package
sudo dpkg -i tauri-hello-world_0.1.0_amd64.deb

# AppImage (portable)
chmod +x tauri-hello-world_0.1.0_amd64.AppImage
./tauri-hello-world_0.1.0_amd64.AppImage

# Binary
./tauri-hello-world
```

### GPU Acceleration

The WebKitGTK renderer can use GPU acceleration:

```bash
# Check GPU support
glxinfo | grep "OpenGL version"

# Enable GPU acceleration (if needed)
export WEBKIT_DISABLE_COMPOSITING_MODE=0
```

### DGX-Specific Considerations

1. **Display Server**: May run headless or with X11/Wayland
2. **Remote Access**: Use X11 forwarding or VNC
3. **Resource Usage**: Minimal compared to compute workloads

```bash
# X11 forwarding
ssh -X user@dgx-spark
./tauri-hello-world

# Or use VNC
vncserver :1
export DISPLAY=:1
./tauri-hello-world
```

### Performance
- **Binary Size**: ~3-4MB (stripped)
- **Startup Time**: ~100-300ms
- **Memory Usage**: ~35-55MB
- **GPU Usage**: Minimal (WebKit rendering only)

### Service Deployment

For running as a service on DGX:

```bash
# Create systemd service
sudo nano /etc/systemd/system/tauri-hello.service
```

```ini
[Unit]
Description=Tauri Hello World
After=network.target

[Service]
Type=simple
User=your-user
Environment="DISPLAY=:0"
ExecStart=/opt/tauri-hello-world/tauri-hello-world
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable tauri-hello.service
sudo systemctl start tauri-hello.service
```

---

## Comparison Matrix

| Feature | macOS M3 | Android | Linux (DGX) |
|---------|----------|---------|-------------|
| Architecture | ARM64 | ARM64/ARMv7 | x86_64 |
| WebView | WKWebView | System WebView | WebKit2GTK |
| Binary Size | ~2-3MB | ~8-12MB | ~3-4MB |
| Startup Time | ~150ms | ~650ms | ~200ms |
| Memory Base | ~40MB | ~50MB | ~45MB |
| GPU Support | Metal | OpenGL ES | OpenGL/Vulkan |
| Package Format | .dmg/.app | .apk/.aab | .deb/.AppImage |
| Auto-Update | ✅ | ✅ (via stores) | ✅ |
| Distribution | App Store/Direct | Play Store/APK | Package repos/Direct |
