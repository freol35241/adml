# ODML - AI-Generated Dynamical Model Library

[![CI](https://github.com/freol35241/odml/actions/workflows/ci.yml/badge.svg)](https://github.com/freol35241/odml/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![AI Generated](https://img.shields.io/badge/100%25-AI%20Generated-purple.svg)](docs/AI_AGENTS.md)

**The world's first library of dynamical models implemented entirely by AI agents.**

ODML is a collection of high-quality, FMI 3.0 compliant dynamical models implemented in Rust, where **every model is written, tested, and documented by AI coding agents**. This project serves as both a practical library of simulation models and a demonstration of AI capabilities in scientific computing.

## 🤖 AI-Generated Code

**Every line of code in this repository is AI-generated.** No human has written the model implementations, tests, or FMI bindings. AI agents:
- ✅ Implement differential equations from specifications
- ✅ Write comprehensive physics validation tests
- ✅ Generate FMI 3.0 compliant bindings using `fmu_from_struct`
- ✅ Create documentation and examples
- ✅ Debug and fix issues autonomously

See [docs/AI_AGENTS.md](docs/AI_AGENTS.md) for guidance on implementing models with AI agents.

## 🎯 Purpose

ODML provides:
- **Simulation Models** - Ready-to-use FMI 3.0 FMUs for control systems, research, and education
- **AI Capability Demonstration** - Proof that AI agents can implement scientifically correct, production-quality code
- **Benchmark for AI Agents** - A framework for evaluating AI coding agents on scientific computing tasks
- **Learning Resource** - Examples of how AI agents approach dynamical systems implementation

All models are:
- ✅ **FMI 3.0 compliant** - Compatible with any FMI-supporting tool (FMPy, Dymola, Simulink, etc.)
- ✅ **Written in Rust** - Safe, fast, and reliable
- ✅ **Thoroughly tested** - 3-tier testing: Rust unit tests, Python FMU integration tests, FMI compliance
- ✅ **Well documented** - Each model includes detailed documentation with equations and physics validation
- ✅ **100% AI-generated** - Demonstrating state-of-the-art AI coding capabilities

**Implementation Note:** Models use the [`fmu_from_struct`](https://github.com/jarlekramer/fmu_from_struct) derive macro for FMI export, which automatically generates FMI C bindings from Rust structs. This eliminates manual FFI code and makes AI implementation more reliable. See [FMI_EXPORT_STATUS.md](FMI_EXPORT_STATUS.md) for details.

## 📦 Available Models

### Mathematical Models

| Model | Description | Version | AI Agent |
|-------|-------------|---------|----------|
| [Dahlquist](models/mathematical/dahlquist/) | Simple ODE test equation: dx/dt = -k·x | 1.0.0 | Claude Sonnet 4.5 |
| [Van der Pol](models/mathematical/van-der-pol/) | Nonlinear oscillator with limit cycle | 1.0.0 | Claude Sonnet 4.5 |

### Mechanical Models

| Model | Description | Version | AI Agent |
|-------|-------------|---------|----------|
| [Bouncing Ball](models/mechanical/bouncing-ball/) | Ball with gravity and elastic collisions | 1.0.0 | Claude Sonnet 4.5 |

## 🚀 Quick Start

### Using Pre-built FMU Files

Download FMU files from the [Releases page](https://github.com/freol35241/odml/releases) and use them directly in any FMI 3.0 compatible tool:

```python
# Example with FMPy
import fmpy
result = fmpy.simulate_fmu('Dahlquist.fmu', stop_time=5.0)
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/freol35241/odml.git
cd odml

# Install FMU packaging tool
cargo install package_fmu_after_build

# Build all FMUs
./scripts/build-all-fmus.sh

# FMU files are created in: fmus/
# Example: fmus/Dahlquist.fmu, fmus/VanDerPol.fmu, fmus/BouncingBall.fmu
```

### Running Tests

```bash
# Run all tests (Rust + Python FMU integration)
./scripts/test-all.sh

# Or test components separately:
cargo test --workspace                    # Rust unit & physics tests
pytest testing/fmu-integration-tests/ -v  # FMU integration tests
```

## 🏗️ Repository Structure

```
odml/
├── models/                        # All AI-generated dynamical models
│   ├── mathematical/              # Mathematical test cases
│   │   ├── dahlquist/            # Dahlquist test equation
│   │   └── van-der-pol/          # Van der Pol oscillator
│   └── mechanical/                # Mechanical systems
│       └── bouncing-ball/         # Bouncing ball with collisions
│
├── testing/                       # Testing infrastructure
│   ├── physics-framework/        # Physics validation utilities
│   ├── fmu-integration-tests/    # Python FMU integration tests
│   └── requirements.txt          # Python dependencies
│
├── scripts/                       # Build and test automation
│   ├── build-all-fmus.sh         # Build all FMU files
│   ├── build-fmu.sh              # Build a single FMU
│   ├── test-all.sh               # Run all tests (Rust + Python)
│   └── check-fmu-compliance.sh   # FMI compliance validation
│
├── docs/                          # Documentation
│   ├── AI_AGENTS.md              # Guide for AI agents implementing models
│   ├── AI_SCAFFOLDING.md         # Model scaffolding templates
│   └── CONTRIBUTING.md           # Contributing guidelines
│
└── .github/workflows/             # CI/CD pipelines
    ├── ci.yml                    # Continuous integration
    └── release.yml               # Release builds
```

## 🧪 3-Tier Testing Philosophy

AI-generated models are validated through three testing tiers:

### Tier 1: Rust Unit & Physics Tests
- **Unit Tests** - Test model initialization, state operations, derivatives
- **Physics Tests** - Validate physical correctness:
  - Analytical solution comparison (where available)
  - Energy conservation laws
  - Stability and convergence
  - Boundary conditions
  - Event handling

### Tier 2: FMU Integration Tests (Python + FMPy)
Once FMUs are built, Python integration tests validate:
- ✅ FMU structure and FMI 3.0 compliance
- ✅ Simulation accuracy with external FMI simulator (FMPy)
- ✅ Physics validation against analytical solutions
- ✅ Parameter sensitivity and edge cases

See [testing/fmu-integration-tests/README.md](testing/fmu-integration-tests/README.md)

### Tier 3: FMI Compliance Checking
Validate FMI standard compliance using the official FMU Checker:
```bash
./scripts/check-fmu-compliance.sh fmus/Dahlquist.fmu
```

## 🤖 Implementing Models with AI Agents

**Want to add a model using an AI agent?** See our comprehensive guides:

- **[docs/AI_AGENTS.md](docs/AI_AGENTS.md)** - Complete guide for AI agents implementing models
  - Task decomposition strategies
  - Physics validation approaches
  - Common pitfalls and solutions
  - Debugging strategies for AI agents

- **[docs/AI_SCAFFOLDING.md](docs/AI_SCAFFOLDING.md)** - Ready-to-use templates
  - Cargo.toml template
  - Model implementation template using `fmu_from_struct`
  - Physics test template
  - Documentation template

- **[docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)** - Development workflow
  - Code quality standards
  - Testing requirements
  - PR process

### Quick Start for AI Agents

1. Read the specification for the model you're implementing
2. Use the scaffolding templates in `docs/AI_SCAFFOLDING.md`
3. Implement using `fmu_from_struct` derive macro (no manual FFI!)
4. Write physics validation tests comparing to analytical solutions
5. Build FMU and run integration tests
6. Submit PR with AI agent identifier

## 🔄 CI/CD Pipeline

### Continuous Integration
On every push to main and pull requests:
- ✅ Rust formatting check (`cargo fmt`)
- ✅ Linting with Clippy (`cargo clippy`)
- ✅ Build all models
- ✅ Run Rust tests (unit + physics)
- ✅ Build all FMUs
- ✅ Run Python FMU integration tests
- ✅ Generate documentation

### Release Workflow
Triggered by git tags or manual dispatch:
- 🔨 Build all FMU files
- 📦 Package FMUs
- 🚀 Create GitHub release with downloadable FMUs

## 🛠️ Model Implementation with `fmu_from_struct`

Models use the `fmu_from_struct` derive macro, which automatically generates FMI C bindings. Here's a minimal example:

```rust
use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct SimpleModel {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub k: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    pub x: f64,

    time: f64,
}

impl FmuFunctions for SimpleModel {
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        // Implement dx/dt = -k*x using Euler integration
        let der_x = -self.k * self.x;
        self.x += der_x * step_size;
        self.time += step_size;
    }
}
```

**No manual FFI code required!** The derive macro handles all FMI C bindings automatically.

See [docs/AI_SCAFFOLDING.md](docs/AI_SCAFFOLDING.md) for complete templates.

## 🌟 Why AI-Generated Models Matter

This project demonstrates that AI agents can:
1. **Understand complex mathematical specifications** - Differential equations, physics constraints
2. **Implement numerically correct code** - Proper integration methods, numerical stability
3. **Write comprehensive tests** - Including non-trivial physics validation
4. **Generate production-quality code** - Passing strict linters, formatters, and compliance checks
5. **Create complete documentation** - With equations, usage examples, and validation methodology

This represents a significant milestone in AI-assisted scientific computing.

## 📝 Model Documentation

Each AI-generated model includes:
- **README.md** - Overview, equations, parameters, usage examples
- **Inline documentation** - Rust doc comments explaining implementation
- **Physics validation** - Explanation of test methodology and analytical comparisons
- **AI agent notes** - Challenges encountered and solutions found

## 🤝 Contributing

We welcome both human and AI contributions!

**For AI Agents:**
- See [docs/AI_AGENTS.md](docs/AI_AGENTS.md) for implementation guidance
- Use templates in [docs/AI_SCAFFOLDING.md](docs/AI_SCAFFOLDING.md)
- Indicate which AI agent implemented the model in your PR

**For Humans:**
- See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines
- You can review AI-generated code, improve tests, or enhance infrastructure
- Help improve the AI agent guidance documentation!

## 📄 License

This project is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
- Apache License 2.0 ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## 🌟 Acknowledgments

- Models inspired by [Modelica Reference-FMUs](https://github.com/modelica/Reference-FMUs)
- FMI bindings via [`fmu_from_struct`](https://github.com/jarlekramer/fmu_from_struct) by Jarle Kramer
- All model implementations by AI coding agents (Claude Sonnet 4.5)

## 📞 Contact

- Issues: [GitHub Issues](https://github.com/freol35241/odml/issues)
- Discussions: [GitHub Discussions](https://github.com/freol35241/odml/discussions)
- AI Agent Questions: See [docs/AI_AGENTS.md](docs/AI_AGENTS.md)

## 🗺️ Roadmap

**Current Focus:**
- [x] ✅ Full FMU packaging with `fmu_from_struct`
- [x] ✅ 3-tier testing (Rust + Python + Compliance)
- [x] ✅ AI agent implementation guides
- [x] ✅ Complete CI/CD pipeline

**Coming Soon:**
- [ ] More mathematical models (Lorenz attractor, pendulum)
- [ ] Electrical models (RC circuits, motors)
- [ ] Thermal models (heat exchangers)
- [ ] Hydraulic models
- [ ] Multi-agent collaboration (different AI agents on same model)
- [ ] AI agent benchmarking framework
- [ ] Model Exchange mode (currently Co-Simulation only)
- [ ] Advanced FMI 3.0 features (clocks, structured naming)

**Join the AI revolution in scientific computing!** 🚀🤖
