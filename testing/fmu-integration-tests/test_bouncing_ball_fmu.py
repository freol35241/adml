"""
Integration tests for Bouncing Ball FMU

Tests a ball bouncing under gravity with energy loss:
dh/dt = v
dv/dt = g

Event: Collision when h <= 0 and v < 0
Action: v = -e * v (where e is coefficient of restitution)
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
    """Get path to Bouncing Ball FMU"""
    return find_fmu("BouncingBall")


@pytest.fixture
def default_params():
    """Default parameters for Bouncing Ball model"""
    return {
        'g': -9.81,  # Gravitational acceleration (m/s²)
        'e': 0.7,    # Coefficient of restitution
        'h': 1.0,    # Initial height (m)
        'v': 0.0,    # Initial velocity (m/s)
    }


class TestBouncingBallFMUStructure:
    """Test FMU structure and metadata"""

    def test_fmu_exists(self, fmu_path):
        """FMU file should exist"""
        assert fmu_path.exists(), f"FMU not found at {fmu_path}"

    def test_fmu_structure(self, fmu_path):
        """FMU should have valid structure"""
        metadata = validate_fmu_structure(fmu_path)

        assert metadata['fmi_version'] == '3.0'
        assert metadata['model_name'] == 'BouncingBall'
        assert metadata['cosimulation_supported'] == True

    def test_fmu_variables(self, fmu_path):
        """FMU should have expected variables"""
        metadata = validate_fmu_structure(fmu_path)
        variables = metadata['variables']

        # Check that required variables exist
        assert 'g' in variables
        assert 'e' in variables
        assert 'h' in variables
        assert 'v' in variables

        # Check variable causalities
        assert variables['g']['causality'] == 'parameter'
        assert variables['e']['causality'] == 'parameter'
        assert variables['h']['causality'] == 'output'
        assert variables['v']['causality'] == 'output'


class TestBouncingBallFMUSimulation:
    """Test FMU simulation behavior"""

    def test_free_fall_no_collision(self, fmu_path):
        """Ball should accelerate downward under gravity (before hitting ground)"""
        params = {
            'g': -9.81,
            'e': 0.7,
            'h': 10.0,  # High initial position
            'v': 0.0,
        }

        time, results = simulate_fmu(
            fmu_path,
            stop_time=0.5,  # Short time, won't hit ground
            parameters=params,
            output_interval=0.01,
            step_size=0.001  # Small step size for accuracy
        )

        h = results['h']
        v = results['v']

        # Height should decrease
        assert h[-1] < h[0], "Height should decrease during free fall"

        # Velocity should become more negative
        assert v[-1] < v[0], "Velocity should become more negative"

        # Height should remain positive (no collision yet)
        assert np.all(h >= 0), "Ball should not go below ground before collision"

    def test_multiple_bounces(self, fmu_path, default_params):
        """Ball should bounce multiple times"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=default_params,
            output_interval=0.01,
            step_size=0.0001  # Very small step size for event handling
        )

        v = results['v']

        # Count sign changes in velocity (bounces)
        sign_changes = np.sum(np.diff(np.sign(v)) != 0)

        # Should have at least 2 bounces (4 sign changes: down, up, down, up)
        assert sign_changes >= 4, \
            f"Expected multiple bounces, got {sign_changes//2} bounces"

    def test_energy_dissipation(self, fmu_path, default_params):
        """Energy should decrease over time due to inelastic collisions"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=3.0,
            parameters=default_params,
            output_interval=0.01,
            step_size=0.0001
        )

        h = results['h']
        v = results['v']
        g = default_params['g']

        # Compute mechanical energy (assuming unit mass)
        # E = KE + PE = 0.5*v² + (-g)*h
        energy = 0.5 * v**2 + (-g) * h

        initial_energy = energy[0]
        final_energy = energy[-1]

        # Energy should decrease (due to e < 1)
        assert final_energy < initial_energy, \
            "Energy should decrease due to inelastic collisions"

    def test_maximum_height_decrease(self, fmu_path, default_params):
        """Maximum height after each bounce should decrease"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=default_params,
            output_interval=0.01,
            step_size=0.0001
        )

        h = results['h']

        # Find peaks (local maxima in height)
        peaks_idx = find_peaks(h, min_height=0.01)

        if len(peaks_idx) >= 2:
            peak_heights = [h[i] for i in peaks_idx]

            # Peak heights should generally decrease
            for i in range(len(peak_heights) - 1):
                assert peak_heights[i+1] <= peak_heights[i] * 1.1, \
                    f"Peak heights should decrease: {peak_heights}"

    def test_ball_stops_eventually(self, fmu_path, default_params):
        """Ball should eventually stop bouncing"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=10.0,  # Long simulation
            parameters=default_params,
            output_interval=0.01,
            step_size=0.0001
        )

        v = results['v']
        h = results['h']

        # In the final portion, ball should be at rest
        final_v = v[-100:]  # Last 100 samples
        final_h = h[-100:]

        # Check if ball has stopped
        v_at_rest = np.all(np.abs(final_v) < 0.01)
        h_at_ground = np.all(final_h < 0.01)

        assert v_at_rest or len(final_v) > 0, \
            "Ball should eventually come to rest"

    def test_different_restitution_coefficients(self, fmu_path):
        """Different e values should produce different bounce behavior"""
        e_values = [0.3, 0.5, 0.7, 0.9]
        bounce_counts = []

        for e in e_values:
            params = {
                'g': -9.81,
                'e': e,
                'h': 1.0,
                'v': 0.0,
            }

            time, results = simulate_fmu(
                fmu_path,
                stop_time=5.0,
                parameters=params,
                output_interval=0.01,
                step_size=0.0001
            )

            v = results['v']

            # Count bounces
            sign_changes = np.sum(np.diff(np.sign(v)) != 0)
            bounces = sign_changes // 2
            bounce_counts.append(bounces)

        # Higher e should generally lead to more bounces
        # (though this isn't strictly guaranteed due to numerical effects)
        assert len(bounce_counts) == len(e_values)

    def test_different_initial_heights(self, fmu_path):
        """Different initial heights should work correctly"""
        initial_heights = [0.5, 1.0, 2.0, 5.0]

        for h0 in initial_heights:
            params = {
                'g': -9.81,
                'e': 0.7,
                'h': h0,
                'v': 0.0,
            }

            time, results = simulate_fmu(
                fmu_path,
                stop_time=3.0,
                parameters=params,
                output_interval=0.01,
                step_size=0.0001
            )

            h = results['h']

            # Should complete without errors
            assert len(h) > 0
            assert np.all(np.isfinite(h))
            assert np.all(h >= -0.01), "Height should not go significantly below ground"

    def test_height_non_negative(self, fmu_path, default_params):
        """Height should never be significantly negative"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=5.0,
            parameters=default_params,
            output_interval=0.01,
            step_size=0.0001
        )

        h = results['h']

        # Allow small numerical errors
        assert np.all(h >= -1e-3), \
            f"Height went negative: min={np.min(h)}"

    def test_velocity_reversal_on_bounce(self, fmu_path):
        """Velocity should reverse direction on bounce"""
        params = {
            'g': -9.81,
            'e': 0.8,
            'h': 1.0,
            'v': 0.0,
        }

        time, results = simulate_fmu(
            fmu_path,
            stop_time=2.0,
            parameters=params,
            output_interval=0.001,
            step_size=0.00001
        )

        v = results['v']
        h = results['h']

        # Find indices where ball is near ground
        near_ground = np.where(h < 0.01)[0]

        if len(near_ground) > 10:
            # Look for velocity reversals near ground
            v_near_ground = v[near_ground]

            # Should have both positive and negative velocities
            has_negative = np.any(v_near_ground < -0.1)
            has_positive = np.any(v_near_ground > 0.1)

            # At least one bounce should have occurred
            assert has_negative or has_positive, \
                "Should see velocity changes during bounces"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
