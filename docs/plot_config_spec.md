# FMU Plot Configuration Specification

This document defines the TOML configuration format for customizing FMU simulation plots in CI workflows.

## Overview

Each FMU model can optionally provide a `plot_config.toml` file in its model directory to customize how simulation results are visualized. If no configuration exists, a generic time-series plot is generated automatically.

**Key Constraint:** Each FMU generates **exactly ONE figure** with a fixed size of 10×6 inches (1000×600 pixels at 100 DPI). Multiple subplots are allowed within this single figure.

## File Location

Place the configuration file alongside your model source:
```
models/{category}/{model-name}/plot_config.toml
```

Example:
```
models/mathematical/van-der-pol/plot_config.toml
```

## Configuration Schema

### Top-Level Fields

```toml
# Optional: Overall figure title (appears at the top of the figure)
title = "Model Name - Simulation Results"

# Optional: Simulation parameters (overrides defaults)
[simulation]
stop_time = 30.0        # Simulation end time (default: 5.0)
step_size = 0.01        # Integration step size (default: 0.01)
output_interval = 0.05  # Output sampling interval (default: 0.01)

# Optional: Parameter values to pass to the FMU
[parameters]
param_name = 1.0
another_param = -9.81

# Required: Array of subplot configurations (at least one)
[[subplot]]
# ... subplot configuration (see below)
```

### Subplot Configuration

Each `[[subplot]]` block defines one subplot in the figure. Subplots are arranged automatically in a grid layout.

#### Common Subplot Fields

```toml
[[subplot]]
# Required: Type of plot
type = "timeseries"  # Options: "timeseries", "phase_portrait"

# Required: Human-readable title for this subplot
title = "State Evolution"

# Optional: X-axis label (default depends on plot type)
xlabel = "Time [s]"

# Optional: Y-axis label (default: empty)
ylabel = "Position [m]"

# Optional: Grid display (default: true)
grid = true
```

#### Plot Type: `timeseries`

Plots one or more variables against time.

```toml
[[subplot]]
type = "timeseries"
title = "Height and Velocity"
ylabel = "Value"

# Required: List of output variable names to plot
variables = ["h", "v"]

# Optional: Custom labels for each variable (must match length of variables)
labels = ["Height [m]", "Velocity [m/s]"]

# Optional: Colors for each variable (must match length of variables)
# Uses matplotlib color strings: 'r', 'b', 'g', 'cyan', '#FF5733', etc.
colors = ["blue", "red"]

# Optional: Line styles for each variable
# Options: '-' (solid), '--' (dashed), '-.' (dash-dot), ':' (dotted)
linestyles = ["-", "--"]

# Optional: Line widths for each variable
linewidths = [2.0, 1.5]

# Optional: Add horizontal reference line
reference_line = 0.0           # Y-value for reference line
reference_label = "Ground"      # Label for reference line
reference_color = "black"       # Color (default: "black")
reference_style = "--"          # Line style (default: "--")
```

#### Plot Type: `phase_portrait`

Plots one variable against another (e.g., velocity vs. position).

```toml
[[subplot]]
type = "phase_portrait"
title = "Phase Space - Limit Cycle"

# Required: Variable for x-axis
x_variable = "x0"

# Required: Variable for y-axis
y_variable = "x1"

# Optional: X-axis label (default: x_variable name)
xlabel = "Position"

# Optional: Y-axis label (default: y_variable name)
ylabel = "Velocity"

# Optional: Force equal aspect ratio (default: false)
equal_aspect = true

# Optional: Color of the trajectory line
color = "green"

# Optional: Line width
linewidth = 1.5

# Optional: Show start/end markers (default: true)
show_markers = true

# Optional: Marker colors
start_marker_color = "green"   # Default: "green"
end_marker_color = "red"       # Default: "red"
```

## Layout Rules

