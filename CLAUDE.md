# CLAUDE.md - AI Assistant Configuration for Grimware

## Repository Overview

**Grimware** is a reference implementation library containing four distinct Rust-based projects that demonstrate best practices for modern development:

1. **bevy-mcp-ref** - AI-assisted game development with Bevy + BRP
2. **bevy-mcp-ratatui-ref** - 3D game development in terminal with AI control
3. **tauri-ref** - Cross-platform desktop/mobile applications
4. **webatui-ref** - Terminal UI library (native + WASM)

## Project Structure

```
grimware/
├── bevy-mcp-ref/          # AI game development
├── bevy-mcp-ratatui-ref/  # Terminal 3D rendering
├── tauri-ref/             # Cross-platform apps
├── webatui-ref/           # Terminal UI library
├── docs/                  # Consolidated documentation
│   ├── getting-started.md
│   ├── bevy-mcp.md
│   ├── bevy-mcp-ratatui.md
│   ├── tauri.md
│   ├── webatui.md
│   ├── mcp-integration.md
│   ├── platforms.md
│   └── ...
├── README.md              # Table of contents / index
└── CLAUDE.md              # This file
```

## Working with This Repository

### Navigation

Each subdirectory is a **complete, independent project** with its own:
- README.md - Project-specific overview
- CLAUDE.md - Project-specific AI configuration
- docs/ - Project-specific documentation
- Cargo.toml / package.json - Dependencies
- src/ - Source code

### Documentation Structure

**Root Documentation** (`docs/`):
- Consolidated guides covering multiple projects
- Cross-cutting concerns (platforms, MCP, architecture)
- Getting started guides for the entire repository

**Project Documentation** (e.g., `bevy-mcp-ref/docs/`):
- Project-specific implementation details
- API references and examples
- Architecture and design documents

**Rule**: Always check both root docs and project-specific docs.

## Common Commands by Project

### Bevy MCP (`bevy-mcp-ref/`)

```bash
# Run with BRP for AI assistance
cargo run --features brp

# Run interactive demo
cargo run --example brp_demo --features brp

# Quick commands (if just installed)
just demo
just watch-demo
just check-all
```

**MCP Tools**: Prefixed with `mcp__brp__*`
- Entity management, component operations, resource access
- See `docs/mcp-integration.md` for complete list

### Bevy MCP Ratatui (`bevy-mcp-ratatui-ref/`)

```bash
# Basic TUI rendering (window + terminal ASCII)
cargo run --example tui_basic --features tui

# With BRP for AI control
cargo run --example tui_brp --features full

# Exit: Ctrl+C or close window
```

**Custom BRP Methods**: `bevy/spawn_cube`, `bevy/spawn_sphere`
- Solves asset handle serialization limitation
- See `docs/bevy-mcp-ratatui.md` for usage

### Tauri (`tauri-ref/`)

```bash
# Desktop development (hot reload)
npm run tauri:dev

# Android development
npm run tauri:android

# Build for production
npm run tauri:build
```

**IPC Pattern**: Frontend ↔ Rust via `invoke()`
- Commands defined in `src-tauri/src/commands.rs`
- Register in both `main.rs` and `mobile.rs`

### WebATUI (`webatui-ref/`)

```bash
# Run example (with just)
just example basic
just example dashboard

# Or with cargo
cargo run --example basic --features terminal

# Watch mode with bacon
bacon example-basic
```

**This is a library** - no binary, only examples.

## AI-Assisted Development Patterns

### Pattern 1: Inspecting Running Applications

For Bevy projects with BRP:

```
User: "What entities are in the scene?"

AI:
1. Use mcp__brp__bevy_query to list all entities
2. Parse and format results
3. Present in readable format
```

### Pattern 2: Live Code Modification

```
User: "Move the player cube up by 5 units"

AI:
1. Query for entity named "Player"
2. Get current Transform component
3. Use bevy_mutate_component to adjust translation.y
4. Confirm change visually (in Ratatui projects)
```

### Pattern 3: Code Generation with Testing

```
User: "Add a new Tauri command to get system info"

AI:
1. Read existing commands in src-tauri/src/commands.rs
2. Write new command following pattern
3. Register in main.rs and mobile.rs
4. Show frontend usage example
5. Suggest testing approach
```

## Best Practices for AI Assistants

### File Organization

**NEVER save to repository root** unless it's a README, CLAUDE.md, or LICENSE file.

Organize by project:
- Code → `<project>/src/`
- Tests → `<project>/tests/`
- Examples → `<project>/examples/`
- Docs → `<project>/docs/` (project-specific) or `docs/` (cross-project)

### Reading Before Writing

**Always read existing files** before modifying:
1. Understand current patterns
2. Match coding style
3. Avoid breaking changes
4. Maintain consistency

### Documentation Updates

