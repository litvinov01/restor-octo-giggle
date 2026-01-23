# Cross-Platform Build Guide

This guide explains how to build the restor-octo-giggle transport bus for different platforms and create agile, automated builds.

## Quick Start

### Build for Current Platform

```bash
# Rust server
cd rust_samples
cargo build --release

# JavaScript driver (no build needed, uses ES modules)
cd drivers
node example.js
```

### Build for All Platforms (Using Scripts)

```bash
# Linux/macOS
./scripts/build-all.sh

# Windows PowerShell
.\scripts\build-all.ps1

# Windows CMD
scripts\build-all.bat
```

## Supported Platforms

### Rust Server
- ✅ Windows (x86_64-pc-windows-msvc, x86_64-pc-windows-gnu)
- ✅ Linux (x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl)
- ✅ macOS (x86_64-apple-darwin, aarch64-apple-darwin)
- ✅ WebAssembly (wasm32-unknown-unknown) - future

### JavaScript Driver
- ✅ All platforms with Node.js 14+

## Prerequisites

### Rust Cross-Compilation

Install `cross` tool for easy cross-compilation:

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

Or use rustup targets directly:

```bash
# Install targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

## Manual Builds

### Windows (from Windows)

```bash
cd rust_samples
cargo build --release --target x86_64-pc-windows-msvc
# Output: target/x86_64-pc-windows-msvc/release/rust_samples.exe
```

### Linux (from Linux)

```bash
cd rust_samples
cargo build --release --target x86_64-unknown-linux-gnu
# Output: target/x86_64-unknown-linux-gnu/release/rust_samples
```

### macOS (from macOS)

```bash
cd rust_samples
# Intel Mac
cargo build --release --target x86_64-apple-darwin

# Apple Silicon Mac
cargo build --release --target aarch64-apple-darwin
```

### Cross-Compile Linux from Windows

```bash
# Using cross (recommended)
cross build --release --target x86_64-unknown-linux-gnu

# Or using Docker + rustup
docker run --rm -v "%cd%\rust_samples":/project rust:latest \
  cargo build --release --target x86_64-unknown-linux-gnu
```

### Cross-Compile Windows from Linux

```bash
# Install mingw-w64
sudo apt-get install gcc-mingw-w64-x86-64

# Add target
rustup target add x86_64-pc-windows-gnu

# Configure linker
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml <<EOF
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
EOF

# Build
cd rust_samples
cargo build --release --target x86_64-pc-windows-gnu
```

## Automated Builds

### Using GitHub Actions

The `.github/workflows/build.yml` workflow automatically builds for all platforms on every push and release.

**Trigger manually:**
```bash
git push origin main  # Triggers build
```

**Create release:**
```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0  # Triggers release build with artifacts
```

### Using Local Build Scripts

```bash
# Build all targets
./scripts/build-all.sh

# Build specific target
./scripts/build.sh linux

# Build and package
./scripts/package.sh
```

## Docker Builds

### Build Server Container

```bash
# Build for current platform
docker build -t rust-samples:latest -f docker/Dockerfile.server .

# Build for specific platform
docker buildx build --platform linux/amd64 -t rust-samples:linux-amd64 .
docker buildx build --platform linux/arm64 -t rust-samples:linux-arm64 .
docker buildx build --platform windows/amd64 -t rust-samples:windows-amd64 .
```

### Multi-Platform Build (Docker Buildx)

```bash
# Enable buildx
docker buildx create --use

# Build for all platforms
docker buildx build --platform linux/amd64,linux/arm64,windows/amd64 \
  -t rust-samples:latest \
  -f docker/Dockerfile.server \
  --push  # or --load for local
```

## Build Outputs

### Release Artifacts

After building, artifacts are in:

```
rust_samples/target/{target}/release/
├── rust_samples         # Linux/macOS executable
└── rust_samples.exe     # Windows executable
```

### Packaged Releases

```
dist/
├── restor-octo-giggle-{version}-linux-x86_64.tar.gz
├── restor-octo-giggle-{version}-macos-x86_64.tar.gz
├── restor-octo-giggle-{version}-macos-aarch64.tar.gz
└── restor-octo-giggle-{version}-windows-x86_64.zip
```

Note: The binary is renamed to `restor-octo-giggle` (or `restor-octo-giggle.exe` on Windows) in distribution packages.

## Platform-Specific Considerations

### Windows
- Use `.exe` extension
- May need Visual Studio Build Tools or MinGW
- Port binding may require admin rights for ports < 1024

### Linux
- Requires `libc` (or use `musl` for static linking)
- May need `libssl` for TLS (future)
- Use `ldd` to check dependencies

### macOS
- Universal binaries possible (x86_64 + aarch64)
- Code signing required for distribution
- Notarization needed for Gatekeeper

## Static Linking (Musl)

For fully static binaries on Linux:

```bash
# Add musl target
rustup target add x86_64-unknown-linux-musl

# Install musl toolchain
# On Ubuntu/Debian
sudo apt-get install musl-tools

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

## Size Optimization

```bash
# Strip symbols
strip target/x86_64-unknown-linux-gnu/release/rust_samples

# Use LTO (Link Time Optimization)
# Add to Cargo.toml:
[profile.release]
lto = true
codegen-units = 1
```

## Continuous Integration

See `.github/workflows/` for:
- Build on push/PR
- Build on release tags
- Upload artifacts
- Run tests
- Check formatting/linting

## Troubleshooting

### Cross-compilation fails

1. Install correct toolchain
2. Check `~/.cargo/config.toml` for linker settings
3. Use `cross` tool for easier cross-compilation

### Missing dependencies

- Linux: `apt-get install build-essential libssl-dev`
- macOS: `xcode-select --install`
- Windows: Install Visual Studio Build Tools

### Port already in use

```bash
# Check what's using the port
# Linux/macOS
lsof -i :49152

# Windows
netstat -ano | findstr :49152
```

## Next Steps

- [ ] Set up CI/CD pipeline
- [ ] Create Docker images
- [ ] Set up automated releases
- [ ] Add code signing for macOS
- [ ] Create installers/packages
- [ ] Add WebAssembly target
