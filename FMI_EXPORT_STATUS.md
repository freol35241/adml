# FMI Export Status

## Summary

All three models (Dahlquist, Van der Pol, Bouncing Ball) are currently implemented using the **`fmu_from_struct` crate** ([github.com/jarlekramer/fmu_from_struct](https://github.com/jarlekramer/fmu_from_struct)). This is a **temporary solution** until the official `fmi-export` crate from [rust-fmi](https://github.com/jondo2010/rust-fmi) is published to crates.io.

## Current Implementation

Models use the `fmu_from_struct` derive macro with the following pattern:

- `#[derive(Fmu, Default, Debug, Clone)]` for model definition
- `#[fmu_from_struct(fmi_version = 3)]` to specify FMI version
- `#[fmu_from_struct(parameter)]`, `#[fmu_from_struct(output)]` for variable attributes
- `#[fmu_from_struct(start_value="...")]` for initial values
- `FmuFunctions` trait implementation with `exit_initialization_mode()` and `do_step()` methods

## Current Status

✅ **All models are working** - All three models build successfully and pass all tests:
- Dahlquist: Simple ODE with parameter k and state x
- Van der Pol: Nonlinear oscillator with two states (x0, x1)
- Bouncing Ball: Event-driven system with collision handling

✅ **Tests Pass** - All unit tests, physics tests pass successfully

✅ **Co-Simulation FMUs** - Models are implemented as Co-Simulation FMUs using the `do_step()` method

## Migration Path

### Plan

1. ✅ **Phase 1 (Current)**: Use `fmu_from_struct` crate (v0.2.1) as interim solution
2. 🔄 **Phase 2 (Future)**: Migrate to `fmi-export` once it's published to crates.io
3. 🔄 **Phase 3 (Future)**: Add Model Exchange support if needed

### When to Migrate

We will migrate from `fmu_from_struct` to `fmi-export` when:
- The `fmi-export` crate is published to crates.io as a standalone package
- The rust-fmi project stabilizes its API
- Build issues in the rust-fmi repository are resolved

### Migration Notes

The `fmi-export` crate provides more advanced features:
- Full event handling with `event_update()` and `get_event_indicators()`
- Both Model Exchange and Co-Simulation modes
- More sophisticated variable attributes
- Better FMI 3.0 compliance

The `fmu_from_struct` crate is simpler but sufficient for our current needs:
- Co-Simulation only
- Event handling must be done manually within `do_step()`
- Simpler attribute system

## Example Usage (Current)

```rust
use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct Dahlquist {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value="1.0")]
    pub k: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value="1.0")]
    pub x: f64,

    pub fmu_info: FmuInfo,
}

impl FmuFunctions for Dahlquist {
    fn exit_initialization_mode(&mut self) {
        // Initialization code
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Integration step (e.g., Euler method)
        let der_x = -self.k * self.x;
        self.x += der_x * time_step;
    }
}
```

## Example Usage (Future with fmi-export)

```rust
use fmi_export::{FmuModel, fmi3::{ModelContext, UserModel}};

#[derive(FmuModel, Default, Debug)]
#[model()]
pub struct Dahlquist {
    #[variable(causality = Output, state, start = 1.0, initial = Exact)]
    pub x: f64,

    #[variable(causality = Local, derivative = x, initial = Calculated)]
    der_x: f64,

    #[variable(causality = Parameter, start = 1.0, initial = Exact)]
    pub k: f64,
}

impl UserModel for Dahlquist {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &ModelContext<Self>)
        -> Result<Fmi3Res, Fmi3Error> {
        self.der_x = -self.k * self.x;
        Ok(Fmi3Res::OK)
    }
}

fmi_export::export_fmu!(Dahlquist);
```

## Testing

All tests are passing with the current `fmu_from_struct` implementation:
- ✅ Unit tests for all models
- ✅ Physics validation tests
- ✅ Integration tests

## Dependencies

Current workspace dependencies:
```toml
[workspace.dependencies]
fmu_from_struct = "0.2"
```

## References

- fmu_from_struct GitHub: https://github.com/jarlekramer/fmu_from_struct
- rust-fmi GitHub: https://github.com/jondo2010/rust-fmi
- FMI Standard: https://fmi-standard.org/

---

**Last Updated**: 2025-11-17
**Status**: ✅ Working with fmu_from_struct v0.2.1
**Next Step**: Migrate to fmi-export once published
