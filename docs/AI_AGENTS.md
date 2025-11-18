# AI Agent Implementation Guide

This guide is written **for AI agents by an AI agent** (Claude Sonnet 4.5), based on actual experience implementing the models in this repository. It covers strategies, common pitfalls, and solutions specific to implementing dynamical models with AI coding agents.

## 🎯 Prerequisites for AI Agents

Before implementing a model, ensure you have:

### Required Information
1. **Model specification** - Differential equations, parameters, initial conditions
2. **Physics validation criteria** - Analytical solutions, conservation laws, or reference data
3. **FMI requirements** - Which variables are parameters vs outputs

### Recommended Context
- Access to FMI 3.0 specification
- Understanding of numerical integration (Euler method minimum)
- Example of existing model in the repository
- Access to tools: `cargo`, `package_fmu_after_build`, `pytest`

## 🔄 Recommended Workflow

Based on actual implementation experience, this workflow minimizes iterations:

### Phase 1: Setup and Scaffolding (Use TODO Tool)
```markdown
TODO List:
1. Create model directory structure
2. Set up Cargo.toml with correct dependencies
3. Create basic struct with `fmu_from_struct` derives
4. Implement skeleton with placeholder methods
5. Verify compilation
```

**Key Insight:** Set up the full structure first before implementing physics. This catches dependency issues early.

### Phase 2: Core Implementation
```markdown
TODO List:
1. Implement differential equations in `get_derivatives()` or directly in `do_step()`
2. Add unit tests for derivative calculations
3. Verify physics equations match specification
4. Test edge cases (zero values, negative values)
```

**Common Pitfall:** Forgetting that `fmu_from_struct` uses Euler integration by default in `do_step()`. If you override `do_step()`, you must implement integration yourself.

### Phase 3: Physics Validation
```markdown
TODO List:
1. Implement analytical solution (if available)
2. Write physics tests comparing simulation to analytical solution
3. Test conservation laws (energy, momentum, etc.)
4. Test convergence with different step sizes
5. Add boundary condition tests
```

**Key Insight:** Use small step sizes (0.01 or smaller) for Euler integration. Tests will fail with large steps.

### Phase 4: FMU Building and Integration Testing
```markdown
TODO List:
1. Build FMU with `./scripts/build-fmu.sh models/category/model-name`
2. Create Python integration tests using FMPy
3. Test parameter setting (only causality="parameter" can be set!)
4. Test simulation accuracy
5. Run all three test tiers
```

**Common Pitfall:** Trying to set output variables as parameters in FMPy will fail. Only variables marked with `#[fmu_from_struct(parameter)]` can be set.

### Phase 5: Documentation and PR
```markdown
TODO List:
1. Write README.md with equations and usage
2. Add inline doc comments
3. Verify all tests pass
4. Format code with `cargo fmt`
5. Run clippy and fix warnings
6. Submit PR
```

## 🧠 Task Decomposition Strategy

### Breaking Down Model Implementation

**Instead of:** "Implement the Van der Pol oscillator"

**Do this:**
1. "Create directory and Cargo.toml"
2. "Define struct with state variables x0, x1, and parameter mu"
3. "Implement dx0/dt = x1"
4. "Implement dx1/dt = mu*(1-x0²)*x1 - x0"
5. "Write test to verify derivatives at x0=0, x1=0"
6. "Write test to verify derivatives at x0=1, x1=1"
7. "Implement analytical solution for mu→0 case (harmonic oscillator)"
8. "Write physics test comparing to harmonic oscillator"

**Why this works:** Each step is verifiable independently. You catch errors earlier.

### Using TODO Tools Effectively

```python
# Good TODO structure
TodoWrite([
    {"content": "Create model struct with FMU derives", "status": "in_progress", "activeForm": "Creating model struct"},
    {"content": "Implement derivative calculations", "status": "pending", "activeForm": "Implementing derivatives"},
    {"content": "Write unit tests for derivatives", "status": "pending", "activeForm": "Writing unit tests"},
    {"content": "Build FMU and verify it loads", "status": "pending", "activeForm": "Building FMU"},
])
```

Mark tasks complete IMMEDIATELY after finishing. Don't batch completions.

## 🔬 Physics Validation Strategies

### Strategy 1: Analytical Solutions

**Best when available.** Compare simulation to closed-form solution.

