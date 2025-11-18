# Contributing to ODML

Thank you for your interest in contributing to the Open Dynamical Model Library!

## 🎯 Ways to Contribute

- **Add new models** - Implement additional dynamical systems
- **Improve existing models** - Enhance accuracy or performance
- **Improve tests** - Add more physics validation tests
- **Fix bugs** - Report or fix issues
- **Improve documentation** - Enhance README files or code comments
- **Enhance infrastructure** - Improve CI/CD, tooling, or build systems

## 📋 Prerequisites

Before contributing, ensure you have:
- Rust 1.70 or later installed
- Git for version control
- Familiarity with dynamical systems and differential equations
- Understanding of the FMI standard (recommended)

## 🚀 Getting Started

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/odml.git
   cd odml
   ```

2. **Create a branch**
   ```bash
   git checkout -b feature/your-model-name
   ```

3. **Make your changes**
   - Follow the guidelines below
   - Test thoroughly

4. **Submit a pull request**
   - Describe your changes
   - Reference any related issues

## 🏗️ Adding a New Model

### 1. Choose the Right Category

Place your model in the appropriate category:
- `models/mathematical/` - Mathematical test equations, benchmark problems
- `models/mechanical/` - Mechanical systems (rigid body dynamics, kinematics)
- `models/electrical/` - Electrical circuits and systems
- `models/thermal/` - Heat transfer and thermodynamics
- `models/hydraulic/` - Fluid dynamics and hydraulic systems

Create a new category if needed.

### 2. Directory Structure

```
models/category/model-name/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
│   └── physics_tests.rs
└── README.md
```

### 3. Cargo.toml Template

```toml
[package]
name = "odml-model-name"
version = "1.0.0"
edition = "2021"
description = "Brief description of your model"
authors = ["Your Name <your.email@example.com>"]
license.workspace = true
repository.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
fmi = { workspace = true }
# Add other dependencies as needed

[dev-dependencies]
physics-framework = { path = "../../../testing/physics-framework" }
approx = { workspace = true }

[package.metadata.fmi]
model_name = "YourModelName"
fmi_version = "3.0"
guid = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"  # Generate a unique GUID
description = "Detailed model description"
```

Generate a GUID online or use: `uuidgen` (Linux/Mac) or `[guid]::NewGuid()` (PowerShell)

### 4. Model Implementation

Your `src/lib.rs` should include:

```rust
//! Model Name
//!
//! Detailed description of the model, including:
//! - Physical system being modeled
//! - Differential equations
//! - Assumptions and limitations

use std::ffi::c_void;

/// Model state structure
#[repr(C)]
pub struct YourModel {
    // State variables
    pub state1: f64,
    pub state2: f64,
    // Parameters
    pub param1: f64,
    // Time
    pub time: f64,
}

impl YourModel {
    pub fn new() -> Self {
        Self {
            // Default initial conditions
            state1: 0.0,
            state2: 0.0,
            param1: 1.0,
            time: 0.0,
        }
    }

    pub fn get_number_of_continuous_states(&self) -> usize {
        2 // Number of differential states
    }

    pub fn get_continuous_states(&self) -> Vec<f64> {
        vec![self.state1, self.state2]
    }

    pub fn set_continuous_states(&mut self, states: &[f64]) {
        if states.len() >= 2 {
            self.state1 = states[0];
            self.state2 = states[1];
        }
    }

    pub fn get_derivatives(&self) -> Vec<f64> {
        // Implement your differential equations here
        let der_state1 = /* ... */;
        let der_state2 = /* ... */;
        vec![der_state1, der_state2]
    }

    pub fn do_step(&mut self, dt: f64) {
        // Simple Euler integration
        let derivatives = self.get_derivatives();
        self.state1 += derivatives[0] * dt;
        self.state2 += derivatives[1] * dt;
        self.time += dt;
    }
}

