# Review: AI Agent Structure and Documentation

**Reviewer:** AI Agent (Claude Opus 4.5)
**Date:** 2025-12-26
**Purpose:** Critical review of repository structure for generic AI agent support

---

## Executive Summary

ADML has excellent foundational documentation for AI agents, but it's currently **Claude-specific** in several places and lacks **machine-readable context** that modern AI agents expect. This review proposes changes to make the repository truly **agent-agnostic** while increasing **discoverability** and **ease of contribution** for any AI coding assistant.

### Key Recommendations (Priority Order)

1. **Add root-level `AGENTS.md`** - Single entry point for all AI agents
2. **Create machine-readable model specifications** - `model.toml` files
3. **Add GitHub issue/PR templates** - Structured contribution workflow
4. **Remove Claude-specific language** - Make documentation agent-agnostic
5. **Create `models.json` catalog** - Machine-readable model registry
6. **Add validation schema files** - Formal specification of expected behavior
7. **Create `.claude/` and `.cursor/` directories** - Agent-specific configs

---

## Detailed Analysis

### 1. Root-Level Context File (Critical)

**Current State:** No single entry point. AI agents must read multiple files:
- `README.md` (project overview)
- `docs/AI_AGENTS.md` (implementation guide)
- `docs/AI_SCAFFOLDING.md` (templates)
- `docs/CONTRIBUTING.md` (contribution process)

**Problem:** Many AI agents (Cursor, GitHub Copilot Workspace, Windsurf, Aider, etc.) look for specific files like `AGENTS.md`, `.cursorrules`, or `CLAUDE.md` for project context.

**Recommendation:** Create `/AGENTS.md` with:
- Concise project overview (what this repo is)
- Quick-start for contributing a model (5 steps)
- Links to detailed documentation
- Common commands cheat sheet
- Critical constraints and conventions

```markdown
# AGENTS.md - AI Agent Instructions

## Project Summary
ADML is a library of FMI 3.0 compliant dynamical models. All code is AI-generated.

## Quick Start (New Model)
1. Copy template: `docs/AI_SCAFFOLDING.md`
2. Create: `models/{category}/{model-name}/`
3. Implement: differential equations in `do_step()`
4. Test: `cargo test -p adml-{model-name}`
5. Build FMU: `./scripts/build-fmu.sh models/{category}/{model-name}`

## Critical Conventions
- Struct name: CamelCase from directory (rc-thermal → RcThermal)
- Time units: Always seconds (FMI standard)
- Step size: Use 0.01 for Euler integration tests
- Parameters vs Outputs: Only outputs appear in simulation results

## Commands
cargo test --workspace              # Run all tests
./scripts/build-fmu.sh <model-dir>  # Build single FMU
./scripts/test-all.sh               # Full test suite
```

---

### 2. Agent-Specific Bias (High Priority)

**Current State:** Documentation contains Claude-specific references:

| File | Issue |
|------|-------|
| `docs/AI_AGENTS.md:3` | "written for AI agents by an AI agent (Claude Sonnet 4.5)" |
| `docs/AI_AGENTS.md:100-112` | Claude-specific TODO tool examples (`TodoWrite`) |
| `docs/AI_SCAFFOLDING.md:37` | `authors = ["AI Agent <agent@anthropic.com>"]` |
| `docs/AI_SCAFFOLDING.md:801` | "These templates are maintained by the ODML community" (outdated name) |
| `README.md` | Multiple references to "Claude Sonnet 4.5" |
| `docs/CONTRIBUTING.md` | Mentions Claude for feedback |

**Recommendation:** Replace with agent-agnostic language:

```diff
- written for AI agents by an AI agent (Claude Sonnet 4.5)
+ written for AI agents, based on actual implementation experience

- authors = ["AI Agent <agent@anthropic.com>"]
+ authors = ["AI Agent"]

- TodoWrite([...])  # Claude-specific
+ # Use your agent's task tracking mechanism if available

- | Dahlquist | ... | Claude Sonnet 4.5 |
+ | Dahlquist | ... | AI Agent (v1.0) |
```

