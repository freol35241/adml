#!/bin/bash
# Package a model as an FMU in a Cargo workspace environment
#
# Usage: ./scripts/package-fmu.sh <model-path> [--release]
#
# Example: ./scripts/package-fmu.sh models/mathematical/dahlquist --release

set -e

if [ -z "$1" ]; then
    echo "Error: Model path required"
    echo "Usage: $0 <model-path> [--release]"
    echo "Example: $0 models/mathematical/dahlquist --release"
    exit 1
fi

MODEL_PATH="$1"
RELEASE_FLAG="${2:-}"

# Determine build type
if [ "$RELEASE_FLAG" == "--release" ]; then
    BUILD_TYPE="release"
    CARGO_FLAGS="--release"
else
    BUILD_TYPE="debug"
    CARGO_FLAGS=""
fi

echo "========================================="
echo "Packaging FMU for: $MODEL_PATH"
echo "Build type: $BUILD_TYPE"
echo "========================================="

# Get the model package name from Cargo.toml
MODEL_NAME=$(grep "^name" "$MODEL_PATH/Cargo.toml" | head -1 | sed 's/name = "\(.*\)"/\1/' | tr -d '"' | tr -d ' ')
echo "Model package name: $MODEL_NAME"

# Build the model
echo "Building model..."
cargo build -p "$MODEL_NAME" $CARGO_FLAGS

# Create temporary directory for packaging
TEMP_DIR=$(mktemp -d)
echo "Using temp directory: $TEMP_DIR"

# Copy modelDescription.xml if it exists in the model directory
if [ -f "$MODEL_PATH/modelDescription.xml" ]; then
    cp "$MODEL_PATH/modelDescription.xml" "$TEMP_DIR/"
else
    echo "Error: modelDescription.xml not found in $MODEL_PATH"
    echo "Please build the model first to generate modelDescription.xml"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Create target symlink in temp directory
ln -s "$(pwd)/target" "$TEMP_DIR/target"

# Run package_fmu_after_build from temp directory
cd "$TEMP_DIR"
if [ "$BUILD_TYPE" == "release" ]; then
    package_fmu_after_build --release
else
    package_fmu_after_build
fi

# Copy the generated FMU back to the model directory
FMU_FILE=$(ls *.fmu)
cp "$FMU_FILE" "$(pwd -P)/../$MODEL_PATH/"
cd - > /dev/null

# Clean up
rm -rf "$TEMP_DIR"

echo "========================================="
echo "FMU created: $MODEL_PATH/$FMU_FILE"
echo "========================================="