When modifying code:
1. Update relevant README.md
2. Update CLAUDE.md if workflow changes
3. Update docs/ files if architecture changes
4. Keep examples in sync with code

### Testing Strategy

For each project:
1. Run tests: `cargo test --all-features`
2. Check compilation: `cargo check --all-features`
3. Run examples: Verify they work
4. Format: `cargo fmt`
5. Lint: `cargo clippy --all-features`

## Common Patterns by Technology

### Bevy ECS Patterns

```rust
// Always name entities
commands.spawn((
    Transform::default(),
    Name::new("Player Character"),
));

// Register custom components
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Health { value: f32 }

app.register_type::<Health>();
```

### Tauri IPC Patterns

```rust
// Backend command
#[tauri::command]
fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Processed: {}", param))
}

// Register in both entry points
invoke_handler![commands::my_command]
```

```javascript
// Frontend usage
import { invoke } from '@tauri-apps/api/core'

const result = await invoke('my_command', { param: 'value' })
```

### Ratatui Component Patterns

```rust
pub struct MyWidget {
    title: String,
}

impl MyWidget {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);
        f.render_widget(block, area);
    }
}
```

## Project Selection Guide

Help users choose the right project:

**"I want to build a game with AI assistance"**
→ Start with `bevy-mcp-ref`

**"I want terminal-based 3D visualization"**
→ Use `bevy-mcp-ratatui-ref`

**"I need a cross-platform desktop/mobile app"**
→ Use `tauri-ref`

**"I'm building a terminal UI application"**
→ Use `webatui-ref`

**"I want to visualize data in the terminal"**
→ Use `webatui-ref` (examples include dashboard)

## Important Notes

### Multi-Project Repository

- Each project is **independent** - can be used separately
- Projects share common patterns (Rust, similar build tools)
- Documentation is split: root docs (cross-cutting) + project docs (specific)

### Feature Flags

Most projects use feature flags extensively:
- Check `Cargo.toml` `[features]` section
- Use appropriate flags: `cargo run --features <flags>`
- Common pattern: `default`, `full`, project-specific features

### Platform Considerations

- **Bevy projects**: Desktop only (macOS, Linux, Windows)
- **Tauri**: Desktop + Android (+ iOS experimental)
- **WebATUI**: Terminal (native) + Web (WASM)

### MCP/BRP Availability

- Only Bevy projects have MCP/BRP integration
- BRP listens on `localhost:15702` by default
- Custom methods available in `bevy-mcp-ratatui-ref`

## Troubleshooting

### "Cargo build is slow"

For Bevy projects, use dynamic linking in dev:
```bash
cargo run --features bevy/dynamic_linking
```

### "Can't find MCP tools"

Ensure:
1. Application is running with BRP enabled (`--features brp`)
2. Check port 15702 is not blocked: `lsof -i :15702`
3. Tools are prefixed: `mcp__brp__*`

### "Cross-platform build fails"

Check platform-specific requirements:
- See `docs/platforms.md`
- See project-specific docs (e.g., `tauri-ref/docs/SETUP.md`)
- Ensure all system dependencies installed

### "Tests are failing"

```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Check specific feature combination
cargo test --features <flags>
```

## Quick Reference Links

**Root Documentation**:
- [Getting Started](docs/getting-started.md)
- [MCP Integration](docs/mcp-integration.md)
- [Platform Support](docs/platforms.md)

**Project Documentation**:
- [Bevy MCP](docs/bevy-mcp.md)
- [Bevy MCP Ratatui](docs/bevy-mcp-ratatui.md)
- [Tauri](docs/tauri.md)
- [WebATUI](docs/webatui.md)

**Project READMEs**:
- [bevy-mcp-ref/README.md](bevy-mcp-ref/README.md)
- [bevy-mcp-ratatui-ref/README.md](bevy-mcp-ratatui-ref/README.md)
- [tauri-ref/README.md](tauri-ref/README.md)
- [webatui-ref/README.md](webatui-ref/README.md)

**Project CLAUDE.md files** (for project-specific details):
- [bevy-mcp-ref/CLAUDE.md](bevy-mcp-ref/CLAUDE.md)
- [bevy-mcp-ratatui-ref/CLAUDE.md](bevy-mcp-ratatui-ref/CLAUDE.md)
- [tauri-ref/CLAUDE.md](tauri-ref/CLAUDE.md)

## Repository Maintenance

### When Adding New Projects

1. Create project directory with standard structure
2. Add README.md and CLAUDE.md
3. Update root README.md with new entry
4. Add consolidated docs to `docs/`
5. Update this CLAUDE.md

### When Updating Documentation

1. Update project-specific docs in `<project>/docs/`
2. Update consolidated docs in `docs/` if cross-cutting
3. Update root README.md if structure changes
4. Update CLAUDE.md files for workflow changes

---

**This is a reference implementation library. Each project demonstrates best practices for its domain while maintaining consistency across the repository.**