**Rationale:** Agent-agnostic documentation allows:
- Any AI agent to use the guidance
- Fair benchmarking between agents
- No perception of vendor lock-in

---

### 3. Machine-Readable Model Specifications (High Priority)

**Current State:** Model specifications exist only in prose (README.md files).

**Problem:** AI agents cannot programmatically parse model requirements or validate completeness.

**Recommendation:** Add `model.toml` to each model directory:

```toml
# models/mathematical/dahlquist/model.toml
[model]
name = "Dahlquist"
category = "mathematical"
description = "Simple exponential decay test equation"
version = "1.0.0"
fmi_version = "3.0"

[equations]
# Differential equations in a parseable format
state_variables = ["x"]
differential_equations = [
    "dx/dt = -k * x"
]
analytical_solution = "x(t) = x0 * exp(-k * t)"

[parameters]
k = { type = "Real", default = 1.0, unit = "1/s", description = "Decay rate" }

[initial_conditions]
x = { type = "Real", default = 1.0, unit = "1", description = "Initial state" }

[outputs]
x = { type = "Real", unit = "1", description = "Current state" }

[validation]
has_analytical_solution = true
conservation_laws = []
known_properties = ["exponential_decay", "stability"]

[tests]
required = [
    "analytical_solution_comparison",
    "convergence_with_step_size",
    "parameter_sensitivity"
]
```

**Benefits:**
- AI agents can validate model completeness before implementation
- Automated test generation from specifications
- Model comparison and cataloging
- IDE/editor support for schema validation

---

### 4. Models Catalog (Medium Priority)

**Current State:** Models are listed in README.md only (prose format).

**Recommendation:** Create `/models.json` as a machine-readable catalog:

```json
{
  "$schema": "./schemas/models-catalog.schema.json",
  "version": "1.0.0",
  "models": [
    {
      "name": "Dahlquist",
      "package": "adml-dahlquist",
      "path": "models/mathematical/dahlquist",
      "category": "mathematical",
      "version": "1.0.0",
      "complexity": "simple",
      "features": ["analytical_solution", "single_state"],
      "equations": 1,
      "states": 1,
      "parameters": 1,
      "fmu_name": "Dahlquist.fmu"
    },
    {
      "name": "VanDerPol",
      "package": "adml-van-der-pol",
      "path": "models/mathematical/van-der-pol",
      "category": "mathematical",
      "version": "1.0.0",
      "complexity": "medium",
      "features": ["limit_cycle", "nonlinear", "multi_state"],
      "equations": 2,
      "states": 2,
      "parameters": 1,
      "fmu_name": "VanDerPol.fmu"
    }
  ]
}
```

**Usage:**
- CI can validate all models exist and pass tests
- AI agents can query available models and their features
- Documentation can be auto-generated

---

### 5. GitHub Templates (Medium Priority)

**Current State:** No issue or PR templates exist.

**Recommendation:** Create `.github/ISSUE_TEMPLATE/` and `.github/PULL_REQUEST_TEMPLATE.md`:

#### `.github/ISSUE_TEMPLATE/new_model_request.md`
```markdown
---
name: New Model Request
about: Request a new dynamical model for AI implementation
labels: model-request
---

## Model Name
<!-- e.g., Lorenz Attractor -->

## Category
<!-- mathematical / mechanical / electrical / thermal / hydraulic -->

## Differential Equations
<!-- Use LaTeX or plain text -->
```
dx/dt = σ(y - x)
dy/dt = x(ρ - z) - y
dz/dt = xy - βz
```

## Parameters
| Name | Symbol | Default | Unit | Description |
|------|--------|---------|------|-------------|
| sigma | σ | 10.0 | 1 | ... |

## Validation Approach
<!-- How can we verify correctness? -->
- [ ] Analytical solution available
- [ ] Known properties (e.g., attractor shape)
- [ ] Conservation laws
- [ ] Reference implementation

## References
<!-- Papers, books, or links -->
```

