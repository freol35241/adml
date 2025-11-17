# ODML - Open Dynamical Model Library

[![CI](https://github.com/freol35241/odml/actions/workflows/ci.yml/badge.svg)](https://github.com/freol35241/odml/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

An open-source library of dynamical models implemented in Rust, adhering to the FMI 3.0 (Functional Mock-up Interface) standard.

## 🎯 Purpose

ODML provides a collection of high-quality, well-tested dynamical models that can be used for:
- Control system design and testing
- Simulation and co-simulation
- Model validation and benchmarking
- Educational purposes
- Research and development

All models are:
- ✅ **FMI 3.0 compliant** - Compatible with any FMI-supporting tool
- ✅ **Written in Rust** - Safe, fast, and reliable
- ✅ **Thoroughly tested** - Unit tests, API tests, and physics validation
- ✅ **Well documented** - Each model includes detailed documentation
- ✅ **Cross-platform** - Built for Linux, Windows, and ARM architectures

**Note:** Models currently use the [`fmu_from_struct`](https://github.com/jarlekramer/fmu_from_struct) crate for FMI export. See [FMI_EXPORT_STATUS.md](FMI_EXPORT_STATUS.md) for details and future migration plans to the official `fmi-export` crate.

## 📦 Available Models

### Mathematical Models

| Model | Description | Version |
|-------|-------------|---------|
| [Dahlquist](models/mathematical/dahlquist/) | Simple ODE test equation: dx/dt = -k·x | 1.0.0 |
| [Van der Pol](models/mathematical/van-der-pol/) | Nonlinear oscillator with limit cycle | 1.0.0 |

### Mechanical Models

| Model | Description | Version |
|-------|-------------|---------|
| [Bouncing Ball](models/mechanical/bouncing-ball/) | Ball with gravity and elastic collisions | 1.0.0 |

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building All Models

```bash
# Clone the repository
git clone https://github.com/freol35241/odml.git
cd odml

# Build all models
cargo build --workspace --release

# Or use the helper script
./scripts/build-all.sh
```

### Running Tests

```bash
# Test all models
cargo test --workspace

# Or use the helper script
./scripts/test-all.sh
```

### Working with a Single Model

```bash
# Build a specific model
cd models/mathematical/dahlquist
cargo build --release

# Run tests for a specific model
cargo test

# Check code quality (format, clippy, tests, build)
../../scripts/check-model.sh models/mathematical/dahlquist
```

## 🏗️ Repository Structure

```
odml/
├── models/                    # All dynamical models
│   ├── mathematical/          # Mathematical test cases
│   │   ├── dahlquist/        # Dahlquist test equation
│   │   └── van-der-pol/      # Van der Pol oscillator
│   └── mechanical/            # Mechanical systems
│       └── bouncing-ball/     # Bouncing ball with collisions
│
├── testing/                   # Shared testing infrastructure
│   ├── fmi-compliance/       # FMI API compliance tests
│   └── physics-framework/    # Physics validation utilities
│
├── .github/workflows/         # CI/CD pipelines
│   ├── ci.yml                # Continuous integration
│   └── release.yml           # Release builds
│
├── scripts/                   # Helper scripts
│   ├── build-all.sh          # Build all models
│   ├── test-all.sh           # Test all models
│   └── check-model.sh        # Check a single model
│
└── docs/                      # Documentation
```

## 🧪 Testing Philosophy

Each model includes three layers of testing:

1. **Unit Tests** - Test individual functions and logic
2. **FMI API Tests** - Verify FMI 3.0 compliance
3. **Physics Tests** - Validate physical correctness:
   - Energy conservation (where applicable)
   - Analytical solution comparison
   - Stability and convergence
   - Boundary conditions
   - Event handling

Example test output:
```bash
$ cargo test -p odml-dahlquist

running 8 tests
test tests::test_initial_values ... ok
test tests::test_derivatives ... ok
test physics_tests::test_exponential_decay ... ok
test physics_tests::test_analytical_solution ... ok
test physics_tests::test_half_life ... ok
test physics_tests::test_convergence ... ok
```

## 🔄 CI/CD Pipeline

### Continuous Integration (CI)

On every push and pull request:
- ✅ Code formatting check (`cargo fmt`)
- ✅ Linting with Clippy (`cargo clippy`)
- ✅ Build all models
- ✅ Run all tests (unit + integration + physics)
- ✅ Generate documentation

Smart detection:
- PRs: Only test changed models
- Main branch: Test all models

### Release Workflow

Triggered manually or by tag:
- 🔨 Cross-compile for multiple platforms:
  - Linux x86_64
  - Windows x86_64
  - Linux ARM64
- 📦 Package binaries
- 🚀 Create GitHub release with all artifacts

Each model has independent versioning!

## 🛠️ Adding a New Model

1. **Create the model directory:**
   ```bash
   mkdir -p models/category/model-name
   cd models/category/model-name
   ```

2. **Set up Cargo.toml:**
   ```toml
   [package]
   name = "odml-model-name"
   version = "1.0.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib", "rlib"]

   [dependencies]
   fmu_from_struct = { workspace = true }

   [dev-dependencies]
   fmi-compliance = { path = "../../../testing/fmi-compliance" }
   physics-framework = { path = "../../../testing/physics-framework" }
   approx = { workspace = true }

   [package.metadata.fmi]
   model_name = "ModelName"
   fmi_version = "3.0"
   guid = "your-unique-guid-here"
   ```

3. **Implement the model** in `src/lib.rs`

4. **Add tests:**
   - `src/lib.rs` - Unit tests
   - `tests/physics_tests.rs` - Physics validation

5. **Add documentation:**
   - `README.md` - Model description, equations, parameters

6. **Verify:**
   ```bash
   ../../scripts/check-model.sh models/category/model-name
   ```

7. **The CI will automatically:**
   - Test your model on PRs
   - Build cross-platform binaries on release
   - Generate documentation

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for detailed guidelines.

## 📝 Model Documentation

Each model includes:
- **README.md** - Overview, equations, parameters, usage
- **Inline documentation** - Rust doc comments
- **Physics validation** - Explanation of test methodology

Generated API documentation is available at: https://freol35241.github.io/odml/

## 🔧 Cross-Compilation

Using `cross` for reproducible builds:

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Build for Windows
cross build --target x86_64-pc-windows-gnu --release

# Build for ARM Linux
cross build --target aarch64-unknown-linux-gnu --release
```

Configuration is in `Cross.toml`.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for:
- Code style guidelines
- Testing requirements
- Model development process
- Pull request process

## 📄 License

This project is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
- Apache License 2.0 ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## 🌟 Acknowledgments

Models are inspired by and validated against the [Modelica Reference-FMUs](https://github.com/modelica/Reference-FMUs).

## 📞 Contact

- Issues: [GitHub Issues](https://github.com/freol35241/odml/issues)
- Discussions: [GitHub Discussions](https://github.com/freol35241/odml/discussions)

## 🗺️ Roadmap

- [ ] Full FMU packaging tool
- [ ] More mechanical models (pendulum, spring-damper)
- [ ] Electrical models (RC circuits, motors)
- [ ] Thermal models (heat exchangers)
- [ ] Hydraulic models
- [ ] Model Exchange and Co-Simulation modes
- [ ] FMI 3.0 advanced features (clocks, structured naming)
- [ ] Python bindings for easy testing
- [ ] Performance benchmarks

---

**Note:** This is an early-stage project. Models are provided as cdylib binaries. Full FMU packaging (with modelDescription.xml and proper directory structure) is planned for future releases.
