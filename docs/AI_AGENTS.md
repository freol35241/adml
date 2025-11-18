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
# build-fmu.sh creates directory "adml-dahlquist" (hyphen)
# But package_fmu_after_build expects "adml_dahlquist" (underscore)
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
cargo build -p adml-model-name
cargo test -p adml-model-name

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

## 🔥 Critical Lessons from Recent Implementations

This section contains crucial insights from implementing the thermal RC model that aren't obvious from the templates.

### Lesson 1: FMU Naming Convention is STRICT

**Critical Discovery:** The FMU filename comes from the **struct name**, and CI expects a specific naming convention.

**Directory name:**
```
models/thermal/rc-thermal-single-zone/
```

**MUST have struct name:**
```rust
pub struct RcThermalSingleZone {  // ✅ CORRECT
    // Each hyphen-separated word capitalized: Rc, Thermal, Single, Zone
}

pub struct RCThermalSingleZone {  // ❌ WRONG - CI expects RcThermalSingleZone.fmu
pub struct rc_thermal_single_zone {  // ❌ WRONG - Not CamelCase
```

**How the conversion works:**
- Directory: `rc-thermal-single-zone`
- Split on hyphens: `["rc", "thermal", "single", "zone"]`
- Capitalize each word: `["Rc", "Thermal", "Single", "Zone"]`
- Join: `RcThermalSingleZone`

The `model_name` field in `Cargo.toml` metadata is for documentation - the actual FMU filename comes from the struct name via the `fmu_from_struct` macro.

**Finding this bug took multiple iterations.** Always verify your struct name matches this pattern!

### Lesson 2: Parameters vs Outputs in plot_config.toml

**Critical Discovery:** Only `#[fmu_from_struct(output)]` variables appear in simulation results. Parameters do NOT.

**Problem Code:**
```toml
# plot_config.toml
[parameters]
Q_heat = 5000.0  # This is a parameter

[[subplot]]
variables = ["Q_heat", "Q_loss"]  # ❌ FAILS: Q_heat not in results!
```

**Error:**
```
ValueError: Variable 'Q_heat' not found in simulation results
```

**Solution:**
```toml
[[subplot]]
variables = ["Q_loss"]  # ✅ Only plot outputs
reference_line = 5000.0  # ✅ Show parameter as reference line
reference_label = "Heat Input (5000 W)"
```

**Why this happens:** FMI distinguishes between:
- **Parameters** (`causality="parameter"`): Set before simulation, not recorded in results
- **Outputs** (`causality="output"`): Recorded during simulation

Only outputs appear in the results array, so only outputs can be plotted as variables.

### Lesson 3: Time Units in FMI are ALWAYS Seconds

**Critical Discovery:** FMI standard uses seconds. The plotting script does NOT convert units.

**Problem Code:**
```toml
# plot_config.toml
[simulation]
stop_time = 200000.0  # This is 200,000 SECONDS (~55 hours)

[[subplot]]
xlabel = "Time [hours]"  # ❌ WRONG - actual data is in seconds!
```

**The plot will show:**
- X-axis labeled "Time [hours]"
- X-axis values ranging from 0 to 200,000 (which are actually seconds!)
- User confusion

**Solution:**
```toml
[[subplot]]
xlabel = "Time [s]"  # ✅ CORRECT - matches actual data units
```

If you want to display hours, you must:
1. Either convert in your plotting script (not currently implemented), OR
2. Label correctly and let users understand the units

**Always use FMI standard units:**
- Time: seconds (s)
- Length: meters (m)
- Mass: kilograms (kg)
- Temperature: Kelvin (K) or Celsius (°C) if clearly documented

### Lesson 4: Physics Notation Requires non_snake_case Allow

**Problem:**
```rust
pub struct RcThermalSingleZone {
    pub R_th: f64,  // Physics notation uses underscores
    pub C_th: f64,
}
```

**Error:**
```
warning: structure field `R_th` should have snake_case name
warning: structure field `C_th` should have snake_case name
```

**Solution:**
Add at the top of `src/lib.rs`:
```rust
#![allow(non_snake_case)]  // For physics notation like R_th, C_th
```

**Why this matters:** Physics conventions often use subscripts (rendered as underscores in code). Thermal resistance is R_th (R thermal), thermal capacitance is C_th (C thermal). Using r_th and c_th loses physical meaning.

### Lesson 5: CI Plot Filtering Requires Full Git History

**Critical Discovery:** GitHub Actions checkout is shallow by default, breaking git diff.

**Problem:**
```yaml
steps:
  - uses: actions/checkout@v4  # Shallow clone (only latest commit)

  - name: Generate plots
    run: |
      git diff $BASE_SHA...HEAD  # ❌ FAILS: BASE_SHA not in history
```

**Error:**
```
fatal: Invalid symmetric difference expression 8db11989...HEAD
Error: Process completed with exit code 128
```

**Solution:**
```yaml
steps:
  - uses: actions/checkout@v4
    with:
      fetch-depth: 0  # ✅ Fetch full history for git diff
```

