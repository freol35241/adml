//! Physics validation tests for the Dahlquist model

use approx::assert_relative_eq;
use odml_dahlquist::Dahlquist;
use physics_framework::assertions::assert_relative_eq as assert_physics_relative_eq;
use physics_framework::numerical::compare_with_analytical;

#[test]
fn test_exponential_decay() {
    let mut model = Dahlquist::new();
    model.x = 1.0;
    model.k = 1.0;

    let dt = 0.001;
    let t_final = 5.0;
    let mut results = Vec::new();

    let mut time = 0.0;
    while time < t_final {
        results.push((time, model.x));
        model.do_step(dt);
        time += dt;
    }

    // Compare with analytical solution: x(t) = exp(-k*t)
    let analytical = |t: f64| (-t).exp();

    assert!(
        compare_with_analytical(&results, analytical, 0.01),
        "Simulation does not match analytical solution"
    );
}

#[test]
fn test_analytical_solution() {
    let model = Dahlquist::new();

    // Test analytical solution at various times
    assert_relative_eq!(model.analytical_solution(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(model.analytical_solution(1.0), (-1.0f64).exp(), epsilon = 1e-10);
    assert_relative_eq!(model.analytical_solution(2.0), (-2.0f64).exp(), epsilon = 1e-10);
}

#[test]
fn test_half_life() {
    let mut model = Dahlquist::new();
    model.x = 1.0;
    model.k = 1.0;

    let dt = 0.01;

    // Find time when x reaches 0.5 (half-life)
    let mut time = 0.0;
    while model.x > 0.5 {
        model.do_step(dt);
        time += dt;
    }

    // Theoretical half-life: t_half = ln(2) / k
    let expected_half_life = 2.0f64.ln() / model.k;

    assert_relative_eq!(time, expected_half_life, epsilon = 0.05);
}

#[test]
fn test_different_decay_rates() {
    for k in [0.5, 1.0, 2.0, 5.0] {
        let mut model = Dahlquist::new();
        model.k = k;
        model.x = 1.0;

        let dt = 0.001;
        let t_final = 2.0 / k; // Simulate for 2 time constants

        let mut time = 0.0;
        while time < t_final {
            model.do_step(dt);
            time += dt;
        }

        let expected = (-k * time).exp();
        assert_physics_relative_eq(model.x, expected, 0.01);
    }
}

#[test]
fn test_convergence() {
    // Test that smaller step sizes give more accurate results
    let model = Dahlquist::new();
    let t_final = 1.0;
    let expected = (-1.0f64).exp();

    let step_sizes = [0.1, 0.01, 0.001];
    let mut errors = Vec::new();

    for &dt in &step_sizes {
        let mut test_model = Dahlquist::new();
        let mut time = 0.0;

        while time < t_final {
            test_model.do_step(dt);
            time += dt;
        }

        let error = (test_model.x - expected).abs();
        errors.push(error);
    }

    // Check that errors decrease with smaller step sizes
    for i in 1..errors.len() {
        assert!(
            errors[i] < errors[i - 1],
            "Error should decrease with smaller step size"
        );
    }
}
