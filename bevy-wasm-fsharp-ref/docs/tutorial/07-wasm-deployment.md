# Chapter 7: WASM Deployment

## What You'll Learn

In this chapter, you'll deploy your game to the web using WebAssembly. You'll learn optimization techniques, handle web-specific challenges, and create a polished browser experience.

**Time Required**: 30 minutes

## Why WebAssembly?

WebAssembly (WASM) enables:
- **Native Performance**: Near-native speed in browsers
- **No Installation**: Players access instantly via URL
- **Cross-Platform**: Works on any device with a browser
- **Easy Distribution**: Deploy to any web host

## Step 1: WASM Configuration

### Update Cargo.toml

```toml
# Cargo.toml

[package]
name = "bevy-wasm-fsharp-ref"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.14", default-features = false, features = [
    "bevy_winit",
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_sprite",
    "bevy_text",
    "bevy_ui",
    "webgl2",
    "wav",  # Audio format for web
] }
logic-fsharp = { path = "crates/logic-fsharp" }

# WASM-specific dependencies
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "Console",
] }
console_error_panic_hook = "0.1"

# Optimization for WASM
[profile.wasm-release]
inherits = "release"
opt-level = "z"  # Optimize for size
lto = true       # Link-time optimization
codegen-units = 1
strip = true     # Strip symbols
```

### Create WASM Entry Point

```rust
// src/wasm.rs

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    // Better panic messages in browser console
    console_error_panic_hook::set_once();

    // Log to browser console
    web_sys::console::log_1(&"Starting WASM game...".into());

    // Run the game
    crate::run_game();
}

// Utility functions for JavaScript interaction
#[wasm_bindgen]
pub fn get_game_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn resize_canvas(width: u32, height: u32) {
    // Handled by Bevy, but exposed for JS if needed
    web_sys::console::log_1(&format!("Resizing to {}x{}", width, height).into());
}
```

### Platform-Specific Code

```rust
// src/main.rs

mod combat;
mod components;

#[cfg(target_arch = "wasm32")]
mod wasm;

use bevy::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        run_game();
    }

    #[cfg(target_arch = "wasm32")]
    {
        // WASM entry is handled by wasm_bindgen
    }
}

pub fn run_game() {
    let mut app = App::new();

    // Platform-specific window settings
    #[cfg(target_arch = "wasm32")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#game-canvas".to_string()), // Canvas element ID
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        ..default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "F# Combat Game".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    };

    app.add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(combat::CombatPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Game setup
    commands.spawn(Camera2dBundle::default());
}
```

## Step 2: Build Configuration

### Create Build Script

```bash
# build-wasm.sh

#!/bin/bash

echo "Building WASM target..."

# Install wasm target if not present
rustup target add wasm32-unknown-unknown

# Build the WASM binary
cargo build --target wasm32-unknown-unknown --profile wasm-release

# Use wasm-bindgen to generate JS bindings
wasm-bindgen --out-dir wasm/pkg \
    --target web \
    --no-typescript \
    target/wasm32-unknown-unknown/wasm-release/bevy-wasm-fsharp-ref.wasm

# Optimize WASM file size (optional but recommended)
if command -v wasm-opt &> /dev/null; then
    echo "Optimizing WASM with wasm-opt..."
    wasm-opt -Oz \
        wasm/pkg/bevy_wasm_fsharp_ref_bg.wasm \
        -o wasm/pkg/bevy_wasm_fsharp_ref_bg.wasm
else
    echo "wasm-opt not found. Install with: npm install -g wasm-opt"
fi

echo "Build complete! Output in wasm/pkg/"
```

### HTML Template

