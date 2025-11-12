# AI -> Bevy MCP -> TUI Rendering System - Usage Examples

## Table of Contents

1. [Quick Start Guide](#quick-start-guide)
2. [AI Prompt Examples](#ai-prompt-examples)
3. [MCP Tool Usage](#mcp-tool-usage)
4. [Interactive Workflows](#interactive-workflows)
5. [Advanced Use Cases](#advanced-use-cases)
6. [Troubleshooting](#troubleshooting)

---

## System Overview

This reference implementation demonstrates a powerful pipeline that lets AI control 3D Bevy scenes rendered directly to the terminal via MCP (Model Context Protocol). The system combines:

- **Bevy 0.16+** - Modern ECS game engine
- **BRP (Bevy Remote Protocol)** - Live entity manipulation via JSON-RPC
- **bevy_ratatui_camera** - TUI rendering with multiple strategies (ASCII art, edge detection, depth buffering)
- **Claude Code AI** - Intelligent scene manipulation via MCP tools

### Architecture Flow

```
┌─────────────────┐         ┌──────────────┐         ┌─────────────────┐
│   Claude Code   │ ◄─────► │  BRP MCP     │ ◄─────► │  Bevy Game      │
│   (AI Agent)    │  MCP    │  Server      │  HTTP   │  (Port 15702)   │
└─────────────────┘         └──────────────┘         └─────────────────┘
                                                               │
                                                               ▼
                                                      ┌─────────────────┐
                                                      │ bevy_ratatui    │
                                                      │ Camera Plugin   │
                                                      └─────────────────┘
                                                               │
                                                               ▼
                                                      ┌─────────────────┐
                                                      │ Terminal (TUI)  │
                                                      │ ASCII/Unicode   │
                                                      └─────────────────┘
```

---

## Quick Start Guide

### Prerequisites

```bash
# Install Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Claude Code CLI
# Follow instructions at: https://docs.claude.com/en/docs/claude-code

# Verify installations
rustc --version
claude --version
```

### Installation Steps

#### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/your-username/bevy-mcp-ratatui-ref.git
cd bevy-mcp-ratatui-ref

# Add bevy_ratatui_camera dependency to Cargo.toml
[dependencies]
bevy = { version = "0.16", features = ["dynamic_linking"] }
bevy_brp_extras = { version = "0.1", optional = true }
bevy_ratatui_camera = "0.3"  # Terminal rendering
ratatui = "0.29"              # TUI framework

[features]
default = []
brp = ["bevy/bevy_remote", "bevy_brp_extras"]
tui = ["bevy_ratatui_camera"]
```

#### 2. Running the TUI Demo

```bash
# Run with both BRP and TUI features enabled
cargo run --features brp,tui

# OR use with just installed
just tui-demo

# Expected output:
# 🎮 BRP enabled on port 15702
# 🖥️  TUI rendering active
# 🤖 MCP tools ready for interaction
```

#### 3. First AI Interaction

Open Claude Code and try:

```
"Show me all entities in the TUI scene"
```

Claude will execute:
```javascript
mcp__brp__bevy_query({
  data: {
    components: ["bevy_core::name::Name"]
  },
  filter: {}
})
```

---

## AI Prompt Examples

### Basic Scene Manipulation

#### Example 1: Spawn a Red Cube in TUI

**User Prompt:**
```
"Spawn a red cube at position (0, 5, 0) in the TUI"
```

**AI Execution:**
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 5.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_pbr::pbr_material::StandardMaterial": {
      "base_color": [1.0, 0.0, 0.0, 1.0]  // Red
    },
    "bevy_core::name::Name": "TUI Red Cube"
  }
})
```

**Expected TUI Output:**
```
╔════════════════════════════╗
║         ┌──────┐           ║
║         │▓▓▓▓▓▓│           ║
║         │▓▓▓▓▓▓│ ← Red     ║
║         └──────┘   Cube    ║
║                            ║
╚════════════════════════════╝
```

---

#### Example 2: Change Rendering Strategy

**User Prompt:**
```
"Change the rendering strategy to edge detection"
```

**AI Execution:**
```javascript
// Query for RatatuiCamera components
mcp__brp__bevy_query({
  data: {
    components: ["bevy_ratatui_camera::RatatuiCamera"]
  },
  filter: {
    with: ["bevy_ratatui_camera::RatatuiCamera"]
  }
})

// Mutate the camera strategy
mcp__brp__bevy_mutate_component({
  entity: 42,  // From query results
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".strategy",
  value: "EdgeDetection"
})
```

**Expected Change:**
```
Before (ASCII):              After (Edge Detection):
╔══════════════╗            ╔══════════════╗
║  ████████    ║            ║  ┌──────┐    ║
║  ████████    ║            ║  │      │    ║
║  ████████    ║     →      ║  │      │    ║
║              ║            ║  └──────┘    ║
╚══════════════╝            ╚══════════════╝
```

---

#### Example 3: Create a Rotating Pyramid

**User Prompt:**
```
"Create a rotating pyramid and render it with ASCII art"
```

**AI Execution:**
```rust
// AI generates Rust code for custom mesh
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

fn spawn_pyramid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let vertices = vec![
        // Base vertices
        [-0.5, 0.0, -0.5], [0.5, 0.0, -0.5],
        [0.5, 0.0, 0.5],   [-0.5, 0.0, 0.5],
        // Apex
        [0.0, 1.0, 0.0],
    ];

    let indices = vec![
        // Base
        0, 2, 1, 0, 3, 2,
        // Sides
        0, 1, 4, 1, 2, 4,
        2, 3, 4, 3, 0, 4,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh.compute_flat_normals();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.8, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingShape { speed: 1.5 },
        Name::new("TUI Pyramid"),
    ));
}

#[derive(Component)]
struct RotatingShape { speed: f32 }

fn rotate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingShape)>) {
    for (mut transform, shape) in &mut query {
        transform.rotate_y(time.delta_secs() * shape.speed);
    }
}
```

**Then spawn via MCP:**
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 1.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "TUI Pyramid"
  }
})
```

**TUI Animation:**
```
Frame 1:       Frame 2:       Frame 3:
   ▲              ◢◣             ╱╲
  ╱ ╲            ◢  ◣          ╱  ╲
 ╱   ╲          ◢    ◣        ╱    ╲
╱─────╲        ◢──────◣      ╱──────╲
```

---

#### Example 4: Make Camera Orbit Scene

**User Prompt:**
```
"Make the camera orbit around the scene"
```

**AI Execution:**
```javascript
// First, find the camera
const cameras = await mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {
    with: ["bevy_render::camera::camera::Camera"]
  }
});

const cameraEntity = cameras[0].entity;

// Add orbit behavior (AI creates system code)
// Then mutate camera continuously in a loop
for (let angle = 0; angle < 360; angle += 5) {
  const rad = angle * Math.PI / 180;
  const radius = 10.0;
  const x = Math.cos(rad) * radius;
  const z = Math.sin(rad) * radius;

  await mcp__brp__bevy_mutate_component({
    entity: cameraEntity,
    component: "bevy_transform::components::transform::Transform",
    path: ".translation",
    value: [x, 5.0, z]
  });

  await sleep(50); // 50ms delay between updates
}
```

**TUI Effect:**
```
Camera orbiting - view changes dynamically:

0°:   ║     [Front]     ║
90°:  ║     [Right]     ║
180°: ║     [Back]      ║
270°: ║     [Left]      ║
```

---

#### Example 5: Add Colored Lighting

**User Prompt:**
```
"Add colored lighting to the TUI scene"
```

**AI Execution:**
```javascript
// Spawn red light
mcp__brp__bevy_spawn({
  components: {
    "bevy_pbr::light::PointLight": {
      "intensity": 3000.0,
      "color": [1.0, 0.0, 0.0, 1.0],  // Red
      "shadows_enabled": true
    },
    "bevy_transform::components::transform::Transform": {
      "translation": [-5.0, 5.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Red Light"
  }
});

// Spawn blue light
mcp__brp__bevy_spawn({
  components: {
    "bevy_pbr::light::PointLight": {
      "intensity": 3000.0,
      "color": [0.0, 0.0, 1.0, 1.0],  // Blue
      "shadows_enabled": true
    },
    "bevy_transform::components::transform::Transform": {
      "translation": [5.0, 5.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Blue Light"
  }
});

// Spawn green light
mcp__brp__bevy_spawn({
  components: {
    "bevy_pbr::light::PointLight": {
      "intensity": 3000.0,
      "color": [0.0, 1.0, 0.0, 1.0],  // Green
      "shadows_enabled": true
    },
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 8.0, 5.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Green Light"
  }
});
```

**TUI Output with Colors:**
```
╔════════════════════════════════╗
║  🔴 Red tint on left side      ║
║  🔵 Blue tint on right side    ║
║  🟢 Green highlight from above ║
║      ╱█████╲                   ║
║     ▕███████▏ ← Multi-colored  ║
║      ╲█████╱    lighting       ║
╚════════════════════════════════╝
```

---

## MCP Tool Usage

### Query Entities Visible in TUI

**Goal:** Find all entities currently being rendered in the TUI viewport.

```javascript
// List all entities with transforms and names
mcp__brp__bevy_query({
  data: {
    components: [
      "bevy_transform::components::transform::Transform",
      "bevy_core::name::Name"
    ]
  },
  filter: {
    with: ["bevy_transform::components::transform::Transform"]
  }
})
```

**Response Example:**
```json
[
  {
    "entity": 4294967301,
    "components": {
      "bevy_core::name::Name": "TUI Red Cube",
      "bevy_transform::components::transform::Transform": {
        "translation": [0.0, 5.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0]
      }
    }
  },
  {
    "entity": 4294967302,
    "components": {
      "bevy_core::name::Name": "Main Light",
      "bevy_transform::components::transform::Transform": {
        "translation": [4.0, 8.0, 4.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0]
      }
    }
  }
]
```

---

### Mutate Transforms and See TUI Update

**Goal:** Move entities and watch the TUI re-render in real-time.

```javascript
// 1. Get current position
const cube = await mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {
    with: ["bevy_core::name::Name"]
  }
});

// 2. Animate movement (create smooth motion)
for (let y = 0; y <= 10; y += 0.5) {
  await mcp__brp__bevy_mutate_component({
    entity: cube[0].entity,
    component: "bevy_transform::components::transform::Transform",
    path: ".translation.y",
    value: y
  });

  // TUI updates automatically on each frame
  await sleep(100); // 100ms between frames
}
```

**TUI Animation Sequence:**
```
Frame 1 (y=0):     Frame 2 (y=2.5):   Frame 3 (y=5.0):
║              ║   ║     ▓▓▓▓▓       ║   ║              ║
║              ║   ║     ▓▓▓▓▓       ║   ║     ▓▓▓▓▓    ║
║     ▓▓▓▓▓    ║   ║                ║   ║     ▓▓▓▓▓    ║
║     ▓▓▓▓▓    ║   ║                ║   ║              ║
```

---

### Spawn Entities Optimized for Terminal Rendering

**Goal:** Create entities that look good in ASCII/TUI format.

```javascript
// High-contrast sphere with edge detection
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 2.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [2.0, 2.0, 2.0]  // Larger scale for better TUI visibility
    },
    "bevy_pbr::pbr_material::StandardMaterial": {
      "base_color": [1.0, 1.0, 1.0, 1.0],  // Pure white for high contrast
      "metallic": 0.0,
      "perceptual_roughness": 1.0  // Matte finish (better for edge detection)
    },
    "bevy_core::name::Name": "TUI Optimized Sphere"
  }
});

