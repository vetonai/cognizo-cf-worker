#!/bin/bash
# Cloudflare Build Script for Rust Workers
set -e

export PATH="$HOME/.cargo/bin:$PATH"

echo "Checking Rust installation..."

# Check if Rust is already installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal

    # Re-export PATH after installation
    export PATH="$HOME/.cargo/bin:$PATH"

    # Source cargo env if it exists
    if [ -f "$HOME/.cargo/env" ]; then
        . "$HOME/.cargo/env"
    fi

    echo "Rust installed: $(rustc --version)"
else
    echo "Rust found: $(rustc --version)"
fi

# Verify tools are accessible
echo "Verifying cargo is accessible..."
which cargo || { echo "ERROR: cargo not found in PATH"; exit 1; }
echo "Cargo location: $(which cargo)"

echo "Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

echo "Installing worker-build..."
cargo install -q worker-build 2>/dev/null || echo "worker-build already installed"

echo "Building worker..."
worker-build --release

echo "Build completed successfully!"
echo "Build artifacts:"
ls -lh build/ || echo "Build directory not found"
