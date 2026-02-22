#!/bin/bash
# package.sh - Prepare NPM package with compiled binaries
#
# This script builds binaries and prepares a complete NPM package
# for publishing to npm registry or for distribution.
#
# REQUIREMENTS:
# - npm installed and available in PATH
# - build-all.sh must be executable
# - Properly configured package.json in cli/ directory
#
# Usage: ./package.sh [--dry-run] [--skip-build]
#
# Options:
#   --dry-run      - Validate package without publishing
#   --skip-build   - Skip building binaries (use existing)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="ex-g-se"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
PACKAGE_DIR="$DIST_DIR/package"
BIN_DIR="$PROJECT_ROOT/bin"

# Parse arguments
DRY_RUN=false
SKIP_BUILD=false

for arg in "$@"; do
    case $arg in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        *)
            ;;
    esac
done

echo -e "${BLUE}=== EX-G-SE NPM Package Preparation Script ===${NC}\n"

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm not found. Please install Node.js from https://nodejs.org/${NC}"
    exit 1
fi

if [[ "$DRY_RUN" == true ]]; then
    echo -e "${YELLOW}DRY RUN MODE: Will validate package without publishing${NC}"
fi

if [[ "$SKIP_BUILD" == true ]]; then
    echo -e "${YELLOW}SKIP BUILD MODE: Will use existing binaries${NC}"
fi

echo -e "${GREEN}✓ Node.js found: $(node --version)${NC}"
echo -e "${GREEN}✓ npm found: $(npm --version)${NC}\n"

# Step 1: Build binaries
if [[ "$SKIP_BUILD" == false ]]; then
    echo -e "${YELLOW}Step 1: Building binaries...${NC}"
    if [[ -f "$SCRIPT_DIR/build-all.sh" ]]; then
        if bash "$SCRIPT_DIR/build-all.sh" $([[ "$DRY_RUN" == true ]] && echo "--dry-run"); then
            echo -e "${GREEN}✓ Binaries built successfully${NC}\n"
        else
            echo -e "${RED}✗ Failed to build binaries${NC}"
            exit 1
        fi
    else
        echo -e "${RED}Error: build-all.sh not found${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Step 1: Skipping binary build (using existing)...${NC}\n"
fi

# Verify binaries exist
echo -e "${YELLOW}Verifying binaries...${NC}"
if [[ ! -d "$BIN_DIR" ]] || [[ -z "$(ls -A "$BIN_DIR" 2>/dev/null)" ]]; then
    echo -e "${RED}Error: No binaries found in $BIN_DIR${NC}"
    echo "Run with --skip-build only if binaries already exist."
    exit 1
fi

ls -lh "$BIN_DIR"
echo -e "${GREEN}✓ Binaries verified${NC}\n"

# Step 2: Prepare package directory
echo -e "${YELLOW}Step 2: Creating package directory...${NC}"
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/bin"
echo -e "${GREEN}✓ Package directory created: $PACKAGE_DIR${NC}\n"

# Step 3: Copy CLI files
echo -e "${YELLOW}Step 3: Copying CLI files...${NC}"
if [[ -d "$PROJECT_ROOT/cli" ]]; then
    # Copy all CLI files except node_modules
    rsync -av --exclude='node_modules' --exclude='.git' \
        "$PROJECT_ROOT/cli/" "$PACKAGE_DIR/" > /dev/null 2>&1 || \
        cp -r "$PROJECT_ROOT/cli"/* "$PACKAGE_DIR/" 2>/dev/null || true

    # Remove node_modules if it was copied
    rm -rf "$PACKAGE_DIR/node_modules"

    echo -e "${GREEN}✓ CLI files copied${NC}\n"
else
    echo -e "${YELLOW}⚠ Warning: cli/ directory not found, skipping...${NC}\n"
fi

# Step 4: Copy binaries
echo -e "${YELLOW}Step 4: Copying binaries...${NC}"
for binary in "$BIN_DIR"/*; do
    if [[ -f "$binary" ]]; then
        cp "$binary" "$PACKAGE_DIR/bin/"
        echo -e "${GREEN}  ✓ Copied: $(basename "$binary")${NC}"
    fi
done
echo ""

# Step 5: Validate package structure
echo -e "${YELLOW}Step 5: Validating package structure...${NC}"

# Check for package.json
if [[ ! -f "$PACKAGE_DIR/package.json" ]]; then
    echo -e "${YELLOW}⚠ Warning: package.json not found in package directory${NC}"
    echo "  Creating minimal package.json..."

    cat > "$PACKAGE_DIR/package.json" << EOF
{
  "name": "$PROJECT_NAME",
  "version": "0.1.0",
  "description": "EX-G-SE - Execute Google Search Engine",
  "main": "bin/index.js",
  "bin": {
    "$PROJECT_NAME": "./bin/index.js"
  },
  "os": [
    "darwin",
    "linux",
    "win32"
  ],
  "cpu": [
    "x64",
    "arm64"
  ],
  "files": [
    "bin/",
    "lib/"
  ],
  "keywords": [
    "search",
    "google",
    "cli"
  ],
  "license": "MIT",
  "engines": {
    "node": ">=18.0.0"
  }
}
EOF
    echo -e "${GREEN}✓ Created package.json${NC}"
fi

# Check for README
if [[ ! -f "$PACKAGE_DIR/README.md" ]] && [[ -f "$PROJECT_ROOT/README.md" ]]; then
    cp "$PROJECT_ROOT/README.md" "$PACKAGE_DIR/"
    echo -e "${GREEN}✓ Copied README.md${NC}"
fi

# Check for LICENSE
if [[ ! -f "$PACKAGE_DIR/LICENSE" ]] && [[ -f "$PROJECT_ROOT/LICENSE" ]]; then
    cp "$PROJECT_ROOT/LICENSE" "$PACKAGE_DIR/"
    echo -e "${GREEN}✓ Copied LICENSE${NC}"
fi

echo ""

# Step 6: Dry run validation
echo -e "${YELLOW}Step 6: Validating package...${NC}"
cd "$PACKAGE_DIR"

if npm pack --dry-run 2>&1 | grep -q "npm ERR"; then
    echo -e "${RED}✗ Package validation failed${NC}"
    echo "Run 'npm pack' in $PACKAGE_DIR to see detailed errors."
    exit 1
else
    echo -e "${GREEN}✓ Package validation successful${NC}"
fi

echo ""

# Step 7: Show package contents
echo -e "${YELLOW}Package structure:${NC}"
tree -L 2 "$PACKAGE_DIR" 2>/dev/null || find "$PACKAGE_DIR" -type f -o -type d | head -20

echo ""

# Step 8: Publish (if not dry run)
if [[ "$DRY_RUN" == false ]]; then
    echo -e "${YELLOW}Step 7: Publishing package...${NC}"
    echo -e "${YELLOW}To publish, run: cd $PACKAGE_DIR && npm publish${NC}"
    echo -e "${YELLOW}For public scoped package: npm publish --access public${NC}"
else
    echo -e "${YELLOW}Step 7: Dry run complete${NC}"
    echo -e "${YELLOW}To publish for real, run: cd $PACKAGE_DIR && npm publish${NC}"
fi

echo ""
echo -e "${BLUE}=== Package Preparation Complete ===${NC}"
echo "Package directory: $PACKAGE_DIR"
echo "Binary files:"
ls -1 "$PACKAGE_DIR/bin/" 2>/dev/null || echo "  (none)"