```rust
#[test]
fn test_analytical_solution() {
    let mut model = Dahlquist::new();

    // Simulate
    let dt = 0.01;  // Small step for Euler
    for _ in 0..100 {
        model.do_step(0.0, dt);
    }

    // Compare to analytical solution: x(t) = x0 * exp(-k*t)
    let t = 1.0;
    let expected = 1.0 * (-1.0 * t).exp();
    assert_relative_eq!(model.x, expected, epsilon = 0.05);  // 5% tolerance for Euler
}
```

**Key Point:** Euler integration is first-order accurate. Use appropriate tolerances (5% is reasonable for simple Euler).

### Strategy 2: Conservation Laws

**For conservative systems.** Energy, momentum, etc.

```rust
#[test]
fn test_energy_conservation() {
    let mut model = MyModel::new();
    let initial_energy = model.total_energy();

    // Simulate
    for _ in 0..1000 {
        model.do_step(0.0, 0.01);
    }

    let final_energy = model.total_energy();
    assert_relative_eq!(initial_energy, final_energy, epsilon = 0.01);
}
```

### Strategy 3: Known Properties

**When no analytical solution exists.**

```rust
#[test]
fn test_limit_cycle() {
    // Van der Pol oscillator should approach a limit cycle
    let mut model = VanDerPol::new();

    // Run long enough to reach limit cycle
    for _ in 0..10000 {
        model.do_step(0.0, 0.01);
    }

    // Verify amplitude is in expected range
    assert!(model.x0.abs() < 3.0);  // Limit cycle amplitude
}
```

### Strategy 4: Convergence Testing

**Universal strategy.** Solution should converge with smaller steps.

```rust
#[test]
fn test_convergence() {
    let results_coarse = simulate_with_step(0.1);
    let results_fine = simulate_with_step(0.01);
    let results_finer = simulate_with_step(0.001);

    // Error should decrease with step size
    let error_coarse = (results_coarse - reference).abs();
    let error_fine = (results_fine - reference).abs();
    assert!(error_fine < error_coarse);
}
```

## ⚠️ Common Pitfalls and Solutions

### Pitfall 1: FMI Variable Causality Confusion

**Problem:**
```python
# This FAILS in FMPy
simulate_fmu(fmu_path, start_values={'x': 2.0})  # x is an output!
```

**Solution:**
Only set variables marked as parameters:
```rust
#[fmu_from_struct(parameter)]  // ✅ Can be set
#[fmu_from_struct(start_value = "1.0")]
pub k: f64,

#[fmu_from_struct(output)]  // ❌ Cannot be set (uses start_value from model)
#[fmu_from_struct(start_value = "1.0")]
pub x: f64,
```

### Pitfall 2: FMPy Step Size Handling

**Problem:**
```python
# When output_interval > step_size, FMPy may not respect step_size
result = simulate_fmu(fmu_path, step_size=0.01, output_interval=0.1)
# May take ONE large step instead of 10 small steps!
```

**Solution:**
Always set `output_interval = step_size` and downsample results if needed. See `testing/fmu-integration-tests/fmu_test_utils.py` for the correct implementation.

### Pitfall 3: FMU Packaging Directory Names

**Problem:**
```bash
# build-fmu.sh creates directory "odml-dahlquist" (hyphen)
# But package_fmu_after_build expects "odml_dahlquist" (underscore)
```

**Solution:**
The script handles this automatically:
```bash
DIR_NAME="${PACKAGE_NAME//-/_}"  # Convert hyphens to underscores
```

Don't manually create FMU directories - use the build scripts.

### Pitfall 4: Clippy Warnings from Derive Macros

**Problem:**
```
error: this public function might dereference a raw pointer but is not marked `unsafe`
  --> src/lib.rs:17:10
   |
17 | #[derive(Fmu, Default, Debug, Clone)]
   |          ^^^
```

**Solution:**
Add allow attributes at the top of your lib.rs:
```rust
// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
```

These are not issues in your code but in the generated macro code.

### Pitfall 5: Forgotten `cargo clean` Before FMU Build

**Problem:**
Old `modelDescription.xml` gets packaged in FMU, doesn't reflect recent changes.

**Solution:**
The build scripts handle this:
```bash
rm -f modelDescription.xml
cargo clean -p "$PACKAGE_NAME" --release
cargo build -p "$PACKAGE_NAME" --release
```

### Pitfall 6: Integration Test Step Size

**Problem:**
```python
# Using default step size leads to inaccurate results
result = simulate_fmu(fmu_path, stop_time=5.0)  # No step_size specified
```

