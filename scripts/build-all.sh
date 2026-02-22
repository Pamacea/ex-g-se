#!/bin/bash
# Cross-compile EX-G-SE for all supported platforms
set -e

echo "[EX-G-SE] Building for all platforms..."

# Add all targets
echo "[EX-G-SE] Adding Rust targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for Linux
echo "[EX-G-SE] Building for Linux x64..."
cd core
cargo build --release --target x86_64-unknown-linux-gnu
cd ..

# Build for Windows
echo "[EX-G-SE] Building for Windows x64..."
cd core
cargo build --release --target x86_64-pc-windows-msvc
cd ..

# Build for macOS Intel
echo "[EX-G-SE] Building for macOS Intel..."
cd core
cargo build --release --target x86_64-apple-darwin
cd ..

echo "[EX-G-SE] Note: macOS ARM64 build requires macOS host"

# Create bin directory and copy binaries
echo "[EX-G-SE] Copying binaries to bin/..."
mkdir -p bin

cp core/target/x86_64-unknown-linux-gnu/release/ex-g-se bin/ex-g-se-linux
cp core/target/x86_64-pc-windows-msvc/release/ex-g-se.exe bin/ex-g-se-win.exe
cp core/target/x86_64-apple-darwin/release/ex-g-se bin/ex-g-se-macos-intel

chmod +x bin/*

echo "[EX-G-SE] Build complete!"
ls -la bin/
