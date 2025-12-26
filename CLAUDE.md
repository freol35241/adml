# Claude Instructions for ADML

## Project Summary
ADML (AI-Generated Dynamical Model Library) is a collection of FMI 3.0 compliant dynamical models where all code is AI-generated. Models are implemented in Rust using the `fmu_from_struct` derive macro.

## Quick Reference

### Key Files
| Purpose | File |
|---------|------|
| Quick reference | `AGENTS.md` |
| Step-by-step guide | `docs/AI_QUICK_START.md` |
| Comprehensive guide | `docs/AI_AGENTS.md` |
| Templates | `docs/AI_SCAFFOLDING.md` |
| Error solutions | `docs/ERROR_CATALOG.md` |
| Model catalog | `models.json` |

### Commands
```bash
cargo test -p adml-{model}              # Test single model
cargo test --workspace                   # Test all
./scripts/build-fmu.sh models/...       # Build FMU
./scripts/test-all.sh                   # Full test suite
cargo fmt && cargo clippy               # Format and lint
```

## Critical Rules

### 1. Naming Convention (MUST follow exactly)
| Directory Name | Struct Name | Package Name |
|----------------|-------------|--------------|
| `my-model` | `MyModel` | `adml-my-model` |
| `van-der-pol` | `VanDerPol` | `adml-van-der-pol` |
| `rc-thermal` | `RcThermal` | `adml-rc-thermal` |

**Rule:** Split directory on hyphens → capitalize each word → join.

### 2. FMI Variables
```rust
#[fmu_from_struct(parameter)]  // Can SET before simulation
pub k: f64,                    // NOT in results

#[fmu_from_struct(output)]     // READ during simulation
pub x: f64,                    // IN results
```

### 3. Required Imports
```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
pub use fmu_from_struct::prelude::*;
```

### 4. Euler Integration
- Step size: ≤ 0.01
- Tolerance: 5% for short simulations
- Error accumulates over time

## New Model Checklist

1. [ ] Create directory: `models/{category}/{name}/`
2. [ ] Add to workspace in root `Cargo.toml`
3. [ ] Create `Cargo.toml` with correct package name
4. [ ] Implement struct (name matches directory pattern!)
5. [ ] Implement `do_step()` with differential equations
6. [ ] Add unit tests in `src/lib.rs`
7. [ ] Add physics tests in `tests/physics_tests.rs`
8. [ ] Build FMU: `./scripts/build-fmu.sh ...`
9. [ ] Run: `cargo fmt && cargo clippy`
10. [ ] Write `README.md` with equations

## Model Template

```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
pub use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct ModelName {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub param: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    pub state: f64,

    time: f64,
}

impl FmuFunctions for ModelName {
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        let derivative = /* your equation */;
        self.state += derivative * step_size;
        self.time += step_size;
    }
}
```

## Study These Examples

1. **Simple:** `models/mathematical/dahlquist/` (1 state, analytical solution)
2. **Multi-state:** `models/mathematical/van-der-pol/` (2 states, nonlinear)
3. **Events:** `models/mechanical/bouncing-ball/` (collision handling)
4. **Thermal:** `models/thermal/rc-thermal-single-zone/` (derived outputs)

## Common Errors

| Error | Solution |
|-------|----------|
| `cannot find type Fmu` | Add `pub use fmu_from_struct::prelude::*;` |
| `clippy::not_unsafe_ptr_arg_deref` | Add `#![allow(...)]` at top |
| Variable not in results | It's a parameter, not output |
| Large test error (>20%) | Use smaller step size (0.01) |
| FMU name mismatch | Fix struct name to match directory |
