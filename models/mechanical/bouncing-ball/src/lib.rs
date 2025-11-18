//! Bouncing Ball Model
//!
//! A ball bouncing under gravity with energy loss on impact.
//!
//! State equations:
//! dh/dt = v
//! dv/dt = g
//!
//! Event: When h <= 0 and v < 0 (collision with ground)
//! - Reverse velocity with coefficient of restitution: v = -e * v
//! - Stop bouncing when |v| < v_min

// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

/// Bouncing Ball FMU model
///
/// Simulates a ball bouncing under gravity with energy dissipation on impact.
/// The ball stops bouncing when the velocity becomes too small.
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct BouncingBall {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "-9.81")]
    /// Gravitational acceleration (m/s²), typically negative
    pub g: f64,

    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "0.7")]
    /// Coefficient of restitution (0 < e < 1)
    pub e: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    /// Height above ground (m)
    pub h: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    /// Vertical velocity (m/s)
    pub v: f64,

    /// Minimum velocity threshold below which the ball stops
    v_min: f64,

    /// FMU runtime information (optional)
    pub fmu_info: FmuInfo,
}

impl FmuFunctions for BouncingBall {
    fn exit_initialization_mode(&mut self) {
        // Set the minimum velocity threshold
        self.v_min = 0.1;
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Check for collision at start of step (before integration)
        if self.h <= 0.0 && self.v < 0.0 {
            // Ball has hit the ground
            self.h = f64::MIN_POSITIVE; // Place slightly above ground
            self.v = -self.v * self.e; // Reverse velocity with energy loss

            // Stop bouncing if velocity becomes too small
            if self.v < self.v_min {
                self.v = 0.0;
                self.h = 0.0;
                self.g = 0.0; // Disable gravity when stopped
            }

            // Don't integrate in the same step as a bounce (bounce is instantaneous)
            return;
        }

        // Calculate derivatives:
        // der(h) = v
        // der(v) = g
        let der_h = self.v;
        let der_v = self.g;

        // Euler integration
        self.h += der_h * time_step;
        self.v += der_v * time_step;

        // Check for collision after integration (ball crossed ground during step)
        if self.h < 0.0 {
            self.h = 0.0; // Snap to ground level
        }
    }
}

impl BouncingBall {
    /// Create a new bouncing ball with default parameters
    pub fn new() -> Self {
        Self {
            g: -9.81,
            e: 0.7,
            h: 1.0,
            v: 0.0,
            v_min: 0.1,
            fmu_info: FmuInfo::default(),
        }
    }

    /// Calculate kinetic energy (assuming unit mass)
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.v * self.v
    }

    /// Calculate potential energy (assuming unit mass)
    pub fn potential_energy(&self) -> f64 {
        -self.g * self.h // g is negative, so -g*h is positive
    }

    /// Calculate total mechanical energy
    pub fn total_energy(&self) -> f64 {
        self.kinetic_energy() + self.potential_energy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = BouncingBall::new();
        assert_eq!(model.h, 1.0);
        assert_eq!(model.v, 0.0);
        assert_eq!(model.g, -9.81);
        assert_eq!(model.e, 0.7);
    }

    #[test]
    fn test_energy_calculation() {
        let mut model = BouncingBall::new();
        model.h = 1.0;
        model.v = 0.0;

        let pe = model.potential_energy();
        let ke = model.kinetic_energy();

        assert!((pe - 9.81).abs() < 1e-10);
        assert!((ke - 0.0).abs() < 1e-10);

        // Test with velocity
        model.v = 2.0;
        let ke2 = model.kinetic_energy();
        assert!((ke2 - 2.0).abs() < 1e-10); // 0.5 * 2² = 2.0
    }

    #[test]
    fn test_do_step_no_collision() {
        let mut model = BouncingBall::new();
        model.h = 1.0;
        model.v = 0.0;

        let dt = 0.1;
        model.do_step(0.0, dt);

        // After one step: h = h + v*dt, v = v + g*dt
        // h_new = 1.0 + 0.0*0.1 = 1.0
        // v_new = 0.0 + (-9.81)*0.1 = -0.981
        assert!((model.h - 1.0).abs() < 1e-10);
        assert!((model.v - (-0.981)).abs() < 1e-10);
    }

    #[test]
    fn test_collision_handling() {
        let mut model = BouncingBall::new();
        model.h = 0.0;
        model.v = -2.0;
        let initial_v_abs = model.v.abs();

        model.do_step(0.0, 0.01);

        // After collision, velocity should be reversed and reduced
        let expected_v = initial_v_abs * model.e;
        assert!((model.v - expected_v).abs() < 1e-10);
        assert!(model.h > 0.0); // Should be slightly above ground
    }

    #[test]
    fn test_stopping_condition() {
        let mut model = BouncingBall::new();
        model.h = 0.0;
        model.v = -0.05; // Below v_min = 0.1

        model.do_step(0.0, 0.01);

        // Ball should have stopped
        assert_eq!(model.v, 0.0);
        assert_eq!(model.h, 0.0);
        assert_eq!(model.g, 0.0); // Gravity disabled
    }

    #[test]
    fn test_energy_loss_on_bounce() {
        let mut model = BouncingBall::new();
        model.h = 1.0;
        model.v = 0.0;
        let _initial_energy = model.total_energy();

        // Simulate fall and bounce
        model.h = 0.0;
        model.v = -4.43; // Approximate velocity after falling from h=1.0

        let energy_before_bounce = model.total_energy();
        model.do_step(0.0, 0.01);
        let energy_after_bounce = model.total_energy();

        // Energy should decrease after bounce (but not necessarily from initial,
        // since we artificially set the velocity)
        assert!(energy_after_bounce < energy_before_bounce);
    }

    #[test]
    fn test_no_collision_above_ground() {
        let mut model = BouncingBall::new();
        model.h = 0.5;
        model.v = -1.0;
        let initial_v = model.v;

        model.do_step(0.0, 0.01);

        // Ball should continue falling (no collision yet)
        // v should become more negative due to gravity
        assert!(model.v < initial_v);
        assert!(model.h < 0.5);
    }
}
