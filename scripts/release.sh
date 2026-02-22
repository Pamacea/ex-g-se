#!/bin/bash
# Create a release tag and push to trigger GitHub Actions

set -e

if [ -z "$1" ]; then
    echo "Usage: ./release.sh X.Y.Z"
    echo "Example: ./release.sh 0.1.0"
    exit 1
fi

VERSION=$1

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format X.Y.Z"
    exit 1
fi

TAG="v$VERSION"

echo "[EX-G-SE] Creating release $TAG"

# Update version in Cargo.toml
echo "[EX-G-SE] Updating version in core/Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" core/Cargo.toml
rm -f core/Cargo.toml.bak

# Update version in package.json
echo "[EX-G-SE] Updating version in package.json..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
rm -f package.json.bak

sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" cli/package.json
rm -f cli/package.json.bak

# Commit changes
echo "[EX-G-SE] Committing version updates..."
git add core/Cargo.toml package.json cli/package.json
git commit -m "chore: bump version to $VERSION"

# Create tag
echo "[EX-G-SE] Creating git tag $TAG..."
git tag -a "$TAG" -m "Release $TAG"

# Push
echo "[EX-G-SE] Pushing to remote..."
git push
git push --tags

echo "[EX-G-SE] Release $TAG triggered!"
echo "[EX-G-SE] Watch progress at: https://github.com/oalacea/ex-g-se/actions"
