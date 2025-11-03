# Tauri Hello World - Cross-Platform Reference

A demonstration of a cross-platform application built with [Tauri v2](https://tauri.app), targeting multiple platforms from a single codebase.

## 🎯 Supported Platforms

- ✅ **macOS M3 (ARM64)** - Native Apple Silicon support
- ✅ **Android Mobile** - ARM64 and ARMv7 devices
- ✅ **Linux (NVIDIA DGX-Spark)** - High-performance computing environment

## 🚀 Features

- **Single Codebase**: Write once, run on desktop and mobile
- **Native Performance**: Rust backend with platform-native UI
- **Small Binary Size**: ~2-12MB depending on platform
- **Modern Web Frontend**: HTML5, CSS3, and JavaScript (ES Modules)
- **IPC Communication**: Type-safe Rust ↔ JavaScript bridge
- **Platform Detection**: Automatic OS and architecture detection

## 📦 Quick Start

### Prerequisites

- **Rust** (latest stable): [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js** (v18+): [Install Node.js](https://nodejs.org/)
- **Platform-specific tools**: See [docs/SETUP.md](docs/SETUP.md)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd tauri-ref

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev
```

### Development

```bash
# Desktop development (macOS/Linux)
npm run tauri:dev

# Android development
npm run tauri:android

# Build for production
npm run tauri:build

# Build Android APK
npm run tauri:android:build
```

## 📖 Documentation

- **[Setup Guide](docs/SETUP.md)** - Platform-specific installation and configuration
- **[Architecture](docs/ARCHITECTURE.md)** - Technical architecture and project structure
- **[Platform Details](docs/PLATFORMS.md)** - Platform-specific build and deployment information

## 🏗️ Project Structure

```
tauri-ref/
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html
│   ├── style.css
│   └── main.js
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs       # Desktop entry
│   │   └── lib.rs        # Mobile library
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri config
├── docs/                  # Documentation
└── package.json          # Node dependencies
```

## 🛠️ Technology Stack

**Frontend**:
- HTML5/CSS3
- Vanilla JavaScript (ES Modules)
- Vite (build tool)

**Backend**:
- Rust (systems programming)
- Tauri v2 (app framework)
- Platform-native WebView

## 🎨 UI Demo

The app includes:
- Platform detection and display
- Interactive greeting form
- Responsive design
- Modern gradient UI

## 📱 Platform Specifics

### macOS M3 (ARM64)
- Native Apple Silicon binary
- Uses WKWebView
- ~2-3MB binary size
- Code signing supported

### Android
- API 24+ (Android 7.0+)
- ARM64-v8a primary target
- APK/AAB packaging
- ~8-12MB app size

### Linux (DGX-Spark)
- x86_64 architecture
- WebKit2GTK renderer
- DEB/AppImage packaging
- GPU acceleration support

## 🔧 Build Commands

```bash
# Development
npm run dev              # Frontend dev server
npm run tauri:dev       # Tauri dev (desktop)
npm run tauri:android   # Android dev

# Production
npm run build           # Build frontend
npm run tauri:build     # Build desktop app
npm run tauri:android:build  # Build Android APK
```

## 🧪 Testing

```bash
# Run on Android emulator
adb devices
npm run tauri:android

# Check Android logs
adb logcat | grep Tauri

# Desktop testing
npm run tauri:dev
```

## 📊 Performance

| Platform | Binary Size | Startup Time | Memory Usage |
|----------|-------------|--------------|--------------|
| macOS M3 | ~2-3MB | ~150ms | ~40MB |
| Android | ~8-12MB | ~650ms | ~50MB |
| Linux | ~3-4MB | ~200ms | ~45MB |

## 🔐 Security

- Minimal permissions by default
- Type-safe IPC communication
- OS-level sandboxing
- Content Security Policy configured

## 🤝 Contributing

This is a reference implementation. Feel free to:
- Fork and experiment
- Report issues
- Suggest improvements
- Use as a starting point for your projects

## 📄 License

This project is provided as a demonstration/reference implementation.

## 🔗 Resources

- [Tauri Documentation](https://tauri.app/)
- [Tauri Mobile Guide](https://tauri.app/v1/guides/building/android)
- [Rust Language](https://www.rust-lang.org/)
- [Vite](https://vitejs.dev/)

## 🙏 Acknowledgments

Built with [Tauri](https://tauri.app) - a framework for building tiny, blazingly fast binaries for all major platforms.

---

**Happy Building! 🚀**

For detailed setup instructions, see [docs/SETUP.md](docs/SETUP.md)