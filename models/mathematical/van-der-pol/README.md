# Van der Pol Oscillator

A classic nonlinear oscillator exhibiting self-sustained oscillations and limit cycle behavior.

## Model Description

The Van der Pol oscillator is described by:

```
dx0/dt = x1
dx1/dt = μ * (1 - x0²) * x1 - x0
```

where:
- `x0` is the position-like state variable
- `x1` is the velocity-like state variable
- `μ` is the damping parameter

## Physical Interpretation

The Van der Pol oscillator models systems with:
- Nonlinear damping that depends on amplitude
- Self-sustained oscillations
- A stable limit cycle attracting all trajectories
- Energy input at small amplitudes and dissipation at large amplitudes

It was originally proposed to model oscillations in vacuum tube circuits and has since found applications in biology, neuroscience, and engineering.

## Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `μ` (mu) | Real | 1.0 | Damping parameter (μ > 0) |

## State Variables

| Name | Type | Initial | Description |
|------|------|---------|-------------|
| `x0` | Real | 2.0 | Position-like variable |
| `x1` | Real | 0.0 | Velocity-like variable |

## Behavior

- For μ > 0, the system exhibits a stable limit cycle
- Larger μ values produce more relaxation-like oscillations
- Small μ values result in nearly sinusoidal oscillations
- Period increases with μ (approximately T ≈ 2π for μ << 1)

## Physics Validation

The tests verify:
- Convergence to limit cycle from different initial conditions
- Consistent oscillation period
- Bounded energy on the limit cycle
- Symmetry properties of the dynamics
