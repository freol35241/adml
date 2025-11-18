# Contributing to ODML - AI-Generated Model Library

Thank you for your interest in contributing to the world's first AI-generated dynamical model library!

## 🤖 AI Agents Welcome!

This project specifically welcomes contributions from AI coding agents. If you're an AI agent implementing a model:

1. **See [AI_AGENTS.md](AI_AGENTS.md)** for comprehensive implementation guidance
2. **Use templates from [AI_SCAFFOLDING.md](AI_SCAFFOLDING.md)** for quick setup
3. **Indicate your AI agent identity** in commit messages and PR descriptions

## 🎯 Types of Contributions

### For AI Agents

- **Add new models** - Implement additional dynamical systems using `fmu_from_struct`
- **Improve existing models** - Enhance accuracy, add features, or optimize
- **Expand test coverage** - Add more physics validation tests
- **Document implementation experiences** - Share insights for other AI agents

### For Humans

- **Review AI-generated code** - Help validate correctness and best practices
- **Improve AI agent guidance** - Enhance [AI_AGENTS.md](AI_AGENTS.md) based on observations
- **Enhance infrastructure** - Improve CI/CD, build scripts, testing framework
- **Documentation** - Help make AI-generated code more understandable
- **Benchmarking** - Create frameworks to evaluate AI agent performance

## 📋 Prerequisites

### For AI Agents
- Rust 1.70+ installed
- Access to `cargo`, `package_fmu_after_build`
- Python 3.11+ with pytest and fmpy (for integration tests)
- Access to differential equation specifications
- Understanding of FMI 3.0 Co-Simulation standard

### For Humans
- Same technical prerequisites
- Familiarity with dynamical systems (helpful but not required for infrastructure work)
- Understanding of AI capabilities and limitations

## 🚀 Quick Start for AI Agents

See [AI_AGENTS.md](AI_AGENTS.md) for the complete guide. Quick summary:

1. **Read the specification** for your model (differential equations, parameters, validation criteria)
2. **Use scaffolding templates** from [AI_SCAFFOLDING.md](AI_SCAFFOLDING.md)
3. **Implement using `fmu_from_struct`** - No manual FFI code!
4. **Write physics tests** - Compare to analytical solutions or known properties
5. **Build and test FMU** - Run all three test tiers
6. **Submit PR** - Include AI agent identifier

## 🏗️ Implementation Approach: `fmu_from_struct`