// Simple geometric shapes work best
const tuiOptimalShapes = [
  "Cubes",      // ▓▓▓▓ or ████
  "Spheres",    // ●○◉◎
  "Cylinders",  // ║║║║
  "Cones",      // ▲△▴
];
```

**Best Practices for TUI Entities:**

1. **High Contrast Colors**
   - Pure white (#FFFFFF)
   - Pure black (#000000)
   - Saturated colors (Red: #FF0000, Blue: #0000FF)

2. **Appropriate Scales**
   - Minimum scale: 1.5x (for visibility)
   - Recommended: 2.0-3.0x
   - Maximum: 5.0x (beyond this, detail is lost)

3. **Simple Geometry**
   - Prefer primitives (cubes, spheres, cylinders)
   - Avoid complex meshes with fine details
   - Use matte materials (roughness = 1.0)

---

### Control Camera and Rendering Settings

**Goal:** Adjust camera and TUI rendering parameters.

#### Adjust Camera FOV

```javascript
mcp__brp__bevy_mutate_component({
  entity: cameraEntity,
  component: "bevy_render::camera::camera::Camera",
  path: ".projection.fov",
  value: 1.2  // Radians (wider FOV)
})
```

#### Change TUI Rendering Strategy

```javascript
// Available strategies:
// - "DepthBuffer"     - 3D depth-based rendering
// - "EdgeDetection"   - Sobel edge detection
// - "ASCIIArt"        - Brightness-based ASCII
// - "Unicode"         - Full Unicode character set

mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".strategy",
  value: "DepthBuffer"
})
```

#### Adjust TUI Resolution

```javascript
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".resolution",
  value: [80, 24]  // Standard terminal size: 80 cols × 24 rows
})
```

#### Enable/Disable TUI Features

```javascript
// Enable depth buffering for occlusion
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".depth_buffer_enabled",
  value: true
})

// Adjust edge detection sensitivity
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".edge_threshold",
  value: 0.3  // Lower = more edges detected
})
```

---

## Interactive Workflows

### Scene Composition via AI

**Workflow:** Build a complete TUI scene through conversation.

```
User: "Create a forest scene in the TUI"

AI: "I'll create a forest scene with trees, rocks, and grass. Let me start by setting up the environment."

[AI spawns multiple entities via MCP]

User: "Make the trees taller"

AI: [Queries trees, adjusts scale]

User: "Add a campfire in the center"

AI: [Spawns fire with particle effects, adds orange light]

User: "Perfect! Now add a starry sky"

AI: [Spawns skybox with star particles]
```

**Implementation:**

```javascript
// Step 1: Ground plane
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 0.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [50.0, 1.0, 50.0]
    },
    "bevy_pbr::pbr_material::StandardMaterial": {
      "base_color": [0.2, 0.6, 0.2, 1.0]  // Grass green
    },
    "bevy_core::name::Name": "Forest Ground"
  }
});

// Step 2: Spawn 10 trees in random positions
for (let i = 0; i < 10; i++) {
  const x = (Math.random() - 0.5) * 40;
  const z = (Math.random() - 0.5) * 40;

  // Tree trunk
  await mcp__brp__bevy_spawn({
    components: {
      "bevy_transform::components::transform::Transform": {
        "translation": [x, 3.0, z],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [0.5, 6.0, 0.5]  // Tall cylinder
      },
      "bevy_core::name::Name": `Tree Trunk ${i}`
    }
  });

  // Tree foliage
  await mcp__brp__bevy_spawn({
    components: {
      "bevy_transform::components::transform::Transform": {
        "translation": [x, 7.0, z],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [2.5, 2.5, 2.5]
      },
      "bevy_pbr::pbr_material::StandardMaterial": {
        "base_color": [0.1, 0.5, 0.1, 1.0]  // Dark green
      },
      "bevy_core::name::Name": `Tree Foliage ${i}`
    }
  });
}

