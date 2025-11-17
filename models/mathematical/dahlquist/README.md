# Dahlquist Test Equation

A simple first-order ODE used as a fundamental test case for numerical ODE solvers.

## Model Description

The Dahlquist test equation is:

```
dx/dt = -k * x
```

where:
- `x` is the state variable
- `k` is the decay constant (k > 0)

## Analytical Solution

The exact solution is:

```
x(t) = x(0) * exp(-k * t)
```

## Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `k` | Real | 1.0 | Decay constant |

## State Variables

| Name | Type | Initial | Description |
|------|------|---------|-------------|
| `x` | Real | 1.0 | State variable |

## Usage

This model is primarily used for:
- Testing numerical integration schemes
- Studying stability regions of ODE solvers
- Validating step size control algorithms

## Physics Validation

The tests verify:
- Exponential decay behavior
- Convergence with decreasing step sizes
- Match with analytical solution
- Correct half-life calculation