```html
<!-- wasm/index.html -->

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>F# Combat Game</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        #game-container {
            position: relative;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
            border-radius: 8px;
            overflow: hidden;
        }

        #game-canvas {
            display: block;
            width: 100%;
            max-width: 1280px;
            height: auto;
            aspect-ratio: 16/9;
        }

        #loading {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            text-align: center;
            color: white;
        }

        .spinner {
            border: 4px solid rgba(255, 255, 255, 0.3);
            border-top: 4px solid white;
            border-radius: 50%;
            width: 40px;
            height: 40px;
            animation: spin 1s linear infinite;
            margin: 0 auto 20px;
        }

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        .error {
            background: #ef4444;
            color: white;
            padding: 20px;
            border-radius: 8px;
            margin: 20px;
        }

        .controls {
            position: absolute;
            bottom: 10px;
            right: 10px;
            display: flex;
            gap: 10px;
        }

        button {
            background: rgba(0, 0, 0, 0.5);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        }

        button:hover {
            background: rgba(0, 0, 0, 0.7);
        }
    </style>
</head>
<body>
    <div id="game-container">
        <canvas id="game-canvas"></canvas>
        <div id="loading">
            <div class="spinner"></div>
            <p>Loading game assets...</p>
        </div>
        <div class="controls" style="display: none;">
            <button onclick="toggleFullscreen()">Fullscreen</button>
            <button onclick="toggleAudio()">🔊 Audio</button>
        </div>
    </div>

    <script type="module">
        import init from './pkg/bevy_wasm_fsharp_ref.js';

        let audioEnabled = true;

        async function loadGame() {
            try {
                // Initialize WASM module
                await init();

                // Hide loading screen
                document.getElementById('loading').style.display = 'none';

                // Show controls
                document.querySelector('.controls').style.display = 'flex';

                console.log('Game loaded successfully!');
            } catch (error) {
                console.error('Failed to load game:', error);
                document.getElementById('loading').innerHTML = `
                    <div class="error">
                        <h2>Failed to load game</h2>
                        <p>${error.message}</p>
                        <p>Please try refreshing the page.</p>
                    </div>
                `;
            }
        }

        window.toggleFullscreen = () => {
            const canvas = document.getElementById('game-canvas');
            if (!document.fullscreenElement) {
                canvas.requestFullscreen().catch(err => {
                    console.error('Failed to enter fullscreen:', err);
                });
            } else {
                document.exitFullscreen();
            }
        };

        window.toggleAudio = () => {
            audioEnabled = !audioEnabled;
            // This would communicate with the game via wasm-bindgen
            console.log('Audio:', audioEnabled ? 'Enabled' : 'Disabled');
        };

        // Start loading
        loadGame();
    </script>
</body>
</html>
```

## Step 3: Asset Optimization

### Compress Assets

```rust
// src/assets.rs

use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
pub fn load_assets_web(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Use smaller assets for web
    commands.insert_resource(GameAssets {
        font: asset_server.load("fonts/FiraSans-Regular.woff2"), // Web font format
        sprites: asset_server.load("sprites_web.png"),  // Texture atlas
        // Don't load large audio files on web initially
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_assets_native(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        sprites: asset_server.load("sprites.png"),
        music: asset_server.load("audio/battle.ogg"),
    });
}
```

### Texture Atlas

```rust
// Use texture atlases to reduce draw calls

#[derive(Component)]
pub struct CharacterSprite {
    pub index: usize,
}

pub fn setup_texture_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),  // Tile size
        8,   // Columns
        4,   // Rows
        None, // No padding
        None, // No offset
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Spawn character with sprite from atlas
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0, // First sprite
            },
            ..default()
        },
        CharacterSprite { index: 0 },
    ));
}
```

## Step 4: Performance Optimization

### Reduce Bundle Size

```rust
// Conditional compilation to exclude unnecessary code

#[cfg(not(target_arch = "wasm32"))]
mod development_tools {
    use bevy::prelude::*;

    pub struct DevToolsPlugin;

    impl Plugin for DevToolsPlugin {
        fn build(&self, app: &mut App) {
            // Debug overlays, performance monitors, etc.
            // Not included in WASM build
        }
    }
}
```

### Optimize Rendering

```rust
// src/rendering.rs

use bevy::prelude::*;

/// Frustum culling - don't render off-screen entities
pub fn cull_offscreen_entities(
    camera_query: Query<&Transform, With<Camera>>,
    mut sprite_query: Query<(&Transform, &mut Visibility), With<Sprite>>,
) {
    let camera_transform = camera_query.single();
    let screen_bounds = 1000.0; // Adjust based on your game

    for (transform, mut visibility) in &mut sprite_query {
        let distance = camera_transform
            .translation
            .truncate()
            .distance(transform.translation.truncate());

        *visibility = if distance < screen_bounds {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Batch similar draw calls
pub fn setup_batching(mut commands: Commands) {
    // Use the same material for similar sprites
    let material = StandardMaterial {
        base_color: Color::WHITE,
        unlit: true, // Faster rendering
        ..default()
    };

    // Spawn multiple entities with same material
    // They'll be batched automatically
}
```

### Memory Management

```rust
// Pool commonly spawned entities

#[derive(Resource)]
pub struct EntityPool<T: Component> {
    available: Vec<Entity>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Component> EntityPool<T> {
    pub fn new() -> Self {
        EntityPool {
            available: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&mut self, commands: &mut Commands) -> Entity {
        if let Some(entity) = self.available.pop() {
            entity
        } else {
            commands.spawn_empty().id()
        }
    }

    pub fn return_entity(&mut self, entity: Entity) {
        self.available.push(entity);
    }
}
```

## Step 5: Web-Specific Features

### Browser Storage

```rust
// src/save_system_web.rs

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Storage};

#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[cfg(target_arch = "wasm32")]
pub fn save_game_web(save_data: &SaveData) -> Result<(), String> {
    let storage = get_local_storage()
        .ok_or("LocalStorage not available")?;

    let json = serde_json::to_string(save_data)
        .map_err(|e| e.to_string())?;

    storage
        .set_item("game_save", &json)
        .map_err(|_| "Failed to save to LocalStorage")?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn load_game_web() -> Result<SaveData, String> {
    let storage = get_local_storage()
        .ok_or("LocalStorage not available")?;

    let json = storage
        .get_item("game_save")
        .map_err(|_| "Failed to read from LocalStorage")?
        .ok_or("No save data found")?;

    serde_json::from_str(&json)
        .map_err(|e| e.to_string())
}
```

