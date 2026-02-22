#!/bin/bash
# Local test script for EX-G-SE

echo "=================================================================="
echo "[EX-G-SE] Local Testing Script"
echo "=================================================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust/Cargo not found. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

echo "[1/4] Building Rust core..."
cd core
cargo build --release
if [ $? -ne 0 ]; then
    echo "[ERROR] Rust build failed"
    exit 1
fi
echo "[OK] Rust core built successfully"
cd ..

echo ""
echo "[2/4] Setting up binary directory..."
mkdir -p bin

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux*)
        BINARY_NAME="ex-g-se"
        ;;
    Darwin*)
        BINARY_NAME="ex-g-se"
        ;;
    MINGW*|MSYS*|Windows*)
        BINARY_NAME="ex-g-se.exe"
        ;;
    *)
        echo "[ERROR] Unsupported platform: $OS"
        exit 1
        ;;
esac

# Copy binary
cp core/target/release/$BINARY_NAME bin/$BINARY_NAME
chmod +x bin/$BINARY_NAME
echo "[OK] Binary copied to bin/$BINARY_NAME"

echo ""
echo "[3/4] Running tests..."
cd core
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "[ERROR] Tests failed"
    exit 1
fi
echo "[OK] All tests passed"
cd ..

echo ""
echo "[4/4] Testing binary execution..."
echo "[INFO] Creating test directory..."
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

echo "[INFO] Starting EX-G-SE in test mode..."
echo "[INFO] Press Ctrl+C to stop after a few seconds..."

# Run for 5 seconds then stop
timeout 5s ../bin/$BINARY_NAME || true

if [ -f "raw_logs.json" ]; then
    echo ""
    echo "[OK] Logs generated successfully!"
    echo "[INFO] Log file: $TEST_DIR/raw_logs.json"
    echo ""
    echo "[INFO] First 20 lines of logs:"
    head -n 20 raw_logs.json
    echo ""
    echo "[SUCCESS] EX-G-SE is working correctly!"
else
    echo "[WARNING] No logs generated (expected if stopped immediately)"
fi

cd ..
rm -rf "$TEST_DIR"

echo ""
echo "=================================================================="
echo "[EX-G-SE] Test Complete"
echo "=================================================================="
echo ""
echo "Next steps:"
echo "  1. Run manually: ./bin/$BINARY_NAME"
echo "  2. Or use npm: npm run dev"
echo "  3. Build all platforms: npm run build"
echo ""
