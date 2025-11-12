# Tauri Hello World - Setup Guide

## Prerequisites

### All Platforms

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18 or later)
   ```bash
   # Install via nvm or from nodejs.org
   ```

### macOS M3 (ARM64)

1. **Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Homebrew** (optional but recommended)
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

### Android

1. **Android Studio** or **Android SDK**
   - Download from: https://developer.android.com/studio

2. **Android NDK**
   ```bash
   # Install via Android Studio SDK Manager
   # Or use sdkmanager:
   sdkmanager --install "ndk;25.1.8937393"
   ```

3. **Java Development Kit (JDK 17+)**
   ```bash
   # macOS
   brew install openjdk@17

   # Linux
   sudo apt install openjdk-17-jdk
   ```

4. **Configure Environment Variables**
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk
   export NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   ```

5. **Add Android Rust Targets**
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

6. **Initialize Tauri Android**
   ```bash
   npm install
   npm run tauri android init
   ```

### Linux (NVIDIA DGX-Spark)

1. **System Dependencies**
   ```bash
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
     librsvg2-dev
   ```

2. **NVIDIA GPU Support** (Optional)
   - The application will run on DGX-Spark's Linux OS
   - GPU acceleration is handled by the system's WebKit renderer

## Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd tauri-ref
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build Rust dependencies**
   ```bash
   cd src-tauri
   cargo build
   cd ..
   ```

## Development

### Desktop (macOS M3 / Linux)

```bash
# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### Android

```bash
# Run on Android emulator/device
npm run tauri:android

# Build Android APK
npm run tauri:android:build
```

## Building

### macOS M3

```bash
npm run tauri:build
```

Output: `src-tauri/target/release/bundle/macos/`

### Linux (DGX-Spark)

```bash
npm run tauri:build
```

Output: `src-tauri/target/release/bundle/deb/` or `appimage/`

### Android

```bash
npm run tauri:android:build
```

Output: `src-tauri/gen/android/app/build/outputs/apk/`

## Platform-Specific Notes

### macOS M3 (ARM64)
- Native ARM64 support is automatic
- Code signing may be required for distribution
- Notarization needed for App Store or Gatekeeper

### Android
- Minimum SDK: API 24 (Android 7.0)
- Target SDK: API 34 (Android 14)
- Supports ARM64, ARMv7, x86, and x86_64

### Linux (NVIDIA DGX-Spark)
- Built on Ubuntu-based OS
- Hardware acceleration via WebKitGTK
- GPU support through system drivers

## Troubleshooting

### macOS
- If build fails, ensure Xcode Command Line Tools are up to date
- Check Rust toolchain: `rustup update`

### Android
- Ensure ANDROID_HOME and NDK_HOME are set correctly
- Check USB debugging is enabled on device
- Verify NDK version matches Cargo.toml requirements

### Linux
- If webkit2gtk is missing, install system dependencies
- For GPU issues, check NVIDIA driver installation
- Verify OpenGL/Vulkan support

## Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Tauri Mobile Guide](https://tauri.app/v1/guides/building/android)
- [Rust Installation](https://www.rust-lang.org/tools/install)