### JavaScript Interop

```rust
// src/js_bridge.rs

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Import JavaScript functions
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    fn show_achievement(title: &str, description: &str);
}

#[cfg(target_arch = "wasm32")]
pub fn show_achievement_notification(title: &str, description: &str) {
    show_achievement(title, description);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn show_achievement_notification(title: &str, description: &str) {
    println!("Achievement: {} - {}", title, description);
}
```

## Step 6: Deployment

### Local Testing

```bash
# Install HTTP server
cargo install basic-http-server

# Build WASM
./build-wasm.sh

# Serve locally
cd wasm
basic-http-server --addr 127.0.0.1:8080

# Open browser to http://localhost:8080
```

### GitHub Pages Deployment

```yaml
# .github/workflows/deploy.yml

name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Install wasm-bindgen
        run: cargo install wasm-bindgen-cli

      - name: Build WASM
        run: |
          cargo build --target wasm32-unknown-unknown --release
          wasm-bindgen --out-dir wasm/pkg \
            --target web \
            target/wasm32-unknown-unknown/release/bevy-wasm-fsharp-ref.wasm

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./wasm
```

### Netlify Deployment

```toml
# netlify.toml

[build]
  command = "./build-wasm.sh"
  publish = "wasm/"

[[headers]]
  for = "*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
    Content-Encoding = "gzip"

[[headers]]
  for = "/*"
  [headers.values]
    Cross-Origin-Embedder-Policy = "require-corp"
    Cross-Origin-Opener-Policy = "same-origin"
```

## Step 7: Optimization Checklist

### Before Deployment

- [ ] **Minimize Assets**
  ```bash
  # Compress images
  pngquant assets/*.png

  # Convert audio to lower bitrate
  ffmpeg -i audio.wav -b:a 64k audio.mp3
  ```

- [ ] **Enable Compression**
  ```nginx
  # nginx.conf
  gzip_types application/wasm;
  ```

- [ ] **Set Caching Headers**
  ```apache
  # .htaccess
  <FilesMatch "\.(wasm)$">
    Header set Cache-Control "max-age=31536000"
  </FilesMatch>
  ```

- [ ] **Profile Performance**
  ```rust
  #[cfg(debug_assertions)]
  fn profile_systems(time: Res<Time>) {
      if time.elapsed_seconds() % 5.0 < time.delta_seconds() {
          web_sys::console::log_1(&format!("FPS: {}", 1.0 / time.delta_seconds()).into());
      }
  }
  ```

## Common Issues and Solutions

### Issue: Large WASM File Size

**Solution**: Use optimization flags and strip unused code:

```toml
[profile.wasm-release]
opt-level = "z"
lto = true
strip = true
```

### Issue: Audio Not Playing

**Solution**: Handle browser autoplay policies:

```javascript
// User must interact before audio plays
document.addEventListener('click', () => {
    // Resume audio context
    if (audioContext.state === 'suspended') {
        audioContext.resume();
    }
}, { once: true });
```

### Issue: Poor Mobile Performance

**Solution**: Detect mobile and reduce quality:

```rust
#[cfg(target_arch = "wasm32")]
fn detect_mobile() -> bool {
    // Check user agent via JavaScript
    // Reduce particles, shadows, etc. on mobile
}
```

## Exercises

### Exercise 1: Add Loading Progress

Show actual loading progress:

```rust
#[wasm_bindgen]
pub fn report_loading_progress(percent: f32) {
    // Call JavaScript to update progress bar
}
```

### Exercise 2: Implement Touch Controls

Add mobile-friendly controls:

```rust
pub fn handle_touch_input(
    touches: Res<Touches>,
    // Convert touches to game actions
) {}
```

### Exercise 3: Add PWA Support

Make the game installable:

```json
// manifest.json
{
    "name": "F# Combat Game",
    "short_name": "Combat",
    "start_url": "/",
    "display": "fullscreen",
    "orientation": "landscape"
}
```

## Performance Metrics

Target performance for web deployment:

| Metric | Target | Achieved |
|--------|--------|----------|
| Initial Load | < 3s | Check with Lighthouse |
| WASM Size | < 5MB | Use wasm-opt |
| FPS | 60 | Profile in browser |
| Memory | < 100MB | Monitor in DevTools |

## Summary

You've successfully deployed your game to the web:

✅ Configured WASM build
✅ Optimized assets and bundle size
✅ Created responsive HTML wrapper
✅ Handled web-specific features
✅ Set up deployment pipeline
✅ Optimized for performance

## Next Steps

In the final chapter, we'll explore advanced topics like AI, ability systems, and future improvements.

[Next: Advanced Topics →](08-advanced-topics.md)

[← Previous: Combat System](06-combat-system.md) | [Tutorial Index](README.md)