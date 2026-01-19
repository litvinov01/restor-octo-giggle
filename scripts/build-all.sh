#!/bin/bash
# Build script for all platforms (Linux/macOS)

set -e

echo "🚀 Building restor-octo-giggle for all platforms..."
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$PROJECT_ROOT/rust_samples"

cd "$RUST_DIR"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if cross is installed
if ! command -v cross &> /dev/null; then
    echo "⚠️  'cross' not found. Install with: cargo install cross"
    USE_CROSS=false
else
    USE_CROSS=true
fi

# Function to build target
build_target() {
    local target=$1
    local name=$2
    echo -e "${BLUE}Building for ${name}...${NC}"
    
    if [ "$USE_CROSS" = true ]; then
        cross build --release --target "$target"
    else
        cargo build --release --target "$target"
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ ${name} build successful${NC}"
    else
        echo -e "❌ ${name} build failed"
        return 1
    fi
    echo ""
}

# Detect current platform
UNAME=$(uname -s)
ARCH=$(uname -m)

# Build targets based on current platform
if [[ "$UNAME" == "Linux"* ]]; then
    echo "Detected Linux platform"
    
    # Add targets
    rustup target add x86_64-unknown-linux-gnu || true
    rustup target add x86_64-pc-windows-gnu || true
    rustup target add x86_64-apple-darwin || true
    
    build_target "x86_64-unknown-linux-gnu" "Linux (x86_64)"
    # Windows cross-compile (if mingw available)
    # build_target "x86_64-pc-windows-gnu" "Windows (x86_64)"
    
elif [[ "$UNAME" == "Darwin"* ]]; then
    echo "Detected macOS platform"
    
    # Add targets
    if [[ "$ARCH" == "arm64" ]]; then
        rustup target add aarch64-apple-darwin || true
        rustup target add x86_64-apple-darwin || true
        build_target "aarch64-apple-darwin" "macOS (Apple Silicon)"
    else
        rustup target add x86_64-apple-darwin || true
        rustup target add aarch64-apple-darwin || true
    fi
    
    build_target "x86_64-apple-darwin" "macOS (Intel)"
    
else
    echo "Unknown platform: $UNAME"
    echo "Building for current platform only..."
    cargo build --release
fi

echo -e "${GREEN}✅ All builds completed!${NC}"
echo ""
echo "Build artifacts are in:"
echo "  $RUST_DIR/target/*/release/"