1. **Single Subplot:** If only one `[[subplot]]` is defined, it fills the entire 10×6 inch figure
2. **Two Subplots:** Arranged horizontally (1 row × 2 columns)
3. **Three or Four Subplots:** Arranged in 2×2 grid
4. **More than Four:** Error (not supported - use at most 4 subplots)

## Fallback Behavior (No Configuration)

If no `plot_config.toml` exists, the system automatically:
1. Reads all output variables from the FMU's `modelDescription.xml`
2. Creates a single time-series plot with all variables
3. Uses the FMU model name as the figure title
4. Uses default simulation parameters (stop_time=5.0, step_size=0.01)

## Complete Examples

### Example 1: Simple Time Series (Dahlquist)

```toml
# models/mathematical/dahlquist/plot_config.toml
title = "Dahlquist Test Equation: dx/dt = -kx"

[simulation]
stop_time = 5.0

[parameters]
k = 1.0

[[subplot]]
type = "timeseries"
title = "Exponential Decay"
xlabel = "Time [s]"
ylabel = "State x"
variables = ["x"]
colors = ["blue"]
linewidths = [2.0]
```

### Example 2: Multiple Subplots (Van der Pol)

```toml
# models/mathematical/van-der-pol/plot_config.toml
title = "Van der Pol Oscillator (μ=1.0)"

[simulation]
stop_time = 30.0
output_interval = 0.05

[parameters]
mu = 1.0

[[subplot]]
type = "timeseries"
title = "State Variables vs Time"
xlabel = "Time [s]"
ylabel = "State"
variables = ["x0", "x1"]
labels = ["x₀ (position)", "x₁ (velocity)"]
colors = ["blue", "red"]
linewidths = [1.5, 1.5]

[[subplot]]
type = "phase_portrait"
title = "Phase Portrait - Limit Cycle"
x_variable = "x0"
y_variable = "x1"
xlabel = "x₀ (position)"
ylabel = "x₁ (velocity)"
equal_aspect = true
color = "green"
show_markers = true
```

### Example 3: Event-Based System (Bouncing Ball)

```toml
# models/mechanical/bouncing-ball/plot_config.toml
title = "Bouncing Ball with Elastic Collisions (e=0.7)"

[simulation]
stop_time = 5.0
step_size = 0.001
output_interval = 0.01

[parameters]
g = -9.81
e = 0.7

[[subplot]]
type = "timeseries"
title = "Height"
xlabel = "Time [s]"
ylabel = "Height h [m]"
variables = ["h"]
colors = ["blue"]
linewidths = [1.5]
reference_line = 0.0
reference_label = "Ground"

[[subplot]]
type = "timeseries"
title = "Velocity"
xlabel = "Time [s]"
ylabel = "Velocity v [m/s]"
variables = ["v"]
colors = ["red"]
linewidths = [1.5]
reference_line = 0.0
```

## Validation Rules

The plotting script will validate configurations and fail with clear error messages if:
- No `[[subplot]]` blocks are defined
- More than 4 subplots are specified
- Required fields are missing (type, title, variables/x_variable/y_variable)
- Variable names don't exist in the FMU
- Array lengths don't match (e.g., variables vs. colors)
- Unknown plot type is specified

## For AI Agents

When creating or modifying `plot_config.toml` files:

1. **Start with the model's physics** - What variables are most meaningful to visualize?
2. **Use appropriate plot types** - Time series for evolution, phase portraits for state space
3. **Limit to 4 subplots** - Keep it readable at 10×6 inches
4. **Label clearly** - Include units in axis labels
5. **Choose meaningful colors** - Blue for primary, red for secondary, green for trajectories
6. **Add reference lines** - For ground level, equilibrium points, zero-crossings, etc.
7. **Match simulation time to dynamics** - Fast systems need shorter stop_time, slow systems need longer

## Error Handling

If configuration parsing fails, the script will:
1. Print a clear error message with line numbers
2. Fall back to generic plotting for that FMU
3. Continue processing other FMUs
4. Report the error in CI logs