// Step 3: Campfire (orange point light)
await mcp__brp__bevy_spawn({
  components: {
    "bevy_pbr::light::PointLight": {
      "intensity": 5000.0,
      "color": [1.0, 0.5, 0.0, 1.0],  // Orange
      "shadows_enabled": true
    },
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 1.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Campfire Light"
  }
});
```

**TUI Output:**
```
╔═══════════════════════════════════════════════════╗
║                   ✧ ✧ ✧ ✧  (stars)              ║
║      🌲           🌲              🌲              ║
║  🌲      🌲            🌲                         ║
║             🔥 (campfire)                        ║
║  🌲            🌲         🌲           🌲         ║
║      🌲                        🌲                 ║
║═══════════════════════════════════════════════════║
```

---

### Live Debugging with TUI Visualization

**Workflow:** Debug physics/gameplay issues by visualizing in TUI.

```
User: "My player is falling through the floor. Can you visualize this in TUI?"

AI: [Spawns TUI camera focused on player]
    [Enables edge detection to show collision boundaries]
    [Adds debug markers for physics components]

User: "I see the problem - the collider is offset"

AI: [Adjusts collider position via MCP]
    [Shows real-time fix in TUI]
```

**Debug Visualization Setup:**

```javascript
// 1. Spawn debug camera focused on problem area
mcp__brp__bevy_spawn({
  components: {
    "bevy_render::camera::camera::Camera": {
      "order": 1  // Secondary camera
    },
    "bevy_ratatui_camera::RatatuiCamera": {
      "strategy": "EdgeDetection",
      "resolution": [60, 20]
    },
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 5.0, 10.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Debug TUI Camera"
  }
});

// 2. Add debug visualization components
mcp__brp__bevy_insert({
  entity: playerEntity,
  components: {
    "bevy_debug_draw::DebugLines": {
      "color": [1.0, 0.0, 0.0, 1.0],
      "duration": 9999.0
    }
  }
});

// 3. Watch for collision events
mcp__brp__bevy_get_watch({
  entity: playerEntity,
  components: [
    "bevy_transform::components::transform::Transform",
    "bevy_physics::components::collider::Collider"
  ]
});
```

**TUI Debug View:**
```
╔══════════════════════════════════╗
║ DEBUG: Player Collision          ║
║ ┌────────────────┐               ║
║ │                │ ← Collider    ║
║ │    ┌──────┐    │   (red box)   ║
║ │    │Player│    │               ║
║ │    └──────┘    │               ║
║ └────────────────┘               ║
║ ──────────────────── ← Floor     ║
║ ⚠ OFFSET: Collider 0.5 units     ║
║           below player!          ║
╚══════════════════════════════════╝
```

---

### Iterative Design in Terminal

**Workflow:** Design and refine game levels without leaving the terminal.

```bash
# 1. Start TUI-only mode (no window)
cargo run --features brp,tui --no-default-features

# 2. Use AI to iterate on design
User: "Create a platformer level"
AI: [Spawns platforms in TUI]

User: "Make the third platform higher"
AI: [Adjusts via MCP, TUI updates]

User: "Add more spacing between platforms"
AI: [Redistributes positions]

# 3. Export final design
User: "Save this level as level1.ron"
AI: [Serializes entity data to file]
```

**Level Design Iteration:**

```javascript
// Iteration 1: Basic platforms
const platforms = [
  { x: 0, y: 0, width: 5 },
  { x: 7, y: 3, width: 4 },
  { x: 13, y: 6, width: 3 },
];

// Iteration 2: Adjust based on TUI visualization
const platforms = [
  { x: 0, y: 0, width: 5 },
  { x: 7, y: 4, width: 4 },    // Higher
  { x: 15, y: 7, width: 3 },   // More spacing
];

