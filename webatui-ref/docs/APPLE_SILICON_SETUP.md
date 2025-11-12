# Apple Silicon (M1/M2/M3) Setup Guide

## Issue Resolved

The project had a dependency installation issue on Apple Silicon Macs due to `cargo-watch` having a dependency on `mac-notification-sys` that fails to link with the AppKit framework on ARM64 architecture.

**Error encountered:**
```
Undefined symbols for architecture arm64:
  "_OBJC_CLASS_$_NSImage", referenced from:
       in libmac_notification_sys-ea8cac83aba67f7b.rlib
ld: symbol(s) not found for architecture arm64
```

## Solution

Replaced `cargo-watch` with `bacon`, a modern Rust file watcher that:
- ✅ Works perfectly on Apple Silicon
- ✅ Provides better terminal UI with live output
- ✅ Has more features (test filtering, better error display)
- ✅ Doesn't require system frameworks that cause linking issues

## Installation

Run the dependency installation command:

```bash
just install-deps
```

This will install:
- ✅ `bacon` (file watcher - replaces cargo-watch)
- ✅ `wasm-pack` (WASM build tool)
- ✅ `wasm-bindgen-cli` (WASM bindings)
- ✅ `basic-http-server` (local web server)

## Updated Commands

All watch commands now use `bacon`:

### Development Watching
```bash
# Watch and rebuild on changes
just watch

# Watch and run tests
just watch-test

# Watch specific example
just watch-example basic

# Watch WASM build
just watch-wasm
```

### Direct Bacon Usage
```bash
# Run bacon with default configuration
bacon

# Run bacon in test mode
bacon test

# Run bacon with specific features
bacon --features terminal

# Run bacon in release mode
bacon --release
```

## Bacon Features

Bacon provides several advantages over cargo-watch:

1. **Better UI**: Shows compilation progress, errors, and warnings in real-time
2. **Smart Filtering**: Can filter by specific tests or examples
3. **Keyboard Shortcuts**: Interactive navigation through errors
4. **No Notifications Needed**: Works without system notification frameworks
5. **Performance**: Faster rebuild detection and incremental compilation

## Configuration

Bacon can be configured via `.bacon/bacon.toml` if you want custom jobs. Example:

```toml
# .bacon/bacon.toml
[jobs.wasm]
command = ["cargo", "build", "--target", "wasm32-unknown-unknown"]
need_stdout = true
```

Then run: `bacon wasm`

## Verification

Verify all tools are installed:

```bash
bacon --version      # Should show: bacon 3.19.0
wasm-pack --version  # Should show: wasm-pack 0.13.1
basic-http-server --version  # Should show: basic-http-server 0.8.1
```

## Compatibility

| Platform | Status | Notes |
|----------|--------|-------|
| Apple Silicon (M1/M2/M3) | ✅ Working | Bacon resolves linking issues |
| Intel Mac | ✅ Working | Both bacon and cargo-watch work |
| Linux | ✅ Working | Both bacon and cargo-watch work |
| Windows | ✅ Working | Both bacon and cargo-watch work |

## Alternative: Using cargo-watch

If you prefer `cargo-watch` and aren't on Apple Silicon, you can still use it:

```bash
# Install cargo-watch (works on Intel Macs and other platforms)
cargo install cargo-watch

# Use it directly
cargo watch -x build
cargo watch -x test
cargo watch -x "run --example basic"
```

However, **on Apple Silicon**, `cargo-watch` will fail to compile due to the NSImage linking issue.

## Troubleshooting

### Bacon not found after installation
```bash
# Make sure cargo bin is in your PATH
echo $PATH | grep -q .cargo/bin || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Want to go back to cargo-watch?
If you're not on Apple Silicon:
```bash
cargo install cargo-watch
# Then manually update justfile watch commands to use cargo-watch
```

### Bacon shows "No bacon job found"
```bash
# Run bacon without arguments to see available jobs
bacon --jobs

# Or specify a job explicitly
bacon test
bacon clippy
bacon run
```

## Additional Resources

- [Bacon Documentation](https://dystroy.org/bacon/)
- [Bacon GitHub](https://github.com/Canop/bacon)
- [cargo-watch Alternative](https://github.com/watchexec/cargo-watch)

## Summary

✅ **Problem**: cargo-watch fails to compile on Apple Silicon due to NSImage linking errors
✅ **Solution**: Use bacon instead - works on all platforms
✅ **Status**: All dependencies installed successfully
✅ **Commands**: All watch commands updated in justfile
