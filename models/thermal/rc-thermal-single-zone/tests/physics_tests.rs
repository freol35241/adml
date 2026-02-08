//! Physics validation tests for RcThermalSingleZone
//!
//! These tests verify physical correctness of the thermal RC model by comparing
//! simulation results against analytical solutions, energy balance, and known
//! thermal properties.

use adml_rc_thermal_single_zone::RcThermalSingleZone;
use approx::assert_relative_eq;

// === Analytical Solution Tests ===

#[test]
fn test_analytical_solution_default_parameters() {
    // Verify simulation matches analytical solution with default parameters
    let mut model = RcThermalSingleZone::new();

    // Simulation parameters
    let dt = 0.01; // Small step for Euler accuracy
    let t_final = 10000.0; // 10,000 seconds (~2.8 hours)
    let steps = (t_final / dt) as usize;

    // Run simulation
    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // Compare to analytical solution
    let expected = RcThermalSingleZone::analytical_solution(
        model.R,
        model.C,
        model.T_outdoor,
        model.Q_heating,
        20.0, // Initial temperature
        t_final,
    );

    assert_relative_eq!(
        model.T_indoor,
        expected,
        epsilon = 0.01 // 1% tolerance for Euler integration
    );
}

#[test]
fn test_analytical_solution_heating_from_cold() {
    // Test heating up from cold start (T_indoor < T_steady_state)
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 0.0; // Start at ambient temperature
    model.T_outdoor = 0.0;
    model.Q_heating = 5000.0;

    let dt = 0.01;
    let t_final = 50000.0; // 50,000 seconds
    let steps = (t_final / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    let expected = RcThermalSingleZone::analytical_solution(
        model.R, model.C, 0.0, 5000.0, 0.0, // Initial at ambient
        t_final,
    );

    assert_relative_eq!(model.T_indoor, expected, epsilon = 0.01);
}

#[test]
fn test_analytical_solution_cooling_down() {
    // Test cooling down without heating (T_indoor > T_steady_state)
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 50.0; // Start warm
    model.T_outdoor = 0.0;
    model.Q_heating = 0.0; // No heating

    let dt = 0.01;
    let t_final = 50000.0;
    let steps = (t_final / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    let expected = RcThermalSingleZone::analytical_solution(
        model.R, model.C, 0.0, 0.0,  // No heating
        50.0, // Initial warm
        t_final,
    );

    assert_relative_eq!(model.T_indoor, expected, epsilon = 0.01);
}

#[test]
fn test_analytical_solution_with_warm_ambient() {
    // Test with non-zero ambient temperature
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 15.0;
    model.T_outdoor = 10.0;
    model.Q_heating = 3000.0;

    let dt = 0.01;
    let t_final = 30000.0;
    let steps = (t_final / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    let expected =
        RcThermalSingleZone::analytical_solution(model.R, model.C, 10.0, 3000.0, 15.0, t_final);

    assert_relative_eq!(model.T_indoor, expected, epsilon = 0.01);
}

// === Steady-State Tests ===

#[test]
fn test_reaches_steady_state() {
    // Verify system reaches steady state after sufficient time
    let mut model = RcThermalSingleZone::new();

    let expected_ss = model.steady_state_temperature();

    // Simulate for 5 time constants (> 99% of final value)
    let tau = model.time_constant();
    let t_final = 5.0 * tau;
    let dt = 0.01;
    let steps = (t_final / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // Should be very close to steady state (within 0.5% after 5 time constants)
    assert_relative_eq!(model.T_indoor, expected_ss, epsilon = 0.25);

    // dT/dt should be nearly zero (< 0.0001 K/s)
    assert!(model.der_T_indoor.abs() < 1e-4);
}

#[test]
fn test_steady_state_energy_balance() {
    // At steady state, heat input should equal heat loss
    let mut model = RcThermalSingleZone::new();

    // Set to steady state
    model.T_indoor = model.steady_state_temperature();

    // Take one step (should stay at steady state)
    model.do_step(0.0, 0.1);

    // Q_heating should equal Q_envelope (energy balance)
    assert_relative_eq!(model.Q_heating, model.Q_envelope, epsilon = 0.1);

    // Temperature change should be negligible
    assert_relative_eq!(model.der_T_indoor, 0.0, epsilon = 1e-6);
}

// === Time Constant Tests ===

#[test]
fn test_time_constant_63_percent_response() {
    // After one time constant, system should reach ~63.2% of final value
    let mut model = RcThermalSingleZone::new();

    let t_initial = 20.0;
    let t_ss = model.steady_state_temperature(); // 50.0°C with defaults

    model.T_indoor = t_initial;

    // Simulate for exactly one time constant
    let tau = model.time_constant();
    let dt = 0.01;
    let steps = (tau / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // After τ, should reach 63.2% of the way from initial to steady state
    let expected_change = 0.632 * (t_ss - t_initial);
    let actual_change = model.T_indoor - t_initial;

    assert_relative_eq!(actual_change, expected_change, epsilon = 0.02);
}

#[test]
fn test_time_constant_affects_response_speed() {
    // Larger time constant should result in slower fractional response
    let mut model1 = RcThermalSingleZone::new();
    let mut model2 = RcThermalSingleZone::new();

    model1.R = 0.01;
    model1.C = 10_000_000.0;
    model1.T_indoor = 20.0;
    model1.Q_heating = 5000.0;

    model2.R = 0.01; // Same R
    model2.C = 20_000_000.0; // Double capacitance = double time constant
    model2.T_indoor = 20.0;
    model2.Q_heating = 5000.0;

    let t_ss1 = model1.steady_state_temperature();
    let t_ss2 = model2.steady_state_temperature();

    // Simulate both for same time
    let dt = 0.01;
    let t_final = 50000.0;
    let steps = (t_final / dt) as usize;

    for _ in 0..steps {
        model1.do_step(0.0, dt);
        model2.do_step(0.0, dt);
    }

    // Calculate fractional progress toward steady state
    let progress1 = (model1.T_indoor - 20.0) / (t_ss1 - 20.0);
    let progress2 = (model2.T_indoor - 20.0) / (t_ss2 - 20.0);

    assert!(
        progress1 > progress2,
        "Faster system (smaller tau) should have made more fractional progress"
    );
}

// === Energy Balance Tests ===

#[test]
fn test_energy_balance_heating() {
    // Energy added by heating should equal energy stored + energy lost
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 20.0;
    model.T_outdoor = 0.0;
    model.Q_heating = 5000.0;

    let initial_energy = model.stored_energy();

    let dt = 1.0; // 1 second
    let q_heat_integrated = model.Q_heating * dt;

    // Average Q_envelope over the time step (approximate)
    let q_loss_avg = model.Q_envelope * dt;

    model.do_step(0.0, dt);

    let final_energy = model.stored_energy();
    let energy_change = final_energy - initial_energy;

    // Energy balance: ΔE = Q_heating*dt - Q_envelope*dt
    let expected_energy_change = q_heat_integrated - q_loss_avg;

    // Allow some tolerance due to averaging and Euler method
    assert_relative_eq!(energy_change, expected_energy_change, epsilon = 100.0);
}

#[test]
fn test_no_heating_loses_energy() {
    // Without heating, stored energy should decrease (if above ambient)
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 30.0;
    model.T_outdoor = 0.0;
    model.Q_heating = 0.0; // No heating

    let initial_energy = model.stored_energy();

    for _ in 0..1000 {
        model.do_step(0.0, 0.1);
    }

    let final_energy = model.stored_energy();

    assert!(
        final_energy < initial_energy,
        "Energy should decrease without heating"
    );
}

// === Convergence Tests ===

#[test]
fn test_convergence_with_step_size() {
    // Verify solution converges as step size decreases
    fn simulate_until(dt: f64, t_final: f64) -> f64 {
        let mut model = RcThermalSingleZone::new();
        model.T_indoor = 20.0;

        let steps = (t_final / dt) as usize;
        for _ in 0..steps {
            model.do_step(0.0, dt);
        }
        model.T_indoor
    }

    let t_final = 10000.0;
    let result_coarse = simulate_until(1.0, t_final);
    let result_medium = simulate_until(0.1, t_final);
    let result_fine = simulate_until(0.01, t_final);

    // Get reference from analytical solution
    let reference =
        RcThermalSingleZone::analytical_solution(0.01, 10_000_000.0, 0.0, 5000.0, 20.0, t_final);

    // Error should decrease with step size
    let error_coarse = (result_coarse - reference).abs();
    let error_medium = (result_medium - reference).abs();
    let error_fine = (result_fine - reference).abs();

    assert!(
        error_medium < error_coarse,
        "Medium step should be more accurate than coarse"
    );
    assert!(
        error_fine < error_medium,
        "Fine step should be more accurate than medium"
    );
}

// === Physical Property Tests ===

#[test]
fn test_insulation_quality_affects_steady_state() {
    // Better insulation (higher R_th) should result in higher steady-state temperature
    let mut model_poor_insulation = RcThermalSingleZone::new();
    model_poor_insulation.R = 0.005; // Poor insulation

    let mut model_good_insulation = RcThermalSingleZone::new();
    model_good_insulation.R = 0.02; // Good insulation

    // Same heating power
    model_poor_insulation.Q_heating = 5000.0;
    model_good_insulation.Q_heating = 5000.0;

    let t_ss_poor = model_poor_insulation.steady_state_temperature();
    let t_ss_good = model_good_insulation.steady_state_temperature();

    assert!(
        t_ss_good > t_ss_poor,
        "Better insulation should give higher steady-state temp"
    );
}

#[test]
fn test_thermal_mass_affects_time_constant() {
    // Larger thermal mass should result in slower response
    let mut model_low_mass = RcThermalSingleZone::new();
    model_low_mass.R = 0.01;
    model_low_mass.C = 5_000_000.0;

    let mut model_high_mass = RcThermalSingleZone::new();
    model_high_mass.R = 0.01;
    model_high_mass.C = 20_000_000.0;

    let tau_low = model_low_mass.time_constant();
    let tau_high = model_high_mass.time_constant();

    assert!(
        tau_high > tau_low,
        "Higher thermal mass should have larger time constant"
    );
    assert_eq!(
        tau_high,
        4.0 * tau_low,
        "Time constant should scale linearly with C_th"
    );
}

// === Boundary Condition Tests ===

#[test]
fn test_zero_heating() {
    // With zero heating, temperature should approach ambient
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 30.0;
    model.T_outdoor = 10.0;
    model.Q_heating = 0.0;

    // Simulate for a long time
    let tau = model.time_constant();
    let dt = 0.01;
    let steps = ((5.0 * tau) / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // Should approach ambient temperature (within 0.2°C after 5 time constants)
    assert_relative_eq!(model.T_indoor, 10.0, epsilon = 0.2);
}

#[test]
fn test_massive_heating() {
    // Very large heating power should result in high temperature
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 20.0;
    model.Q_heating = 50_000.0; // 50 kW - massive heating

    // Simulate to steady state
    let tau = model.time_constant();
    let dt = 0.01;
    let steps = ((5.0 * tau) / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    let expected_ss = model.steady_state_temperature(); // 0 + 50000 * 0.01 = 500°C

    assert_relative_eq!(model.T_indoor, expected_ss, epsilon = 5.0);
    assert!(
        model.T_indoor > 400.0,
        "Massive heating should produce very high temperature"
    );
}

#[test]
fn test_negative_ambient_temperature() {
    // System should work correctly with negative ambient temperature (winter)
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 20.0;
    model.T_outdoor = -20.0; // Cold winter day
    model.Q_heating = 10_000.0; // Need more heating in winter

    let dt = 0.01;
    for _ in 0..10000 {
        model.do_step(0.0, dt);
    }

    // Should maintain reasonable indoor temperature
    assert!(model.T_indoor.is_finite());
    assert!(model.T_indoor > model.T_outdoor);

    // Heat loss should be positive (losing heat to cold outside)
    assert!(model.Q_envelope > 0.0);
}

#[test]
fn test_indoor_equals_ambient_no_heating() {
    // When indoor equals ambient with no heating, should stay constant
    let mut model = RcThermalSingleZone::new();
    model.T_indoor = 15.0;
    model.T_outdoor = 15.0;
    model.Q_heating = 0.0;

    let initial_t = model.T_indoor;

    model.do_step(0.0, 1.0);

    // No temperature difference and no heating = no change
    assert_relative_eq!(model.T_indoor, initial_t, epsilon = 1e-10);
    assert_relative_eq!(model.Q_envelope, 0.0, epsilon = 1e-10);
    assert_relative_eq!(model.der_T_indoor, 0.0, epsilon = 1e-10);
}

// === Edge Case Tests ===

#[test]
fn test_very_small_step() {
    // Verify model works with very small time steps
    let mut model = RcThermalSingleZone::new();

    model.do_step(0.0, 1e-6);

    assert!(model.T_indoor.is_finite());
}

#[test]
fn test_stability_over_long_simulation() {
    // Model should remain stable over very long simulation times
    let mut model = RcThermalSingleZone::new();

    let dt = 0.1;
    // Simulate for 100 time constants
    let steps = ((100.0 * model.time_constant()) / dt) as usize;

    for _ in 0..steps {
        model.do_step(0.0, dt);
    }

    // Should be finite and at steady state
    assert!(model.T_indoor.is_finite());
    assert_relative_eq!(
        model.T_indoor,
        model.steady_state_temperature(),
        epsilon = 0.01
    );
}

#[test]
fn test_different_initial_conditions_converge() {
    // Different initial conditions should converge to same steady state
    let mut model1 = RcThermalSingleZone::new();
    let mut model2 = RcThermalSingleZone::new();

    model1.T_indoor = 0.0; // Start cold
    model2.T_indoor = 100.0; // Start hot

    let tau = model1.time_constant();
    let dt = 0.01;
    let steps = ((10.0 * tau) / dt) as usize;

    for _ in 0..steps {
        model1.do_step(0.0, dt);
        model2.do_step(0.0, dt);
    }

    // Both should converge to same steady state
    assert_relative_eq!(model1.T_indoor, model2.T_indoor, epsilon = 0.1);
}
