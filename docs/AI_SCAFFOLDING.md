# AI Agent Scaffolding Templates

This document provides ready-to-use templates for AI agents implementing dynamical models in ADML. Simply copy the templates and fill in your model-specific details.

## 📁 Directory Structure Template

Create this structure for your new model:

```
models/category/model-name/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
│   └── physics_tests.rs
└── README.md
```

**Commands:**
```bash
mkdir -p models/category/model-name/src
mkdir -p models/category/model-name/tests
touch models/category/model-name/Cargo.toml
touch models/category/model-name/src/lib.rs
touch models/category/model-name/tests/physics_tests.rs
touch models/category/model-name/README.md
```

## 📦 Cargo.toml Template

```toml
[package]
name = "adml-model-name"        # Use lowercase with hyphens
version = "1.0.0"
edition = "2021"
description = "Brief description of your model"
authors = ["AI Agent"]  # Indicate AI-generated code
license.workspace = true
repository.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]  # Required for FMU generation

[dependencies]
fmu_from_struct = { workspace = true }
# Add other dependencies if needed (rare for simple models)

[dev-dependencies]
physics-framework = { path = "../../../testing/physics-framework" }
approx = { workspace = true }

[package.metadata.fmi]
model_name = "ModelName"         # CamelCase name for FMU
fmi_version = "3.0"
guid = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"  # Generate unique GUID
description = "Detailed model description"
```

**Generate GUID:**
- Linux/Mac: `uuidgen`
- PowerShell: `[guid]::NewGuid()`
- Online: https://www.uuidgenerator.net/

## 📝 src/lib.rs Template (Simple Single-State Model)

```rust
//! Model Name
//!
//! Brief description of the model.
//!
//! Mathematical model:
//! dx/dt = f(x, parameters, t)
//!
//! Where:
//! - x: state variable
//! - parameters: model parameters
//! - t: time

// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

/// Model Name struct
///
/// Detailed description of what this model represents.
///
/// # Equations
///
/// The model implements:
/// - State equation: dx/dt = ...
///
/// # Parameters
///
/// - `param1`: Description (units)
///
/// # State Variables
///
/// - `state1`: Description (units)
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct ModelName {
    // === Parameters (can be set via FMI) ===
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    /// Parameter description
    /// Units: specify units here
    pub param1: f64,

    // === State Variables / Outputs (read-only via FMI) ===
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    /// State variable description
    /// Units: specify units here
    pub state1: f64,

    // === Internal variables (not exposed via FMI) ===
    /// Current simulation time
    time: f64,
}

impl FmuFunctions for ModelName {
    /// Perform one integration step using Euler method
    ///
    /// # Arguments
    ///
    /// * `current_time` - Current simulation time (not used in this implementation)
    /// * `step_size` - Time step for integration (dt)
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        // Calculate derivative: der_state1 = f(state1, param1)
        let der_state1 = /* YOUR EQUATION HERE */;

        // Euler integration: state(t+dt) = state(t) + der_state * dt
        self.state1 += der_state1 * step_size;

        // Update time
        self.time += step_size;
    }
}

// === Helper Methods (for testing and validation) ===
impl ModelName {
    /// Calculate analytical solution (if available)
    ///
    /// # Arguments
    ///
    /// * `param1` - Parameter value
    /// * `initial_state` - Initial value of state1
    /// * `t` - Time at which to evaluate solution
    ///
    /// # Returns
    ///
    /// Value of state1 at time t
    pub fn analytical_solution(param1: f64, initial_state: f64, t: f64) -> f64 {
        // YOUR ANALYTICAL SOLUTION HERE
        // Example: initial_state * (-param1 * t).exp()
        todo!("Implement analytical solution or remove if not available")
    }
}

// === Unit Tests ===
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_initialization() {
        let model = ModelName::new();

        // Test default values
        assert_eq!(model.param1, 1.0);
        assert_eq!(model.state1, 1.0);
        assert_eq!(model.time, 0.0);
    }

    #[test]
    fn test_derivative_calculation() {
        let model = ModelName::new();

        // Calculate derivative manually
        let der_state1 = /* YOUR EQUATION */;

        // Test that it matches expected value
        assert_relative_eq!(der_state1, /* EXPECTED */, epsilon = 1e-10);
    }

    #[test]
    fn test_parameter_effect() {
        let mut model = ModelName::new();

        // Test with different parameter value
        model.param1 = 2.0;
        let der_with_param2 = /* calculate derivative */;

        model.param1 = 1.0;
        let der_with_param1 = /* calculate derivative */;

        // Verify parameter affects derivative as expected
        assert!(der_with_param2 != der_with_param1);
    }

    #[test]
    fn test_one_step() {
        let mut model = ModelName::new();

        // Take one integration step
        let dt = 0.1;
        model.do_step(0.0, dt);

        // Verify state changed as expected
        // Example: after one step, state1 should be initial + derivative*dt
        let expected = 1.0 + /* derivative */ * dt;
        assert_relative_eq!(model.state1, expected, epsilon = 1e-10);
    }
}
```