// Iteration 3: Add collectibles
platforms.forEach((p, i) => {
  mcp__brp__bevy_spawn({
    components: {
      "bevy_transform::components::transform::Transform": {
        "translation": [p.x + p.width/2, p.y + 1, 0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [0.5, 0.5, 0.5]
      },
      "bevy_core::name::Name": `Coin ${i}`
    }
  });
});
```

**TUI Design View:**
```
Iteration 1:           Iteration 2:           Iteration 3:
                                              💰
──────┐                      ────┐           ────┐
      │                          │  💰           │
      │     ────┐                │  ────┐        │
      │         │                │      │ 💰     │
      │         │  ───┐          │      │  ───┐  │
──────┘         │     │          │      │      │  │
                └─────┘          └──────┘      └──┘
```

---

### Performance Monitoring

**Workflow:** Track performance metrics in TUI.

```javascript
// Setup performance monitoring overlay
mcp__brp__bevy_spawn({
  components: {
    "bevy_core::name::Name": "TUI Perf Monitor",
    "custom::PerformanceMonitor": {
      "show_fps": true,
      "show_entity_count": true,
      "show_render_time": true
    }
  }
});

// Query performance data
const time = await mcp__brp__bevy_get_resource({
  resource: "bevy_time::time::Time"
});

const entityCount = await mcp__brp__bevy_query({
  data: { components: [] },
  filter: {}
});

console.log(`FPS: ${1.0 / time.delta_secs}`);
console.log(`Entities: ${entityCount.length}`);
```

**TUI Performance Overlay:**
```
╔═══════════════════════════════════════════════╗
║ 🎮 Game Scene                                 ║
║                                               ║
║      [Your game content here]                 ║
║                                               ║
╠═══════════════════════════════════════════════╣
║ 📊 Performance Monitor                        ║
║ FPS:        60.2                              ║
║ Entities:   247                               ║
║ Render:     12.3ms                            ║
║ TUI Render: 1.8ms                             ║
║ Memory:     124 MB                            ║
╚═══════════════════════════════════════════════╝
```

---

## Advanced Use Cases

### Multi-Camera TUI Layouts

**Goal:** Display multiple TUI views simultaneously (split-screen, PiP, etc.).

```rust
use bevy::prelude::*;
use bevy_ratatui_camera::*;
use ratatui::layout::{Layout, Constraint, Direction};

fn setup_multi_camera_tui(
    mut commands: Commands,
) {
    // Main gameplay camera (left panel)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        RatatuiCamera {
            strategy: RatatuiCameraStrategy::DepthBuffer,
            resolution: (40, 24),
        },
        Name::new("Main TUI Camera"),
    ));

    // Top-down minimap camera (right panel, top)
    commands.spawn((
        Camera3d {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Z),
        RatatuiCamera {
            strategy: RatatuiCameraStrategy::ASCIIArt,
            resolution: (30, 12),
        },
        Name::new("Minimap TUI Camera"),
    ));

    // Debug camera (right panel, bottom)
    commands.spawn((
        Camera3d {
            order: 2,
            ..default()
        },
        Transform::from_xyz(5.0, 2.0, 5.0)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        RatatuiCamera {
            strategy: RatatuiCameraStrategy::EdgeDetection,
            resolution: (30, 12),
        },
        Name::new("Debug TUI Camera"),
    ));
}
```

**TUI Layout:**
```
╔═══════════════════════════════╦══════════════════╗
║                               ║ MINIMAP          ║
║                               ║ ┌──────────────┐ ║
║                               ║ │ •  Player    │ ║
║    MAIN GAMEPLAY VIEW         ║ │              │ ║
║                               ║ │   Enemy  ▲   │ ║
║    [Player perspective]       ║ │              │ ║
║                               ║ └──────────────┘ ║
║                               ╠══════════════════╣
║                               ║ DEBUG VIEW       ║
║                               ║ ┌──────────────┐ ║
║                               ║ │ Colliders:   │ ║
║                               ║ │ [Edge view]  │ ║
║                               ║ │              │ ║
╚═══════════════════════════════╩══════════════════╝
```

---

### Custom Rendering Strategies

**Goal:** Implement custom TUI rendering logic.

```rust
use bevy::prelude::*;
use bevy_ratatui_camera::*;
use ratatui::style::{Color as RatatuiColor, Style};

// Custom rendering strategy for retro ASCII art
pub struct RetroASCIIStrategy {
    char_palette: Vec<char>,
    color_palette: Vec<RatatuiColor>,
}

impl RetroASCIIStrategy {
    pub fn new() -> Self {
        Self {
            // Brightness gradient (darkest to lightest)
            char_palette: vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            // Retro color palette (CGA-style)
            color_palette: vec![
                RatatuiColor::Black,
                RatatuiColor::Blue,
                RatatuiColor::Green,
                RatatuiColor::Cyan,
                RatatuiColor::Red,
                RatatuiColor::Magenta,
                RatatuiColor::Yellow,
                RatatuiColor::White,
            ],
        }
    }

    pub fn render_pixel(&self, brightness: f32, color: Color) -> (char, Style) {
        // Map brightness to character
        let char_index = (brightness * (self.char_palette.len() - 1) as f32) as usize;
        let ch = self.char_palette[char_index.min(self.char_palette.len() - 1)];

        // Map color to retro palette
        let color_index = self.quantize_color(color);
        let style = Style::default().fg(self.color_palette[color_index]);

        (ch, style)
    }

    fn quantize_color(&self, color: Color) -> usize {
        // Quantize to nearest CGA color
        // (Simplified color distance calculation)
        let [r, g, b, _] = color.to_srgba().to_u8_array();

        if r < 128 && g < 128 && b < 128 { 0 } // Black
        else if b > r && b > g { 1 }             // Blue
        else if g > r && g > b { 2 }             // Green
        else if r > 128 && g > 128 && b > 128 { 7 } // White
        else { 6 }                                  // Yellow (default)
    }
}

