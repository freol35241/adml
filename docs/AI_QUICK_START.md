# AI Agent Quick Start Guide

A streamlined guide for implementing a new dynamical model. For comprehensive details, see [AI_AGENTS.md](AI_AGENTS.md).

---

## 5-Minute Overview

ADML models are:
- Rust structs with `fmu_from_struct` derives
- Implementing `do_step()` for integration
- Packaged as FMI 3.0 FMUs
- Tested at 3 tiers: Rust, FMU, Physics

---

## Step 1: Create Structure (2 min)

```bash
# Create directories
mkdir -p models/{category}/{model-name}/src
mkdir -p models/{category}/{model-name}/tests

# Create files
touch models/{category}/{model-name}/Cargo.toml
touch models/{category}/{model-name}/src/lib.rs
touch models/{category}/{model-name}/tests/physics_tests.rs
touch models/{category}/{model-name}/README.md
```

Add to workspace in root `Cargo.toml`:
```toml
[workspace]
members = [
    "models/{category}/{model-name}",
    # ... others
]
```

---

## Step 2: Cargo.toml (1 min)

```toml
[package]
name = "adml-{model-name}"
version = "1.0.0"
edition = "2021"
license.workspace = true
repository.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
fmu_from_struct = { workspace = true }

[dev-dependencies]
physics-framework = { path = "../../../testing/physics-framework" }
approx = { workspace = true }
```

---

## Step 3: Implement Model (5 min)

```rust
// src/lib.rs
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct ModelName {  // CamelCase from directory!
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
        // Your differential equation
        let derivative = /* -self.param * self.state */;
        self.state += derivative * step_size;
        self.time += step_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let m = ModelName::new();
        assert_eq!(m.state, 1.0);
    }
}
```

---

## Step 4: Add Physics Tests (3 min)

```rust
// tests/physics_tests.rs
use approx::assert_relative_eq;
use adml_model_name::{FmuFunctions, ModelName};

#[test]
fn test_simulation() {
    let mut model = ModelName::new();
    let dt = 0.01;

    for _ in 0..100 {
        model.do_step(0.0, dt);
    }

    // Compare to analytical solution or expected behavior
    assert_relative_eq!(model.state, expected_value, epsilon = 0.05);
}
```

---

## Step 5: Build and Test (2 min)

```bash
# Test Rust code
cargo test -p adml-{model-name}

# Build FMU
./scripts/build-fmu.sh models/{category}/{model-name}

# Run all tests
./scripts/test-all.sh
```

---

## Critical Rules

### Naming Convention
| Directory | Struct Name |
|-----------|-------------|
| `my-model` | `MyModel` |
| `van-der-pol` | `VanDerPol` |
| `rc-thermal` | `RcThermal` |

**Rule:** Split on hyphens, capitalize each word, join.

### Parameters vs Outputs
```rust
#[fmu_from_struct(parameter)]  // Can SET before simulation
pub k: f64,

#[fmu_from_struct(output)]     // Can READ during simulation
pub x: f64,
```
- Only outputs appear in simulation results
- Only parameters can be set in `start_values`

### Euler Integration
- Use step size ≤ 0.01
- Tolerance ≈ 5% for Euler
- Error accumulates over time

---

## Common Commands

```bash
# Format and lint
cargo fmt --all
cargo clippy --workspace --all-targets

# Build single model
cargo build -p adml-{model-name}

# Test single model
cargo test -p adml-{model-name}

# Build FMU
./scripts/build-fmu.sh models/{category}/{model-name}

# Run all tests
./scripts/test-all.sh
```

---

## Minimal Checklist

- [ ] Directory structure created
- [ ] Added to workspace `Cargo.toml`
- [ ] Struct name matches directory pattern
- [ ] `do_step()` implements equations
- [ ] Basic unit test works
- [ ] Physics test validates solution
- [ ] FMU builds: `./scripts/build-fmu.sh`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings

---

## Next Steps

1. **More detail:** [AI_AGENTS.md](AI_AGENTS.md)
2. **Templates:** [AI_SCAFFOLDING.md](AI_SCAFFOLDING.md)
3. **Errors:** [ERROR_CATALOG.md](ERROR_CATALOG.md)
4. **Examples:** Study `models/mathematical/dahlquist/`
