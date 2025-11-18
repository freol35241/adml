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

# Convert hyphens to underscores for library name
LIB_NAME="${PACKAGE_NAME//-/_}"

echo "=========================================="
echo "Building Multi-Platform FMU for: $PACKAGE_NAME"
echo "Library name: $LIB_NAME"
echo "=========================================="

# Step 1: Build for Linux x86_64
echo "Step 1: Building for Linux x86_64..."
rm -f modelDescription.xml
cargo clean -p "$PACKAGE_NAME" --release
cargo build -p "$PACKAGE_NAME" --release --target x86_64-unknown-linux-gnu

# Save the modelDescription.xml
if [ ! -f "modelDescription.xml" ]; then
    echo "Error: modelDescription.xml not found after Linux build"
    exit 1
fi
cp modelDescription.xml modelDescription.xml.tmp

# Step 2: Build for Windows x86_64 using cross
echo "Step 2: Building for Windows x86_64 using cross..."
cross build -p "$PACKAGE_NAME" --release --target x86_64-pc-windows-gnu

# Step 3: Read model information from modelDescription.xml
MODEL_NAME=$(grep -oP 'modelName="\K[^"]+' modelDescription.xml.tmp)
FMI_VERSION=$(grep -oP 'fmiVersion="\K[^"]+' modelDescription.xml.tmp)

echo "Model name: $MODEL_NAME"
echo "FMI version: $FMI_VERSION"

# Determine binary folder paths based on FMI version
if [[ "$FMI_VERSION" == "3.0" ]]; then
    LINUX_BIN_DIR="binaries/x86_64-linux"
    WINDOWS_BIN_DIR="binaries/x86_64-windows"
elif [[ "$FMI_VERSION" == "2.0" ]]; then
    LINUX_BIN_DIR="binaries/linux64"
    WINDOWS_BIN_DIR="binaries/win64"
else
    echo "Error: Unsupported FMI version: $FMI_VERSION"
    exit 1
fi

# Step 4: Create FMU directory structure
echo "Step 3: Creating multi-platform FMU..."
FMU_TEMP_DIR=$(mktemp -d)
FMU_NAME="${MODEL_NAME}.fmu"

# Create directory structure
mkdir -p "$FMU_TEMP_DIR/$LINUX_BIN_DIR"
mkdir -p "$FMU_TEMP_DIR/$WINDOWS_BIN_DIR"
mkdir -p "$FMU_TEMP_DIR/resources"

# Copy modelDescription.xml
cp modelDescription.xml.tmp "$FMU_TEMP_DIR/modelDescription.xml"

# Copy Linux binary
LINUX_SO="target/x86_64-unknown-linux-gnu/release/lib${LIB_NAME}.so"
if [ ! -f "$LINUX_SO" ]; then
    echo "Error: Linux binary not found at $LINUX_SO"
    rm -rf "$FMU_TEMP_DIR"
    rm -f modelDescription.xml.tmp
    exit 1
fi
cp "$LINUX_SO" "$FMU_TEMP_DIR/$LINUX_BIN_DIR/${MODEL_NAME}.so"
echo "  ✓ Added Linux binary: $LINUX_BIN_DIR/${MODEL_NAME}.so"

# Copy Windows binary
WINDOWS_DLL="target/x86_64-pc-windows-gnu/release/${LIB_NAME}.dll"
if [ ! -f "$WINDOWS_DLL" ]; then
    echo "Error: Windows binary not found at $WINDOWS_DLL"
    rm -rf "$FMU_TEMP_DIR"
    rm -f modelDescription.xml.tmp
    exit 1
fi
cp "$WINDOWS_DLL" "$FMU_TEMP_DIR/$WINDOWS_BIN_DIR/${MODEL_NAME}.dll"
echo "  ✓ Added Windows binary: $WINDOWS_BIN_DIR/${MODEL_NAME}.dll"

# Step 5: Create the FMU (zip file)
FMU_OUTPUT_DIR="fmus"
mkdir -p "$FMU_OUTPUT_DIR"

cd "$FMU_TEMP_DIR"
zip -r -q "../${FMU_NAME}" .
cd - > /dev/null

mv "$FMU_TEMP_DIR/../${FMU_NAME}" "$FMU_OUTPUT_DIR/"

# Cleanup
rm -rf "$FMU_TEMP_DIR"
rm -f modelDescription.xml.tmp

echo "=========================================="
echo "✅ Multi-platform FMU created: $FMU_OUTPUT_DIR/$FMU_NAME"
echo ""
echo "Platforms included:"
echo "  • Linux x86_64"
echo "  • Windows x86_64"
echo "=========================================="

exit 0