**Important:** This project uses the [`fmu_from_struct`](https://github.com/jarlekramer/fmu_from_struct) derive macro for FMI bindings.

### Why `fmu_from_struct`?

1. **Eliminates manual FFI code** - Automatically generates FMI C bindings
2. **Reduces implementation errors** - No manual pointer arithmetic or type conversions
3. **More reliable for AI agents** - Declarative approach via derive macros
4. **Cleaner code** - Focus on physics, not boilerplate

### Basic Structure

```rust
use fmu_from_struct::prelude::*;

#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct MyModel {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub k: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    pub x: f64,
}

impl FmuFunctions for MyModel {
    fn do_step(&mut self, _current_time: f64, step_size: f64) {
        let der_x = -self.k * self.x;
        self.x += der_x * step_size;
    }
}
```

**That's it!** No FFI code required.

## ✅ Code Quality Standards

All contributions must meet these standards:

### Rust Code Quality

```bash
# Format code
cargo fmt --all

# Check for warnings
cargo clippy --workspace --all-targets -- -D warnings

# Build successfully
cargo build --workspace

# All tests pass
cargo test --workspace
```

### FMU Quality

```bash
# FMU builds successfully
./scripts/build-fmu.sh models/category/model-name

# FMU integration tests pass
cd testing/fmu-integration-tests
pytest test_model_name_fmu.py -v
```

### Testing Requirements

All models must include:

#### 1. Rust Unit Tests (in `src/lib.rs`)
- [ ] Default initialization
- [ ] Derivative calculations at known points
- [ ] Parameter effects on behavior
- [ ] Edge cases (zero, negative, large values)

#### 2. Rust Physics Tests (in `tests/physics_tests.rs`)
- [ ] Analytical solution comparison (if available) OR
- [ ] Conservation laws (energy, momentum) OR
- [ ] Known physical properties (limit cycles, frequencies)
- [ ] Convergence with decreasing step size
- [ ] Boundary conditions
- [ ] Event handling (if applicable)

#### 3. Python FMU Integration Tests (in `testing/fmu-integration-tests/`)
- [ ] FMU loads successfully
- [ ] Parameter setting works correctly
- [ ] Simulation runs without errors
- [ ] Results match expectations from physics tests
- [ ] Edge cases with different parameters

### Documentation Requirements

- [ ] Model `README.md` with equations and usage
- [ ] Inline doc comments (`///`) for public items
- [ ] Physics validation methodology explained
- [ ] AI agent identifier and implementation notes

## 🔍 Code Review Process

### For AI-Generated Code

1. **Automated Checks** - CI must pass:
   - ✅ Formatting (`cargo fmt --check`)
   - ✅ Linting (`cargo clippy`)
   - ✅ Build (`cargo build --workspace`)
   - ✅ All three test tiers pass

2. **Human Review** (if available):
   - Physics correctness
   - Test coverage and quality
   - Code clarity
   - Documentation completeness

3. **Physics Validation**:
   - Are equations implemented correctly?
   - Do tests validate the right properties?
   - Are tolerances appropriate for Euler integration?

### Feedback for AI Agents

Reviewers should provide:
- **Specific, actionable feedback** - "Variable should be named `der_x` not `dx_dt`" vs "naming unclear"
- **Physics-focused critiques** - Point out equation errors, not style preferences
- **Test suggestions** - "Add test for negative parameter values"
- **Patience** - AI agents learn from iteration

## 📐 Physics and Math Guidelines

### Differential Equations

- **Document clearly** - Use LaTeX notation in doc comments if helpful
- **Match specifications** - Implement exactly as specified
- **Use standard notation** - `der_x` for dx/dt, `x0`/`x1` for state vectors
- **Cite sources** - Reference papers, books, or online resources

### Numerical Integration

- **Euler is OK** - Simple explicit Euler is fine for demonstrations
- **Note limitations** - Document stability requirements if any
- **Small time steps** - Recommend dt ≤ 0.01 for accuracy
- **Test convergence** - Verify solution improves with smaller steps

### Units

- **Always specify** - Document units for all variables and parameters
- **Use SI by default** - Unless there's a good reason not to
- **Be consistent** - Within a model, maintain consistent unit systems

### Validation

- **Analytical when possible** - Best validation method
- **Conservation laws** - Next best option
- **Known properties** - Limit cycles, frequencies, steady states
- **Convergence** - Universal fallback test

## 🐛 Reporting Issues

### For AI Agents

When you encounter an issue during implementation:

1. **Document the problem** - What you tried, what happened
2. **Include error messages** - Full error text, not summaries
3. **Share context** - Model equations, test code, parameters used
4. **Indicate if you solved it** - Share the solution for other AI agents

### For Humans

When reporting issues with AI-generated code:

1. **Be specific** - Which model, which function, which test
2. **Provide reproduction steps** - How to trigger the issue
3. **Include expected vs actual behavior**
4. **Be constructive** - Remember, this is experimental AI-generated code

## 🤝 PR Submission Guidelines

### AI Agent PRs

Your PR should include:

```markdown
## Model Implementation: [Model Name]

**Implemented by:** [AI Agent Name/Version] (e.g., Claude Sonnet 4.5)
**Implementation date:** YYYY-MM-DD

### Model Description
Brief description of the model and equations implemented.

### Validation Approach
- How physics correctness was verified
- Which analytical solutions or conservation laws were tested
- Convergence testing results

### Challenges Encountered
- Any difficulties during implementation
- How they were resolved
- Insights for other AI agents

### Test Results
- [ ] All Rust tests pass (X unit tests, Y physics tests)
- [ ] FMU builds successfully
- [ ] All Python integration tests pass (Z tests)
- [ ] Code formatted and clippy clean

### Checklist
- [ ] Model implemented using `fmu_from_struct`
- [ ] Physics validation tests included
- [ ] README.md with equations and usage
- [ ] FMU integration tests added
- [ ] AI agent identified in PR and commits
```

### Human PRs

For infrastructure, documentation, or review improvements:

```markdown
## [Type]: [Brief Description]

**Type:** Infrastructure / Documentation / Test Enhancement / Review

### Changes Made
- List specific changes

### Motivation
- Why these changes are beneficial

### Testing
- How changes were verified

### Impact on AI Agents
- If applicable, how this helps AI agent implementations
```

## 📞 Getting Help

### For AI Agents
- Read [AI_AGENTS.md](AI_AGENTS.md) for detailed guidance
- Study existing models: Dahlquist (simple), Van der Pol (multi-state), Bouncing Ball (events)
- Check common pitfalls section in AI_AGENTS.md
- Open a discussion if stuck

### For Humans
- Check [GitHub Discussions](https://github.com/freol35241/odml/discussions)
- Review existing issues
- Ask about AI agent capabilities/limitations

## 🌟 Recognition

### AI Agent Contributions

AI-generated models are recognized by:
- Attribution in model table in README
- AI agent identification in model README.md
- Contribution to AI capability benchmarks
- Demonstration of state-of-the-art AI coding abilities

### Human Contributions

Human contributions are essential for:
- Validating AI-generated code
- Improving AI agent guidance
- Enhancing infrastructure
- Making the project sustainable

All contributions are valued and recognized!

## 📜 License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache 2.0, matching the project license.

This applies to both AI-generated and human-written contributions.

## 🎓 Learning Resources

### For Implementing Dynamical Models
- [Modelica Reference-FMUs](https://github.com/modelica/Reference-FMUs) - Inspiration and validation
- [FMI 3.0 Specification](https://fmi-standard.org/) - Official FMI standard
- [`fmu_from_struct` documentation](https://github.com/jarlekramer/fmu_from_struct) - FMI derive macro

### For Understanding AI Capabilities
- [AI_AGENTS.md](AI_AGENTS.md) - Written by AI for AI, based on actual implementation experience
- Existing models in this repository - Real examples of AI-generated scientific code
- PR history - See how AI agents iterate and improve

## 🚀 Future Directions

We're interested in:

- **Multi-agent collaboration** - Multiple AI agents working on same model
- **AI agent benchmarking** - Systematic evaluation of different AI agents
- **More complex models** - Pushing boundaries of what AI agents can implement
- **Better AI agent guidance** - Learning from AI agent experiences
- **Hybrid human-AI development** - Combining strengths of both

Your contributions (AI or human) help advance these goals!

## 💬 Community Guidelines

### For All Contributors

- **Be respectful** - Whether AI or human
- **Be patient** - AI agents learn through iteration, humans learn about AI capabilities
- **Be constructive** - Focus on improvement, not criticism
- **Be curious** - This is an experimental project exploring new frontiers
- **Be transparent** - Indicate if you're an AI agent or human

### Unique Aspects of This Project

This project is **intentionally experimental**:
- Testing the limits of AI coding capabilities
- Exploring AI agents in scientific computing
- Creating resources for future AI agent developers
- Demonstrating that AI can produce scientifically correct code

Contributions should embrace this experimental nature!

## 🙏 Thank You!

Whether you're an AI agent implementing a new model or a human improving the project infrastructure, your contributions are advancing the future of AI-assisted scientific computing!

**Welcome to the AI revolution in dynamical modeling!** 🤖🚀
