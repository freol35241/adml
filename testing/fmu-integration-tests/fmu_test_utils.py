"""
Utilities for FMU integration testing

This module provides helper functions for loading, simulating, and validating
FMU models using FMPy.
"""

import os
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import numpy as np


def find_fmu(model_name: str, fmu_dir: str = "../../fmus") -> Path:
    """
    Find an FMU file by model name.

    Args:
        model_name: Name of the model (e.g., "Dahlquist")
        fmu_dir: Directory containing FMU files

    Returns:
        Path to the FMU file

    Raises:
        FileNotFoundError: If FMU file is not found
    """
    script_dir = Path(__file__).parent
    fmu_path = script_dir / fmu_dir / f"{model_name}.fmu"

    if not fmu_path.exists():
        # Try alternative locations
        alt_path = Path(fmu_dir) / f"{model_name}.fmu"
        if alt_path.exists():
            return alt_path
        raise FileNotFoundError(
            f"FMU file not found: {model_name}.fmu\n"
            f"Searched in: {fmu_path}, {alt_path}\n"
            f"Please build the FMU first using: ./scripts/build-fmu.sh"
        )

    return fmu_path


def simulate_fmu(
    fmu_path: Path,
    stop_time: float,
    step_size: Optional[float] = None,
    parameters: Optional[Dict[str, float]] = None,
    output_interval: Optional[float] = None
) -> Tuple[np.ndarray, Dict[str, np.ndarray]]:
    """
    Simulate an FMU and return results.

    Args:
        fmu_path: Path to the FMU file
        stop_time: Simulation end time
        step_size: Integration step size (optional, auto if None)
        parameters: Dictionary of parameter values to set
        output_interval: Output sampling interval (optional)

    Returns:
        Tuple of (time_array, results_dict) where results_dict maps
        variable names to their time series arrays
    """
    try:
        from fmpy import simulate_fmu as fmpy_simulate
        from fmpy import read_model_description
    except ImportError:
        raise ImportError(
            "FMPy is required for FMU testing. Install with:\n"
            "pip install fmpy"
        )

    # Read model description to get variable names
    model_description = read_model_description(fmu_path, validate=True)

    # Prepare start values from parameters
    start_values = parameters if parameters else {}

    # IMPORTANT: FMPy's step_size is the communication step size for co-simulation.
    # When output_interval > step_size, FMPy may not respect the step_size properly.
    # To ensure accurate integration with Euler method, we:
    # 1. Always use step_size as the communication step size
    # 2. Record at step_size intervals (not output_interval)
    # 3. Downsample results if needed for output_interval

    # Determine actual step size to use
    if step_size is not None:
        actual_step_size = step_size
    else:
        # Use output_interval or auto-determine
        actual_step_size = output_interval if output_interval is not None else None

    # Run simulation with step_size for both communication and output
    # This ensures the FMU doStep() is called with the correct small time steps
    result = fmpy_simulate(
        str(fmu_path),
        stop_time=stop_time,
        step_size=actual_step_size,
        output_interval=actual_step_size,  # Record at every step for accuracy
        start_values=start_values,
        validate=True,
        fmi_call_logger=None,  # Can enable for debugging
    )

    # Extract time and convert to dictionary format
    time = result['time']
    results = {col: result[col] for col in result.dtype.names if col != 'time'}

    # Downsample if output_interval was specified and is larger than step_size
    if output_interval is not None and step_size is not None and output_interval > step_size:
        # Create mask for output_interval sampling
        # Include first and last points, plus points at output_interval
        mask = np.zeros(len(time), dtype=bool)
        mask[0] = True  # Always include start
        mask[-1] = True  # Always include end

        # Add points at output_interval
        current_time = 0.0
        while current_time <= stop_time:
            idx = np.argmin(np.abs(time - current_time))
            mask[idx] = True
            current_time += output_interval

        # Downsample
        time = time[mask]
        results = {key: val[mask] for key, val in results.items()}

    return time, results


def compare_with_analytical(
    time: np.ndarray,
    numerical: np.ndarray,
    analytical_func,
    rtol: float = 1e-3,
    atol: float = 1e-6
) -> Tuple[bool, float]:
    """
    Compare numerical FMU results with analytical solution.

    Args:
        time: Time array
        numerical: Numerical results from FMU
        analytical_func: Function that takes time and returns analytical solution
        rtol: Relative tolerance
        atol: Absolute tolerance

    Returns:
        Tuple of (all_close, max_error)
    """
    analytical = np.array([analytical_func(t) for t in time])
    all_close = np.allclose(numerical, analytical, rtol=rtol, atol=atol)
    max_error = np.max(np.abs(numerical - analytical))

    return all_close, max_error


def check_energy_conservation(
    time: np.ndarray,
    energy: np.ndarray,
    tolerance: float = 0.1
) -> Tuple[bool, float]:
    """
    Check if total energy is conserved within tolerance.

    Args:
        time: Time array
        energy: Energy time series
        tolerance: Maximum allowed relative change in energy

    Returns:
        Tuple of (is_conserved, max_relative_change)
    """
    initial_energy = energy[0]
    if abs(initial_energy) < 1e-10:
        # Energy is near zero, use absolute change
        max_change = np.max(np.abs(energy - initial_energy))
        return max_change < tolerance, max_change

    relative_changes = np.abs((energy - initial_energy) / initial_energy)
    max_relative_change = np.max(relative_changes)
    is_conserved = max_relative_change < tolerance

    return is_conserved, max_relative_change


def find_peaks(signal: np.ndarray, min_height: float = 0.0) -> List[int]:
    """
    Find peaks in a signal (simple implementation).

    Args:
        signal: Input signal
        min_height: Minimum peak height

    Returns:
        List of peak indices
    """
    peaks = []
    for i in range(1, len(signal) - 1):
        if signal[i] > signal[i-1] and signal[i] > signal[i+1] and signal[i] > min_height:
            peaks.append(i)
    return peaks


def validate_fmu_structure(fmu_path: Path) -> Dict[str, any]:
    """
    Validate FMU structure and extract metadata.

    Args:
        fmu_path: Path to the FMU file

    Returns:
        Dictionary with FMU metadata
    """
    try:
        from fmpy import read_model_description, extract
        from fmpy.util import fmu_info
    except ImportError:
        raise ImportError("FMPy is required. Install with: pip install fmpy")

    # Extract FMU to temporary directory for inspection
    model_description = read_model_description(fmu_path, validate=True)

    metadata = {
        'fmi_version': model_description.fmiVersion,
        'model_name': model_description.modelName,
        'guid': model_description.guid,
        'description': model_description.description,
        'generation_tool': model_description.generationTool,
        'cosimulation_supported': model_description.coSimulation is not None,
        'model_exchange_supported': model_description.modelExchange is not None,
        'variables': {}
    }

    # Extract variable information
    for variable in model_description.modelVariables:
        metadata['variables'][variable.name] = {
            'causality': variable.causality,
            'variability': variable.variability,
            'start': getattr(variable, 'start', None)
        }

    return metadata
