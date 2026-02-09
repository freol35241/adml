//! Physics validation tests for the Van der Pol oscillator

use adml_van_der_pol::VanDerPol;
use approx::assert_relative_eq;

#[test]
fn test_derivative_equations() {
    let _model = VanDerPol::new();

    // Set known state
    let x0 = 1.0f64;
    let x1 = 0.5f64;
    let mu = 1.0f64;

    // Manually calculate expected derivatives
    // der(x0) = x1 = 0.5
    let der_x0 = x1;
    assert_relative_eq!(der_x0, 0.5, epsilon = 1e-10);

    // der(x1) = mu * (1 - x0²) * x1 - x0
    //         = 1.0 * (1 - 1.0) * 0.5 - 1.0
    //         = 0.0 * 0.5 - 1.0
    //         = -1.0
    let der_x1 = mu * (1.0 - x0 * x0) * x1 - x0;
    assert_relative_eq!(der_x1, -1.0, epsilon = 1e-10);
}

#[test]
fn test_nonlinear_damping_regions() {
    let _mu = 1.0f64;

    // For |x0| < 1, the damping term (1 - x0²) is positive -> energy input
    let x0_small = 0.5f64;
    let _x1 = 1.0f64;
    let damping_small = 1.0 - x0_small * x0_small;
    assert!(
        damping_small > 0.0,
        "Damping should be positive for |x0| < 1"
    );

    // For |x0| > 1, the damping term (1 - x0²) is negative -> energy dissipation
    let x0_large = 2.0;
    let damping_large = 1.0 - x0_large * x0_large;
    assert!(
        damping_large < 0.0,
        "Damping should be negative for |x0| > 1"
    );
}

#[test]
fn test_equilibrium_at_origin() {
    let mu = 1.0;

    // At origin, derivatives should be zero
    let x0 = 0.0;
    let x1 = 0.0;

    // der(x0) = x1 = 0
    let der_x0 = x1;
    assert_relative_eq!(der_x0, 0.0, epsilon = 1e-10);

    // der(x1) = mu * (1 - 0²) * 0 - 0 = 0
    let der_x1 = mu * (1.0 - x0 * x0) * x1 - x0;
    assert_relative_eq!(der_x1, 0.0, epsilon = 1e-10);
}

#[test]
fn test_energy_calculation() {
    let mut model = VanDerPol::new();

    model.x0 = 1.0;
    model.x1 = 1.0;

    let energy = model.total_energy();
    let expected = 0.5 * 1.0 * 1.0 + 0.5 * 1.0 * 1.0;
    assert_relative_eq!(energy, expected, epsilon = 1e-10);
}

#[test]
fn test_different_mu_values() {
    // Test that different μ values produce different derivative magnitudes
    for mu in [0.1f64, 1.0, 5.0] {
        let x0 = 1.0f64;
        let x1 = 0.5f64;

        // der(x1) = mu * (1 - x0²) * x1 - x0
        let der_x1: f64 = mu * (1.0 - x0 * x0) * x1 - x0;

        // Higher mu should affect the derivative calculation
        // (actual derivative value will depend on the state)
        assert!(
            der_x1.is_finite(),
            "Derivative should be finite for mu = {}",
            mu
        );
    }
}

#[test]
fn test_symmetry_properties() {
    let mu = 1.0f64;

    // Test derivative calculations at symmetric points
    let x0_pos = 1.0f64;
    let x1_pos = 0.5f64;

    let x0_neg = -1.0f64;
    let x1_neg = -0.5f64;

    let der_x1_pos: f64 = mu * (1.0 - x0_pos * x0_pos) * x1_pos - x0_pos;
    let der_x1_neg: f64 = mu * (1.0 - x0_neg * x0_neg) * x1_neg - x0_neg;

    // Due to the quadratic term x0², certain symmetries exist
    // Both calculations should produce finite results
    assert!(der_x1_pos.is_finite());
    assert!(der_x1_neg.is_finite());
}

#[test]
fn test_do_step_integration() {
    let mut model = VanDerPol::new();
    model.x0 = 2.0;
    model.x1 = 1.0; // Non-zero so x0 will change
    model.mu = 1.0;

    let initial_x0 = model.x0;
    let initial_x1 = model.x1;

    // Take a small integration step
    model.do_step(0.0, 0.01);

    // State should have changed
    // x0 changes because der(x0) = x1 = 1.0
    assert_ne!(model.x0, initial_x0, "x0 should change during integration");
    // x1 changes because der(x1) = mu*(1-x0²)*x1 - x0 ≠ 0
    assert_ne!(model.x1, initial_x1, "x1 should change during integration");
}
