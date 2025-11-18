#!/bin/bash
# Build all models in the workspace

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT"

echo "🔨 Building all models..."
echo ""

cargo build --workspace --release

echo ""
echo "✅ All models built successfully!"
echo "📂 Binaries are in target/release/"
