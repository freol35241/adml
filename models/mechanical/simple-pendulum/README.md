# Simple Pendulum

A classical simple pendulum model with nonlinear dynamics and optional damping, implemented as an FMI 3.0 compliant Functional Mock-up Unit (FMU).

## Overview

The simple pendulum is one of the fundamental systems in classical mechanics. This model simulates a point mass suspended from a fixed pivot by a massless, rigid rod, free to swing in a vertical plane under the influence of gravity.

## Physics

### Governing Equations

The motion of the simple pendulum is described by the following system of first-order differential equations:

```
dθ/dt = ω
dω/dt = -(g/L) * sin(θ) - (b/m) * ω
```

where:
- `θ` (theta): Angular displacement from vertical downward equilibrium [rad]
- `ω` (omega): Angular velocity [rad/s]
- `g`: Gravitational acceleration [m/s²]
- `L`: Length of pendulum [m]
- `m`: Mass of bob [kg]
- `b`: Damping coefficient [kg/s]

### Key Physics Concepts

**Nonlinear Dynamics**: The term `sin(θ)` makes this a nonlinear system. For small angles (θ << 1), we can approximate `sin(θ) ≈ θ`, which yields a linear harmonic oscillator.

**Energy**:
- Kinetic Energy: `KE = (1/2) * m * (L * ω)²`
- Potential Energy: `PE = m * g * L * (1 - cos(θ))` (reference at lowest point)
- Total Energy: `E = KE + PE`

For an undamped pendulum (`b = 0`), total energy is conserved. With damping, energy monotonically decreases.

**Period**: For small-angle oscillations, the period is:
```
T = 2π * sqrt(L/g)
```

For large amplitudes, the period increases (the system is not isochronous).

### Small-Angle Approximation

When `θ << 1` rad (typically `|θ| < 0.2` rad ≈ 11.5°), the system behaves as a simple harmonic oscillator:

```
θ(t) = θ₀ * cos(ω_n * t) + (ω₀/ω_n) * sin(ω_n * t)
ω(t) = -θ₀ * ω_n * sin(ω_n * t) + ω₀ * cos(ω_n * t)
```

where `ω_n = sqrt(g/L)` is the natural frequency.

## Parameters

| Name | Symbol | Default | Unit | Description |
|------|--------|---------|------|-------------|
| `g` | g | 9.81 | m/s² | Gravitational acceleration |
| `L` | L | 1.0 | m | Pendulum length |
| `m` | m | 1.0 | kg | Mass of bob |
| `b` | b | 0.0 | kg/s | Damping coefficient |

## Initial Conditions

| Name | Symbol | Default | Unit | Description |
|------|--------|---------|------|-------------|
| `theta` | θ | 0.1 | rad | Initial angular position |
| `omega` | ω | 0.0 | rad/s | Initial angular velocity |

## Outputs

| Name | Unit | Description |
|------|------|-------------|
| `theta` | rad | Angular position |
| `omega` | rad/s | Angular velocity |
| `energy` | J | Total mechanical energy |
| `KE` | J | Kinetic energy |
| `PE` | J | Potential energy |

## Building the FMU

```bash
./scripts/build-fmu.sh models/mechanical/simple-pendulum
```

The generated FMU will be located at:
```
models/mechanical/simple-pendulum/target/SimplePendulum.fmu
```

## Testing

### Rust Tests

Run unit tests and physics validation tests:

```bash
cargo test -p adml-simple-pendulum
```

The physics tests validate:
- Analytical solution for small angles
- Energy conservation (undamped case)
- Energy dissipation (damped case)
- Oscillation frequency
- Phase space trajectories
- Large amplitude behavior

### Python Integration Tests

Run FMU integration tests:

```bash
cd testing/fmu-integration-tests
pytest test_simple_pendulum_fmu.py -v
```

## Usage Examples

### Example 1: Small-Angle Oscillation

Classic textbook pendulum with small initial displacement:

```python
from fmpy import simulate_fmu

result = simulate_fmu(
    'SimplePendulum.fmu',
    stop_time=10.0,
    step_size=0.01,
    start_values={
        'g': 9.81,
        'L': 1.0,
        'm': 1.0,
        'b': 0.0,
        'theta': 0.1,  # ~5.7 degrees
        'omega': 0.0
    }
)
```

Expected behavior:
- Period: T ≈ 2.006 s
- Energy conserved (within numerical error)
- Nearly sinusoidal motion

### Example 2: Damped Oscillation

