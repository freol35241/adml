//! Physics validation tests for the Bouncing Ball model

use approx::assert_relative_eq;
use odml_bouncing_ball::BouncingBall;

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
    use fmi::{EventFlags};
    use fmi_export::fmi3::{ModelContext, UserModel};

    let mut model = BouncingBall::new();
    let context = ModelContext::default();
    let mut event_flags = EventFlags::default();

    // Setup collision scenario
    model.h = 0.0;
    model.v = -2.0;
    let initial_v = model.v.abs();

    // Trigger collision
    model.event_update(&context, &mut event_flags).unwrap();

    // Check that event was handled
    assert!(event_flags.values_of_continuous_states_changed);

    // Check velocity was reversed with restitution
    let expected_v = initial_v * model.e;
    assert_relative_eq!(model.v, expected_v, epsilon = 1e-10);

    // Check height is slightly positive
    assert!(model.h > 0.0);
}

#[test]
fn test_energy_loss_per_bounce() {
    use fmi::EventFlags;
    use fmi_export::fmi3::{ModelContext, UserModel};

    let mut model = BouncingBall::new();
    let context = ModelContext::default();
    let mut event_flags = EventFlags::default();

    model.h = 1.0;
    model.v = 0.0;
    let initial_energy = model.total_energy();

    // Simulate a bounce
    model.h = 0.0;
    model.v = -4.43; // Approximate velocity when falling from h=1.0

    model.event_update(&context, &mut event_flags).unwrap();

    let energy_after_bounce = model.total_energy();

    // Energy should decrease (not perfectly e² because of height adjustment)
    assert!(energy_after_bounce < initial_energy);
}

#[test]
fn test_stopping_condition() {
    use fmi::EventFlags;
    use fmi_export::fmi3::{ModelContext, UserModel};

    let mut model = BouncingBall::new();
    let context = ModelContext::default();
    let mut event_flags = EventFlags::default();

    // Set velocity below threshold
    model.h = 0.0;
    model.v = -0.05; // Below v_min = 0.1

    model.event_update(&context, &mut event_flags).unwrap();

    // Ball should have stopped
    assert_eq!(model.v, 0.0);
    assert_eq!(model.g, 0.0); // Gravity disabled
}

#[test]
fn test_no_collision_above_ground() {
    use fmi::EventFlags;
    use fmi_export::fmi3::{ModelContext, UserModel};

    let mut model = BouncingBall::new();
    let context = ModelContext::default();
    let mut event_flags = EventFlags::default();

    // Ball above ground
    model.h = 0.5;
    model.v = -1.0;
    let initial_v = model.v;

    model.event_update(&context, &mut event_flags).unwrap();

    // No event should have occurred
    assert!(!event_flags.values_of_continuous_states_changed);
    assert_eq!(model.v, initial_v); // Velocity unchanged
}