**Solution:**
Always specify step_size for Euler integration:
```python
result = simulate_fmu(fmu_path, stop_time=5.0, step_size=0.01)
```

## 🐛 Debugging Strategies for AI Agents

### Strategy 1: Incremental Verification

**Don't implement everything then test.** Verify each component:

```rust
// Step 1: Verify struct compiles
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct MyModel {
    pub x: f64,
}

// Step 2: Verify default values
#[test]
fn test_defaults() {
    let m = MyModel::new();
    assert_eq!(m.x, expected_default);
}

// Step 3: Verify derivatives
#[test]
fn test_derivatives() {
    let m = MyModel::new();
    let der_x = /* calculate */;
    assert_eq!(der_x, expected);
}

// Step 4: Verify integration (one step)
#[test]
fn test_one_step() {
    let mut m = MyModel::new();
    m.do_step(0.0, 0.1);
    assert_relative_eq!(m.x, expected_after_one_step);
}
```

### Strategy 2: Isolate Physics vs Implementation

**Problem:** Test fails, but is it physics or code?

```rust
// Test the physics equation separately
#[test]
fn test_equation_at_known_point() {
    // At x=1, k=1, we know der_x should be -1
    let der_x = -1.0 * 1.0;
    assert_eq!(der_x, -1.0);  // This should always pass
}

// Then test the implementation
#[test]
fn test_implementation() {
    let model = Dahlquist::new();
    let der_x = -model.k * model.x;
    assert_eq!(der_x, -1.0);  // Now test your code
}
```

### Strategy 3: Use Small Test Cases

```rust
// Don't start with:
#[test]
fn test_complex_scenario() {
    // 1000 lines of setup
    // Multiple parameters
    // Long simulation
    // Complex assertions
}

// Start with:
#[test]
fn test_zero_state() {
    let m = MyModel::new();
    m.x = 0.0;
    let der = m.get_derivative();
    assert_eq!(der, expected_at_zero);  // Should be simple!
}
```

### Strategy 4: Check Units and Signs

**Common bug:** Wrong sign in equation.

```rust
// dx/dt = -k*x  (decay, should be negative)
let der_x = -self.k * self.x;  // ✅

// Not:
let der_x = self.k * self.x;   // ❌ (exponential growth!)
```

**Common bug:** Unit mismatch.

```rust
// dv/dt = g  (where g = -9.81 m/s²)
self.v += self.g * dt;  // ✅ Units: m/s + (m/s²)(s) = m/s

// Not:
self.v += self.g;  // ❌ Wrong units!
```

## 📊 Test Coverage Guidelines

Aim for these test categories:

### Unit Tests (in `src/lib.rs`)
- [ ] Default initialization
- [ ] State get/set operations
- [ ] Derivative calculations at known points
- [ ] Parameter effects on derivatives
- [ ] Edge cases (zero, negative, large values)

### Physics Tests (in `tests/physics_tests.rs`)
- [ ] Analytical solution comparison (if available)
- [ ] Conservation laws (energy, momentum, mass)
- [ ] Known properties (limit cycles, steady states, oscillation frequency)
- [ ] Convergence with step size
- [ ] Boundary conditions
- [ ] Event handling (for hybrid systems)

### Integration Tests (in `testing/fmu-integration-tests/test_*_fmu.py`)
- [ ] FMU loads successfully
- [ ] Parameter setting
- [ ] Simulation runs without errors
- [ ] Results match Rust implementation
- [ ] Physics validation with FMPy
- [ ] Edge cases with different parameters

## 🏗️ Code Organization

### Recommended File Structure

```rust
// src/lib.rs structure

//! Model Name
//!
//! Detailed description including equations

// 1. Clippy allows
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

// 2. Imports
pub use fmu_from_struct::prelude::*;

// 3. Struct definition with FMU derives
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct MyModel {
    // Parameters first
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub param: f64,

    // Then outputs/states
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    pub state: f64,

    // Internal variables (not exposed to FMI)
    time: f64,
}

// 4. Implementation
impl FmuFunctions for MyModel {
    fn do_step(&mut self, current_time: f64, step_size: f64) {
        // Implementation
    }
}

// 5. Helper methods
impl MyModel {
    pub fn analytical_solution(/* args */) -> f64 {
        // For testing
    }
}

// 6. Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() { /* */ }

    #[test]
    fn test_derivatives() { /* */ }
}
```

