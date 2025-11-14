# Grimware - Reference Implementation Library

A curated collection of reference implementations for the Raibid Labs organization, demonstrating best practices, architectural patterns, and development workflows across multiple technology stacks.

## 📚 Reference Implementations

### 1. [Bevy MCP](./bevy-mcp-ref/)
**AI-Assisted Game Development with Bevy Remote Protocol**

Demonstrates real-time game development with AI assistance using Bevy game engine and the Bevy Remote Protocol (BRP) MCP server.

- **Technology**: Bevy 0.16+, Rust, MCP/BRP
- **Key Features**: Live entity manipulation, AI-controlled development, real-time debugging
- **Platform**: Desktop (macOS, Linux, Windows)
- **Documentation**: [docs/bevy-mcp.md](./docs/bevy-mcp.md)
- **Quick Start**: [bevy-mcp-ref/README.md](./bevy-mcp-ref/README.md)

**Use Cases**: Game prototyping, AI-assisted development, live debugging, rapid iteration

---

### 2. [Bevy MCP Ratatui](./bevy-mcp-ratatui-ref/)
**3D Game Development in Your Terminal**

Combines Bevy game engine with terminal UI rendering, enabling AI-controlled 3D game development with visual feedback directly in the terminal.

- **Technology**: Bevy 0.16+, Ratatui, BRP, MCP
- **Key Features**: Terminal 3D rendering, AI prompt visualization, headless development
- **Platform**: Terminal (macOS, Linux, Windows)
- **Documentation**: [docs/bevy-mcp-ratatui.md](./docs/bevy-mcp-ratatui.md)
- **Quick Start**: [bevy-mcp-ratatui-ref/README.md](./bevy-mcp-ratatui-ref/README.md)

**Use Cases**: Terminal-based game dev, CI/CD visualization, headless testing, ASCII art generation

---

### 3. [Tauri Cross-Platform](./tauri-ref/)
**Multi-Platform Desktop and Mobile Applications**

Production-ready Tauri v2 application demonstrating cross-platform development for desktop and mobile from a single codebase.

- **Technology**: Tauri v2, Rust, JavaScript, Vite
- **Key Features**: Single codebase, native performance, small binaries (2-12MB)
- **Platform**: macOS (M3 ARM64), Android, Linux (NVIDIA DGX-Spark)
- **Documentation**: [docs/tauri.md](./docs/tauri.md)
- **Quick Start**: [tauri-ref/README.md](./tauri-ref/README.md)

**Use Cases**: Cross-platform apps, mobile-first development, small footprint applications

---

### 4. [WebATUI](./webatui-ref/)
**Universal Terminal UI Library**

Terminal UI library that works seamlessly in both native terminals and web browsers via WebAssembly.

- **Technology**: Rust, Ratatui, Yew, WASM
- **Key Features**: Dual-target (terminal + browser), component system, state management
- **Platform**: Native terminals + Web browsers
- **Documentation**: [docs/webatui.md](./docs/webatui.md)
- **Quick Start**: [webatui-ref/README.md](./webatui-ref/README.md)

**Use Cases**: Terminal applications, web-based TUIs, cross-platform CLIs, educational tools

---

### 5. [Bevy WASM F#](./bevy-wasm-fsharp-ref/)
**F# → Rust → Bevy → WASM Game Development Path**

Reference implementation demonstrating F# game logic transpiled to Rust and deployed as Bevy games (native + WASM).

- **Technology**: F#, Rust, Bevy 0.16+, Fable, fsrs, WASM
- **Key Features**: Functional game logic, type-safe domain modeling, dual-target deployment
- **Platform**: Desktop (native) + Web browsers (WASM)
- **Documentation**: [docs/bevy-wasm-fsharp.md](./docs/bevy-wasm-fsharp.md)
- **Quick Start**: [bevy-wasm-fsharp-ref/README.md](./bevy-wasm-fsharp-ref/README.md)

**Use Cases**: F# game development, functional game logic, WASM games, type-safe game systems

---

## 🚀 Getting Started

### Prerequisites