## 📝 src/lib.rs Template (Multi-State Model)

For models with multiple coupled differential equations:

```rust
//! Multi-State Model
//!
//! Model description with coupled differential equations.
//!
//! State equations:
//! dx0/dt = f0(x0, x1, parameters)
//! dx1/dt = f1(x0, x1, parameters)

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct MultiStateModel {
    // === Parameters ===
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub param1: f64,

    // === State Variables ===
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "2.0")]
    /// First state variable
    pub x0: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    /// Second state variable
    pub x1: f64,

    time: f64,
}

impl FmuFunctions for MultiStateModel {
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        // Calculate derivatives for all states
        let der_x0 = /* f0(x0, x1, param1) */;
        let der_x1 = /* f1(x0, x1, param1) */;

        // Euler integration for all states
        self.x0 += der_x0 * step_size;
        self.x1 += der_x1 * step_size;

        self.time += step_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_initialization() {
        let model = MultiStateModel::new();
        assert_eq!(model.x0, 2.0);
        assert_eq!(model.x1, 0.0);
    }

    #[test]
    fn test_coupling() {
        let model = MultiStateModel::new();

        // Calculate derivatives
        let der_x0 = /* ... */;
        let der_x1 = /* ... */;

        // Verify x0 and x1 are coupled (changing one affects the other's derivative)
        // Test your specific coupling here
    }
}
```

## 🧪 tests/physics_tests.rs Template

```rust
//! Physics validation tests for ModelName
//!
//! These tests verify physical correctness of the model by comparing
//! simulation results against analytical solutions, conservation laws,
//! and known physical properties.

use approx::assert_relative_eq;
use adml_model_name::{FmuFunctions, ModelName};

// === Analytical Solution Tests ===

#[test]
fn test_analytical_solution_default() {
    /// Verify simulation matches analytical solution with default parameters
    let mut model = ModelName::new();

    // Simulation parameters
    let dt = 0.01; // Small step for Euler accuracy
    let t_final = 5.0;
    let steps = (t_final / dt) as usize;

    // Run simulation
    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // Compare to analytical solution
    let expected = ModelName::analytical_solution(model.param1, 1.0, t_final);
    assert_relative_eq!(
        model.state1,
        expected,
        epsilon = 0.05 // 5% tolerance for Euler integration
    );
}

#[test]
fn test_analytical_solution_custom_params() {
    /// Test with non-default parameter values
    let mut model = ModelName::new();
    model.param1 = 2.0;

    let dt = 0.01;
    let t_final = 2.0;

    for _ in 0..(t_final / dt) as usize {
        model.do_step(0.0, dt);
    }

    let expected = ModelName::analytical_solution(2.0, 1.0, t_final);
    assert_relative_eq!(model.state1, expected, epsilon = 0.05);
}

// === Conservation Law Tests ===

#[test]
fn test_energy_conservation() {
    /// For conservative systems: verify energy is conserved
    let mut model = ModelName::new();

    let initial_energy = model.total_energy(); // Implement this method

    // Simulate
    for _ in 0..1000 {
        model.do_step(0.0, 0.01);
    }

    let final_energy = model.total_energy();
    assert_relative_eq!(initial_energy, final_energy, epsilon = 0.01);
}

// === Physical Property Tests ===

#[test]
fn test_stability() {
    /// Verify system is stable (doesn't grow unbounded)
    let mut model = ModelName::new();

    for _ in 0..10000 {
        model.do_step(0.0, 0.01);
    }

    // State should remain bounded
    assert!(model.state1.abs() < 100.0, "System became unstable");
}

#[test]
fn test_steady_state() {
    /// Verify system reaches expected steady state
    let mut model = ModelName::new();

    // Run long enough to reach steady state
    for _ in 0..10000 {
        model.do_step(0.0, 0.01);
    }

    // Check steady state value
    let expected_steady_state = 0.0; // YOUR EXPECTED VALUE
    assert_relative_eq!(model.state1, expected_steady_state, epsilon = 0.01);
}

// === Convergence Tests ===

#[test]
fn test_convergence_with_step_size() {
    /// Verify solution converges as step size decreases
    fn simulate_until(dt: f64, t_final: f64) -> f64 {
        let mut model = ModelName::new();
        for _ in 0..(t_final / dt) as usize {
            model.do_step(0.0, dt);
        }
        model.state1
    }

    let t_final = 1.0;
    let result_coarse = simulate_until(0.1, t_final);
    let result_medium = simulate_until(0.01, t_final);
    let result_fine = simulate_until(0.001, t_final);

    // Get reference from analytical solution
    let reference = ModelName::analytical_solution(1.0, 1.0, t_final);

    // Error should decrease with step size
    let error_coarse = (result_coarse - reference).abs();
    let error_medium = (result_medium - reference).abs();
    let error_fine = (result_fine - reference).abs();

    assert!(error_medium < error_coarse);
    assert!(error_fine < error_medium);
}

// === Boundary Condition Tests ===

#[test]
fn test_zero_initial_condition() {
    /// Test behavior with zero initial state
    let mut model = ModelName::new();
    model.state1 = 0.0;

    model.do_step(0.0, 0.1);

    // Verify behavior is correct at boundary
    // (Depends on your specific model)
}

#[test]
fn test_negative_parameter() {
    /// Test with negative parameter (if physically meaningful)
    let mut model = ModelName::new();
    model.param1 = -1.0;

    // Should either work correctly or handle gracefully
    model.do_step(0.0, 0.1);

    // Add assertions based on expected behavior
}

// === Edge Case Tests ===

#[test]
fn test_large_parameter() {
    /// Test with large parameter values
    let mut model = ModelName::new();
    model.param1 = 100.0;

    // Should remain stable
    for _ in 0..100 {
        model.do_step(0.0, 0.001); // Small dt for stiff systems
    }

    assert!(model.state1.is_finite());
}

#[test]
fn test_very_small_step() {
    /// Verify model works with very small time steps
    let mut model = ModelName::new();

    model.do_step(0.0, 1e-6);

    assert!(model.state1.is_finite());
    assert!(model.time > 0.0);
}
```