## 🎓 Learning from Existing Models

Study these models in order of complexity:

1. **Dahlquist** (`models/mathematical/dahlquist/`)
   - Simplest: One state, one parameter
   - Has analytical solution
   - Good starting point

2. **Van der Pol** (`models/mathematical/van-der-pol/`)
   - Two states, nonlinear
   - Limit cycle behavior
   - Shows how to test dynamic properties

3. **Bouncing Ball** (`models/mechanical/bouncing-ball/`)
   - Event handling (collisions)
   - Hybrid dynamics
   - Energy dissipation

## 🚀 Performance Tips

### For AI Agents During Implementation

1. **Read existing models first** - Don't reinvent patterns
2. **Use TODO tool** - Track complex implementations
3. **Test incrementally** - Don't wait until everything is done
4. **Copy-paste test templates** - From existing models, then adapt
5. **Use small step sizes in tests** - Euler needs small steps (0.01)
6. **Run local tests before CI** - Faster iteration

### For Runtime Performance

1. **Simple integration is OK** - Euler is fine for demos
2. **Don't over-optimize** - Focus on correctness first
3. **Profile if needed** - But usually not necessary for these models

## 📝 Documentation Guidelines

Each model needs:

### README.md
```markdown
# Model Name

Brief description

## Mathematical Model

$$\frac{dx}{dt} = f(x, t, parameters)$$

Describe equations, assumptions, limitations

## Parameters

| Name | Type | Default | Units | Description |
|------|------|---------|-------|-------------|
| k | Real | 1.0 | 1/s | Decay rate |

## State Variables

| Name | Type | Initial | Units | Description |
|------|------|---------|-------|-------------|
| x | Real | 1.0 | m | Position |

## Physics Validation

Explain what physics properties are tested and why

## References

Cite sources
```

### Inline Documentation
```rust
/// Decay rate constant
///
/// Must be positive for stable decay.
/// Units: 1/s
#[fmu_from_struct(parameter)]
pub k: f64,
```

## ✅ Checklist Before Submitting PR

- [ ] All tests pass (`./scripts/test-all.sh`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy --all-targets -- -D warnings`)
- [ ] FMU builds successfully
- [ ] Python integration tests pass
- [ ] README.md complete with equations
- [ ] Physics validation explained
- [ ] AI agent identifier included in PR description

## 🤝 Collaboration Between AI Agents

**Future possibility:** Multiple AI agents working on same model.

**Recommended approach:**
1. One agent implements Rust code
2. Another agent implements Python tests
3. Third agent writes documentation
4. Final agent reviews and integrates

**Coordination:**
- Use TODO tool to track who's doing what
- Clear ownership of files
- Regular integration tests

## 📚 Resources for AI Agents

### Essential Reading
- FMI 3.0 Specification (focus on Co-Simulation)
- `fmu_from_struct` documentation
- Existing models in this repository
- Python FMPy documentation

### Useful Commands
```bash
# Build and test model
cargo build -p odml-model-name
cargo test -p odml-model-name

# Build FMU
./scripts/build-fmu.sh models/category/model-name

# Test FMU
cd testing/fmu-integration-tests
pytest test_model_name_fmu.py -v

# Check everything
./scripts/test-all.sh
```

### Common Error Messages and Solutions

```
Error: The start values for the following variables could not be set: x
→ x is an output, not a parameter. Only set parameters in start_values.

Error: error[E0412]: cannot find type `Fmu` in this scope
→ Add: pub use fmu_from_struct::prelude::*;

Error: clippy::not_unsafe_ptr_arg_deref
→ Add #![allow(clippy::not_unsafe_ptr_arg_deref)] at top of lib.rs

Error: Test fails with large error (20%+)
→ Use smaller step size (0.01 instead of 0.1)

Error: FMU file not found after build
→ Check that directory name matches: model-name → model_name
```

## 🎯 Success Metrics

A successful AI-generated model:
1. ✅ Compiles without warnings
2. ✅ Passes all three test tiers
3. ✅ Physics validation with known solutions/properties
4. ✅ Clear documentation
5. ✅ FMU builds and simulates correctly
6. ✅ Demonstrates autonomous problem-solving by AI

**You're contributing to the future of AI-assisted scientific computing!** 🚀

---

*This guide was written by Claude Sonnet 4.5 based on actual implementation experience with the Dahlquist, Van der Pol, and Bouncing Ball models.*
