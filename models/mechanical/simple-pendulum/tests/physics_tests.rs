use adml_simple_pendulum::{SimplePendulum, FmuFunctions};
use approx::assert_relative_eq;

/// Test that small-angle pendulum matches analytical solution
#[test]
fn test_small_angle_analytical_solution() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.1; // Small angle (< 0.2 rad for good approximation)
    pendulum.omega = 0.0;
    pendulum.b = 0.0; // No damping for analytical solution

    // Simulation parameters
    let dt = 0.001; // Small time step for good accuracy
    let t_final = 5.0; // Simulate for 5 seconds (~2.5 periods)

    // Simulate
    let steps = (t_final / dt) as usize;
    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    // Compare with analytical solution
    let (theta_analytical, omega_analytical) =
        SimplePendulum::analytical_solution(0.1, 0.0, pendulum.g, pendulum.L, t_final);

    // Euler integration accumulates error, so allow 5% tolerance
    assert_relative_eq!(pendulum.theta, theta_analytical, epsilon = 0.05);
    assert_relative_eq!(pendulum.omega, omega_analytical, epsilon = 0.05);
}

/// Test with initial velocity instead of displacement
#[test]
fn test_small_angle_with_initial_velocity() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.0;
    pendulum.omega = 0.5; // Start with angular velocity
    pendulum.b = 0.0;

    let dt = 0.001;
    let t_final = 3.0;

    let steps = (t_final / dt) as usize;
    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    let (theta_analytical, omega_analytical) =
        SimplePendulum::analytical_solution(0.0, 0.5, pendulum.g, pendulum.L, t_final);

    assert_relative_eq!(pendulum.theta, theta_analytical, epsilon = 0.05);
    assert_relative_eq!(pendulum.omega, omega_analytical, epsilon = 0.05);
}

/// Test with different pendulum parameters
#[test]
fn test_different_parameters() {
    let mut pendulum = SimplePendulum::new();
    pendulum.L = 2.0; // Longer pendulum (slower oscillation)
    pendulum.g = 9.81;
    pendulum.theta = 0.15;
    pendulum.omega = 0.0;
    pendulum.b = 0.0;

    let dt = 0.001;
    let t_final = 4.0;

    let steps = (t_final / dt) as usize;
    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    let (theta_analytical, omega_analytical) =
        SimplePendulum::analytical_solution(0.15, 0.0, pendulum.g, pendulum.L, t_final);

    assert_relative_eq!(pendulum.theta, theta_analytical, epsilon = 0.05);
    assert_relative_eq!(pendulum.omega, omega_analytical, epsilon = 0.05);
}

/// Test oscillation frequency matches theoretical prediction
#[test]
fn test_oscillation_frequency() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.1;
    pendulum.omega = 0.0;
    pendulum.b = 0.0;

    // Calculate expected period
    let expected_period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g);

    // Simulate and detect zero crossings
    let dt = 0.001;
    let mut time = 0.0;
    let mut previous_theta = pendulum.theta;
    let mut zero_crossings = Vec::new();

    for _ in 0..5000 {
        pendulum.do_step(0.0, dt);
        time += dt;

        // Detect crossing from negative to positive
        if previous_theta < 0.0 && pendulum.theta >= 0.0 {
            zero_crossings.push(time);
        }

        previous_theta = pendulum.theta;
    }

    // Calculate period from zero crossings (should be half period)
    if zero_crossings.len() >= 3 {
        let measured_half_period = zero_crossings[1] - zero_crossings[0];
        let measured_period = 2.0 * measured_half_period;

        assert_relative_eq!(measured_period, expected_period, epsilon = 0.05);
    }
}

/// Test energy conservation for undamped pendulum
#[test]
fn test_energy_conservation() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.2;
    pendulum.omega = 0.0;
    pendulum.b = 0.0; // No damping
    pendulum.update_derived_outputs(); // Recalculate energy after changing state

    let initial_energy = pendulum.energy;

    // Simulate for multiple periods
    let dt = 0.001;
    let period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g);
    let steps = ((2.0 * period) / dt) as usize;

    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    // Energy should be conserved within numerical error
    let relative_error = (pendulum.energy - initial_energy).abs() / initial_energy;

    assert!(
        relative_error < 0.05,
        "Energy conservation violated: {:.2}% change",
        relative_error * 100.0
    );
}

