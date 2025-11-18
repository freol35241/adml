#!/bin/bash
# Test all models in the workspace (Rust + Python FMU integration tests)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT"

echo "==============================================="
echo "🧪 Running All Tests (3-Tier Testing Strategy)"
echo "==============================================="
echo ""

# Tier 1: Rust Unit & Physics Tests
echo "📍 Tier 1: Rust Unit & Physics Tests"
echo "-----------------------------------------------"
cargo test --workspace
echo "✅ Rust tests passed"
echo ""

# Tier 2: FMU Integration Tests (Python + FMPy)
echo "📍 Tier 2: FMU Integration Tests (Python + FMPy)"
echo "-----------------------------------------------"
if [ -d "fmus" ] && [ -n "$(ls -A fmus/*.fmu 2>/dev/null)" ]; then
    echo "FMU files found, running integration tests..."
    cd testing/fmu-integration-tests

    # Check if Python dependencies are installed
    if command -v python >/dev/null 2>&1 && python -c "import fmpy" 2>/dev/null; then
        python -m pytest -v --tb=short
        echo "✅ FMU integration tests passed"
    else
        echo "⚠️  Skipping FMU integration tests (Python dependencies not installed)"
        echo "   Install with: pip install -r testing/requirements.txt"
    fi
    cd "$REPO_ROOT"
else
    echo "⚠️  Skipping FMU integration tests (no FMU files found)"
    echo "   Build FMUs first with: ./scripts/build-all-fmus.sh"
fi
echo ""

# Tier 3: FMI Compliance Checking
echo "📍 Tier 3: FMI Compliance Checking"
echo "-----------------------------------------------"
echo "ℹ️  Run manually with: ./scripts/check-fmu-compliance.sh fmus/<ModelName>.fmu"
echo ""

echo "==============================================="
echo "✅ All Available Tests Passed!"
echo "==============================================="