Pendulum with viscous damping:

```python
result = simulate_fmu(
    'SimplePendulum.fmu',
    stop_time=20.0,
    step_size=0.01,
    start_values={
        'g': 9.81,
        'L': 1.0,
        'm': 1.0,
        'b': 0.1,    # Damping coefficient
        'theta': 0.3,
        'omega': 0.0
    }
)
```

Expected behavior:
- Amplitude decays exponentially
- Energy decreases monotonically
- Eventually settles to equilibrium

### Example 3: Large-Angle Oscillation

Demonstrates nonlinear effects:

```python
result = simulate_fmu(
    'SimplePendulum.fmu',
    stop_time=10.0,
    step_size=0.01,
    start_values={
        'g': 9.81,
        'L': 1.0,
        'm': 1.0,
        'b': 0.0,
        'theta': 1.5,  # ~86 degrees - nearly horizontal
        'omega': 0.0
    }
)
```

Expected behavior:
- Period longer than small-angle prediction
- Non-sinusoidal waveform
- Energy still conserved

### Example 4: Different Gravity

Pendulum on the Moon (g ≈ 1.62 m/s²):

```python
result = simulate_fmu(
    'SimplePendulum.fmu',
    stop_time=20.0,
    step_size=0.01,
    start_values={
        'g': 1.62,
        'L': 1.0,
        'm': 1.0,
        'b': 0.0,
        'theta': 0.2,
        'omega': 0.0
    }
)
```

Expected behavior:
- Period: T ≈ 4.93 s (much slower than on Earth)

## Physical Interpretations

### Phase Portrait

The phase space plot (θ vs ω) shows:
- **Undamped**: Closed elliptical orbits (energy conserved)
- **Damped**: Spirals inward to equilibrium (energy dissipated)
- **Large amplitude**: Distorted from elliptical (nonlinear effects)

### Energy Exchange

During oscillation:
- At maximum displacement (θ = ±θ_max): All potential energy, ω = 0
- At equilibrium (θ = 0): All kinetic energy, maximum |ω|
- Energy continuously trades between KE and PE

### Damping Effects

The damping term `-b*ω/m` represents:
- Air resistance (proportional to velocity)
- Bearing friction
- Other velocity-dependent dissipation

The quality factor Q ≈ `m * ω_n / b` characterizes damping:
- Q >> 1: Lightly damped, many oscillations before decay
- Q ≈ 1: Critically damped, fastest return without overshoot
- Q < 1: Overdamped, slow return without oscillation

## Numerical Considerations

This model uses **symplectic Euler integration** (also called semi-implicit Euler), which:
- Is simple and fast (same cost as forward Euler)
- Has 1st-order accuracy: error ∝ Δt
- **Is symplectic**: conserves energy for undamped Hamiltonian systems
- Provides excellent long-term stability for oscillatory systems

### Symplectic Euler vs Forward Euler

Unlike forward Euler, symplectic Euler updates velocity first, then uses the *new* velocity to update position:
```
ω[n+1] = ω[n] + α[n] * Δt     (update velocity using current position)
θ[n+1] = θ[n] + ω[n+1] * Δt   (update position using NEW velocity)
```

This synchronization ensures that energy is conserved (undamped) or properly dissipated (damped), avoiding the artificial energy growth seen with forward Euler.

Typical accuracy with `step_size = 0.01`:
- Small angle, undamped: ~0.5% energy drift over 10 seconds
- Small angle, damped: Energy monotonically decreases (physically correct)
- Large angle: ~1-2% error over multiple periods

## Physical Limits and Validity

**Small-Angle Approximation**: Valid for `|θ| < 0.2` rad (≈ 11.5°)

**Large Angles**: The full nonlinear model is valid for all angles, including:
- Complete rotations (θ > π)
- Over-the-top motion (if given sufficient initial energy)

**Damping**: The linear damping model `F = -b*v` is valid for:
- Low velocities (laminar flow regime)
- Typical air resistance and bearing friction

For high velocities, quadratic drag `F ∝ v²` may be more appropriate (not implemented).

## References

1. **Classical Mechanics**: Goldstein, Poole, and Safko, "Classical Mechanics" (3rd ed.)
2. **Nonlinear Dynamics**: Strogatz, "Nonlinear Dynamics and Chaos"
3. **Pendulum Physics**: Baker and Blackburn, "The Pendulum: A Case Study in Physics"

## License

This model is part of the ADML (AI-Generated Dynamical Model Library) project and is licensed under the MIT License.
