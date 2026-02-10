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
use fmi::EventFlags;
use fmi_export::fmi3::{Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

/// Hysteresis epsilon for event indicator near zero (matches Reference-FMUs)
const EVENT_EPSILON: f64 = 1e-10;

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

    /// Event indicator for ground collision (positive above ground, negative below).
    /// This field is used by the FmuModel derive macro to set MAX_EVENT_INDICATORS = 1.
    #[variable(event_indicator = true, skip)]
    #[allow(dead_code)]
    event_indicator_h: f64,
}

impl UserModel for BouncingBall {
    type LoggingCategory = DefaultLoggingCategory;

    fn configurate(&mut self, _context: &dyn Context<Self>) -> Result<(), Fmi3Error> {
        // Set the minimum velocity threshold
        self.v_min = 0.1;
        Ok(())
    }

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        if self.stopped {
            self.der_h = 0.0;
            self.der_v = 0.0;
        } else {
            self.der_h = self.v;
            self.der_v = self.g;
        }
        Ok(Fmi3Res::OK)
    }

    fn get_event_indicators(
        &mut self,
        _context: &dyn Context<Self>,
        indicators: &mut [f64],
    ) -> Result<bool, Fmi3Error> {
        // Hysteresis: when h is near zero and ball is moving up,
        // keep indicator negative to prevent retriggering
        if self.h > -EVENT_EPSILON && self.h <= 0.0 && self.v > 0.0 {
            indicators[0] = -EVENT_EPSILON;
        } else {
            indicators[0] = self.h;
        }
        Ok(true)
    }

    fn event_update(
        &mut self,
        _context: &dyn Context<Self>,
        event_flags: &mut EventFlags,
    ) -> Result<Fmi3Res, Fmi3Error> {
        event_flags.reset();

        if self.h <= 0.0 && self.v < 0.0 {
            self.h = f64::MIN_POSITIVE;
            self.v = -self.v * self.e;

            if self.v < self.v_min {
                self.v = 0.0;
                self.h = 0.0;
                self.stopped = true;
            }

            event_flags.values_of_continuous_states_changed = true;
        }

        Ok(Fmi3Res::OK)
    }

    adml_solver::euler_cs_step_with_events!(0.001);
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
            event_indicator_h: 0.0,
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

    /// Compute the event indicator value (positive above ground)
    fn event_indicator(&self) -> f64 {
        if self.h > -EVENT_EPSILON && self.h <= 0.0 && self.v > 0.0 {
            -EVENT_EPSILON
        } else {
            self.h
        }
    }

    /// Handle a collision event (reverses velocity with restitution)
    fn handle_collision(&mut self) {
        if self.h <= 0.0 && self.v < 0.0 {
            self.h = f64::MIN_POSITIVE;
            self.v = -self.v * self.e;

            if self.v < self.v_min {
                self.v = 0.0;
                self.h = 0.0;
                self.stopped = true;
            }
        }
    }

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        if self.stopped {
            return;
        }

        let prev_z = self.event_indicator();

        // Calculate derivatives and integrate
        let der_h = self.v;
        let der_v = self.g;
        self.h += der_h * time_step;
        self.v += der_v * time_step;

        // Check for zero-crossing event
        let z = self.event_indicator();
        if prev_z * z < 0.0 {
            self.handle_collision();
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
        // Start above ground with downward velocity — ball will cross ground during step
        model.h = 0.01;
        model.v = -2.0;

        model.do_step(0.0, 0.01);

        // After collision, velocity should be positive (bounced) and reduced by e
        assert!(model.v > 0.0);
        assert!(model.h > 0.0);
    }

    #[test]
    fn test_stopping_condition() {
        let mut model = BouncingBall::new();
        let original_g = model.g;
        // Start just above ground with small downward velocity.
        // After integration: h crosses zero and v is small enough that
        // the bounced velocity (|v| * e) < v_min.
        model.h = 0.0002;
        model.v = -0.03;

        model.do_step(0.0, 0.01);

        // Ball should have stopped after the zero-crossing event
        assert_eq!(model.v, 0.0);
        assert_eq!(model.h, 0.0);
        assert!(model.is_stopped());
        assert_eq!(model.g, original_g);
    }

    #[test]
    fn test_energy_loss_on_bounce() {
        let mut model = BouncingBall::new();
        // Start just above ground with high downward velocity
        model.h = 0.01;
        model.v = -4.43;

        let energy_before_bounce = model.total_energy();
        model.do_step(0.0, 0.01);
        let energy_after_bounce = model.total_energy();

        // Energy should decrease after bounce
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

    #[test]
    fn test_event_indicator_positive_above_ground() {
        let model = BouncingBall::new();
        // h = 1.0, ball is above ground — indicator should be positive
        assert!(model.event_indicator() > 0.0);
    }

    #[test]
    fn test_event_indicator_negative_below_ground() {
        let mut model = BouncingBall::new();
        model.h = -0.1;
        model.v = -1.0;
        // Ball is below ground — indicator should be negative
        assert!(model.event_indicator() < 0.0);
    }

    #[test]
    fn test_event_indicator_hysteresis() {
        let mut model = BouncingBall::new();
        // h near zero, ball moving up — hysteresis should keep indicator negative
        model.h = 0.0;
        model.v = 1.0;
        assert!(model.event_indicator() < 0.0);
        assert!((model.event_indicator() - (-EVENT_EPSILON)).abs() < 1e-20);
    }

    #[test]
    fn test_handle_collision_reverses_velocity() {
        let mut model = BouncingBall::new();
        model.h = 0.0;
        model.v = -2.0;

        model.handle_collision();

        let expected_v = 2.0 * model.e;
        assert!((model.v - expected_v).abs() < 1e-10);
        assert!(model.h > 0.0);
        assert!(!model.is_stopped());
    }

    #[test]
    fn test_handle_collision_stops_at_low_velocity() {
        let mut model = BouncingBall::new();
        model.h = 0.0;
        model.v = -0.05; // Below v_min

        model.handle_collision();

        assert_eq!(model.v, 0.0);
        assert_eq!(model.h, 0.0);
        assert!(model.is_stopped());
    }
}