// Register custom strategy
fn setup_custom_rendering(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        RatatuiCamera {
            strategy: RatatuiCameraStrategy::Custom("RetroASCII".to_string()),
            resolution: (80, 24),
        },
        RetroASCIIStrategy::new(),
        Name::new("Retro TUI Camera"),
    ));
}
```

**Retro ASCII Output:**
```
╔════════════════════════════════════════════════╗
║                  :::::::                       ║
║              ====++++++====                    ║
║            ==++++******++++==                  ║
║          ==+++****@@@@****+++==                ║
║         =+++***@@@@@@@@@@***+++=               ║
║        ==++***@@@      @@@***++==              ║
║        =++***@@@        @@@***++=              ║
║        =++***@@@        @@@***++=              ║
║        ==++***@@@      @@@***++==              ║
║         =+++***@@@@@@@@@@***+++=               ║
║          ==+++****@@@@****+++==                ║
║            ==++++******++++==                  ║
║              ====++++++====                    ║
║                  :::::::                       ║
╚════════════════════════════════════════════════╝
```

---

### TUI-Based Game Development

**Goal:** Build a complete game playable in the terminal.

```rust
use bevy::prelude::*;
use bevy_ratatui_camera::*;

// Roguelike dungeon crawler example
fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            RatatuiCameraPlugin,
        ))
        .add_plugins(bevy_brp_extras::BrpExtrasPlugin)
        .add_systems(Startup, setup_dungeon)
        .add_systems(Update, (
            player_movement,
            enemy_ai,
            collision_detection,
            render_to_tui,
        ))
        .run();
}

fn setup_dungeon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Generate procedural dungeon
    let dungeon = generate_dungeon(30, 20);

    // Spawn walls
    for (x, y) in dungeon.walls.iter() {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
            Transform::from_xyz(*x as f32, 1.0, *y as f32),
            Name::new(format!("Wall ({}, {})", x, y)),
            Wall,
        ));
    }

    // Spawn player
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.8, 0.0))),
        Transform::from_xyz(5.0, 0.5, 5.0),
        Name::new("Player"),
        Player { health: 100, inventory: vec![] },
    ));

    // Top-down TUI camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 30.0, 10.0)
            .looking_at(Vec3::new(15.0, 0.0, 10.0), Vec3::Z),
        RatatuiCamera {
            strategy: RatatuiCameraStrategy::ASCIIArt,
            resolution: (80, 24),
        },
        Name::new("Dungeon TUI Camera"),
    ));
}

#[derive(Component)]
struct Player {
    health: i32,
    inventory: Vec<Item>,
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();
    let speed = 0.1;

    if keys.pressed(KeyCode::KeyW) { transform.translation.z -= speed; }
    if keys.pressed(KeyCode::KeyS) { transform.translation.z += speed; }
    if keys.pressed(KeyCode::KeyA) { transform.translation.x -= speed; }
    if keys.pressed(KeyCode::KeyD) { transform.translation.x += speed; }
}
```

**Dungeon TUI Game:**
```
╔════════════════════════════════════════════════════════════════════════════╗
║ ████████████████████████████████████████████████████████████████████████   ║
║ ██                                                                    ██   ║
║ ██  ████████████    ████████    ██████████    ████████████████████   ██   ║
║ ██  ██        ██    ██    ██    ██      ██    ██                ██   ██   ║
║ ██  ██  @     ██    ██    ██    ██  E   ██    ██    💰          ██   ██   ║
║ ██  ██        ██    ██    ██    ██      ██    ██                ██   ██   ║
║ ██  ████████████    ██████████████████████    ████████    ████████   ██   ║
║ ██                                                  ██    ██          ██   ║
║ ██  ████████████████████████████████████████████   ██    ██  E       ██   ║
║ ██  ██                                        ██   ██    ██          ██   ║
║ ██  ██  ████████████████████    ████████████  ██   ██████████████████ ██   ║
║ ██      ██              ██      ██        ██                          ██   ║
║ ████████████████████████████████████████████████████████████████████████   ║
╠════════════════════════════════════════════════════════════════════════════╣
║ HP: ████████░░ 80/100  |  Inventory: 💰×3  🗡️×1  🛡️×1                     ║
╚════════════════════════════════════════════════════════════════════════════╝

Legend: @ = Player  E = Enemy  💰 = Treasure  ██ = Wall
```

---

### Headless CI/CD Visualization

**Goal:** Visualize game tests in CI/CD pipelines without graphics.

```yaml
# .github/workflows/game-tests.yml
name: Game Tests with TUI Visualization

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-setup-rust@v1

      - name: Run tests with TUI output
        run: |
          cargo test --features brp,tui --no-default-features -- \
            --test-threads=1 \
            --nocapture > test_output.txt

      - name: Upload TUI test visualization
        uses: actions/upload-artifact@v3
        with:
          name: tui-test-output
          path: test_output.txt
