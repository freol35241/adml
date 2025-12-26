# Error Catalog for AI Agents

This document provides a searchable reference of common errors, their causes, and solutions. Organized by error type for quick lookup.

---

## Build Errors

### E001: Cannot find type `Fmu` in this scope

**Error Message:**
```
error[E0412]: cannot find type `Fmu` in this scope
  --> src/lib.rs:10:10
   |
10 | #[derive(Fmu, Default, Debug, Clone)]
   |          ^^^ not found in this scope
```

**Cause:** Missing prelude import from `fmu_from_struct`.

**Solution:**
```rust
pub use fmu_from_struct::prelude::*;
```

---

### E002: clippy::not_unsafe_ptr_arg_deref

**Error Message:**
```
error: this public function might dereference a raw pointer but is not marked `unsafe`
  --> src/lib.rs:17:10
   |
17 | #[derive(Fmu, Default, Debug, Clone)]
   |          ^^^
```

**Cause:** Clippy warning from generated FFI code in the `fmu_from_struct` macro.

**Solution:**
Add at the top of `src/lib.rs`:
```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
```

---

### E003: structure field should have snake_case name

**Error Message:**
```
warning: structure field `R_th` should have snake_case name
  --> src/lib.rs:25:9
   |
25 |     pub R_th: f64,
   |         ^^^^ help: convert the identifier to snake case: `r_th`
```

**Cause:** Physics notation uses uppercase letters (e.g., `R_th` for thermal resistance).

**Solution:**
Add at the top of `src/lib.rs`:
```rust
#![allow(non_snake_case)]  // For physics notation like R_th, C_th
```

---

### E004: Cannot find package in workspace

**Error Message:**
```
error: package `adml-model-name` is not a member of the workspace
```

**Cause:** Model not added to root `Cargo.toml` workspace members.

**Solution:**
Edit `/Cargo.toml`:
```toml
[workspace]
members = [
    # ... existing members
    "models/category/model-name",  # Add your model
]
```

---

## FMU Build Errors

### E101: FMU filename mismatch

**Symptom:** CI expects `RcThermal.fmu` but finds `RCThermal.fmu` (or similar).

**Cause:** Struct name doesn't match expected CamelCase conversion from directory name.

**Rule:** Directory name converts to struct name as follows:
- Split on hyphens: `rc-thermal-single-zone` → `["rc", "thermal", "single", "zone"]`
- Capitalize each word: `["Rc", "Thermal", "Single", "Zone"]`
- Join: `RcThermalSingleZone`

**Common Mistakes:**
| Directory | Wrong | Correct |
|-----------|-------|---------|
| `rc-thermal` | `RCThermal` | `RcThermal` |
| `van-der-pol` | `VanDerPOL` | `VanDerPol` |
| `my-model` | `MyMODEL` | `MyModel` |

**Solution:** Rename struct to match the pattern exactly.

---

### E102: modelDescription.xml not updated

**Symptom:** FMU has old variable names or parameters after code changes.

**Cause:** Cached build artifacts include old `modelDescription.xml`.

**Solution:**
```bash
cd models/category/model-name
rm -f modelDescription.xml
cargo clean -p adml-model-name --release
./scripts/build-fmu.sh models/category/model-name
```

---

### E103: package_fmu_after_build not found

**Error Message:**
```
error: command not found: package_fmu_after_build
```

**Cause:** FMU packaging tool not installed.

**Solution:**
```bash
cargo install package_fmu_after_build
```

---

## FMPy / FMU Runtime Errors

### E201: Variable not found in simulation results

**Error Message:**
```
ValueError: Variable 'Q_heat' not found in simulation results
```

**Cause:** Attempting to access a parameter (not an output) in results.

**Explanation:** Only `#[fmu_from_struct(output)]` variables appear in simulation results. Parameters are set before simulation but not recorded.

**Solution:**
- To plot parameter values, use reference lines instead
- To record parameter values, create a derived output that copies it:
  ```rust
  // In do_step(), after using Q_heat:
  self.Q_heat_output = self.Q_heat;
  ```

---

### E202: Start values could not be set

**Error Message:**
```
FMPy: The start values for the following variables could not be set: x
```

**Cause:** Trying to set an output variable as if it were a parameter.

**Solution:**
Only set variables marked as `#[fmu_from_struct(parameter)]` in `start_values`:

```python
# Wrong - x is an output
simulate_fmu(fmu_path, start_values={'x': 2.0})

# Correct - k is a parameter
simulate_fmu(fmu_path, start_values={'k': 2.0})
```

---

### E203: FMU simulation gives wrong results

**Symptom:** FMPy results differ significantly from Rust tests.

**Common Causes:**

1. **Step size too large for Euler integration**
   ```python
   # Wrong - default step size may be too large
   result = simulate_fmu(fmu_path, stop_time=5.0)

   # Correct - specify small step size
   result = simulate_fmu(fmu_path, stop_time=5.0, step_size=0.01)
   ```