## 🐍 Python FMU Integration Test Template

Create `testing/fmu-integration-tests/test_model_name_fmu.py`:

```python
"""
FMU integration tests for ModelName

These tests validate the FMU file using FMPy and verify:
1. FMU structure and FMI 3.0 compliance
2. Simulation accuracy
3. Physics validation
4. Parameter sensitivity
"""

import pytest
import numpy as np
from pathlib import Path
from fmu_test_utils import (
    get_fmu_path,
    simulate_fmu,
    compare_with_analytical,
)


class TestModelNameFMU:
    """Integration tests for ModelName FMU"""

    @pytest.fixture
    def fmu_path(self):
        """Path to the ModelName FMU file"""
        return get_fmu_path("ModelName")

    @pytest.fixture
    def default_params(self):
        """Default parameters for ModelName"""
        return {
            'param1': 1.0,
        }

    def test_fmu_exists(self, fmu_path):
        """FMU file should exist"""
        assert fmu_path.exists(), f"FMU not found at {fmu_path}"

    def test_fmu_loads(self, fmu_path):
        """FMU should load without errors"""
        import fmpy
        model_description = fmpy.read_model_description(str(fmu_path))
        assert model_description is not None
        assert model_description.fmiVersion == "3.0"

    def test_basic_simulation(self, fmu_path, default_params):
        """FMU should simulate without errors"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=1.0,
            parameters=default_params,
            step_size=0.01,
        )

        assert len(time) > 0
        assert 'state1' in results
        assert np.all(np.isfinite(results['state1']))

    def test_analytical_solution_default(self, fmu_path, default_params):
        """FMU results should match analytical solution with default parameters"""
        stop_time = 5.0
        time, results = simulate_fmu(
            fmu_path,
            stop_time=stop_time,
            parameters=default_params,
            step_size=0.01,
            output_interval=0.1,
        )

        def analytical(t):
            # YOUR ANALYTICAL SOLUTION HERE
            # Example: np.exp(-default_params['param1'] * t)
            return np.ones_like(t)  # REPLACE THIS

        matches, max_error = compare_with_analytical(
            time, results['state1'], analytical,
            rtol=5e-2,  # 5% relative tolerance
            atol=1e-3,  # Absolute tolerance for small values
        )

        assert matches, f"FMU results don't match analytical solution. Max error: {max_error}"

    def test_parameter_sensitivity(self, fmu_path):
        """Changing parameters should affect results"""
        time1, results1 = simulate_fmu(
            fmu_path,
            stop_time=1.0,
            parameters={'param1': 1.0},
            step_size=0.01,
        )

        time2, results2 = simulate_fmu(
            fmu_path,
            stop_time=1.0,
            parameters={'param1': 2.0},
            step_size=0.01,
        )

        # Results should be different
        assert not np.allclose(results1['state1'], results2['state1'])

    def test_steady_state(self, fmu_path, default_params):
        """Model should reach steady state"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=100.0,  # Long enough to reach steady state
            parameters=default_params,
            step_size=0.01,
            output_interval=1.0,
        )

        # Check last few values are approximately constant
        final_values = results['state1'][-10:]
        assert np.std(final_values) < 0.01, "Did not reach steady state"

    @pytest.mark.slow
    def test_long_simulation(self, fmu_path, default_params):
        """Model should remain stable over long simulations"""
        time, results = simulate_fmu(
            fmu_path,
            stop_time=1000.0,
            parameters=default_params,
            step_size=0.01,
            output_interval=1.0,
        )

        # All values should remain finite
        assert np.all(np.isfinite(results['state1']))

        # Should not grow unbounded
        assert np.max(np.abs(results['state1'])) < 1000.0


# Add model-specific tests below
```

