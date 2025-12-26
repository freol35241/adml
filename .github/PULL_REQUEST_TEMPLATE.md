## Summary

<!-- Brief description of changes -->

## Type of Change

- [ ] New model implementation
- [ ] Bug fix
- [ ] Documentation improvement
- [ ] Infrastructure/tooling
- [ ] Test enhancement

---

## For Model Implementations

**Model Name:**
**Category:** <!-- mathematical / mechanical / thermal / etc. -->
**Contributor:** <!-- AI Agent / Human / Hybrid -->

### Differential Equations

<!-- Brief summary of implemented equations -->

### Validation Approach

<!-- How was correctness verified? -->

- [ ] Compared to analytical solution
- [ ] Conservation law tests
- [ ] Convergence tests
- [ ] Reference implementation comparison

### Checklist

- [ ] Model implemented using `fmu_from_struct`
- [ ] Struct name follows naming convention (CamelCase from directory)
- [ ] Unit tests in `src/lib.rs`
- [ ] Physics tests in `tests/physics_tests.rs`
- [ ] FMU builds successfully (`./scripts/build-fmu.sh`)
- [ ] Python integration tests added (if applicable)
- [ ] README.md with equations and usage
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Added to workspace in root `Cargo.toml`

---

## For Other Changes

### What Changed

<!-- List the specific changes made -->

### Testing

<!-- How were changes tested? -->

### Checklist

- [ ] Tests pass locally
- [ ] Documentation updated (if applicable)
- [ ] No breaking changes (or documented if breaking)

---

## Notes

<!-- Any additional context, challenges encountered, or recommendations -->