All reference implementations require:
- **Rust** (latest stable) - [Install here](https://www.rust-lang.org/tools/install)
- **Git** - For cloning repositories

Additional requirements vary by project - see individual documentation.

### Quick Navigation

```bash
# Clone the repository
git clone https://github.com/raibid-labs/grimware.git
cd grimware

# Explore a specific reference implementation
cd bevy-mcp-ref          # Game development with AI
cd bevy-mcp-ratatui-ref  # Terminal 3D rendering
cd tauri-ref             # Cross-platform apps
cd webatui-ref           # Terminal UI library
cd bevy-wasm-fsharp-ref  # F# → Rust → Bevy → WASM
```

## 📖 Documentation

### Consolidated Guides

- **[Getting Started](./docs/getting-started.md)** - Universal setup guide for all implementations
- **[Architecture Overview](./docs/architecture.md)** - High-level architecture patterns
- **[Development Workflows](./docs/development.md)** - Common development patterns and best practices
- **[Platform Support](./docs/platforms.md)** - Platform-specific guides and requirements

### Reference Implementation Docs

- **[Bevy MCP Guide](./docs/bevy-mcp.md)** - AI-assisted game development
- **[Bevy MCP Ratatui Guide](./docs/bevy-mcp-ratatui.md)** - Terminal 3D rendering
- **[Tauri Guide](./docs/tauri.md)** - Cross-platform application development
- **[WebATUI Guide](./docs/webatui.md)** - Terminal UI library usage
- **[Bevy WASM F# Guide](./docs/bevy-wasm-fsharp.md)** - F# to Rust game development

### Specialized Topics

- **[MCP Integration](./docs/mcp-integration.md)** - Model Context Protocol patterns
- **[BRP Usage](./docs/brp-usage.md)** - Bevy Remote Protocol best practices
- **[Terminal Rendering](./docs/terminal-rendering.md)** - Terminal UI patterns and optimization
- **[Cross-Platform Development](./docs/cross-platform.md)** - Multi-platform strategies

## 🛠️ Technology Stack

### Languages
- **Rust** - Primary language for all implementations
- **F#** - Game logic (bevy-wasm-fsharp-ref)
- **JavaScript/TypeScript** - Frontend (Tauri, WebATUI)

### Frameworks & Libraries
- **Bevy** - Game engine (0.16+)
- **Tauri** - Desktop/mobile framework (v2)
- **Ratatui** - Terminal UI framework
- **Yew** - Rust web framework

### Protocols & APIs
- **MCP** - Model Context Protocol (AI integration)
- **BRP** - Bevy Remote Protocol (live game manipulation)
- **IPC** - Inter-process communication (Tauri)

## 🎯 Use Case Matrix

| Use Case | Best Implementation |
|----------|-------------------|
| AI-assisted game development | [Bevy MCP](./bevy-mcp-ref/) |
| Terminal-based visualization | [Bevy MCP Ratatui](./bevy-mcp-ratatui-ref/) |
| Cross-platform desktop app | [Tauri](./tauri-ref/) |
| Mobile application | [Tauri](./tauri-ref/) |
| Terminal UI application | [WebATUI](./webatui-ref/) |
| Browser-based TUI | [WebATUI](./webatui-ref/) |
| Headless game testing | [Bevy MCP Ratatui](./bevy-mcp-ratatui-ref/) |
| Live code editing | [Bevy MCP](./bevy-mcp-ref/) |
| F# game development | [Bevy WASM F#](./bevy-wasm-fsharp-ref/) |
| WASM game deployment | [Bevy WASM F#](./bevy-wasm-fsharp-ref/) |
| Functional game logic | [Bevy WASM F#](./bevy-wasm-fsharp-ref/) |

## 🤝 Contributing

These are reference implementations maintained by Raibid Labs. Contributions welcome:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure documentation is updated
5. Submit a pull request

See individual projects for specific contribution guidelines.

## 📄 License

Each reference implementation may have its own license. Check individual project directories for details.

Most projects use MIT or MIT/Apache-2.0 dual licensing.

## 🔗 Resources

### Official Documentation
- [Bevy Engine](https://bevyengine.org/)
- [Tauri Framework](https://tauri.app/)
- [Ratatui](https://ratatui.rs/)
- [Claude Code](https://docs.claude.com/en/docs/claude-code)

### Raibid Labs
- [GitHub Organization](https://github.com/raibid-labs)
- [Website](https://raibid-labs.com) *(if available)*

## 🎓 Learning Path

**For Beginners:**
1. Start with [WebATUI](./webatui-ref/) - Learn TUI basics
2. Try [Tauri](./tauri-ref/) - Understand cross-platform development
3. Explore [Bevy MCP](./bevy-mcp-ref/) - Game development fundamentals

**For Advanced Users:**
1. Deep dive into [Bevy MCP Ratatui](./bevy-mcp-ratatui-ref/) - Complex integrations
2. Study architecture docs for pattern recognition
3. Customize implementations for your use cases

---

**Built with ❤️ by [Raibid Labs](https://github.com/raibid-labs)**

*Reference implementations for modern Rust development*