## 📄 README.md Template

```markdown
# Model Name

Brief one-sentence description.

## Model Description

Detailed description of the physical/mathematical system this model represents.

### Background

Historical context, applications, importance.

## Mathematical Formulation

The model implements the following differential equation(s):

$$\frac{dx}{dt} = f(x, parameters, t)$$

Where:
- $x$ : state variable (units)
- $parameters$ : model parameters
- $t$ : time (s)

### Assumptions

- List key assumptions
- Simplifications made
- Limitations

## Parameters

| Name | Symbol | Type | Default | Units | Description |
|------|--------|------|---------|-------|-------------|
| param1 | k | Real | 1.0 | 1/s | Decay rate constant |

## State Variables

| Name | Symbol | Type | Initial | Units | Description |
|------|--------|------|---------|-------|-------------|
| state1 | x | Real | 1.0 | m | Position |

## FMI Interface

### Inputs
- None (for autonomous systems)
- Or list inputs

### Outputs
- `state1` : Main state variable
- List all outputs

### Parameters
- `param1` : Can be set before simulation

## Usage

### Building the FMU

```bash
./scripts/build-fmu.sh models/category/model-name
```

The FMU file will be created at: `fmus/ModelName.fmu`

### Example Simulation (Python with FMPy)

```python
import fmpy

result = fmpy.simulate_fmu(
    'fmus/ModelName.fmu',
    stop_time=10.0,
    step_size=0.01,
    start_values={'param1': 1.0}
)

import matplotlib.pyplot as plt
plt.plot(result['time'], result['state1'])
plt.show()
```

### Running Tests

```bash
# Rust tests (unit + physics)
cargo test -p adml-model-name

# FMU integration tests (requires FMU to be built first)
pytest testing/fmu-integration-tests/test_model_name_fmu.py -v
```

## Physics Validation

This model is validated against:

1. **Analytical Solution** - Compared to exact solution: $x(t) = ...$
2. **Conservation Laws** - Energy conservation verified (if applicable)
3. **Known Properties** - List specific properties tested
4. **Convergence** - Solution converges with decreasing step size

See `tests/physics_tests.rs` for details.

## Implementation Notes

### Numerical Method

Uses explicit Euler integration:
- First-order accurate
- Requires small time steps (recommend dt ≤ 0.01)
- Unconditionally stable for this system (or note if conditionally stable)

### AI Implementation

This model was implemented by [AI Agent Name] on [Date].

#### Challenges Encountered
- List any difficulties during implementation
- How they were solved

#### Verification Strategy
- How correctness was verified
- What tests were most useful

## References

1. Author, "Title", Journal, Year
2. Book or online resource used
3. Related models or implementations

## Version History

- 1.0.0 (YYYY-MM-DD): Initial implementation by [AI Agent]

## License

Dual licensed under MIT and Apache 2.0 (see repository root).
```

## 🚀 Quick Start Checklist for AI Agents

Using these templates:

- [ ] Create directory structure
- [ ] Copy Cargo.toml template, fill in name and GUID
- [ ] Copy src/lib.rs template (single or multi-state)
- [ ] Implement differential equations in `do_step()`
- [ ] Add unit tests
- [ ] Copy physics_tests.rs template
- [ ] Implement analytical solution (if available)
- [ ] Build FMU: `./scripts/build-fmu.sh models/category/model-name`
- [ ] Copy Python test template
- [ ] Implement Python integration tests
- [ ] Copy README template
- [ ] Run all tests: `./scripts/test-all.sh`
- [ ] Format code: `cargo fmt`
- [ ] Check clippy: `cargo clippy`
- [ ] Submit PR

## 📚 Example Models to Study

Study these for reference while filling in templates:

- **Simple**: `models/mathematical/dahlquist/`
- **Multi-state**: `models/mathematical/van-der-pol/`
- **Events**: `models/mechanical/bouncing-ball/`

---

*These templates are maintained by the ADML community. Suggestions for improvements welcome from all AI agents!*
