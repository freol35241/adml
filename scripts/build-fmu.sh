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

# Get the FMI model_name from Cargo.toml metadata (used for FMU filename)
MODEL_NAME=$(grep 'model_name' "$MODEL_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

# Derive the crate-name-based FMU filename that cargo-fmi produces
# cargo-fmi uses the crate name with hyphens replaced by underscores
CRATE_FMU_NAME="${PACKAGE_NAME//-/_}"

echo "=========================================="
echo "Building FMU for: $PACKAGE_NAME (model: $MODEL_NAME)"
echo "=========================================="

# Build and package the FMU using cargo-fmi
echo "Building and packaging FMU..."
cargo fmi --package "$PACKAGE_NAME" bundle --release

# Move FMU to output directory
FMU_OUTPUT_DIR="fmus"
mkdir -p "$FMU_OUTPUT_DIR"

# cargo-fmi outputs to target/fmu/{crate_name}.fmu
FMU_FILE="target/fmu/${CRATE_FMU_NAME}.fmu"

if [ ! -f "$FMU_FILE" ]; then
    echo "Error: Expected FMU not found at $FMU_FILE"
    echo "Contents of target/fmu/:"
    ls -la target/fmu/ 2>/dev/null || echo "  (directory does not exist)"
    exit 1
fi

# Copy to output dir, renaming to model_name if available
if [ -n "$MODEL_NAME" ]; then
    OUTPUT_FMU="$FMU_OUTPUT_DIR/${MODEL_NAME}.fmu"
else
    OUTPUT_FMU="$FMU_OUTPUT_DIR/${CRATE_FMU_NAME}.fmu"
fi
cp "$FMU_FILE" "$OUTPUT_FMU"

# Fix ModelStructure element ordering (workaround for fmi-export v0.1.1 bug)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
python3 "$SCRIPT_DIR/fix_model_structure.py" "$OUTPUT_FMU"

echo "=========================================="
echo "FMU created: $OUTPUT_FMU"
echo "=========================================="

exit 0
