#!/usr/bin/env python3
"""
Generate visualization figures from FMU simulations for CI feedback.

This script discovers FMUs, reads optional plot configurations, and generates
exactly one figure per FMU. Figures are saved as PNG files for CI artifacts
and PR comments.

See docs/plot_config_spec.md for configuration documentation.
"""

import argparse
import json
import sys
from pathlib import Path
from typing import Dict, List, Optional, Any

import matplotlib
matplotlib.use('Agg')  # Use non-interactive backend for CI
import matplotlib.pyplot as plt
import numpy as np
from fmpy import simulate_fmu
from fmpy.model_description import read_model_description


# Figure size (fixed for all FMUs)
FIGURE_WIDTH = 10  # inches
FIGURE_HEIGHT = 6  # inches
FIGURE_DPI = 150


def load_plot_config(model_dir: Path) -> Optional[Dict[str, Any]]:
    """Load plot configuration TOML file if it exists."""
    config_path = model_dir / 'plot_config.toml'
    if not config_path.exists():
        return None

    try:
        import tomli
    except ImportError:
        # Fall back to tomllib (Python 3.11+)
        import tomllib as tomli

    try:
        with open(config_path, 'rb') as f:
            config = tomli.load(f)
        return config
    except Exception as e:
        print(f"  ⚠️  Error parsing {config_path}: {e}")
        return None


def get_fmu_outputs(fmu_path: Path) -> List[str]:
    """Get list of output variable names from FMU model description."""
    model_desc = read_model_description(str(fmu_path))
    outputs = []
    for var in model_desc.modelVariables:
        if var.causality == 'output':
            outputs.append(var.name)
    return outputs


def create_generic_plot(fmu_path: Path, model_name: str, output_dir: Path) -> bool:
    """Create a generic time-series plot for an FMU without configuration."""
    print(f"  Using generic plotting (no plot_config.toml found)")

    try:
        # Get output variables
        outputs = get_fmu_outputs(fmu_path)
        if not outputs:
            print(f"  ⚠️  No output variables found in FMU")
            return False

        # Simulate with defaults
        result = simulate_fmu(
            str(fmu_path),
            stop_time=5.0,
            step_size=0.01,
            output_interval=0.01
        )

        # Create single plot with all outputs
        fig, ax = plt.subplots(figsize=(FIGURE_WIDTH, FIGURE_HEIGHT))

        for var_name in outputs:
            if var_name in result.dtype.names:
                ax.plot(result['time'], result[var_name], linewidth=1.5, label=var_name)

        ax.set_xlabel('Time [s]', fontsize=12)
        ax.set_ylabel('Output Variables', fontsize=12)
        ax.set_title(f'{model_name} - Simulation Results', fontsize=14, fontweight='bold')
        ax.grid(True, alpha=0.3)
        if len(outputs) > 1:
            ax.legend(fontsize=10)

        # Save
        output_file = output_dir / f'{model_name}.png'
        plt.savefig(output_file, dpi=FIGURE_DPI, bbox_inches='tight')
        plt.close(fig)

        return True

    except Exception as e:
        print(f"  ✗ Error: {e}")
        return False


def plot_timeseries_subplot(ax, result: np.ndarray, config: Dict[str, Any]):
    """Create a time-series subplot based on configuration."""
    variables = config.get('variables', [])
    if not variables:
        raise ValueError("timeseries plot requires 'variables' field")

    labels = config.get('labels', variables)
    colors = config.get('colors', [None] * len(variables))
    linestyles = config.get('linestyles', ['-'] * len(variables))
    linewidths = config.get('linewidths', [1.5] * len(variables))

    # Validate array lengths
    if len(labels) != len(variables):
        raise ValueError(f"labels length ({len(labels)}) must match variables length ({len(variables)})")

    # Plot each variable
    for var, label, color, style, width in zip(variables, labels, colors, linestyles, linewidths):
        if var not in result.dtype.names:
            raise ValueError(f"Variable '{var}' not found in simulation results")

        plot_kwargs = {'linewidth': width, 'linestyle': style, 'label': label}
        if color:
            plot_kwargs['color'] = color

        ax.plot(result['time'], result[var], **plot_kwargs)

    # Add reference line if specified
    if 'reference_line' in config:
        ref_y = config['reference_line']
        ref_label = config.get('reference_label', '')
        ref_color = config.get('reference_color', 'black')
        ref_style = config.get('reference_style', '--')
        ax.axhline(y=ref_y, color=ref_color, linestyle=ref_style,
                   linewidth=1, alpha=0.5, label=ref_label)

    # Formatting
    ax.set_xlabel(config.get('xlabel', 'Time [s]'), fontsize=12)
    ax.set_ylabel(config.get('ylabel', ''), fontsize=12)
    ax.set_title(config['title'], fontsize=13, fontweight='bold')
    if config.get('grid', True):
        ax.grid(True, alpha=0.3)
    if len(variables) > 1 or 'reference_line' in config:
        ax.legend(fontsize=10)


