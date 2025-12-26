---
name: New Model Request
about: Request a new dynamical model for AI implementation
title: "[Model Request] "
labels: model-request, ai-task
assignees: ''
---

## Model Information

**Model Name:**
<!-- e.g., Lorenz Attractor, Mass-Spring-Damper -->

**Category:**
<!-- mathematical / mechanical / electrical / thermal / hydraulic / other -->

**Complexity Level:**
<!-- simple (1-2 states) / medium (3-5 states) / advanced (events, discontinuities) -->

## Differential Equations

<!-- Provide the governing equations. Use LaTeX notation or plain text. -->

```
dx/dt = f(x, y, z, params)
dy/dt = g(x, y, z, params)
...
```

## Parameters

| Name | Symbol | Default | Unit | Description |
|------|--------|---------|------|-------------|
| | | | | |

## State Variables (Initial Conditions)

| Name | Symbol | Default | Unit | Description |
|------|--------|---------|------|-------------|
| | | | | |

## Expected Outputs

<!-- Which variables should be observable via FMI? -->

- [ ] All state variables
- [ ] Derived quantities (energy, etc.)
- [ ] Other:

## Validation Approach

<!-- How can correctness be verified? Check all that apply. -->

- [ ] Analytical solution available
- [ ] Known properties (limit cycles, steady states, frequencies)
- [ ] Conservation laws (energy, momentum, mass)
- [ ] Reference implementation exists
- [ ] Comparison with experimental data
- [ ] Other:

## References

<!-- Papers, textbooks, or online resources describing this model -->

1.
2.

## Additional Context

<!-- Any special considerations, known challenges, or implementation notes -->