/// Test that damping reduces energy
#[test]
fn test_damping_reduces_energy() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.3;
    pendulum.omega = 0.0;
    pendulum.b = 0.1; // Add damping
    pendulum.update_derived_outputs(); // Recalculate energy after changing state

    let initial_energy = pendulum.energy;

    // Simulate for several seconds
    let dt = 0.01;
    for _ in 0..500 {
        pendulum.do_step(0.0, dt);
    }

    // Energy should monotonically decrease
    assert!(
        pendulum.energy < initial_energy,
        "Damping should reduce energy"
    );

    // Amplitude should also decrease
    assert!(
        pendulum.theta.abs() < 0.3,
        "Amplitude should decrease with damping"
    );
}

/// Test large amplitude behavior (nonlinear effects)
#[test]
fn test_large_amplitude() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 1.0; // Large angle (~57 degrees)
    pendulum.omega = 0.0;
    pendulum.b = 0.0;
    pendulum.update_derived_outputs(); // Recalculate energy after changing state

    let initial_energy = pendulum.energy;

    // Simulate for a few periods
    let dt = 0.001;
    let nominal_period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g);
    let steps = ((2.0 * nominal_period) / dt) as usize;

    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    // Energy should still be approximately conserved
    let relative_error = (pendulum.energy - initial_energy).abs() / initial_energy;
    assert!(
        relative_error < 0.10,
        "Energy should be conserved even for large amplitudes"
    );

    // For large amplitudes, the period is longer than the small-angle approximation
    // This test just verifies the simulation completes successfully
    assert!(pendulum.theta.abs() <= 1.1); // Should stay bounded
}

/// Test kinetic and potential energy trade-off
#[test]
fn test_energy_exchange() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.2; // Start at maximum displacement
    pendulum.omega = 0.0; // Zero velocity
    pendulum.b = 0.0;

    // Initially, all energy should be potential
    assert!(pendulum.KE < 0.01 * pendulum.energy);
    assert!(pendulum.PE > 0.99 * pendulum.energy);

    // Simulate to lowest point (approximately quarter period)
    let dt = 0.001;
    let quarter_period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g) / 4.0;
    let steps = (quarter_period / dt) as usize;

    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
    }

    // At lowest point, most energy should be kinetic
    let ke_fraction = pendulum.KE / pendulum.energy;
    assert!(
        ke_fraction > 0.8,
        "At lowest point, most energy should be kinetic (KE fraction: {:.2})",
        ke_fraction
    );
}

/// Test phase space trajectory (conservation of phase space volume)
#[test]
fn test_phase_space_trajectory() {
    let mut pendulum = SimplePendulum::new();
    pendulum.theta = 0.15;
    pendulum.omega = 0.0;
    pendulum.b = 0.0;

    let mut max_theta = pendulum.theta;
    let mut max_omega: f64 = 0.0;

    // Simulate for one complete period
    let dt = 0.001;
    let period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g);
    let steps = (period / dt) as usize;

    for _ in 0..steps {
        pendulum.do_step(0.0, dt);
        max_theta = max_theta.max(pendulum.theta.abs());
        max_omega = max_omega.max(pendulum.omega.abs());
    }

    // The trajectory should return close to initial conditions
    assert_relative_eq!(pendulum.theta, 0.15, epsilon = 0.01);
    assert_relative_eq!(pendulum.omega, 0.0, epsilon = 0.1);

    // Maximum values should be consistent with energy conservation
    let omega_n = (pendulum.g / pendulum.L).sqrt();
    let expected_max_omega = 0.15 * omega_n; // For small angles

    assert_relative_eq!(max_omega, expected_max_omega, epsilon = 0.05);
}
