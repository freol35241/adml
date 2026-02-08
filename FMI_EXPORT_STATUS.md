# FMI Export Status

## Summary

All six models are implemented using the **`fmi-export` crate** (v0.1.1) from [rust-fmi](https://github.com/jondo2010/rust-fmi), providing official FMI 3.0 Co-Simulation support.

## Current Implementation

Models use the `fmi-export` derive macro with the following pattern:

- `#[derive(FmuModel, Default, Debug)]` for model definition
- `#[model(co_simulation = true, user_model = false)]` for model-level configuration
- `#[variable(causality = Parameter, start = ..., initial = Exact)]` for parameters
- `#[variable(causality = Output, start = ..., initial = Exact)]` for outputs
- `#[variable(causality = Local, derivative = x, initial = Calculated)]` for derivatives
- `#[variable(skip)]` for internal fields not exposed via FMI
- `UserModel` trait implementation with `do_step()` and `calculate_values()` methods
- `fmi_export::export_fmu!(ModelName)` macro for FFI export

## Current Status

All six models build and pass all tests:

- **Mathematical**: Dahlquist, Van der Pol, Lorenz
- **Mechanical**: Bouncing Ball, Simple Pendulum
- **Thermal**: RC Thermal Single Zone

All tests pass: unit tests, physics validation tests, and integration tests.

## FMI 3.0 Features

The `fmi-export` crate provides:
- Proper derivative variable declarations in `modelDescription.xml`
- Co-Simulation mode with `do_step()` integration
- `calculate_values()` for algebraic output computation
- `configurate()` for post-initialization setup
- `#[variable(skip)]` for internal state not exposed via FMI
- Automatic `modelDescription.xml` generation via `FmuModel` derive macro
- FMU packaging via `cargo-fmi` tool

## Example Usage

```rust
use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{CSDoStepResult, Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct Dahlquist {
    #[variable(causality = Parameter, start = 1.0, initial = Exact)]
    pub k: f64,

    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub x: f64,

    #[variable(causality = Local, derivative = x, initial = Calculated)]
    der_x: f64,
}

impl UserModel for Dahlquist {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(
        &mut self,
        _context: &dyn Context<Self>,
    ) -> Result<Fmi3Res, Fmi3Error> {
        self.der_x = -self.k * self.x;
        Ok(Fmi3Res::OK)
    }

    fn do_step(
        &mut self,
        context: &mut dyn Context<Self>,
        current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Result<CSDoStepResult, Fmi3Error> {
        self.der_x = -self.k * self.x;
        self.x += self.der_x * communication_step_size;

        let target_time = current_communication_point + communication_step_size;
        context.set_time(target_time);
        Ok(CSDoStepResult::completed(target_time))
    }
}

fmi_export::export_fmu!(Dahlquist);
```

## Dependencies

Workspace dependencies:
```toml
[workspace.dependencies]
fmi-export = "0.1"
fmi = "0.6"
```

## FMU Building

FMUs can be built using the `cargo-fmi` tool:

```bash
cargo install cargo-fmi
cargo fmi --package adml-dahlquist
```

## References

- fmi-export crate: https://crates.io/crates/fmi-export
- rust-fmi GitHub: https://github.com/jondo2010/rust-fmi
- FMI Standard: https://fmi-standard.org/

---

**Last Updated**: 2026-02-08
**Status**: Migrated to fmi-export v0.1.1 from rust-fmi
