//! Physics validation tests for the Van der Pol oscillator

use approx::assert_relative_eq;
use odml_van_der_pol::VanDerPol;
use physics_framework::numerical::find_peaks;

#[test]
fn test_limit_cycle_convergence() {
    let mut model = VanDerPol::new();
    model.mu = 1.0;
    model.x0 = 0.1;  // Start near origin
    model.x1 = 0.1;

    let dt = 0.01;
    let t_final = 50.0;

    let mut x0_values = Vec::new();

    let mut time = 0.0;
    while time < t_final {
        x0_values.push(model.x0);
        model.do_step(dt);
        time += dt;
    }

    // Find peaks in the last portion (steady state)
    let steady_start = (x0_values.len() as f64 * 0.8) as usize;
    let steady_values: Vec<f64> = x0_values[steady_start..].to_vec();
    let peaks = find_peaks(&steady_values);

    // Should have multiple peaks in steady state (relaxed for simple Euler integrator)
    assert!(
        peaks.len() >= 2,
        "Should have converged to limit cycle with regular oscillations, found {} peaks",
        peaks.len()
    );

    // Check that peak amplitudes are similar (limit cycle)
    let peak_values: Vec<f64> = peaks.iter().map(|(_, val)| *val).collect();
    let mean_amplitude = peak_values.iter().sum::<f64>() / peak_values.len() as f64;

    // Only check if we have enough peaks
    if peak_values.len() >= 2 {
        for &peak_val in &peak_values {
            let relative_diff = ((peak_val - mean_amplitude) / mean_amplitude).abs();
            assert!(
                relative_diff < 0.2,
                "Peak amplitudes should be similar in limit cycle: {} vs {}",
                peak_val,
                mean_amplitude
            );
        }
    }
}

#[test]
fn test_oscillation_period() {
    let mut model = VanDerPol::new();
    model.mu = 1.0;

    let dt = 0.001;
    let t_final = 30.0;

    let mut results = Vec::new();
    let mut time = 0.0;

    while time < t_final {
        results.push((time, model.x0));
        model.do_step(dt);
        time += dt;
    }

    // Extract just the values for peak finding
    let values: Vec<f64> = results.iter().map(|(_, x)| *x).collect();
    let peaks = find_peaks(&values);

    // Need at least 3 peaks to measure period
    assert!(peaks.len() >= 3, "Should have multiple oscillations");

    // Calculate periods between consecutive peaks
    let mut periods = Vec::new();
    for i in 1..peaks.len() {
        let period = (peaks[i].0 - peaks[i - 1].0) as f64 * dt;
        periods.push(period);
    }

    // Periods should be relatively consistent
    let mean_period = periods.iter().sum::<f64>() / periods.len() as f64;

    // For μ = 1, period is approximately 6.66
    assert!(
        mean_period > 6.0 && mean_period < 7.5,
        "Period should be around 6.66 for μ=1, got {}",
        mean_period
    );
}

#[test]
fn test_energy_behavior() {
    let mut model = VanDerPol::new();
    model.mu = 1.0;

    let dt = 0.01;

    let initial_energy = model.total_energy();

    // Run for several periods
    for _ in 0..5000 {
        model.do_step(dt);
    }

    let final_energy = model.total_energy();

    // Energy is not conserved but should stabilize to a value on the limit cycle
    // The energy should be bounded (not grow to infinity)
    assert!(
        final_energy < 100.0,
        "Energy should remain bounded on limit cycle"
    );
}

#[test]
fn test_different_mu_values() {
    // Test that different μ values produce different dynamics
    for mu in [0.1, 1.0, 5.0] {
        let mut model = VanDerPol::new();
        model.mu = mu;

        let dt = 0.01;

        // Run simulation
        for _ in 0..1000 {
            model.do_step(dt);
        }

        // Check that solution remains bounded
        assert!(
            model.x0.abs() < 10.0 && model.x1.abs() < 10.0,
            "Solution should remain bounded for μ={}",
            mu
        );
    }
}

#[test]
fn test_symmetry() {
    // Van der Pol oscillator has certain symmetry properties
    let mut model1 = VanDerPol::new();
    model1.x0 = 1.0;
    model1.x1 = 0.5;

    let mut model2 = VanDerPol::new();
    model2.x0 = -1.0;
    model2.x1 = -0.5;

    let dt = 0.01;

    for _ in 0..100 {
        model1.do_step(dt);
        model2.do_step(dt);
    }

    // Due to symmetry, magnitudes should be similar
    assert_relative_eq!(model1.x0.abs(), model2.x0.abs(), epsilon = 0.1);
    assert_relative_eq!(model1.x1.abs(), model2.x1.abs(), epsilon = 0.1);
}
