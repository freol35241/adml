#!/usr/bin/env python3
"""
Generate visualization figures from FMU simulations for CI feedback.

This script simulates all FMUs and generates plots showing their outputs,
which are saved as PNG files for use as CI artifacts and PR comments.
"""

import os
import sys
from pathlib import Path
import matplotlib
matplotlib.use('Agg')  # Use non-interactive backend for CI
import matplotlib.pyplot as plt
import numpy as np
from fmpy import simulate_fmu


def simulate_and_plot_dahlquist(fmu_path: Path, output_dir: Path):
    """Simulate Dahlquist FMU and generate plot."""
    print(f"Simulating Dahlquist FMU: {fmu_path}")

    # Run simulation
    result = simulate_fmu(
        str(fmu_path),
        stop_time=5.0,
        step_size=0.01,
        output_interval=0.01,
        start_values={'k': 1.0}
    )

    # Create plot
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.plot(result['time'], result['x'], 'b-', linewidth=2, label='x(t)')

    # Add analytical solution for comparison
    t = result['time']
    x_analytical = np.exp(-t)
    ax.plot(t, x_analytical, 'r--', linewidth=1, alpha=0.7, label='Analytical: e^(-t)')

    ax.set_xlabel('Time [s]', fontsize=12)
    ax.set_ylabel('State x', fontsize=12)
    ax.set_title('Dahlquist Test Equation: dx/dt = -x', fontsize=14, fontweight='bold')
    ax.grid(True, alpha=0.3)
    ax.legend(fontsize=10)

    # Save figure
    output_file = output_dir / 'dahlquist.png'
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"  ✓ Saved: {output_file}")


def simulate_and_plot_van_der_pol(fmu_path: Path, output_dir: Path):
    """Simulate Van der Pol FMU and generate plot."""
    print(f"Simulating Van der Pol FMU: {fmu_path}")

    # Run simulation
    result = simulate_fmu(
        str(fmu_path),
        stop_time=30.0,
        step_size=0.01,
        output_interval=0.05,
        start_values={'mu': 1.0}
    )

    # Create two subplots: time series and phase portrait
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))

    # Time series plot
    ax1.plot(result['time'], result['x0'], 'b-', linewidth=1.5, label='x₀ (position)')
    ax1.plot(result['time'], result['x1'], 'r-', linewidth=1.5, label='x₁ (velocity)')
    ax1.set_xlabel('Time [s]', fontsize=12)
    ax1.set_ylabel('State Variables', fontsize=12)
    ax1.set_title('Van der Pol Oscillator - Time Series', fontsize=13, fontweight='bold')
    ax1.grid(True, alpha=0.3)
    ax1.legend(fontsize=10)

    # Phase portrait
    ax2.plot(result['x0'], result['x1'], 'g-', linewidth=1.5, alpha=0.8)
    ax2.plot(result['x0'][0], result['x1'][0], 'go', markersize=10, label='Start')
    ax2.plot(result['x0'][-1], result['x1'][-1], 'rs', markersize=10, label='End')
    ax2.set_xlabel('x₀ (position)', fontsize=12)
    ax2.set_ylabel('x₁ (velocity)', fontsize=12)
    ax2.set_title('Phase Portrait - Limit Cycle', fontsize=13, fontweight='bold')
    ax2.grid(True, alpha=0.3)
    ax2.legend(fontsize=10)
    ax2.set_aspect('equal', adjustable='box')

    plt.tight_layout()

    # Save figure
    output_file = output_dir / 'van_der_pol.png'
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"  ✓ Saved: {output_file}")


def simulate_and_plot_bouncing_ball(fmu_path: Path, output_dir: Path):
    """Simulate Bouncing Ball FMU and generate plot."""
    print(f"Simulating Bouncing Ball FMU: {fmu_path}")

    # Run simulation
    result = simulate_fmu(
        str(fmu_path),
        stop_time=5.0,
        step_size=0.001,
        output_interval=0.01,
        start_values={'g': -9.81, 'e': 0.7}
    )

    # Create two subplots: height and velocity
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), sharex=True)

    # Height plot
    ax1.plot(result['time'], result['h'], 'b-', linewidth=1.5)
    ax1.axhline(y=0, color='k', linestyle='--', linewidth=1, alpha=0.5, label='Ground')
    ax1.set_ylabel('Height h [m]', fontsize=12)
    ax1.set_title('Bouncing Ball with Elastic Collisions (e=0.7)', fontsize=14, fontweight='bold')
    ax1.grid(True, alpha=0.3)
    ax1.legend(fontsize=10)
    ax1.set_ylim(bottom=-0.1)

    # Velocity plot
    ax2.plot(result['time'], result['v'], 'r-', linewidth=1.5)
    ax2.axhline(y=0, color='k', linestyle='--', linewidth=1, alpha=0.5)
    ax2.set_xlabel('Time [s]', fontsize=12)
    ax2.set_ylabel('Velocity v [m/s]', fontsize=12)
    ax2.set_title('Velocity with Collision Events', fontsize=13, fontweight='bold')
    ax2.grid(True, alpha=0.3)

    plt.tight_layout()

    # Save figure
    output_file = output_dir / 'bouncing_ball.png'
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"  ✓ Saved: {output_file}")


def main():
    """Main entry point."""
    # Setup paths
    script_dir = Path(__file__).parent.resolve()
    project_root = script_dir.parent
    fmu_dir = project_root / 'fmus'
    output_dir = project_root / 'fmu-figures'

    # Create output directory
    output_dir.mkdir(exist_ok=True)
    print(f"Output directory: {output_dir}")
    print()

    # Define FMU models
    fmus = {
        'Dahlquist': (fmu_dir / 'Dahlquist.fmu', simulate_and_plot_dahlquist),
        'VanDerPol': (fmu_dir / 'VanDerPol.fmu', simulate_and_plot_van_der_pol),
        'BouncingBall': (fmu_dir / 'BouncingBall.fmu', simulate_and_plot_bouncing_ball),
    }

    # Process each FMU
    success_count = 0
    errors = []

    for name, (fmu_path, plot_func) in fmus.items():
        try:
            if not fmu_path.exists():
                raise FileNotFoundError(f"FMU not found: {fmu_path}")

            plot_func(fmu_path, output_dir)
            success_count += 1

        except Exception as e:
            error_msg = f"Error processing {name}: {str(e)}"
            print(f"  ✗ {error_msg}")
            errors.append(error_msg)

    # Summary
    print()
    print("=" * 70)
    print(f"Summary: {success_count}/{len(fmus)} FMUs processed successfully")

    if errors:
        print("\nErrors encountered:")
        for error in errors:
            print(f"  - {error}")
        return 1

    print(f"\nAll figures saved to: {output_dir}")
    return 0


if __name__ == '__main__':
    sys.exit(main())
