//! Physics validation tests for the Bouncing Ball model

use approx::assert_relative_eq;
use odml_bouncing_ball::{BouncingBall, FmuFunctions};

#[test]
fn test_energy_calculation() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;

    let pe = model.potential_energy();
    let ke = model.kinetic_energy();

    assert_relative_eq!(pe, 9.81, epsilon = 1e-10);
    assert_relative_eq!(ke, 0.0, epsilon = 1e-10);

    // Test with velocity
    model.v = 2.0;
    let ke2 = model.kinetic_energy();
    assert_relative_eq!(ke2, 2.0, epsilon = 1e-10); // 0.5 * 2² = 2.0
}

#[test]
fn test_collision_event_handling() {
    let mut model = BouncingBall::new();

    // Setup collision scenario
    model.h = 0.0;
    model.v = -2.0;
    let initial_v = model.v.abs();

    // Trigger collision via do_step
    model.do_step(0.0, 0.01);

    // Check velocity was reversed with restitution
    let expected_v = initial_v * model.e;
    assert_relative_eq!(model.v, expected_v, epsilon = 1e-10);

    // Check height is slightly positive
    assert!(model.h > 0.0);
}

#[test]
fn test_energy_loss_per_bounce() {
    let mut model = BouncingBall::new();

    model.h = 1.0;
    model.v = 0.0;
    let _initial_energy = model.total_energy();

    // Simulate a bounce
    model.h = 0.0;
    model.v = -4.43; // Approximate velocity when falling from h=1.0

    let energy_before_bounce = model.total_energy();
    model.do_step(0.0, 0.01);
    let energy_after_bounce = model.total_energy();

    // Energy should decrease (not perfectly e² because of height adjustment)
    assert!(energy_after_bounce < energy_before_bounce);
}

#[test]
fn test_stopping_condition() {
    let mut model = BouncingBall::new();

    // Set velocity below threshold
    model.h = 0.0;
    model.v = -0.05; // Below v_min = 0.1

    model.do_step(0.0, 0.01);

    // Ball should have stopped
    assert_eq!(model.v, 0.0);
    assert_eq!(model.g, 0.0); // Gravity disabled
}

#[test]
fn test_no_collision_above_ground() {
    let mut model = BouncingBall::new();

    // Ball above ground
    model.h = 0.5;
    model.v = -1.0;
    let initial_v = model.v;

    model.do_step(0.0, 0.01);

    // Velocity should have become more negative due to gravity
    assert!(model.v < initial_v);
}

#[test]
fn test_free_fall_acceleration() {
    let mut model = BouncingBall::new();
    model.h = 10.0;
    model.v = 0.0;

    let dt = 0.1;

    // Take one step
    model.do_step(0.0, dt);

    // Expected velocity change: dv = g * dt
    let expected_v = 0.0 + model.g * dt;
    assert_relative_eq!(model.v, expected_v, epsilon = 1e-10);

    // Expected height change: dh = v_initial * dt (since v_initial = 0)
    let expected_h = 10.0; // No change in first step since v_initial = 0
    assert_relative_eq!(model.h, expected_h, epsilon = 1e-10);
}

#[test]
fn test_multiple_bounces() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;

    let mut bounce_count = 0;
    let dt = 0.001;

    // Simulate for a period of time
    for _ in 0..10000 {
        let v_before = model.v;
        model.do_step(0.0, dt);

        // Detect bounce (velocity changed from negative to positive)
        if v_before < 0.0 && model.v > 0.0 {
            bounce_count += 1;
        }

        // Stop if ball has stopped bouncing
        if model.g == 0.0 {
            break;
        }
    }

    // Ball should have bounced at least once
    assert!(bounce_count > 0, "Ball should bounce at least once");

    // Ball should have stopped eventually
    assert_eq!(model.v, 0.0, "Ball should eventually stop");
    assert_eq!(model.g, 0.0, "Gravity should be disabled when stopped");
}
