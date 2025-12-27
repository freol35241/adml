# Lorenz System

The Lorenz system is a famous chaotic attractor discovered by Edward Lorenz in 1963 while studying atmospheric convection. It is one of the first systems shown to exhibit deterministic chaos.

## Equations

The system is defined by three coupled ordinary differential equations:

$$\frac{dx}{dt} = \sigma(y - x)$$

$$\frac{dy}{dt} = x(\rho - z) - y$$

$$\frac{dz}{dt} = xy - \beta z$$

## Parameters

| Parameter | Symbol | Default | Description |
|-----------|--------|---------|-------------|
| sigma | σ | 10.0 | Prandtl number - ratio of momentum diffusivity to thermal diffusivity |
| rho | ρ | 28.0 | Rayleigh number - ratio of buoyancy to viscous forces |
| beta | β | 8/3 | Geometric factor related to the aspect ratio of convection rolls |

## State Variables

| Variable | Default | Description |
|----------|---------|-------------|
| x | 1.0 | Proportional to intensity of convective motion |
| y | 1.0 | Proportional to temperature difference between ascending and descending currents |
| z | 1.0 | Proportional to distortion of vertical temperature profile from linearity |

## Chaotic Behavior

With the classic parameters (σ=10, ρ=28, β=8/3), the system exhibits chaotic behavior:

- **Sensitivity to initial conditions**: Tiny differences in starting conditions lead to vastly different trajectories
- **Strange attractor**: The trajectory forms a butterfly-shaped attractor in 3D phase space
- **Bounded chaos**: Despite chaotic behavior, trajectories remain bounded

## Equilibrium Points

For ρ > 1, the system has three equilibrium points:

1. **Origin**: (0, 0, 0) - unstable for ρ > 1
2. **C+**: (√(β(ρ-1)), √(β(ρ-1)), ρ-1)
3. **C-**: (-√(β(ρ-1)), -√(β(ρ-1)), ρ-1)

For ρ > 24.74 (approximately), C+ and C- become unstable and the system exhibits chaotic behavior.

## Usage

```rust
use adml_lorenz::{Lorenz, FmuFunctions};

// Create with classic chaotic parameters
let mut model = Lorenz::new();

// Set initial conditions
model.x = 1.0;
model.y = 1.0;
model.z = 1.0;

// Simulate
let dt = 0.01;
for _ in 0..10000 {
    model.do_step(0.0, dt);
    println!("({}, {}, {})", model.x, model.y, model.z);
}
```

## FMI Variables

### Parameters (settable before simulation)
- `sigma` - Prandtl number (default: 10.0)
- `rho` - Rayleigh number (default: 28.0)
- `beta` - Geometric factor (default: 8/3 ≈ 2.667)

### Outputs (available during simulation)
- `x` - State variable x
- `y` - State variable y
- `z` - State variable z

## References

1. Lorenz, E. N. (1963). "Deterministic Nonperiodic Flow". Journal of the Atmospheric Sciences. 20 (2): 130–141.
2. Sparrow, C. (1982). The Lorenz Equations: Bifurcations, Chaos, and Strange Attractors. Springer-Verlag.
