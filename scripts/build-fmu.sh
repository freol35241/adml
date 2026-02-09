#!/bin/bash
# Build and package a single model as an FMU
#
# Usage: ./scripts/build-fmu.sh <model-directory>
# Example: ./scripts/build-fmu.sh models/mathematical/dahlquist

set -e

if [ -z "$1" ]; then
    echo "Error: Model directory required"
    echo "Usage: $0 <model-directory>"
    echo "Example: $0 models/mathematical/dahlquist"
    exit 1
fi

MODEL_DIR="$1"

# Check if directory exists
if [ ! -d "$MODEL_DIR" ]; then
    echo "Error: Directory '$MODEL_DIR' not found"
    exit 1
fi

# Get the package name from Cargo.toml
PACKAGE_NAME=$(grep "^name" "$MODEL_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo "=========================================="
echo "Building FMU for: $PACKAGE_NAME"
echo "=========================================="

# Build and package the FMU using cargo-fmi
echo "Building and packaging FMU..."
cargo fmi --package "$PACKAGE_NAME" bundle --release

# Move FMU to output directory
FMU_OUTPUT_DIR="fmus"
mkdir -p "$FMU_OUTPUT_DIR"

# Find the built FMU in target/fmu/
FMU_FILE=$(find target/fmu -name "*.fmu" -newer "$MODEL_DIR/Cargo.toml" 2>/dev/null | head -1)

if [ -z "$FMU_FILE" ]; then
    # Fallback: find any FMU
    FMU_FILE=$(ls target/fmu/*.fmu 2>/dev/null | head -1)
fi

if [ -n "$FMU_FILE" ]; then
    FMU_NAME=$(basename "$FMU_FILE")
    cp "$FMU_FILE" "$FMU_OUTPUT_DIR/"
    echo "=========================================="
    echo "FMU created: $FMU_OUTPUT_DIR/$FMU_NAME"
    echo "=========================================="
    exit 0
else
    echo "Error: No FMU file found in target/fmu/"
    exit 1
fi