def plot_phase_portrait_subplot(ax, result: np.ndarray, config: Dict[str, Any]):
    """Create a phase portrait subplot based on configuration."""
    x_var = config.get('x_variable')
    y_var = config.get('y_variable')

    if not x_var or not y_var:
        raise ValueError("phase_portrait plot requires 'x_variable' and 'y_variable' fields")

    if x_var not in result.dtype.names:
        raise ValueError(f"Variable '{x_var}' not found in simulation results")
    if y_var not in result.dtype.names:
        raise ValueError(f"Variable '{y_var}' not found in simulation results")

    color = config.get('color', 'green')
    linewidth = config.get('linewidth', 1.5)

    # Plot trajectory
    ax.plot(result[x_var], result[y_var], color=color, linewidth=linewidth, alpha=0.8)

    # Add start/end markers
    if config.get('show_markers', True):
        start_color = config.get('start_marker_color', 'green')
        end_color = config.get('end_marker_color', 'red')
        ax.plot(result[x_var][0], result[y_var][0], 'o', color=start_color,
                markersize=10, label='Start')
        ax.plot(result[x_var][-1], result[y_var][-1], 's', color=end_color,
                markersize=10, label='End')
        ax.legend(fontsize=10)

    # Formatting
    ax.set_xlabel(config.get('xlabel', x_var), fontsize=12)
    ax.set_ylabel(config.get('ylabel', y_var), fontsize=12)
    ax.set_title(config['title'], fontsize=13, fontweight='bold')

    if config.get('grid', True):
        ax.grid(True, alpha=0.3)

    if config.get('equal_aspect', False):
        ax.set_aspect('equal', adjustable='box')


def create_configured_plot(fmu_path: Path, model_name: str, config: Dict[str, Any],
                          output_dir: Path) -> bool:
    """Create a plot based on configuration."""
    print(f"  Using configured plotting from plot_config.toml")

    try:
        # Get simulation parameters
        sim_config = config.get('simulation', {})
        stop_time = sim_config.get('stop_time', 5.0)
        step_size = sim_config.get('step_size', 0.01)
        output_interval = sim_config.get('output_interval', 0.01)

        # Get parameter values
        start_values = config.get('parameters', {})

        # Run simulation
        result = simulate_fmu(
            str(fmu_path),
            stop_time=stop_time,
            step_size=step_size,
            output_interval=output_interval,
            start_values=start_values if start_values else None
        )

        # Get subplot configurations
        subplots = config.get('subplot', [])
        if not subplots:
            raise ValueError("Configuration must include at least one [[subplot]] block")
        if len(subplots) > 4:
            raise ValueError("Maximum 4 subplots allowed per figure")

        # Determine layout
        if len(subplots) == 1:
            nrows, ncols = 1, 1
        elif len(subplots) == 2:
            nrows, ncols = 1, 2
        else:  # 3 or 4
            nrows, ncols = 2, 2

        # Create figure
        fig, axes = plt.subplots(nrows, ncols, figsize=(FIGURE_WIDTH, FIGURE_HEIGHT))
        if len(subplots) == 1:
            axes = [axes]
        else:
            axes = axes.flatten()

        # Create each subplot
        for idx, subplot_config in enumerate(subplots):
            plot_type = subplot_config.get('type')
            if not plot_type:
                raise ValueError(f"Subplot {idx} missing required 'type' field")

            if plot_type == 'timeseries':
                plot_timeseries_subplot(axes[idx], result, subplot_config)
            elif plot_type == 'phase_portrait':
                plot_phase_portrait_subplot(axes[idx], result, subplot_config)
            else:
                raise ValueError(f"Unknown plot type: {plot_type}")

        # Hide unused subplots
        for idx in range(len(subplots), len(axes)):
            axes[idx].set_visible(False)

        # Overall title
        if 'title' in config:
            fig.suptitle(config['title'], fontsize=14, fontweight='bold')
            plt.tight_layout(rect=[0, 0, 1, 0.96])  # Make room for suptitle
        else:
            plt.tight_layout()

        # Save
        output_file = output_dir / f'{model_name}.png'
        plt.savefig(output_file, dpi=FIGURE_DPI, bbox_inches='tight')
        plt.close(fig)

        return True

    except Exception as e:
        print(f"  ✗ Error: {e}")
        import traceback
        traceback.print_exc()
        return False