#### `.github/PULL_REQUEST_TEMPLATE.md`
```markdown
## Model Implementation

**Model:** <!-- Model name -->
**Category:** <!-- mathematical / mechanical / etc. -->
**AI Agent:** <!-- Your AI agent name/version (optional) -->

### Checklist
- [ ] Model implemented using `fmu_from_struct`
- [ ] Unit tests pass (`cargo test -p adml-{name}`)
- [ ] Physics validation tests included
- [ ] FMU builds successfully
- [ ] Python integration tests pass
- [ ] README.md with equations
- [ ] Code formatted (`cargo fmt`)
- [ ] Clippy clean (`cargo clippy`)

### Validation Approach
<!-- Describe how correctness was verified -->

### Notes
<!-- Any challenges, insights, or recommendations -->
```

---

### 6. Validation Schema Files (Medium Priority)

**Current State:** Test expectations are implicit in test code.

**Recommendation:** Add `validation.toml` to each model:

```toml
# models/mathematical/dahlquist/validation.toml

[analytical]
# Expected solution at specific times
[[analytical.checkpoints]]
time = 1.0
x = 0.3679  # exp(-1)
tolerance = 0.05

[[analytical.checkpoints]]
time = 5.0
x = 0.0067  # exp(-5)
tolerance = 0.05

[convergence]
# Step sizes and expected errors
step_sizes = [0.1, 0.01, 0.001]
error_should_decrease = true
convergence_order = 1  # Euler is first-order

[stability]
simulation_time = 100.0
state_must_remain_bounded = true
max_state_value = 1.0

[conservation]
# Empty for dissipative systems
energy_conserved = false
```

**Benefits:**
- Standardized test generation
- Clear pass/fail criteria
- Comparison between implementations

---

### 7. Error Catalog (Medium Priority)

**Current State:** Error messages in `AI_AGENTS.md` are helpful but incomplete.

**Recommendation:** Create `/docs/ERROR_CATALOG.md`:

```markdown
# Error Catalog for AI Agents

## Build Errors

### E001: Cannot find type `Fmu` in this scope
**Cause:** Missing prelude import
**Solution:**
```rust
pub use fmu_from_struct::prelude::*;
```

### E002: clippy::not_unsafe_ptr_arg_deref
**Cause:** Clippy warning from generated FFI code
**Solution:**
```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
```

## FMU Errors

### E101: FMU filename mismatch
**Cause:** Struct name doesn't match expected CamelCase from directory
**Example:** Directory `rc-thermal` expects struct `RcThermal`, not `RCThermal`
**Solution:** Split on hyphens, capitalize each word, join

### E102: Variable not found in simulation results
**Cause:** Trying to plot/access a parameter (not output)
**Solution:** Only `#[fmu_from_struct(output)]` variables appear in results

## Test Errors

### E201: Assertion failed with large error
**Cause:** Euler integration error accumulation
**Solution:** Use smaller step size (0.01) or increase tolerance
```

---

### 8. Agent-Specific Configuration Directories (Low Priority)

**Current State:** No agent-specific configuration.

**Recommendation:** Support multiple AI agent configurations:

```
.agents/
├── cursor/
│   └── rules.md          # Cursor-specific rules
├── copilot/
│   └── instructions.md   # GitHub Copilot instructions
└── common/
    └── context.md        # Shared context for all agents
