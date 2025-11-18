#!/bin/bash
# Build all FMU models in the repository
#
# This script discovers and builds all models into FMUs

set -e

echo "==========================================="
echo "Building All FMUs"
echo "==========================================="
echo ""

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

# Find all model directories (containing Cargo.toml with fmu_from_struct dependency)
MODEL_DIRS=()

echo "Discovering models..."
for toml in models/*/*/Cargo.toml; do
    if [ -f "$toml" ]; then
        dir=$(dirname "$toml")
        # Check if it has fmu_from_struct dependency
        if grep -q "fmu_from_struct" "$toml"; then
            MODEL_DIRS+=("$dir")
            echo "  Found: $dir"
        fi
    fi
done

echo ""
echo "Found ${#MODEL_DIRS[@]} models to build"
echo "==========================================="
echo ""

# Build each model
SUCCESS_COUNT=0
FAIL_COUNT=0
FAILED_MODELS=()

for model_dir in "${MODEL_DIRS[@]}"; do
    model_name=$(basename "$model_dir")
    echo ""
    echo "-------------------------------------------"
    echo "Building: $model_name"
    echo "-------------------------------------------"

    if "$SCRIPT_DIR/build-fmu.sh" "$model_dir"; then
        echo "✅ Successfully built $model_name"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        echo "❌ Failed to build $model_name"
        FAILED_MODELS+=("$model_name")
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

echo ""
echo "==========================================="
echo "Build Summary"
echo "==========================================="
echo "Total models: ${#MODEL_DIRS[@]}"
echo "Successful:   $SUCCESS_COUNT"
echo "Failed:       $FAIL_COUNT"

if [ $FAIL_COUNT -gt 0 ]; then
    echo ""
    echo "Failed models:"
    for model in "${FAILED_MODELS[@]}"; do
        echo "  - $model"
    done
    exit 1
fi

echo ""
echo "✅ All FMUs built successfully!"
echo "FMU files are in: $REPO_ROOT/fmus/"

# List FMU files
echo ""
echo "Generated FMUs:"
ls -lh "$REPO_ROOT/fmus/"*.fmu 2>/dev/null || echo "  (none found)"

exit 0
