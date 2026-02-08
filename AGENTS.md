# AI Agent Instructions

This repository is the **AI-Generated Dynamical Model Library (ADML)** - a collection of FMI 3.0 compliant dynamical models where every line of code is AI-generated.

## Quick Reference

```bash
# Build & Test Commands
cargo test --workspace                              # All Rust tests
cargo test -p adml-{model-name}                     # Single model tests
cargo fmi --package adml-{model-name}               # Build FMU (requires cargo-fmi)
./scripts/build-fmu.sh models/{category}/{name}     # Build FMU (legacy script)
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
use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct ModelName {
    #[variable(causality = Parameter, start = 1.0, initial = Exact)]
    pub param: f64,

    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub state: f64,

    #[variable(causality = Local, derivative = state, initial = Calculated)]
    der_state: f64,
}

impl UserModel for ModelName {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(
        &mut self,
        _context: &dyn Context<Self>,
    ) -> Result<Fmi3Res, Fmi3Error> {
        self.der_state = /* your equation */;
        Ok(Fmi3Res::OK)
    }

    // Forward Euler with 1ms micro-stepping (serves both CS and ME)
    adml_solver::euler_cs_step!(0.001);
}

fmi_export::export_fmu!(ModelName);
```

**Solver choices:**
- `adml_solver::euler_cs_step!(step)` - Forward Euler (most models)
- `adml_solver::symplectic_euler_cs_step!(step)` - Symplectic Euler (Hamiltonian systems like pendulums)
- Custom `do_step` - For models with events (e.g., bouncing ball)

### 4. Cargo.toml Template
```toml
[package]
name = "adml-{model-name}"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
fmi-export = { workspace = true }
fmi = { workspace = true }
adml-solver = { workspace = true }

[dev-dependencies]
physics-framework = { path = "../../../testing/physics-framework" }
approx = { workspace = true }

[package.metadata.fmi]
model_name = "ModelName"
fmi_version = "3.0"
guid = "generate-a-new-uuid"
```

### 5. Validate
```bash
cargo test -p adml-{model-name}
cargo fmi --package adml-{model-name}
```

## Critical Conventions

### Naming (MUST follow exactly)
| Directory | Struct Name | Package Name | FMU File |
|-----------|-------------|--------------|----------|
| `rc-thermal` | `RcThermal` | `adml-rc-thermal` | `RcThermal.fmu` |
| `van-der-pol` | `VanDerPol` | `adml-van-der-pol` | `VanDerPol.fmu` |

**Rule:** Split directory on hyphens, capitalize each word, join for struct name.

### FMI Variable Types
- `#[variable(causality = Parameter, ...)]` - Can be set before simulation, NOT in results
- `#[variable(causality = Output, ...)]` - Read-only, APPEARS in simulation results
- `#[variable(causality = Local, derivative = x, ...)]` - Derivative variable
- `#[variable(skip)]` - Internal field, not exposed via FMI
- No `#[variable]` attribute - not possible; use `skip` for internal fields

### Units
- Time: Always seconds (FMI standard)
- Use SI units unless domain-specific convention exists
- Document units in code comments

### Testing
- Use step size 0.01 for Euler integration tests
- Tolerance 5% for short simulations, 10-25% for long ones
- Include convergence tests (solution improves with smaller steps)
- Provide a `pub fn do_step(&mut self, current_time: f64, time_step: f64)` inherent method for testing without FMI context

## Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `cannot find FmuModel` | Missing import | Add `use fmi_export::FmuModel;` |
| `coverage_nightly` warnings | Upstream macro | Harmless, from `export_fmu!` macro |
| `non_snake_case` warnings | Physics naming | Add `#![allow(non_snake_case)]` at crate level |
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
| Medium | Lorenz | `models/mathematical/lorenz/` | 3-state chaotic system |
| Advanced | Bouncing Ball | `models/mechanical/bouncing-ball/` | Event handling, energy dissipation |
| Advanced | Simple Pendulum | `models/mechanical/simple-pendulum/` | Symplectic Euler, energy conservation |
| Thermal | RC Thermal | `models/thermal/rc-thermal-single-zone/` | Physics notation, derived outputs |

## Workflow Summary

1. **Understand** - Read model specification, identify equations
2. **Scaffold** - Create directory, copy templates
3. **Implement** - Write `calculate_values()` with differential equations, use solver macro
4. **Test** - Unit tests, physics validation, convergence
5. **Build FMU** - Run `cargo fmi` or build script
6. **Document** - README with equations, validation approach
7. **Submit** - PR with all tests passing

## FMI Model Description (Machine-Readable Spec)

Each model's complete specification is in `modelDescription.xml`, auto-generated per FMI 3.0 standard by the `FmuModel` derive macro.

### What's in modelDescription.xml

- All variables with causality (parameter, output, local)
- Derivative relationships between state and derivative variables
- Data types, units, start values
- Model GUID and version
- FMI capabilities (Co-Simulation, etc.)

### FMI Schema Reference

- **FMI 3.0 Specification:** https://fmi-standard.org/docs/3.0/
- **Schema (XSD):** https://github.com/modelica/fmi-standard/tree/main/schema
- **Variable Causalities:** parameter (settable), output (readable), local (internal)

The Rust struct attributes map directly to FMI causalities:
```rust
#[variable(causality = Parameter, ...)]  -> causality="parameter"
#[variable(causality = Output, ...)]     -> causality="output"
#[variable(causality = Local, ...)]      -> causality="local" (internal)
#[variable(skip)]                        -> not in modelDescription.xml
```

## Further Reading

- Full implementation guide: `docs/AI_AGENTS.md`
- Ready-to-use templates: `docs/AI_SCAFFOLDING.md`
- Contribution guidelines: `docs/CONTRIBUTING.md`
- FMI 3.0 specification: https://fmi-standard.org/
- FMI schema files: https://github.com/modelica/fmi-standard/tree/main/schema
- fmi-export docs: https://docs.rs/fmi-export
- rust-fmi GitHub: https://github.com/jondo2010/rust-fmi
