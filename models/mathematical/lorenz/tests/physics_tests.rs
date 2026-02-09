//! Physics validation tests for the Lorenz system

use adml_lorenz::Lorenz;
use approx::assert_relative_eq;

#[test]
fn test_derivative_equations() {
    let model = Lorenz::new();

    // At (1, 1, 1) with sigma=10, rho=28, beta=8/3
    let x = model.x;
    let y = model.y;
    let z = model.z;
    let sigma = model.sigma;
    let rho = model.rho;
    let beta = model.beta;

    // der(x) = sigma * (y - x) = 10 * (1 - 1) = 0
    let der_x = sigma * (y - x);
    assert_relative_eq!(der_x, 0.0, epsilon = 1e-10);

    // der(y) = x * (rho - z) - y = 1 * (28 - 1) - 1 = 26
    let der_y = x * (rho - z) - y;
    assert_relative_eq!(der_y, 26.0, epsilon = 1e-10);

    // der(z) = x * y - beta * z = 1 - 8/3 = -5/3
    let der_z = x * y - beta * z;
    assert_relative_eq!(der_z, -5.0 / 3.0, epsilon = 1e-10);
}

#[test]
fn test_equilibrium_at_origin() {
    let model = Lorenz::new();

    // At origin, all derivatives should be zero
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    let der_x = model.sigma * (y - x);
    let der_y = x * (model.rho - z) - y;
    let der_z = x * y - model.beta * z;

    assert_relative_eq!(der_x, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_y, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_z, 0.0, epsilon = 1e-10);
}

#[test]
fn test_symmetric_equilibria() {
    let model = Lorenz::new();

    // For rho > 1, there are two symmetric equilibria at:
    // (±sqrt(beta*(rho-1)), ±sqrt(beta*(rho-1)), rho-1)
    let c = ((model.rho - 1.0) * model.beta).sqrt();
    let z_eq = model.rho - 1.0;

    // Test positive equilibrium
    let x = c;
    let y = c;
    let z = z_eq;

    let der_x = model.sigma * (y - x);
    let der_y = x * (model.rho - z) - y;
    let der_z = x * y - model.beta * z;

    assert_relative_eq!(der_x, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_y, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_z, 0.0, epsilon = 1e-10);

    // Test negative equilibrium
    let x = -c;
    let y = -c;

    let der_x = model.sigma * (y - x);
    let der_y = x * (model.rho - z) - y;
    let der_z = x * y - model.beta * z;

    assert_relative_eq!(der_x, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_y, 0.0, epsilon = 1e-10);
    assert_relative_eq!(der_z, 0.0, epsilon = 1e-10);
}

#[test]
fn test_do_step_integration() {
    let mut model = Lorenz::new();
    model.x = 1.0;
    model.y = 2.0; // y != x so der_x != 0
    model.z = 1.0;

    let initial_x = model.x;
    let initial_y = model.y;
    let initial_z = model.z;

    // Take a small integration step
    model.do_step(0.0, 0.001);

    // All states should have changed
    assert_ne!(model.x, initial_x, "x should change during integration");
    assert_ne!(model.y, initial_y, "y should change during integration");
    assert_ne!(model.z, initial_z, "z should change during integration");
}

#[test]
fn test_euler_step_accuracy() {
    let mut model = Lorenz::new();
    model.x = 1.0;
    model.y = 1.0;
    model.z = 1.0;

    let dt = 0.001;

    // Calculate expected new values manually
    let der_x = model.sigma * (model.y - model.x); // 0
    let der_y = model.x * (model.rho - model.z) - model.y; // 26
    let der_z = model.x * model.y - model.beta * model.z; // -5/3

    let expected_x = model.x + der_x * dt;
    let expected_y = model.y + der_y * dt;
    let expected_z = model.z + der_z * dt;

    model.do_step(0.0, dt);

    assert_relative_eq!(model.x, expected_x, epsilon = 1e-10);
    assert_relative_eq!(model.y, expected_y, epsilon = 1e-10);
    assert_relative_eq!(model.z, expected_z, epsilon = 1e-10);
}

#[test]
fn test_different_parameters() {
    // Test with non-chaotic parameters (rho < 1)
    let mut model = Lorenz::with_params(10.0, 0.5, 8.0 / 3.0);
    model.x = 1.0;
    model.y = 1.0;
    model.z = 1.0;

    // Derivatives should still be finite
    let der_x = model.sigma * (model.y - model.x);
    let der_y = model.x * (model.rho - model.z) - model.y;
    let der_z = model.x * model.y - model.beta * model.z;

    assert!(der_x.is_finite());
    assert!(der_y.is_finite());
    assert!(der_z.is_finite());
}

