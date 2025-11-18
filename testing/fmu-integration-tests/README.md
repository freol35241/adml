# FMU Integration Tests

This directory contains integration tests for FMU models using FMPy.

## Overview

These tests validate that the FMU models:
1. **Load correctly** - FMU structure and metadata are valid
2. **Simulate accurately** - Results match expected physics and analytical solutions
3. **Handle edge cases** - Different parameter values and initial conditions
4. **Are FMI compliant** - Follow FMI 3.0 standard

## Prerequisites

### Python Environment

```bash
# Create virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install requirements
pip install -r ../requirements.txt
```

### Build FMUs

Before running tests, you need to build the FMU files:

```bash
# From repository root
./scripts/build-fmu.sh models/mathematical/dahlquist
./scripts/build-fmu.sh models/mathematical/van-der-pol
./scripts/build-fmu.sh models/mechanical/bouncing-ball
```

FMU files should be placed in the `fmus/` directory.

## Running Tests

### Run All Tests

```bash
# From this directory
pytest -v

# With coverage
pytest -v --cov=. --cov-report=html
```

### Run Specific Model Tests

```bash
# Test only Dahlquist FMU
pytest test_dahlquist_fmu.py -v

# Test only Van der Pol FMU
pytest test_van_der_pol_fmu.py -v

# Test only Bouncing Ball FMU
pytest test_bouncing_ball_fmu.py -v
```

### Run in Parallel

```bash
# Use pytest-xdist for parallel execution
pytest -v -n auto
```

## Test Structure

### Test Files

- `test_dahlquist_fmu.py` - Tests for Dahlquist test equation
- `test_van_der_pol_fmu.py` - Tests for Van der Pol oscillator
- `test_bouncing_ball_fmu.py` - Tests for bouncing ball model
- `fmu_test_utils.py` - Shared utilities for FMU testing

### Test Categories

Each model has two test classes:

1. **Structure Tests** (`TestXXXFMUStructure`)
   - FMU file exists
   - Valid FMI 3.0 structure
   - Correct variables and metadata

2. **Simulation Tests** (`TestXXXFMUSimulation`)
   - Physics validation
   - Analytical solution comparison
   - Parameter sensitivity
   - Edge cases and boundary conditions

## Test Details

### Dahlquist Model Tests

The Dahlquist test equation (dx/dt = -k*x) has an analytical solution:
**x(t) = x₀ * exp(-k*t)**

Tests verify:
- ✅ Analytical solution matching (within integration error)
- ✅ Half-life calculation
- ✅ Exponential decay behavior
- ✅ Asymptotic approach to zero
- ✅ Different decay rates (k values)
- ✅ Different initial conditions

### Van der Pol Model Tests

The Van der Pol oscillator exhibits limit cycle behavior for μ > 0.

Tests verify:
- ✅ Oscillatory behavior
- ✅ Equilibrium at origin
- ✅ Limit cycle convergence from different initial conditions
- ✅ Different μ values
- ✅ Bounded trajectories (no divergence)
- ✅ Phase space behavior

### Bouncing Ball Model Tests

The bouncing ball has event-driven dynamics with collisions.

Tests verify:
- ✅ Free fall acceleration
- ✅ Multiple bounces
- ✅ Energy dissipation (e < 1)
- ✅ Decreasing bounce heights
- ✅ Ball eventually stops
- ✅ Different restitution coefficients
- ✅ Different initial heights
- ✅ Non-negative height constraint
- ✅ Velocity reversal on bounce

## Utilities

### `fmu_test_utils.py`

Provides helper functions:

- `find_fmu(model_name)` - Locate FMU file
- `simulate_fmu(...)` - Run FMU simulation with FMPy
- `compare_with_analytical(...)` - Compare with analytical solution
- `check_energy_conservation(...)` - Validate energy conservation
- `find_peaks(...)` - Detect oscillation peaks
- `validate_fmu_structure(...)` - Check FMU metadata

## Troubleshooting

### FMU Not Found

If tests fail with "FMU not found", ensure:
1. FMUs are built: `./scripts/build-fmu.sh <model-path>`
2. FMU files are in `../../fmus/` directory
3. FMU filenames match model names (e.g., `Dahlquist.fmu`)

### Import Errors

If you get import errors:
```bash
pip install -r ../requirements.txt
```

### Simulation Failures

If simulations fail or produce incorrect results:
1. Check step size - bouncing ball needs very small steps (1e-4 or smaller)
2. Verify FMU was built from latest code
3. Check FMU structure with: `fmpy info <fmu-file>`

## CI Integration

These tests are designed to run in CI pipelines. See `../.github/workflows/` for integration.

## Adding New Tests

To add tests for a new model:

1. Create `test_<model_name>_fmu.py`
2. Follow the existing structure (Structure + Simulation test classes)
3. Add model-specific physics validation
4. Update this README

## References

- [FMPy Documentation](https://github.com/CATIA-Systems/FMPy)
- [FMI Standard](https://fmi-standard.org/)
- [pytest Documentation](https://docs.pytest.org/)