**Why this matters:** PR-specific plot filtering uses `git diff` to find changed models. Without full history, the base commit from the PR isn't available, causing the diff to fail.

### Lesson 6: Silent Failures in CI Must Be Eliminated

**Problem discovered:** CI was passing even when plots failed to generate.

**Bad pattern:**
```bash
python generate_plots.py || echo "[]"  # Suppresses all errors!
```

**What happened:**
- Plot generation failed with clear error
- Error was suppressed by `|| echo "[]"`
- CI showed green checkmark
- No plots appeared, no error visible

**Solution principles:**
1. **ALWAYS print errors to stderr:**
```python
except Exception as e:
    print(f"✗ Error: {e}", file=sys.stderr)  # Never suppress
    traceback.print_exc(file=sys.stderr)
    return False
```

2. **Track failures explicitly:**
```python
failed_models = []
for model in models:
    if not plot_model(model):
        failed_models.append((model, reason))

if failed_models:
    print(f"\n⚠️ {len(failed_models)} models failed:", file=sys.stderr)
    return 1  # Non-zero exit code
```

3. **Use continue-on-error with warnings:**
```yaml
- name: Generate plots
  continue-on-error: true  # Don't fail workflow
  run: |
    if ! python generate_plots.py; then
      echo "::warning::Plot generation failed. Check stderr above."
    fi
```

**Result:** Errors are visible in CI logs even if workflow continues.

### Lesson 7: plot_config.toml Must Be in Model Directory

**File location:**
```
models/thermal/rc-thermal-single-zone/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
│   └── physics_tests.rs
├── plot_config.toml  ← Must be here!
└── README.md
```

**NOT here:**
```
models/thermal/plot_config.toml  ❌
fmus/plot_config.toml  ❌
.github/plot_config.toml  ❌
```

The plotting script looks for `{model_dir}/plot_config.toml` where `model_dir` is the full path like `models/thermal/rc-thermal-single-zone`.

### Lesson 8: Test Tolerance Must Account for Euler Integration

**Problem:**
```rust
#[test]
fn test_analytical_solution() {
    // Simulate with Euler integration
    let result = simulate(dt=0.01, t=100000.0);

    assert_relative_eq!(result, analytical(100000.0), epsilon = 1e-6);  // ❌ FAILS
}
```

**Why it fails:** Euler integration is first-order accurate. Errors accumulate over time:
- Local error: O(dt²)
- Global error: O(dt)
- Over many steps, errors grow

**Solution:**
```rust
assert_relative_eq!(result, analytical(100000.0), epsilon = 0.01);  // ✅ 1% tolerance
```

**Guidelines:**
- Short simulations (t < 10): epsilon = 0.001 (0.1%)
- Medium simulations (t < 1000): epsilon = 0.01 (1%)
- Long simulations (t > 1000): epsilon = 0.05-0.25 (5-25%)
- Very long or stiff systems: Use relative error checking at multiple points

**Better approach for long simulations:**
```rust
// Check at multiple intermediate points instead of just the end
for t in [100.0, 1000.0, 10000.0, 100000.0] {
    let result = simulate(dt=0.01, t);
    let expected = analytical(t);
    assert_relative_eq!(result, expected, epsilon = 0.05);
}
```

### Lesson 9: Derived Outputs Should Be Calculated After Integration

**Problem:**
```rust
fn do_step(&mut self, _current_time: f64, step_size: f64) {
    // Calculate derived outputs BEFORE integration
    self.Q_loss = (self.T_indoor - self.T_ambient) / self.R_th;  // ❌ Uses OLD state

    // Then integrate
    let der_T = (self.Q_heat - self.Q_loss) / self.C_th;
    self.T_indoor += der_T * step_size;  // Updates state
}
```

**Issue:** `Q_loss` uses the old `T_indoor`, not the updated value.

**Solution:**
```rust
fn do_step(&mut self, _current_time: f64, step_size: f64) {
    // Calculate derivative using current state
    let q_loss = (self.T_indoor - self.T_ambient) / self.R_th;
    let der_T = (self.Q_heat - q_loss) / self.C_th;

    // Integrate state
    self.T_indoor += der_T * step_size;

    // Calculate derived outputs AFTER integration using NEW state
    self.Q_loss = (self.T_indoor - self.T_ambient) / self.R_th;  // ✅ Uses NEW state
    self.dT_dt = der_T;
}
```

**Why this matters:** Outputs should reflect the state at the END of the time step, not the beginning.

## 🎯 Success Metrics

A successful AI-generated model:
1. ✅ Compiles without warnings
2. ✅ Passes all three test tiers
3. ✅ Physics validation with known solutions/properties
4. ✅ Clear documentation
5. ✅ FMU builds and simulates correctly
6. ✅ Demonstrates autonomous problem-solving by AI
7. ✅ Generates correct plots in CI/CD
8. ✅ Uses proper naming conventions
9. ✅ No silent failures in CI

**You're contributing to the future of AI-assisted scientific computing!** 🚀

---

*This guide was written by Claude Sonnet 4.5 based on actual implementation experience with the Dahlquist, Van der Pol, Bouncing Ball, and RC Thermal Single-Zone models.*