```

**Test with TUI Visualization:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_collision_with_tui_viz() {
        // Setup headless TUI-only app
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            RatatuiCameraPlugin,
            bevy_brp_extras::BrpExtrasPlugin,
        ));

        // Spawn test scenario
        app.world.spawn((
            Player,
            Transform::from_xyz(0.0, 5.0, 0.0),
        ));

        app.world.spawn((
            Wall,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Spawn TUI camera for visualization
        app.world.spawn((
            Camera3d::default(),
            RatatuiCamera {
                strategy: RatatuiCameraStrategy::EdgeDetection,
                resolution: (40, 20),
            },
            Transform::from_xyz(5.0, 5.0, 5.0),
        ));

        // Simulate falling
        for frame in 0..100 {
            app.update();

            // Capture TUI frame to test output
            if frame % 10 == 0 {
                println!("Frame {}: {}", frame, capture_tui_frame(&app));
            }
        }

        // Assert collision occurred
        let player = app.world.query::<&Transform>()
            .iter(&app.world)
            .next()
            .unwrap();

        assert!(player.translation.y <= 1.0, "Player should have collided with floor");
    }
}
```

**CI/CD Test Output:**
```
test player_collision_with_tui_viz ...
Frame 0:
╔══════════════════════════════════════╗
║                                      ║
║              ●                       ║
║                                      ║
║                                      ║
╚══════════════════════════════════════╝

Frame 50:
╔══════════════════════════════════════╗
║                                      ║
║                                      ║
║              ●                       ║
║                                      ║
║          ════════ (floor)            ║
╚══════════════════════════════════════╝

Frame 100:
╔══════════════════════════════════════╗
║                                      ║
║                                      ║
║                                      ║
║              ●                       ║
║          ════════ (floor)            ║
╚══════════════════════════════════════╝
✓ Collision detected at frame 87

ok
```

---

## Troubleshooting

### Terminal Compatibility Issues

#### Problem: TUI doesn't render correctly

**Symptoms:**
- Garbled characters
- Missing colors
- Incorrect layout

**Solutions:**

```bash
# 1. Check terminal capabilities
echo $TERM
# Should be: xterm-256color, screen-256color, etc.

# 2. Force 256-color mode
export TERM=xterm-256color
cargo run --features brp,tui

# 3. Update terminal emulator
# - macOS: iTerm2 (recommended), Terminal.app
# - Linux: Alacritty, Kitty, Gnome Terminal
# - Windows: Windows Terminal, WSL2

# 4. Test with basic TUI app
cargo run --example tui_test
```

**Compatibility Matrix:**

| Terminal          | TUI Support | Colors | Unicode | Notes                    |
|-------------------|-------------|--------|---------|--------------------------|
| iTerm2 (macOS)    | ✅ Excellent | 256    | Full    | Recommended              |
| Alacritty         | ✅ Excellent | 256    | Full    | Fast, cross-platform     |
| Kitty             | ✅ Excellent | 256    | Full    | GPU-accelerated          |
| Windows Terminal  | ✅ Good      | 256    | Full    | Windows 10+ recommended  |
| Gnome Terminal    | ✅ Good      | 256    | Full    | Linux default            |
| Terminal.app      | ⚠️ Fair      | 256    | Partial | Limited Unicode support  |
| CMD.exe           | ❌ Poor      | 16     | None    | Not recommended          |

---

### Color Rendering Problems

#### Problem: Colors appear washed out or incorrect

**Diagnosis:**

```rust
// Add color test system
fn color_test_system(mut commands: Commands) {
    // Spawn color test spheres
    let test_colors = vec![
        ("Red", Color::srgb(1.0, 0.0, 0.0)),
        ("Green", Color::srgb(0.0, 1.0, 0.0)),
        ("Blue", Color::srgb(0.0, 0.0, 1.0)),
        ("White", Color::srgb(1.0, 1.0, 1.0)),
        ("Black", Color::srgb(0.0, 0.0, 0.0)),
    ];

    for (i, (name, color)) in test_colors.iter().enumerate() {
        commands.spawn((
            // ... sphere setup
            Name::new(format!("Color Test: {}", name)),
        ));
    }
}
```

**Solutions:**

```bash
# 1. Enable true color support
export COLORTERM=truecolor
cargo run --features brp,tui

# 2. Adjust gamma correction
mcp__brp__bevy_mutate_resource({
  resource: "bevy_render::color_grading::ColorGrading",
  path: ".gamma",
  value: 2.2  # Standard gamma
})

# 3. Use high-contrast materials
mcp__brp__bevy_mutate_component({
  entity: meshEntity,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".base_color",
  value: [1.0, 0.0, 0.0, 1.0]  # Pure saturated red
})

# 4. Disable HDR (if causing issues)
mcp__brp__bevy_mutate_component({
  entity: cameraEntity,
  component: "bevy_render::camera::camera::Camera",
  path: ".hdr",
  value: false
})
```

---

### Performance Optimization Tips

#### Problem: Low FPS or stuttering TUI

**Profiling:**

```javascript
// Check current performance
const time = await mcp__brp__bevy_get_resource({
  resource: "bevy_time::time::Time"
});

const fps = 1.0 / time.delta_secs;
console.log(`Current FPS: ${fps}`);

// Count entities
const entities = await mcp__brp__bevy_query({
  data: { components: [] },
  filter: {}
});
console.log(`Entity count: ${entities.length}`);
```

