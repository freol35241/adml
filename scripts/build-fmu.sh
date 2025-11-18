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

# Convert hyphens to underscores for directory name
# (package_fmu_after_build expects directory name to match library name)
DIR_NAME="${PACKAGE_NAME//-/_}"

echo "=========================================="
echo "Building FMU for: $PACKAGE_NAME"
echo "Directory name: $DIR_NAME"
echo "=========================================="

# Step 1: Build the model (release mode)
echo "Building model..."
# Remove old modelDescription.xml to ensure a fresh one is generated
rm -f modelDescription.xml
# Force rebuild to ensure modelDescription.xml is regenerated
cargo clean -p "$PACKAGE_NAME" --release
cargo build -p "$PACKAGE_NAME" --release

# Step 2: Create a temporary directory with underscore name
TEMP_DIR=$(mktemp -d)
WORK_DIR="$TEMP_DIR/$DIR_NAME"
mkdir -p "$WORK_DIR"

echo "Using temp directory: $WORK_DIR"

# Step 3: Copy modelDescription.xml
# fmu_from_struct generates it in the workspace root
if [ -f "modelDescription.xml" ]; then
    # Copy from workspace root (most recent build)
    cp "modelDescription.xml" "$WORK_DIR/"
elif [ -f "$MODEL_DIR/modelDescription.xml" ]; then
    # Fallback: copy from model directory if it exists there
    cp "$MODEL_DIR/modelDescription.xml" "$WORK_DIR/"
else
    echo "Error: modelDescription.xml not found"
    echo "Expected in workspace root or $MODEL_DIR"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Step 4: Create target symlink
ln -s "$(pwd)/target" "$WORK_DIR/target"

# Step 5: Package the FMU
echo "Packaging FMU..."
cd "$WORK_DIR"
package_fmu_after_build --release
cd - > /dev/null

# Step 6: Move FMU to output directory
FMU_OUTPUT_DIR="fmus"
mkdir -p "$FMU_OUTPUT_DIR"

# Find the FMU file
FMU_FILE=$(ls "$WORK_DIR"/*.fmu 2>/dev/null | head -1)

if [ -n "$FMU_FILE" ]; then
    FMU_NAME=$(basename "$FMU_FILE")
    cp "$FMU_FILE" "$FMU_OUTPUT_DIR/"
    echo "=========================================="
    echo "✅ FMU created: $FMU_OUTPUT_DIR/$FMU_NAME"
    echo "=========================================="

    # Cleanup
    rm -rf "$TEMP_DIR"

    # Return success
    exit 0
else
    echo "Error: No FMU file found"
    rm -rf "$TEMP_DIR"
    exit 1
fi
