#!/bin/bash
# Check FMU compliance using FMU Checker
#
# Downloads and runs the official FMI FMU Checker tool
# Usage: ./scripts/check-fmu-compliance.sh <fmu-file>

set -e

if [ -z "$1" ]; then
    echo "Error: FMU file required"
    echo "Usage: $0 <fmu-file>"
    echo "Example: $0 fmus/Dahlquist.fmu"
    exit 1
fi

FMU_FILE="$1"

if [ ! -f "$FMU_FILE" ]; then
    echo "Error: FMU file not found: $FMU_FILE"
    exit 1
fi

echo "=========================================="
echo "FMU Compliance Checking"
echo "FMU: $FMU_FILE"
echo "=========================================="

# FMU Checker information
FMU_CHECKER_VERSION="2.0.4"
FMU_CHECKER_URL="https://github.com/modelica-tools/FMUComplianceChecker/releases/download/2.0.4/FMUChecker-2.0.4-linux64.zip"
FMU_CHECKER_DIR=".fmu-checker"
FMU_CHECKER_BIN="$FMU_CHECKER_DIR/fmuCheck.linux64"

# Download FMU Checker if not present
if [ ! -f "$FMU_CHECKER_BIN" ]; then
    echo "Downloading FMU Checker..."
    mkdir -p "$FMU_CHECKER_DIR"

    # Download
    wget -q "$FMU_CHECKER_URL" -O "$FMU_CHECKER_DIR/fmuchecker.zip"

    # Extract
    cd "$FMU_CHECKER_DIR"
    unzip -q fmuchecker.zip
    chmod +x fmuCheck.linux64
    cd - > /dev/null

    echo "FMU Checker installed to $FMU_CHECKER_DIR"
fi

# Run FMU Checker
echo ""
echo "Running FMU Checker..."
echo "=========================================="

# Run with various checks
"$FMU_CHECKER_BIN" \
    -h 0.001 \
    -s 10 \
    -o compliance_output.csv \
    -l 5 \
    "$FMU_FILE"

RESULT=$?

echo "=========================================="
if [ $RESULT -eq 0 ]; then
    echo "✅ FMU PASSED compliance check"
else
    echo "❌ FMU FAILED compliance check (exit code: $RESULT)"
fi
echo "=========================================="

exit $RESULT
