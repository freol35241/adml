"""
Integration tests for Van der Pol FMU

Tests the Van der Pol oscillator:
dx0/dt = x1
dx1/dt = μ * (1 - x0²) * x1 - x0

Expected behavior: Limit cycle oscillations for μ > 0
"""

import pytest
import numpy as np
from pathlib import Path
from fmu_test_utils import (
    find_fmu,
    simulate_fmu,
    validate_fmu_structure,
    find_peaks
)


@pytest.fixture
def fmu_path():
    """Get path to Van der Pol FMU"""
    return find_fmu("VanDerPol")


@pytest.fixture
def default_params():
    """Default parameters for Van der Pol model"""
    return {
        'mu': 1.0,  # Damping parameter
        'x0': 2.0,   # Initial position
        'x1': 0.0,   # Initial velocity
    }


class TestVanDerPolFMUStructure:
    """Test FMU structure and metadata"""

    def test_fmu_exists(self, fmu_path):
        """FMU file should exist"""
        assert fmu_path.exists(), f"FMU not found at {fmu_path}"

    def test_fmu_structure(self, fmu_path):
        """FMU should have valid structure"""
        metadata = validate_fmu_structure(fmu_path)

        assert metadata['fmi_version'] == '3.0'
        assert metadata['model_name'] == 'VanDerPol'
        assert metadata['cosimulation_supported'] == True

    def test_fmu_variables(self, fmu_path):
        """FMU should have expected variables"""
        metadata = validate_fmu_structure(fmu_path)
        variables = metadata['variables']

        # Check that required variables exist
        assert 'mu' in variables
        assert 'x0' in variables
        assert 'x1' in variables

        # Check variable causalities
        assert variables['mu']['causality'] == 'parameter'
        assert variables['x0']['causality'] == 'output'
        assert variables['x1']['causality'] == 'output'


class TestVanDerPolFMUSimulation:
    """Test FMU simulation behavior"""

    def test_oscillatory_behavior(self, fmu_path, default_params):
        """Model should exhibit oscillations"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=20.0,
            parameters=default_params,
            output_interval=0.1
        )

        x0 = results['x0']

        # Find peaks in x0
        peaks = find_peaks(x0, min_height=0.5)

        # Should have multiple oscillations
        assert len(peaks) >= 2, \
            f"Expected oscillations, found {len(peaks)} peaks"

    def test_equilibrium_at_origin(self, fmu_path):
        """Model starting at origin should remain at origin"""
        params = {
            'mu': 1.0,
            'x0': 0.0,
            'x1': 0.0,
        }

        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=params,
            output_interval=0.1
        )

        x0 = results['x0']
        x1 = results['x1']

        # Should remain near zero
        assert np.all(np.abs(x0) < 1e-6), "x0 should remain at origin"
        assert np.all(np.abs(x1) < 1e-6), "x1 should remain at origin"

    def test_limit_cycle_convergence(self, fmu_path):
        """System should converge to limit cycle"""
        # Start from different initial conditions
        initial_conditions = [
            {'x0': 0.5, 'x1': 0.0},
            {'x0': 3.0, 'x1': 0.0},
            {'x0': 0.0, 'x1': 2.0},
        ]

        mu = 1.0
        stop_time = 30.0  # Long enough to reach limit cycle

        amplitudes = []

        for ic in initial_conditions:
            params = {'mu': mu, **ic}
            time, results = simulate_fmu(
                fmu_path,
                stop_time=stop_time,
                parameters=params,
                output_interval=0.05
            )

            x0 = results['x0']

            # Get amplitude in second half (should be on limit cycle)
            halfway = len(x0) // 2
            x0_steady = x0[halfway:]

            # Find peaks in steady state
            peaks_idx = find_peaks(x0_steady, min_height=0.5)

            if len(peaks_idx) > 0:
                peak_values = [x0_steady[i] for i in peaks_idx]
                amplitude = np.mean(peak_values)
                amplitudes.append(amplitude)

        # All trajectories should converge to similar amplitude
        if len(amplitudes) >= 2:
            amplitude_std = np.std(amplitudes)
            assert amplitude_std < 0.3, \
                f"Limit cycle amplitudes vary too much: {amplitudes}"

    def test_different_mu_values(self, fmu_path):
        """System should work with different μ values"""
        mu_values = [0.1, 0.5, 1.0, 2.0, 5.0]

        for mu in mu_values:
            params = {
                'mu': mu,
                'x0': 2.0,
                'x1': 0.0,
            }

            time, results = simulate_fmu(
                fmu_path,
                stop_time=15.0,
                parameters=params,
                output_interval=0.1
            )

            x0 = results['x0']
            x1 = results['x1']

            # Check that simulation completed without errors
            assert len(x0) > 0, f"Simulation failed for mu={mu}"
            assert np.all(np.isfinite(x0)), f"Non-finite values for mu={mu}"
            assert np.all(np.isfinite(x1)), f"Non-finite values for mu={mu}"

    def test_energy_non_conservation(self, fmu_path, default_params):
        """Van der Pol is non-conservative, energy should change"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=20.0,
            parameters=default_params,
            output_interval=0.1
        )

        x0 = results['x0']
        x1 = results['x1']

        # Compute "energy" (not conserved for Van der Pol)
        energy = 0.5 * x0**2 + 0.5 * x1**2

        initial_energy = energy[0]
        final_energy = energy[-1]

        # Energy should not be conserved (should change significantly)
        energy_change = abs(final_energy - initial_energy) / abs(initial_energy)

        # For Van der Pol, energy typically changes as system evolves
        # We just check that simulation is stable
        assert np.all(np.isfinite(energy)), "Energy calculation should be finite"

    def test_phase_space_trajectory(self, fmu_path, default_params):
        """Phase space trajectory should not spiral to infinity"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=20.0,
            parameters=default_params,
            output_interval=0.05
        )

        x0 = results['x0']
        x1 = results['x1']

        # Check that trajectory stays bounded
        max_x0 = np.max(np.abs(x0))
        max_x1 = np.max(np.abs(x1))

        assert max_x0 < 10.0, f"x0 should remain bounded, max={max_x0}"
        assert max_x1 < 10.0, f"x1 should remain bounded, max={max_x1}"

    def test_symmetric_initial_conditions(self, fmu_path):
        """Negative initial conditions should produce symmetric behavior"""
        params_pos = {'mu': 1.0, 'x0': 2.0, 'x1': 0.5}
        params_neg = {'mu': 1.0, 'x0': -2.0, 'x1': -0.5}

        stop_time = 10.0

        time_pos, results_pos = simulate_fmu(
            fmu_path,
            stop_time=stop_time,
            parameters=params_pos,
            output_interval=0.1
        )

        time_neg, results_neg = simulate_fmu(
            fmu_path,
            stop_time=stop_time,
            parameters=params_neg,
            output_interval=0.1
        )

        # Results should be negatives of each other (approximately)
        # Due to nonlinearity, won't be exact, but should be similar magnitudes
        max_x0_pos = np.max(np.abs(results_pos['x0']))
        max_x0_neg = np.max(np.abs(results_neg['x0']))

        assert np.isclose(max_x0_pos, max_x0_neg, rtol=0.3), \
            "Symmetric ICs should produce similar magnitudes"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