def simulate_and_plot_fmu(fmu_path: Path, model_dir: Path, output_dir: Path) -> bool:
    """Simulate an FMU and generate its plot."""
    model_name = fmu_path.stem
    print(f"Processing {model_name}...")

    # Load configuration if it exists
    config = load_plot_config(model_dir)

    # Generate plot
    if config:
        success = create_configured_plot(fmu_path, model_name, config, output_dir)
    else:
        success = create_generic_plot(fmu_path, model_name, output_dir)

    if success:
        print(f"  ✓ Saved: {output_dir / model_name}.png")

    return success


def discover_models(project_root: Path) -> Dict[str, Path]:
    """Discover all model directories in the project."""
    models = {}
    models_dir = project_root / 'models'

    if not models_dir.exists():
        return models

    # Search for model directories (2 levels deep: category/model-name)
    for category_dir in models_dir.iterdir():
        if not category_dir.is_dir() or category_dir.name.startswith('.'):
            continue
        for model_dir in category_dir.iterdir():
            if not model_dir.is_dir() or model_dir.name.startswith('.'):
                continue
            # Model name is typically the directory name with underscores replaced
            model_name = model_dir.name.replace('-', '_')
            # Map to FMU name (CamelCase)
            fmu_name = ''.join(word.capitalize() for word in model_dir.name.split('-'))
            models[fmu_name] = model_dir

    return models


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Generate FMU simulation plots',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='See docs/plot_config_spec.md for configuration details.'
    )
    parser.add_argument(
        '--models',
        type=str,
        help='Comma-separated list of model names to plot (default: all models)'
    )
    parser.add_argument(
        '--output-manifest',
        action='store_true',
        help='Output JSON manifest of generated plots to stdout'
    )

    args = parser.parse_args()

    # Setup paths
    script_dir = Path(__file__).parent.resolve()
    project_root = script_dir.parent
    fmu_dir = project_root / 'fmus'
    output_dir = project_root / 'fmu-figures'

    # Discover models
    all_models = discover_models(project_root)

    # Filter models if specified
    if args.models:
        requested = set(args.models.split(','))
        models_to_plot = {name: path for name, path in all_models.items()
                         if name in requested}
        if not models_to_plot:
            print(f"Error: None of the requested models found: {requested}")
            print(f"Available models: {', '.join(all_models.keys())}")
            return 1
    else:
        models_to_plot = all_models

    # Create output directory
    output_dir.mkdir(exist_ok=True)

    if not args.output_manifest:
        print(f"Output directory: {output_dir}")
        print(f"Models to process: {', '.join(models_to_plot.keys())}")
        print()

    # Process each FMU
    results = []
    for fmu_name, model_dir in sorted(models_to_plot.items()):
        fmu_path = fmu_dir / f'{fmu_name}.fmu'

        if not fmu_path.exists():
            if not args.output_manifest:
                print(f"⚠️  Skipping {fmu_name}: FMU not found at {fmu_path}")
            continue

        success = simulate_and_plot_fmu(fmu_path, model_dir, output_dir)

        if success:
            results.append({
                'model': fmu_name,
                'figure': f'{fmu_name}.png',
                'model_dir': str(model_dir.relative_to(project_root))
            })

        if not args.output_manifest:
            print()

    # Output results
    if args.output_manifest:
        print(json.dumps(results, indent=2))
    else:
        print("=" * 70)
        print(f"Summary: {len(results)}/{len(models_to_plot)} figures generated successfully")

        if len(results) < len(models_to_plot):
            failed = set(models_to_plot.keys()) - {r['model'] for r in results}
            print(f"\nFailed: {', '.join(failed)}")
            return 1

        print(f"\nAll figures saved to: {output_dir}")

    return 0


if __name__ == '__main__':
    sys.exit(main())
