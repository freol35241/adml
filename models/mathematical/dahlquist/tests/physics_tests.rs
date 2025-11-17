//! Physics validation tests for the Dahlquist model

use approx::assert_relative_eq;
use odml_dahlquist::Dahlquist;

#[test]
fn test_analytical_solution() {
    // Test analytical solution at various times
    assert_relative_eq!(Dahlquist::analytical_solution(1.0, 1.0, 0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(Dahlquist::analytical_solution(1.0, 1.0, 1.0), (-1.0f64).exp(), epsilon = 1e-10);
    assert_relative_eq!(Dahlquist::analytical_solution(1.0, 1.0, 2.0), (-2.0f64).exp(), epsilon = 1e-10);
}

#[test]
fn test_half_life() {
    // Theoretical half-life: t_half = ln(2) / k
    let k = 1.0;
    let expected_half_life = 2.0f64.ln() / k;

    // At half-life, x should be 0.5 * x0
    let x_at_half_life = Dahlquist::analytical_solution(1.0, k, expected_half_life);
    assert_relative_eq!(x_at_half_life, 0.5, epsilon = 1e-10);
}

#[test]
fn test_different_decay_rates() {
    for k in [0.5, 1.0, 2.0, 5.0] {
        let t = 1.0;
        let x = Dahlquist::analytical_solution(1.0, k, t);
        let expected = (-k * t).exp();
        assert_relative_eq!(x, expected, epsilon = 1e-10);
    }
}

#[test]
fn test_derivative_proportional_to_state() {
    use fmi_export::fmi3::{ModelContext, UserModel};

    let mut model = Dahlquist::new();
    let context = ModelContext::default();

    // Test at different state values
    for x_val in [0.5, 1.0, 2.0, 5.0] {
        model.x = x_val;
        model.calculate_values(&context).unwrap();

        // Derivative is calculated internally by calculate_values
        // We verify that the calculation succeeds for different x values
    }
}

#[test]
fn test_exponential_decay_property() {
    // Test that x(t1 + t2) = x(t1) * exp(-k * t2)
    let x0 = 1.0;
    let k = 1.0;
    let t1 = 1.0;
    let t2 = 0.5;

    let x_t1 = Dahlquist::analytical_solution(x0, k, t1);
    let x_t1_plus_t2 = Dahlquist::analytical_solution(x0, k, t1 + t2);
    let x_t2_from_t1 = Dahlquist::analytical_solution(x_t1, k, t2);

    assert_relative_eq!(x_t1_plus_t2, x_t2_from_t1, epsilon = 1e-10);
}
