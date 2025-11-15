# WASM Optimization - Quick Reference

## Build Commands

### Development (fast iteration)
```bash
cargo build -p app --target wasm32-unknown-unknown --release
```
**Time**: ~45 seconds | **Size**: ~53 MB

### Production (optimized)
```bash
just build-wasm
```
**Time**: ~70 seconds | **Size**: 3.8 MB (Brotli)

### Test Locally
```bash
just serve-wasm
```
Opens http://localhost:8000

## Bundle Sizes

| Format | Size | Reduction | Use Case |
|--------|------|-----------|----------|
| Unoptimized | 53 MB | - | Development only |
| wasm-release | 21 MB | 60% | Fallback |
| Gzip | 5.4 MB | 90% | Older browsers |
| Brotli | 3.8 MB | 93% | Modern browsers |

## Loading Times

| Connection | Brotli (3.8 MB) | Total Load Time |
|-----------|-----------------|----------------|
| Slow 4G (5 Mbps) | 6.1s | ~8-10s |
| Fast 4G (20 Mbps) | 1.5s | ~3-5s |
| WiFi (50 Mbps) | 0.6s | ~2-3s |
| Fiber (100 Mbps) | 0.3s | ~1-2s |

## Build Profile

Location: `Cargo.toml`

```toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"           # Size optimization
lto = true                # Link-time optimization
codegen-units = 1         # Single codegen unit
strip = true              # Strip debug symbols
panic = "abort"           # Abort on panic
```

## Generated Files

```
web/pkg/
├── app_bg.wasm       # 21 MB - Optimized WASM
├── app_bg.wasm.gz    # 5.4 MB - Gzip compressed
├── app_bg.wasm.br    # 3.8 MB - Brotli compressed (best)
└── app.js            # 113 KB - JS bindings
```

## Progressive Loading

Stages shown in browser:

1. **Fetching** (0%) - Initial request
2. **Downloading** (0-70%) - Stream download
3. **Combining** (70-75%) - Merge chunks
4. **Compiling** (75-85%) - WASM compilation
5. **Initializing** (85-100%) - Bevy startup
6. **Ready** (100%) - Game loaded

## Web Server Configuration

### Nginx
```nginx
location /pkg/ {
    gzip_static on;
    brotli_static on;

    location ~ \.(wasm|js)$ {
        add_header Cache-Control "public, max-age=31536000, immutable";
    }
}
```

### Apache
```apache
<IfModule mod_brotli.c>
    AddOutputFilterByType BROTLI_COMPRESS application/wasm
</IfModule>

<FilesMatch "\.(wasm|js)$">
    Header set Cache-Control "max-age=31536000, public, immutable"
</FilesMatch>
```

## Optional: wasm-opt

For additional 10-15% reduction:

```bash
# macOS
brew install binaryen

# npm
npm install -g wasm-opt
```

Build script automatically detects and uses if available.

## Troubleshooting

### Build fails
```bash
# Ensure dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

### Large bundle despite optimization
Check:
- Using `--profile wasm-release`? (not `--release`)
- LTO enabled in Cargo.toml?
- Unnecessary Bevy features included?

### Slow loading in production
Verify:
- Compressed files served (check Network tab)
- `Content-Encoding: br` or `gzip` in headers
- Appropriate cache headers set

## Browser Support

| Browser | Brotli | Gzip | Status |
|---------|--------|------|--------|
| Chrome 50+ | ✅ | ✅ | Fully supported |
| Firefox 44+ | ✅ | ✅ | Fully supported |
| Safari 11+ | ✅ | ✅ | Fully supported |
| Edge 15+ | ✅ | ✅ | Fully supported |
| Older browsers | ❌ | ✅ | Gzip fallback |

## Performance Targets

- ✅ Load time < 3 seconds on good WiFi (achieved: ~2.9s)
- ✅ Bundle size < 5 MB compressed (achieved: 3.8 MB)
- ✅ Progressive loading feedback (implemented)
- ✅ Browser compatibility (all modern browsers)

## Further Reading

- **Optimization Guide**: `docs/wasm-optimization.md`
- **Performance Report**: `docs/performance-report.md`
- **Summary**: `docs/WASM_OPTIMIZATION_SUMMARY.md`

---

**Quick Start**: `just build-wasm && just serve-wasm`
