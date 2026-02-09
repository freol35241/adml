#!/bin/bash
# Build and package a single model as a multi-platform FMU
# Supports both Linux x86_64 and Windows x86_64 binaries in a single FMU
#
# Usage: ./scripts/build-multiplatform-fmu.sh <model-directory>
# Example: ./scripts/build-multiplatform-fmu.sh models/mathematical/dahlquist

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
echo "Building Multi-Platform FMU for: $PACKAGE_NAME"
echo "=========================================="

# Step 1: Build FMU for Linux x86_64 using cargo-fmi
echo "Step 1: Building for Linux x86_64..."
cargo fmi --package "$PACKAGE_NAME" bundle --release --target x86_64-unknown-linux-gnu

# Step 2: Build for Windows x86_64 using cargo-fmi with cross
echo "Step 2: Building for Windows x86_64 using cross..."
# cargo-fmi doesn't directly support cross, so we cross-compile the library
# and then merge it into the FMU
cross build -p "$PACKAGE_NAME" --release --target x86_64-pc-windows-gnu

# Step 3: Find the Linux FMU and merge in the Windows binary
echo "Step 3: Creating multi-platform FMU..."

# Convert hyphens to underscores for library name
LIB_NAME="${PACKAGE_NAME//-/_}"

# Find the Linux FMU
LINUX_FMU=$(find target/fmu -name "*.fmu" 2>/dev/null | head -1)

if [ -z "$LINUX_FMU" ]; then
    echo "Error: No Linux FMU found in target/fmu/"
    exit 1
fi

FMU_NAME=$(basename "$LINUX_FMU")
FMU_TEMP_DIR=$(mktemp -d)

# Extract Linux FMU
cd "$FMU_TEMP_DIR"
unzip -q "$OLDPWD/$LINUX_FMU"

# Add Windows binary
WINDOWS_DLL="$OLDPWD/target/x86_64-pc-windows-gnu/release/${LIB_NAME}.dll"
if [ ! -f "$WINDOWS_DLL" ]; then
    echo "Error: Windows binary not found at $WINDOWS_DLL"
    cd "$OLDPWD"
    rm -rf "$FMU_TEMP_DIR"
    exit 1
fi

# Determine model name from FMU filename (strip .fmu extension)
MODEL_NAME="${FMU_NAME%.fmu}"
WINDOWS_BIN_DIR="binaries/x86_64-windows"
mkdir -p "$WINDOWS_BIN_DIR"
cp "$WINDOWS_DLL" "$WINDOWS_BIN_DIR/${MODEL_NAME}.dll"
echo "  Added Windows binary: $WINDOWS_BIN_DIR/${MODEL_NAME}.dll"

# Repack the FMU
zip -r -q "${FMU_NAME}" .
cd "$OLDPWD"

# Move to output directory
FMU_OUTPUT_DIR="fmus"
mkdir -p "$FMU_OUTPUT_DIR"
cp "$FMU_TEMP_DIR/${FMU_NAME}" "$FMU_OUTPUT_DIR/"

# Cleanup
rm -rf "$FMU_TEMP_DIR"

echo "=========================================="
echo "Multi-platform FMU created: $FMU_OUTPUT_DIR/$FMU_NAME"
echo ""
echo "Platforms included:"
echo "  Linux x86_64"
echo "  Windows x86_64"
echo "=========================================="

exit 0
