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

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{
    CSDoStepResult, Context, DefaultLoggingCategory, ModelGetSetStates, UserModel,
};
use fmi_export::FmuModel;

/// Bouncing Ball FMU model
///
/// Simulates a ball bouncing under gravity with energy dissipation on impact.
/// The ball stops bouncing when the velocity becomes too small.
#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct BouncingBall {
    /// Gravitational acceleration (m/s²), typically negative
    #[variable(causality = Parameter, start = -9.81, initial = Exact)]
    pub g: f64,

    /// Coefficient of restitution (0 < e < 1)
    #[variable(causality = Parameter, start = 0.7, initial = Exact)]
    pub e: f64,

    /// Height above ground (m)
    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub h: f64,

    /// Vertical velocity (m/s)
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub v: f64,

    /// Derivative of h
    #[variable(causality = Local, derivative = h, initial = Calculated)]
    der_h: f64,

    /// Derivative of v
    #[variable(causality = Local, derivative = v, initial = Calculated)]
    der_v: f64,

    /// Minimum velocity threshold below which the ball stops
    #[variable(skip)]
    v_min: f64,

    /// Whether the ball has stopped bouncing (at rest on ground)
    #[variable(skip)]
    stopped: bool,
}

impl UserModel for BouncingBall {
    type LoggingCategory = DefaultLoggingCategory;

    fn configurate(&mut self, _context: &dyn Context<Self>) -> Result<(), Fmi3Error> {
        // Set the minimum velocity threshold
        self.v_min = 0.1;
        Ok(())
    }

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        self.der_h = self.v;
        self.der_v = self.g;
        Ok(Fmi3Res::OK)
    }

    fn do_step(
        &mut self,
        context: &mut dyn Context<Self>,
        current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Result<CSDoStepResult, Fmi3Error> {
        const FIXED_STEP: f64 = 0.001;

        let t_end = current_communication_point + communication_step_size;
        let mut t = current_communication_point;

        let mut x = vec![0.0; Self::NUM_STATES];
        let mut dx = vec![0.0; Self::NUM_STATES];

        while t_end - t > f64::EPSILON * t_end.abs().max(1.0) {
            if self.stopped {
                break;
            }

            let dt = (t_end - t).min(FIXED_STEP);

            // Handle collision event before integration
            if self.h <= 0.0 && self.v < 0.0 {
                self.h = f64::MIN_POSITIVE;
                self.v = -self.v * self.e;

                if self.v < self.v_min {
                    self.v = 0.0;
                    self.h = 0.0;
                    self.stopped = true;
                    break;
                }
            }

            // Forward Euler micro-step
            self.calculate_values(context)?;
            self.get_continuous_states(&mut x)?;
            self.get_continuous_state_derivatives(&mut dx)?;

            for i in 0..Self::NUM_STATES {
                x[i] += dx[i] * dt;
            }

            self.set_continuous_states(&x)?;
            t += dt;
            context.set_time(t);

            // Snap to ground if ball crossed during step
            if self.h < 0.0 {
                self.h = 0.0;
            }
        }

        context.set_time(t_end);
        Ok(CSDoStepResult::completed(t_end))
    }
}

fmi_export::export_fmu!(BouncingBall);

impl BouncingBall {
    /// Create a new bouncing ball with default parameters
    pub fn new() -> Self {
        Self {
            g: -9.81,
            e: 0.7,
            h: 1.0,
            v: 0.0,
            der_h: 0.0,
            der_v: -9.81,
            v_min: 0.1,
            stopped: false,
        }
    }

    /// Check if the ball has stopped bouncing
    pub fn is_stopped(&self) -> bool {
        self.stopped
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

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // If ball has stopped, no dynamics
        if self.stopped {
            return;
        }

        // Check for collision at start of step (before integration)
        if self.h <= 0.0 && self.v < 0.0 {
            self.h = f64::MIN_POSITIVE;
            self.v = -self.v * self.e;

            if self.v < self.v_min {
                self.v = 0.0;
                self.h = 0.0;
                self.stopped = true;
            }

            return;
        }

        // Calculate derivatives
        let der_h = self.v;
        let der_v = self.g;

        // Euler integration
        self.h += der_h * time_step;
        self.v += der_v * time_step;

        if self.h < 0.0 {
            self.h = 0.0;
        }
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
        let original_g = model.g;
        model.h = 0.0;
        model.v = -0.05; // Below v_min = 0.1

        model.do_step(0.0, 0.01);

        // Ball should have stopped
        assert_eq!(model.v, 0.0);
        assert_eq!(model.h, 0.0);
        assert!(model.is_stopped()); // Ball is at rest
        assert_eq!(model.g, original_g); // Gravity parameter unchanged
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
