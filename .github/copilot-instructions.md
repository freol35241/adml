# GitHub Copilot Instructions for ADML

## Project Overview
ADML (AI-Generated Dynamical Model Library) contains FMI 3.0 compliant dynamical models. All code is AI-generated.

## When Generating Code

### Model Implementation
1. Use `fmu_from_struct` derive macros - no manual FFI
2. Implement `do_step()` for Euler integration
3. Mark parameters with `#[fmu_from_struct(parameter)]`
4. Mark state/output variables with `#[fmu_from_struct(output)]`

### Naming Convention (Critical)
```
Directory: my-model-name
Struct:    MyModelName  (capitalize each hyphen-separated word)
Package:   adml-my-model-name
```

### Required Boilerplate
```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
pub use fmu_from_struct::prelude::*;
```

### Physics Tests
- Compare to analytical solutions when available
- Use `approx::assert_relative_eq!` with 5% tolerance
- Test convergence with smaller step sizes

## Key Files
- `/AGENTS.md` - Quick reference
- `/docs/AI_QUICK_START.md` - Implementation guide
- `/docs/AI_SCAFFOLDING.md` - Templates to copy
- `/models/mathematical/dahlquist/` - Simple example

## Common Patterns

### Simple Model
```rust
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct ModelName {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub param: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    pub state: f64,
}

impl FmuFunctions for ModelName {
    fn do_step(&mut self, _t: f64, dt: f64) {
        let derivative = /* equation */;
        self.state += derivative * dt;
    }
}
```

### Physics Test
```rust
#[test]
fn test_analytical() {
    let mut m = ModelName::new();
    for _ in 0..100 { m.do_step(0.0, 0.01); }
    assert_relative_eq!(m.state, expected, epsilon = 0.05);
}
```
