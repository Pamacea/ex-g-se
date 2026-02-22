#!/bin/bash
# Development build and run

set -e

echo "[EX-G-SE] Development mode"

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux*)
        TARGET="x86_64-unknown-linux-gnu"
        ;;
    Darwin*)
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
        ;;
    *)
        echo "[EX-G-SE] Unsupported platform"
        exit 1
        ;;
esac

echo "[EX-G-SE] Building for $TARGET..."

# Build Rust binary
cd core
cargo build --release --target $TARGET
cd ..

# Copy to bin
mkdir -p bin
cp core/target/$TARGET/release/ex-g-se bin/ex-g-se
chmod +x bin/ex-g-se

echo "[EX-G-SE] Running..."

# Run via Node wrapper
cd cli
node bin/index.js