**Optimization Strategies:**

```rust
// 1. Reduce TUI resolution
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".resolution",
  value: [60, 20]  // Lower resolution = faster rendering
})

// 2. Use simpler rendering strategy
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".strategy",
  value: "ASCIIArt"  // Faster than EdgeDetection
})

// 3. Disable depth buffering if not needed
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".depth_buffer_enabled",
  value: false
})

// 4. Reduce entity count
// Remove off-screen entities
for entity in far_entities {
    mcp__brp__bevy_destroy({ entity });
}

// 5. Optimize materials
mcp__brp__bevy_mutate_component({
  entity: meshEntity,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".perceptual_roughness",
  value: 1.0  // Disable specular calculations
})
```

**Performance Targets:**

- Target FPS: 30-60 (terminal can't display more)
- Max entities: 500-1000 (depends on complexity)
- TUI render time: < 5ms per frame
- Total frame time: < 16ms (60 FPS) or < 33ms (30 FPS)

---

### Common Errors and Solutions

#### Error: "BRP not responding"

```bash
# Check if BRP server is running
lsof -i :15702

# Check logs
cargo run --features brp 2>&1 | grep -i "remote"

# Solution: Restart with BRP enabled
cargo clean
cargo run --features brp
```

#### Error: "Component not found in registry"

```javascript
// List all registered components
const components = await mcp__brp__bevy_list({});
console.log("Available components:", components);

// Use exact component name from list
mcp__brp__bevy_mutate_component({
  entity: entity,
  component: "bevy_transform::components::transform::Transform",  // Full path required
  path: ".translation.y",
  value: 5.0
})
```

#### Error: "TUI camera not rendering"

```rust
// Verify TUI camera is spawned correctly
mcp__brp__bevy_query({
  data: {
    components: ["bevy_ratatui_camera::RatatuiCamera"]
  },
  filter: {
    with: ["bevy_ratatui_camera::RatatuiCamera"]
  }
})

// Check camera is enabled
mcp__brp__bevy_get({
  entity: tuiCameraEntity,
  components: [
    "bevy_render::camera::camera::Camera",
    "bevy_ratatui_camera::RatatuiCamera"
  ]
})

// Enable if disabled
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_render::camera::camera::Camera",
  path: ".is_active",
  value: true
})
```

#### Error: "Entity ID not found"

**Cause:** Entity IDs are session-specific and reset on game restart.

**Solution:**
```javascript
// Always query entities by name before mutating
const entities = await mcp__brp__bevy_query({
  data: {
    components: ["bevy_core::name::Name"]
  },
  filter: {}
});

// Find entity by name
const player = entities.find(e =>
  e.components["bevy_core::name::Name"] === "Player"
);

// Use fresh entity ID
mcp__brp__bevy_mutate_component({
  entity: player.entity,  // Current session ID
  // ...
})
```

---

## Additional Resources

### Documentation

- **Bevy Engine**: https://bevyengine.org/learn/
- **Bevy Remote Protocol**: https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote
- **bevy_ratatui_camera**: https://github.com/cxreiff/bevy_ratatui_camera
- **Ratatui TUI Framework**: https://ratatui.rs/
- **Claude Code**: https://docs.claude.com/en/docs/claude-code
- **MCP Protocol**: https://modelcontextprotocol.io/

### Example Repositories

```bash
# Official examples
git clone https://github.com/cxreiff/bevy_ratatui_camera.git
cd bevy_ratatui_camera/examples

# Community showcases
git clone https://github.com/bevyengine/bevy.git
cd bevy/examples/games
```

### Community & Support

- **Bevy Discord**: https://discord.gg/bevy
- **Ratatui Discord**: https://discord.gg/ratatui
- **GitHub Issues**: Report bugs and request features
- **Bevy Assets**: https://bevyengine.org/assets/

---

## Quick Reference

### Essential MCP Commands

```javascript
// Status check
mcp__brp__brp_status({ app_name: "bevy-mcp-ratatui-ref" })

// Query all entities
mcp__brp__bevy_query({
  data: { components: ["bevy_core::name::Name"] },
  filter: {}
})

// Spawn entity
mcp__brp__bevy_spawn({
  components: { /* ... */ }
})

// Mutate component
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 5.0
})

// Get component
mcp__brp__bevy_get({
  entity: 123,
  components: ["bevy_transform::components::transform::Transform"]
})

// Destroy entity
mcp__brp__bevy_destroy({ entity: 123 })
```

### TUI-Specific Commands

```javascript
// Change rendering strategy
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".strategy",
  value: "EdgeDetection"  // or "DepthBuffer", "ASCIIArt", "Unicode"
})

// Adjust resolution
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".resolution",
  value: [80, 24]
})

// Toggle depth buffer
mcp__brp__bevy_mutate_component({
  entity: tuiCameraEntity,
  component: "bevy_ratatui_camera::RatatuiCamera",
  path: ".depth_buffer_enabled",
  value: true
})
```

---

**Happy TUI game development with AI assistance!** 🎮🤖🖥️