// Include unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        // Test default initialization
    }

    #[test]
    fn test_derivatives() {
        // Test derivative calculations
    }

    #[test]
    fn test_state_operations() {
        // Test get/set state operations
    }
}
```

### 5. Physics Validation Tests

Create `tests/physics_tests.rs`:

```rust
use odml_your_model::YourModel;
use physics_framework::assertions::*;
use approx::assert_relative_eq;

#[test]
fn test_physical_property_1() {
    let mut model = YourModel::new();

    // Set up test conditions

    // Run simulation
    let dt = 0.001;
    for _ in 0..1000 {
        model.do_step(dt);
    }

    // Verify physical properties
    // e.g., energy conservation, steady state, analytical solution
}

#[test]
fn test_convergence() {
    // Test that solution converges with smaller step sizes
}

#[test]
fn test_boundary_conditions() {
    // Test behavior at boundaries
}
```

### 6. Documentation

Create a comprehensive `README.md`:

```markdown
# Your Model Name

Brief description of the model.

## Model Description

Detailed description including:
- Physical system
- Mathematical equations (use LaTeX/math notation if needed)
- Assumptions

## Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| param1 | Real | 1.0 | Description |

## State Variables

| Name | Type | Initial | Description |
|------|------|---------|-------------|
| state1 | Real | 0.0 | Description |

## Usage

Examples of how to use the model.

## Physics Validation

Describe what physics properties are tested and why.

## References

Cite any papers, books, or resources used.
```

## ✅ Code Quality Standards

### Formatting

- Run `cargo fmt` before committing
- Use standard Rust formatting conventions

### Linting

- Code must pass `cargo clippy -- -D warnings`
- Fix all warnings

### Testing

All models must include:
1. **Unit tests** in `src/lib.rs`
   - Test initialization
   - Test calculations
   - Test state operations

2. **Physics tests** in `tests/physics_tests.rs`
   - Validate against analytical solutions (where available)
   - Test conservation laws (energy, momentum, etc.)
   - Test convergence properties
   - Test boundary conditions
   - Test event handling (if applicable)

Aim for:
- Clear test names describing what is tested
- Good test coverage
- Fast-running tests (use small time steps only when necessary)

### Documentation

- Public items must have doc comments
- Use `///` for documentation
- Include examples in doc comments where helpful
- Equations should be clearly documented

## 🔍 Code Review Process

1. **Automated checks** - CI must pass:
   - Formatting (`cargo fmt --check`)
   - Linting (`cargo clippy`)
   - Build (`cargo build`)
   - Tests (`cargo test`)

2. **Manual review**:
   - Code quality and clarity
   - Correctness of physics implementation
   - Test coverage and quality
   - Documentation completeness

3. **Physics validation**:
   - Are the equations correct?
   - Are the tests validating the right properties?
   - Is the model physically meaningful?

## 📐 Physics and Mathematics Guidelines

### Differential Equations

- Clearly document the ODE/DAE system
- Use standard notation where possible
- Provide references for complex formulations

### Numerical Methods

- Simple Euler integration is fine for demos
- Note any stability limitations
- Consider providing more sophisticated integrators if needed

### Units

- Clearly document units for all quantities
- Use SI units by default unless there's a good reason not to
- Be consistent within a model

### Validation

- Compare with analytical solutions when available
- Use published data or established models for validation
- Document validation methodology

## 🐛 Reporting Issues

When reporting issues:
1. **Check existing issues** first
2. **Provide details**:
   - What you expected
   - What actually happened
   - Steps to reproduce
   - Your environment (OS, Rust version)
3. **Include code samples** if applicable
4. **Be respectful** and constructive

## 💬 Questions?

- Open a [GitHub Discussion](https://github.com/freol35241/odml/discussions)
- Check existing documentation
- Look at existing models for examples

## 📜 License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache 2.0, matching the project license.

## 🙏 Thank You!

Your contributions help make ODML a valuable resource for the modeling and simulation community!
