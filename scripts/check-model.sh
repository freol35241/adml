#!/bin/bash
# Check a single model (format, clippy, test, build)

set -e

MODEL_PATH=$1

if [ -z "$MODEL_PATH" ]; then
    echo "Usage: $0 <model-path>"
    echo "Example: $0 models/mathematical/dahlquist"
    exit 1
fi

if [ ! -d "$MODEL_PATH" ]; then
    echo "Error: Directory $MODEL_PATH does not exist"
    exit 1
fi

cd "$MODEL_PATH"

echo "🔍 Checking $MODEL_PATH..."
echo ""

echo "📋 Running format check..."
cargo fmt -- --check

echo ""
echo "🔍 Running clippy..."
cargo clippy --all-targets -- -D warnings

echo ""
echo "🧪 Running unit tests..."
cargo test --lib

echo ""
echo "🧪 Running integration tests..."
cargo test --tests

echo ""
echo "🏗️  Building release..."
cargo build --release

echo ""
echo "✅ All checks passed for $MODEL_PATH"