```

Or use established conventions:
- `.cursorrules` for Cursor
- `CLAUDE.md` for Claude
- `.github/copilot-instructions.md` for Copilot

---

### 9. Documentation Consolidation (Low Priority)

**Current State:** AI guidance split across 3 files (900+ lines total):
- `AI_AGENTS.md` (960 lines)
- `AI_SCAFFOLDING.md` (800 lines)
- `CONTRIBUTING.md` (383 lines)

**Issue:** Token budget for AI agents. Loading all 3 files is expensive.

**Recommendation:** Create tiered documentation:

1. **AGENTS.md** (root, ~100 lines) - Quick start, critical info
2. **docs/AI_QUICK_START.md** (~200 lines) - Essential workflow
3. **docs/AI_REFERENCE.md** (full) - Complete reference (current AI_AGENTS.md)
4. **docs/AI_TEMPLATES.md** (templates only) - Copy-paste ready

Most AI agents only need levels 1-2 for typical contributions.

---

### 10. Naming Consistency (Low Priority)

**Current Issue:** Mixed references to "ODML" and "ADML":
- `docs/AI_SCAFFOLDING.md:3` mentions "ODML"
- `docs/AI_SCAFFOLDING.md:717` mentions "odml-model-name"
- Everywhere else uses "ADML"

**Recommendation:** Global find/replace:
- `ODML` → `ADML`
- `odml-` → `adml-`

---

## Implementation Priority

### Phase 1: Critical (Do First)
1. Create `/AGENTS.md` (root-level context)
2. Remove Claude-specific language
3. Fix ODML → ADML naming

### Phase 2: High Value
4. Add GitHub issue/PR templates
5. Create `model.toml` specification format (start with one model)
6. Create `/models.json` catalog

### Phase 3: Nice to Have
7. Add `validation.toml` files
8. Create error catalog
9. Add agent-specific configs
10. Consolidate documentation tiers

---

## Files to Create

| File | Priority | Purpose |
|------|----------|---------|
| `/AGENTS.md` | Critical | Root-level AI agent entry point |
| `/.github/ISSUE_TEMPLATE/new_model_request.md` | High | Structured model requests |
| `/.github/ISSUE_TEMPLATE/bug_report.md` | High | Bug reporting |
| `/.github/PULL_REQUEST_TEMPLATE.md` | High | PR checklist |
| `/models.json` | Medium | Machine-readable model catalog |
| `/schemas/model.schema.json` | Medium | Model specification schema |
| `/docs/ERROR_CATALOG.md` | Medium | Searchable error reference |
| `/models/*/model.toml` | Medium | Per-model specifications |
| `/models/*/validation.toml` | Low | Test specifications |

---

## Files to Modify

| File | Changes |
|------|---------|
| `docs/AI_AGENTS.md` | Remove Claude references, update examples |
| `docs/AI_SCAFFOLDING.md` | Fix ODML→ADML, remove Anthropic email |
| `docs/CONTRIBUTING.md` | Make agent-agnostic |
| `README.md` | Remove specific agent attribution in table |

---

## Appendix: Comparison with Other AI-Friendly Repos

| Feature | ADML (Current) | Best Practices |
|---------|----------------|----------------|
| Root context file | ❌ | ✅ AGENTS.md, CLAUDE.md, .cursorrules |
| Machine-readable specs | ❌ | ✅ JSON/TOML schemas |
| GitHub templates | ❌ | ✅ Issue + PR templates |
| Agent-agnostic | ⚠️ Claude-biased | ✅ Generic language |
| Error catalog | ⚠️ Partial | ✅ Searchable reference |
| Model catalog | ❌ | ✅ models.json / registry |
| Tiered docs | ❌ | ✅ Quick start + Reference |

---

## Conclusion

ADML has strong foundations for AI agent contributions, particularly the detailed `AI_AGENTS.md` guide with real implementation experience. The main gaps are:

1. **Discoverability** - No standard entry point (`AGENTS.md`)
2. **Genericity** - Claude-specific language limits adoption
3. **Machine-readability** - Prose-only specifications

Implementing the Phase 1 recommendations (3 changes) would significantly improve the experience for any AI agent, while Phase 2 would make ADML a model repository for AI-friendly open source projects.

---

*This review was created by an AI agent and should be validated by project maintainers before implementation.*
