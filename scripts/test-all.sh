#!/bin/bash
# Test all models in the workspace

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT"

echo "🧪 Running tests for all models..."
echo ""

cargo test --workspace

echo ""
echo "✅ All tests passed!"
