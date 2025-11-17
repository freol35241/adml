//! Physics validation tests for the Bouncing Ball model

use approx::assert_relative_eq;
use odml_bouncing_ball::BouncingBall;
use physics_framework::numerical::find_peaks;

#[test]
fn test_free_fall() {
    let mut model = BouncingBall::new();
    model.h = 10.0;
    model.v = 0.0;
    model.e = 1.0; // Perfect elasticity to test free fall without bouncing effects

    let dt = 0.001;
    let mut time = 0.0;

    // Fall until just before hitting ground
    while model.h > 0.1 {
        model.do_step(dt);
        time += dt;
    }

    // Check velocity matches free fall: v = g*t
    let expected_v = model.g * time;
    assert_relative_eq!(model.v, expected_v, epsilon = 0.1);
}

#[test]
fn test_energy_loss_per_bounce() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;
    model.e = 0.8;

    let initial_energy = model.total_energy();

    let dt = 0.001;
    let mut bounces = 0;
    let mut prev_h = model.h;

    // Simulate until first bounce
    loop {
        let stopped = model.do_step(dt);
        if stopped {
            break;
        }

        // Detect bounce by checking if velocity changed sign near ground
        if model.h < 0.01 && model.h > prev_h {
            bounces += 1;
            if bounces >= 1 {
                break;
            }
        }
        prev_h = model.h;
    }

    let energy_after_bounce = model.total_energy();

    // Energy after bounce should be less than initial
    assert!(
        energy_after_bounce < initial_energy,
        "Energy should decrease after bounce"
    );

    // Energy ratio should be approximately e²
    let energy_ratio = energy_after_bounce / initial_energy;
    let expected_ratio = model.e * model.e;

    assert_relative_eq!(energy_ratio, expected_ratio, epsilon = 0.2);
}

#[test]
fn test_multiple_bounces() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;
    model.e = 0.7;

    let dt = 0.001;
    let t_final = 10.0;

    let mut heights = Vec::new();
    let mut time = 0.0;

    while time < t_final && !(model.v == 0.0 && model.h < 0.1) {
        heights.push(model.h);
        let stopped = model.do_step(dt);
        time += dt;

        if stopped {
            break;
        }
    }

    // Find peaks (maximum heights after bounces)
    let peaks = find_peaks(&heights);

    // Should have multiple bounces
    assert!(
        peaks.len() >= 3,
        "Should have at least 3 bounces, found {}",
        peaks.len()
    );

    // Check that peak heights decrease
    for i in 1..peaks.len() {
        assert!(
            peaks[i].1 < peaks[i - 1].1,
            "Peak heights should decrease: {} >= {}",
            peaks[i].1,
            peaks[i - 1].1
        );
    }
}

#[test]
fn test_coefficient_of_restitution_effect() {
    // Test different restitution coefficients
    for e in [0.5, 0.7, 0.9] {
        let mut model = BouncingBall::new();
        model.h = 1.0;
        model.v = 0.0;
        model.e = e;

        let dt = 0.001;
        let mut bounce_count = 0;
        let mut prev_h = model.h;

        for _ in 0..10000 {
            let stopped = model.do_step(dt);

            // Count bounces
            if model.h < 0.01 && model.h > prev_h {
                bounce_count += 1;
            }
            prev_h = model.h;

            if stopped {
                break;
            }
        }

        // Higher e should result in more bounces
        assert!(
            bounce_count > 0,
            "Should have at least one bounce for e={}",
            e
        );
    }
}

#[test]
fn test_eventual_stop() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;
    model.e = 0.6;

    let dt = 0.001;
    let max_iterations = 100000;

    for _ in 0..max_iterations {
        let stopped = model.do_step(dt);
        if stopped {
            // Verify ball has stopped
            assert_relative_eq!(model.v, 0.0, epsilon = 1e-10);
            assert_relative_eq!(model.g, 0.0, epsilon = 1e-10);
            assert!(model.h < 0.1);
            return;
        }
    }

    panic!("Ball should eventually stop bouncing");
}

#[test]
fn test_bounce_symmetry() {
    let mut model = BouncingBall::new();
    model.h = 0.0;
    model.v = 5.0; // Start with upward velocity
    model.e = 1.0; // Perfect elasticity

    let dt = 0.001;
    let initial_v = model.v;

    // Go up and come back down
    let mut max_height = 0.0;
    for _ in 0..10000 {
        model.do_step(dt);
        if model.h > max_height {
            max_height = model.h;
        }
        if model.h < 0.01 && model.v < 0.0 {
            break;
        }
    }

    // With perfect elasticity, velocity magnitude when returning should match initial
    assert_relative_eq!(model.v.abs(), initial_v, epsilon = 0.5);

    // Maximum height should match h = v²/(2g)
    let expected_max_h = initial_v * initial_v / (2.0 * model.g.abs());
    assert_relative_eq!(max_height, expected_max_h, epsilon = 0.1);
}

#[test]
fn test_no_negative_height_sustained() {
    let mut model = BouncingBall::new();
    model.h = 1.0;
    model.v = 0.0;

    let dt = 0.001;

    for _ in 0..10000 {
        let stopped = model.do_step(dt);

        // Height should never be significantly negative
        assert!(
            model.h >= -0.001,
            "Height should not be significantly negative: {}",
            model.h
        );

        if stopped {
            break;
        }
    }
}
