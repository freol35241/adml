# Bouncing Ball

A ball bouncing under gravity with energy loss on each collision.

## Model Description

The bouncing ball is modeled with:

```
dh/dt = v
dv/dt = g
```

With event handling for ground collision:
- When h ≤ 0 and v < 0: v ← -e * v (reverse with energy loss)
- When |v| < v_min: Stop bouncing (v = 0, g = 0)

where:
- `h` is the height above ground (m)
- `v` is the vertical velocity (m/s)
- `g` is the gravitational acceleration (m/s², negative)
- `e` is the coefficient of restitution (0 < e < 1)

## Physical Interpretation

This model demonstrates:
- Free fall under constant gravity
- Inelastic collisions with energy dissipation
- Event-based state changes
- Discrete-continuous hybrid dynamics

The coefficient of restitution `e` determines energy loss:
- e = 1.0: Perfect elastic collision (no energy loss)
- e = 0.0: Perfect inelastic collision (no bounce)
- Typical values: 0.5 - 0.9 for real materials

## Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `g` | Real | -9.81 | Gravitational acceleration (m/s²) |
| `e` | Real | 0.7 | Coefficient of restitution |
| `v_min` | Real | 0.1 | Minimum velocity threshold (m/s) |

## State Variables

| Name | Type | Initial | Description |
|------|------|---------|-------------|
| `h` | Real | 1.0 | Height above ground (m) |
| `v` | Real | 0.0 | Vertical velocity (m/s) |

## Event Handling

The model includes sophisticated event detection:
- Zero-crossing detection for ground collision (h = 0)
- Hysteresis to prevent event chattering
- Automatic termination when velocity drops below threshold

## Physics Validation

The tests verify:
- Free fall dynamics (v = g*t)
- Energy loss per bounce (E_after ≈ e² * E_before)
- Decreasing bounce heights
- Effect of different restitution coefficients
- Eventual stopping behavior
- Bounce symmetry with elastic collisions