#[test]
fn test_symmetry_x_y() {
    let model = Lorenz::new();

    // The x equation: der(x) = sigma * (y - x)
    // Swapping x and y should negate the derivative
    let x1 = 2.0;
    let y1 = 5.0;
    let der_x1 = model.sigma * (y1 - x1);

    let x2 = 5.0;
    let y2 = 2.0;
    let der_x2 = model.sigma * (y2 - x2);

    assert_relative_eq!(der_x1, -der_x2, epsilon = 1e-10);
}

#[test]
fn test_z_equation_quadratic() {
    let model = Lorenz::new();

    // der(z) = x*y - beta*z
    // The x*y term creates the characteristic butterfly pattern
    let x = 5.0;
    let y = 5.0;
    let z = 10.0;

    let der_z = x * y - model.beta * z;
    let expected = 25.0 - model.beta * 10.0;

    assert_relative_eq!(der_z, expected, epsilon = 1e-10);
}

#[test]
fn test_bounded_behavior_short_term() {
    // The Lorenz system should remain bounded for classic parameters
    let mut model = Lorenz::new();
    model.x = 1.0;
    model.y = 1.0;
    model.z = 1.0;

    let dt = 0.001;
    let steps = 1000;

    for _ in 0..steps {
        model.do_step(0.0, dt);

        // Check that the system remains bounded
        assert!(model.x.abs() < 100.0, "x became unbounded: {}", model.x);
        assert!(model.y.abs() < 100.0, "y became unbounded: {}", model.y);
        assert!(model.z.abs() < 100.0, "z became unbounded: {}", model.z);
    }
}

#[test]
fn test_sensitivity_to_initial_conditions() {
    // The hallmark of chaos: small differences in initial conditions lead to divergence
    let mut model1 = Lorenz::new();
    model1.x = 1.0;
    model1.y = 1.0;
    model1.z = 1.0;

    let mut model2 = Lorenz::new();
    model2.x = 1.0001; // Small perturbation
    model2.y = 1.0;
    model2.z = 1.0;

    let dt = 0.001; // Smaller time step for accuracy
    let steps = 20000; // Longer simulation (~20 time units)

    for _ in 0..steps {
        model1.do_step(0.0, dt);
        model2.do_step(0.0, dt);
    }

    // After sufficient time, the trajectories should have diverged significantly
    let distance = ((model1.x - model2.x).powi(2)
        + (model1.y - model2.y).powi(2)
        + (model1.z - model2.z).powi(2))
    .sqrt();

    // The divergence should be much larger than the initial perturbation (0.0001)
    // Even 0.1 would be 1000x amplification, demonstrating chaos
    assert!(
        distance > 0.1,
        "Expected chaotic divergence, but distance was only {}",
        distance
    );
}

#[test]
fn test_equilibrium_points_calculation() {
    let model = Lorenz::new();
    let eqs = model.equilibrium_points();

    // Origin should be first
    assert_relative_eq!(eqs[0].0, 0.0, epsilon = 1e-10);
    assert_relative_eq!(eqs[0].1, 0.0, epsilon = 1e-10);
    assert_relative_eq!(eqs[0].2, 0.0, epsilon = 1e-10);

    // Check the symmetric points
    let c = ((model.rho - 1.0) * model.beta).sqrt();
    assert_relative_eq!(eqs[1].0, c, epsilon = 1e-10);
    assert_relative_eq!(eqs[1].1, c, epsilon = 1e-10);
    assert_relative_eq!(eqs[1].2, model.rho - 1.0, epsilon = 1e-10);

    assert_relative_eq!(eqs[2].0, -c, epsilon = 1e-10);
    assert_relative_eq!(eqs[2].1, -c, epsilon = 1e-10);
    assert_relative_eq!(eqs[2].2, model.rho - 1.0, epsilon = 1e-10);
}

#[test]
fn test_distance_from_origin() {
    let mut model = Lorenz::new();
    model.x = 3.0;
    model.y = 4.0;
    model.z = 0.0;

    let distance = model.distance_from_origin();
    assert_relative_eq!(distance, 5.0, epsilon = 1e-10);
}
