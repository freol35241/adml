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
        'x': 1.0,  # Initial value
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
        assert metadata['model_name'] == 'Dahlquist'
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
        # Simulate
        stop_time = 5.0
        time, results = simulate_fmu(
            fmu_path,
            stop_time=stop_time,
            parameters=default_params,
            output_interval=0.1
        )

        # Analytical solution: x(t) = x0 * exp(-k*t)
        def analytical(t):
            return default_params['x'] * np.exp(-default_params['k'] * t)

        # Compare
        matches, max_error = compare_with_analytical(
            time, results['x'], analytical,
            rtol=1e-2,  # 1% relative tolerance (Euler integration)
            atol=1e-6
        )

        assert matches, f"FMU results don't match analytical solution. Max error: {max_error}"

    def test_analytical_solution_different_k(self, fmu_path):
        """FMU should work with different decay rates"""
        for k in [0.5, 1.0, 2.0, 5.0]:
            params = {'k': k, 'x': 1.0}
            time, results = simulate_fmu(
                fmu_path,
                stop_time=3.0,
                parameters=params,
                output_interval=0.1
            )

            def analytical(t):
                return params['x'] * np.exp(-k * t)

            matches, max_error = compare_with_analytical(
                time, results['x'], analytical,
                rtol=1e-2,
                atol=1e-6
            )

            assert matches, f"Failed for k={k}. Max error: {max_error}"

    def test_half_life(self, fmu_path):
        """At half-life, x should be 0.5 * x0"""
        k = 1.0
        x0 = 1.0
        half_life = np.log(2) / k  # t_half = ln(2) / k

        params = {'k': k, 'x': x0}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=half_life,
            parameters=params,
            output_interval=half_life  # Output at end time
        )

        # Get final value
        x_final = results['x'][-1]

        # Should be approximately 0.5
        assert np.isclose(x_final, 0.5, rtol=1e-2), \
            f"At half-life, x should be 0.5, got {x_final}"

    def test_exponential_decay(self, fmu_path):
        """Value should monotonically decrease for k > 0"""
        params = {'k': 1.0, 'x': 1.0}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=params,
            output_interval=0.1
        )

        x = results['x']

        # Check monotonic decrease
        assert np.all(np.diff(x) <= 0), "x should monotonically decrease"

        # Check bounds
        assert np.all(x >= 0), "x should remain non-negative"
        assert np.all(x <= params['x']), "x should not exceed initial value"

    def test_asymptotic_behavior(self, fmu_path):
        """x should approach zero as t -> infinity"""
        params = {'k': 1.0, 'x': 1.0}
        time, results = simulate_fmu(
            fmu_path,
            stop_time=10.0,  # Long simulation
            parameters=params,
            output_interval=0.5
        )

        x_final = results['x'][-1]

        # Should be very close to zero
        assert x_final < 0.01, f"x should approach 0, got {x_final}"

    def test_different_initial_conditions(self, fmu_path):
        """FMU should work with different initial conditions"""
        for x0 in [0.5, 1.0, 2.0, 10.0]:
            params = {'k': 1.0, 'x': x0}
            time, results = simulate_fmu(
                fmu_path,
                stop_time=2.0,
                parameters=params,
                output_interval=0.1
            )

            def analytical(t):
                return x0 * np.exp(-1.0 * t)

            matches, max_error = compare_with_analytical(
                time, results['x'], analytical,
                rtol=1e-2,
                atol=1e-6
            )

            assert matches, f"Failed for x0={x0}. Max error: {max_error}"


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
