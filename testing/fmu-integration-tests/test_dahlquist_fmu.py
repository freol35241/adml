"""
Integration tests for Dahlquist FMU

Tests the Dahlquist test equation: dx/dt = -k*x
Expected behavior: x(t) = x0 * exp(-k*t)
"""

import pytest
import numpy as np
from pathlib import Path
from fmu_test_utils import (
    find_fmu,
    simulate_fmu,
    compare_with_analytical,
    validate_fmu_structure
)


@pytest.fixture
def fmu_path():
    """Get path to Dahlquist FMU"""
    return find_fmu("Dahlquist")


@pytest.fixture
def default_params():
    """Default parameters for Dahlquist model"""
    return {
        'k': 1.0,  # Decay rate
        # Note: 'x' is an output, not settable as a parameter
        # It uses the start value from the model definition (1.0)
    }


class TestDahlquistFMUStructure:
    """Test FMU structure and metadata"""

    def test_fmu_exists(self, fmu_path):
        """FMU file should exist"""
        assert fmu_path.exists(), f"FMU not found at {fmu_path}"

    def test_fmu_structure(self, fmu_path):
        """FMU should have valid structure"""
        metadata = validate_fmu_structure(fmu_path)

        assert metadata['fmi_version'] == '3.0'
        assert metadata['model_name'] == 'adml-dahlquist'
        assert metadata['cosimulation_supported'] == True

    def test_fmu_variables(self, fmu_path):
        """FMU should have expected variables"""
        metadata = validate_fmu_structure(fmu_path)
        variables = metadata['variables']

        # Check that required variables exist
        assert 'k' in variables
        assert 'x' in variables

        # Check variable causalities
        assert variables['k']['causality'] == 'parameter'
        assert variables['x']['causality'] == 'output'


class TestDahlquistFMUSimulation:
    """Test FMU simulation behavior"""

    def test_analytical_solution_default(self, fmu_path, default_params):
        """FMU results should match analytical solution with default parameters"""
        # Simulate with small step size for accuracy
        stop_time = 5.0
        x0 = 1.0  # Default start value from model
        time, results = simulate_fmu(
            fmu_path,
            stop_time=stop_time,
            parameters=default_params,
            step_size=0.01,  # Small step for Euler accuracy
            output_interval=0.1
        )

        # Analytical solution: x(t) = x0 * exp(-k*t)
        def analytical(t):
            return x0 * np.exp(-default_params['k'] * t)

        # Compare
        matches, max_error = compare_with_analytical(
            time, results['x'], analytical,
            rtol=5e-2,  # 5% relative tolerance (Euler integration has some error)
            atol=1e-3   # Absolute tolerance for small values
        )

        assert matches, f"FMU results don't match analytical solution. Max error: {max_error}"

    def test_analytical_solution_different_k(self, fmu_path):
        """FMU should work with different decay rates"""
        x0 = 1.0  # Default start value from model
        for k in [0.5, 1.0, 2.0, 5.0]:
            params = {'k': k}
            time, results = simulate_fmu(
                fmu_path,
                stop_time=3.0,
                parameters=params,
                step_size=0.01,  # Small step for accuracy
                output_interval=0.1
            )

            def analytical(t):
                return x0 * np.exp(-k * t)

            matches, max_error = compare_with_analytical(
                time, results['x'], analytical,
                rtol=5e-2,  # 5% tolerance for Euler method
                atol=1e-2   # Absolute tolerance for small values (Euler accumulates error)
            )

            assert matches, f"Failed for k={k}. Max error: {max_error}"

    def test_half_life(self, fmu_path):
        """At half-life, x should be 0.5 * x0"""
        k = 1.0
        x0 = 1.0  # Default start value from model
        half_life = np.log(2) / k  # t_half = ln(2) / k

        params = {'k': k}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=half_life,
            parameters=params,
            step_size=0.01,  # Small step for accuracy
            output_interval=half_life  # Output at end time
        )

        # Get final value
        x_final = results['x'][-1]

        # Should be approximately 0.5 (with Euler error)
        assert np.isclose(x_final, 0.5, rtol=5e-2), \
            f"At half-life, x should be 0.5, got {x_final}"

    def test_exponential_decay(self, fmu_path):
        """Value should monotonically decrease for k > 0"""
        x0 = 1.0  # Default start value from model
        params = {'k': 1.0}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=params,
            step_size=0.01,  # Small step for stability
            output_interval=0.1
        )

        x = results['x']

        # Check monotonic decrease
        assert np.all(np.diff(x) <= 0), "x should monotonically decrease"

        # Check bounds
        assert np.all(x >= 0), "x should remain non-negative"
        assert np.all(x <= x0), "x should not exceed initial value"

    def test_asymptotic_behavior(self, fmu_path):
        """x should approach zero as t -> infinity"""
        params = {'k': 1.0}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=10.0,  # Long simulation
            parameters=params,
            step_size=0.01,
            output_interval=0.5
        )

        x_final = results['x'][-1]

        # Should be very close to zero
        assert x_final < 0.01, f"x should approach 0, got {x_final}"

    def test_different_k_values_stability(self, fmu_path):
        """FMU should remain stable with different k values"""
        x0 = 1.0  # Default start value from model
        for k in [0.5, 1.0, 2.0, 5.0]:
            params = {'k': k}
            time, results = simulate_fmu(
                fmu_path,
                stop_time=2.0,
                parameters=params,
                step_size=0.01,
                output_interval=0.1
            )

            # Verify simulation completed and is stable
            x = results['x']
            assert np.all(np.isfinite(x)), f"Results should be finite for k={k}"
            assert np.all(x >= 0), f"Results should be non-negative for k={k}"
            assert x[-1] < x[0], f"Should decay for k={k} > 0"


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