2. **Output interval != step size**
   ```python
   # May skip internal steps
   result = simulate_fmu(fmu_path, step_size=0.01, output_interval=0.1)

   # Better - always match them
   result = simulate_fmu(fmu_path, step_size=0.01, output_interval=0.01)
   ```

---

## Test Errors

### E301: Assertion failed with large error

**Error Message:**
```
assertion failed: `(left == right)`
  left: `0.3679`,
 right: `0.4`,
epsilon: `0.001`
```

**Cause:** Euler integration error accumulation over time.

**Solution:**
1. Use smaller step size:
   ```rust
   let dt = 0.01;  // Instead of 0.1
   ```
2. Use appropriate tolerance for Euler:
   ```rust
   // Short simulations: 0.1-1%
   assert_relative_eq!(result, expected, epsilon = 0.01);

   // Long simulations: 5-25%
   assert_relative_eq!(result, expected, epsilon = 0.25);
   ```

---

### E302: Test fails only in CI

**Symptom:** Tests pass locally but fail in GitHub Actions.

**Common Causes:**

1. **Floating-point non-determinism**
   - Use `approx::assert_relative_eq!` instead of `==`
   - Add reasonable tolerance

2. **Missing git history for diff operations**
   ```yaml
   # In CI workflow
   - uses: actions/checkout@v4
     with:
       fetch-depth: 0  # Full history, not shallow clone
   ```

3. **Path issues**
   - Use relative paths from workspace root
   - Don't hardcode absolute paths

---

### E303: Physics test gives unexpected results

**Symptom:** Conservation law test fails, energy grows, etc.

**Debugging Steps:**

1. **Check equation signs:**
   ```rust
   // Decay should be negative
   let der_x = -self.k * self.x;  // ✓
   let der_x = self.k * self.x;   // ✗ (growth instead of decay)
   ```

2. **Check units:**
   ```rust
   // Velocity update: m/s = m/s + (m/s²)(s)
   self.v += self.g * dt;  // ✓
   self.v += self.g;       // ✗ (missing dt)
   ```

3. **Verify at known points:**
   ```rust
   // Test derivative at t=0, x=1
   let expected_der = -1.0;  // For k=1, x=1: -k*x = -1
   assert_eq!(model.get_derivative(), expected_der);
   ```

---

## CI/CD Errors

### E401: Plot generation fails silently

**Symptom:** CI passes but no plots appear.

**Cause:** Error suppressed by `|| echo "[]"` pattern.

**Solution:** Check CI logs for stderr output. Common issues:
- Missing `plot_config.toml`
- Parameter plotted instead of output
- FMU not built

---

### E402: git diff fails in PR workflow

**Error Message:**
```
fatal: Invalid symmetric difference expression abc123...HEAD
Error: Process completed with exit code 128
```

**Cause:** Shallow clone missing base commit.

**Solution:**
```yaml
- uses: actions/checkout@v4
  with:
    fetch-depth: 0  # Fetch full history
```

---

### E403: Workflow timeout

**Symptom:** CI job exceeds time limit.

**Common Causes:**
1. Long simulation in tests (reduce `stop_time`)
2. Building without cache
3. Infinite loop in code

**Solution:**
- Add caching for Cargo registry
- Use reasonable test durations
- Add timeout to simulation steps

---

## Documentation Errors

### E501: plot_config.toml not found

**Symptom:** Plot generation skips model.

**Cause:** `plot_config.toml` in wrong location.

**Correct Location:**
```
models/category/model-name/
├── Cargo.toml
├── src/lib.rs
├── plot_config.toml  ← Here, in model directory
└── README.md
```

**NOT:**
```
models/category/plot_config.toml  ✗
fmus/plot_config.toml  ✗
.github/plot_config.toml  ✗
```

---

### E502: Time axis labeled incorrectly

**Symptom:** Plot shows "Time [hours]" but values are 0-200000.

**Cause:** FMI always uses seconds; label doesn't match data.

**Solution:**
```toml
# In plot_config.toml
[[subplot]]
xlabel = "Time [s]"  # Match actual data units
```

---

## Quick Diagnosis Checklist

When encountering an error:

1. [ ] Check if it's a known error in this catalog
2. [ ] Verify struct name matches directory pattern
3. [ ] Confirm all imports are present
4. [ ] Check Cargo.toml workspace membership
5. [ ] Verify parameter vs output causality
6. [ ] Test with smaller step size
7. [ ] Run `cargo fmt` and `cargo clippy`
8. [ ] Clean build: `cargo clean && cargo build`

---

## Contributing to This Catalog

If you encounter an error not listed here:

1. Document the error message
2. Identify the root cause
3. Provide a clear solution
4. Add to appropriate section
5. Submit PR with error addition

*This catalog grows with each AI agent's experience. Help future agents by documenting errors you solve!*
