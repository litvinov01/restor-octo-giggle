#!/bin/bash
# Setup script for cross-compilation tooling

set -e

echo "🔧 Setting up cross-compilation tooling..."
echo ""

# Install cross (recommended)
if ! command -v cross &> /dev/null; then
    echo "Installing 'cross' tool..."
    cargo install cross --git https://github.com/cross-rs/cross
else
    echo "✓ 'cross' already installed"
fi

# Add common targets
echo ""
echo "Adding common Rust targets..."
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Linux-specific: install mingw for Windows cross-compile
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo ""
    echo "Setting up MinGW for Windows cross-compilation..."
    
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y gcc-mingw-w64-x86-64
        
        # Configure cargo for Windows cross-compile
        mkdir -p ~/.cargo
        cat >> ~/.cargo/config.toml <<EOF
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
EOF
        echo "✓ MinGW configured"
    fi
fi

echo ""
echo "✅ Cross-compilation setup complete!"
echo ""
echo "You can now build for different platforms:"
echo "  cross build --target x86_64-pc-windows-msvc"
echo "  cross build --target x86_64-unknown-linux-gnu"
echo "  cross build --target x86_64-apple-darwin"
