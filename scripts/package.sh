#!/bin/bash
# Package script for creating distribution archives

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$PROJECT_ROOT/rust_samples"
DIST_DIR="$PROJECT_ROOT/dist"
VERSION=$(grep "^version" "$RUST_DIR/Cargo.toml" | cut -d'"' -f2)

echo "📦 Packaging restor-octo-giggle v$VERSION..."
echo ""

# Create dist directory
mkdir -p "$DIST_DIR"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Function to package target
package_target() {
    local target=$1
    local platform=$2
    local ext=$3
    
    echo -e "${BLUE}Packaging ${platform}...${NC}"
    
    local target_dir="$RUST_DIR/target/$target/release"
    local exe_name="rust_samples"
    
    if [[ "$platform" == *"Windows"* ]]; then
        exe_name="rust_samples.exe"
    fi
    
    if [ ! -f "$target_dir/$exe_name" ]; then
        echo "⚠️  Binary not found: $target_dir/$exe_name"
        echo "   Building it first..."
        cd "$RUST_DIR"
        cargo build --release --target "$target"
    fi
    
    # Create temporary directory for package
    local temp_dir=$(mktemp -d)
    local pkg_name="restor-octo-giggle-${VERSION}-${platform}"
    local pkg_dir="$temp_dir/$pkg_name"
    mkdir -p "$pkg_dir"
    
    # Copy binary with project name
    if [[ "$platform" == *"windows"* ]]; then
        cp "$target_dir/$exe_name" "$pkg_dir/restor-octo-giggle.exe"
    else
        cp "$target_dir/$exe_name" "$pkg_dir/restor-octo-giggle"
        chmod +x "$pkg_dir/restor-octo-giggle"
    fi
    
    # Copy README
    cp "$PROJECT_ROOT/rust_samples/README.md" "$pkg_dir/" 2>/dev/null || true
    
    # Create archive
    cd "$temp_dir"
    if [[ "$ext" == "zip" ]]; then
        zip -r "$DIST_DIR/$pkg_name.zip" "$pkg_name" > /dev/null
    else
        tar czf "$DIST_DIR/$pkg_name.tar.gz" "$pkg_name"
    fi
    
    rm -rf "$temp_dir"
    echo -e "${GREEN}✓ Created $DIST_DIR/$pkg_name.$ext${NC}"
    echo ""
}

# Package available targets
if [ -f "$RUST_DIR/target/x86_64-unknown-linux-gnu/release/rust_samples" ]; then
    package_target "x86_64-unknown-linux-gnu" "linux-x86_64" "tar.gz"
fi

if [ -f "$RUST_DIR/target/x86_64-apple-darwin/release/rust_samples" ]; then
    package_target "x86_64-apple-darwin" "macos-x86_64" "tar.gz"
fi

if [ -f "$RUST_DIR/target/aarch64-apple-darwin/release/rust_samples" ]; then
    package_target "aarch64-apple-darwin" "macos-aarch64" "tar.gz"
fi

if [ -f "$RUST_DIR/target/x86_64-pc-windows-msvc/release/rust_samples.exe" ]; then
    package_target "x86_64-pc-windows-msvc" "windows-x86_64" "zip"
fi

echo -e "${GREEN}✅ Packaging completed!${NC}"
echo "Packages are in: $DIST_DIR"
