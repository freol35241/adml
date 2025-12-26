# AI Agent Instructions

This repository is the **AI-Generated Dynamical Model Library (ADML)** - a collection of FMI 3.0 compliant dynamical models where every line of code is AI-generated.

## Quick Reference

```bash
# Build & Test Commands
cargo test --workspace                              # All Rust tests
cargo test -p adml-{model-name}                     # Single model tests
./scripts/build-fmu.sh models/{category}/{name}     # Build FMU
./scripts/test-all.sh                               # Full test suite

# Check Commands
cargo fmt --all -- --check                          # Format check
cargo clippy --workspace --all-targets              # Lint check
```

## Adding a New Model

### 1. Create Directory Structure
```bash
mkdir -p models/{category}/{model-name}/src
mkdir -p models/{category}/{model-name}/tests
```

### 2. Copy Template
See `docs/AI_SCAFFOLDING.md` for complete templates. Minimal structure:

```
models/{category}/{model-name}/
├── Cargo.toml           # Package config with FMI metadata
├── src/lib.rs           # Model implementation
├── tests/physics_tests.rs
├── plot_config.toml     # Visualization config (optional)
└── README.md            # Documentation with equations
```

### 3. Implement Model
```rust
use fmu_from_struct::prelude::*;

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
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        let derivative = /* your equation */;
        self.state += derivative * step_size;  // Euler integration
    }
}
```

### 4. Add to Workspace
Edit root `Cargo.toml`:
```toml
[workspace]
members = [
    # ... existing members
    "models/{category}/{model-name}",
]
```

### 5. Validate
```bash
cargo test -p adml-{model-name}
./scripts/build-fmu.sh models/{category}/{model-name}
```

## Critical Conventions

### Naming (MUST follow exactly)
| Directory | Struct Name | Package Name | FMU File |
|-----------|-------------|--------------|----------|
| `rc-thermal` | `RcThermal` | `adml-rc-thermal` | `RcThermal.fmu` |
| `van-der-pol` | `VanDerPol` | `adml-van-der-pol` | `VanDerPol.fmu` |

**Rule:** Split directory on hyphens, capitalize each word, join for struct name.

### FMI Variable Types
- `#[fmu_from_struct(parameter)]` - Can be set before simulation, NOT in results
- `#[fmu_from_struct(output)]` - Read-only, APPEARS in simulation results
- No attribute - Internal variable, not exposed via FMI

### Units
- Time: Always seconds (FMI standard)
- Use SI units unless domain-specific convention exists
- Document units in code comments

### Testing
- Use step size 0.01 for Euler integration tests
- Tolerance 5% for short simulations, 10-25% for long ones
- Include convergence tests (solution improves with smaller steps)

## Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `cannot find type Fmu` | Missing import | Add `pub use fmu_from_struct::prelude::*;` |
| `clippy::not_unsafe_ptr_arg_deref` | Generated FFI code | Add `#![allow(clippy::not_unsafe_ptr_arg_deref)]` |
| Variable not in results | It's a parameter | Only outputs appear in simulation results |
| Large test errors (>20%) | Step size too large | Use dt=0.01 or smaller |
| FMU name mismatch | Wrong struct name | Struct must match CamelCase of directory |

## File Locations

| What | Where |
|------|-------|
| Model implementation | `models/{category}/{name}/src/lib.rs` |
| Physics tests | `models/{category}/{name}/tests/physics_tests.rs` |
| Python FMU tests | `testing/fmu-integration-tests/test_{name}_fmu.py` |
| Build scripts | `scripts/` |
| Templates | `docs/AI_SCAFFOLDING.md` |
| Full AI guide | `docs/AI_AGENTS.md` |
| Plot config | `models/{category}/{name}/plot_config.toml` |

## Existing Models (Study These)

| Complexity | Model | Path | Key Features |
|------------|-------|------|--------------|
| Simple | Dahlquist | `models/mathematical/dahlquist/` | Single state, analytical solution |
| Medium | Van der Pol | `models/mathematical/van-der-pol/` | Multi-state, nonlinear, limit cycle |
| Advanced | Bouncing Ball | `models/mechanical/bouncing-ball/` | Event handling, energy dissipation |
| Thermal | RC Thermal | `models/thermal/rc-thermal-single-zone/` | Physics notation, derived outputs |

## Workflow Summary

1. **Understand** - Read model specification, identify equations
2. **Scaffold** - Create directory, copy templates
3. **Implement** - Write `do_step()` with differential equations
4. **Test** - Unit tests, physics validation, convergence
5. **Build FMU** - Run build script, verify FMU loads
6. **Document** - README with equations, validation approach
7. **Submit** - PR with all tests passing

## Further Reading

- Full implementation guide: `docs/AI_AGENTS.md`
- Ready-to-use templates: `docs/AI_SCAFFOLDING.md`
- Contribution guidelines: `docs/CONTRIBUTING.md`
- FMI 3.0 specification: https://fmi-standard.org/
- `fmu_from_struct` docs: https://github.com/jarlekramer/fmu_from_struct
