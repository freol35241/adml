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

use fmi::{
    EventFlags,
    fmi3::{Fmi3Error, Fmi3Res},
};
use fmi_export::{
    FmuModel,
    fmi3::{DefaultLoggingCategory, ModelContext, UserModel},
};

/// BouncingBall FMU model that can be exported as a complete FMU
#[derive(FmuModel, Default, Debug)]
#[model()]
pub struct BouncingBall {
    /// Height above ground (m)
    #[variable(causality = Output, state, event_indicator, start = 1.0, initial = Exact)]
    pub h: f64,

    /// Vertical velocity (m/s)
    /// Also serves as der(h)
    #[variable(causality = Output, state, start = 0.0, initial = Exact)]
    #[alias(name="der(h)", causality = Local, derivative = h, initial = Calculated)]
    pub v: f64,

    /// Gravitational acceleration (m/s²), typically negative
    /// Also serves as der(v)
    #[variable(causality = Parameter, start = -9.81, initial = Exact)]
    #[alias(name = "der(v)", causality = Local, derivative = v, initial = Calculated)]
    pub g: f64,

    /// Coefficient of restitution (0 < e < 1)
    #[variable(causality = Parameter, start = 0.7, initial = Exact)]
    pub e: f64,

    /// Minimum velocity threshold
    #[variable(causality = Local, start = 0.1, initial = Exact)]
    v_min: f64,
}

impl BouncingBall {
    /// Create a new bouncing ball with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate kinetic energy
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.v * self.v // Assuming unit mass
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

impl UserModel for BouncingBall {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &ModelContext<Self>) -> Result<Fmi3Res, Fmi3Error> {
        // Derivatives are handled by the FMI framework via aliases
        // der(h) = v and der(v) = g are specified in the variable attributes
        Ok(Fmi3Res::OK)
    }

    fn event_update(
        &mut self,
        context: &ModelContext<Self>,
        event_flags: &mut EventFlags,
    ) -> Result<Fmi3Res, Fmi3Error> {
        // Handle ball bouncing off the ground
        if self.h <= 0.0 && self.v < 0.0 {
            context.log(
                Fmi3Res::OK,
                Self::LoggingCategory::default(),
                format_args!("Ball bounced! h={:.3}, v={:.3}", self.h, self.v),
            );

            self.h = f64::MIN_POSITIVE; // Slightly above ground
            self.v = -self.v * self.e; // Reverse velocity with energy loss

            // Stop bouncing if velocity becomes too small
            if self.v < self.v_min {
                context.log(
                    Fmi3Res::OK,
                    Self::LoggingCategory::default(),
                    format_args!("Ball stopped bouncing"),
                );
                self.v = 0.0;
                self.g = 0.0; // Disable gravity when stopped
            }

            event_flags.values_of_continuous_states_changed = true;
        } else {
            event_flags.values_of_continuous_states_changed = false;
        }

        Ok(Fmi3Res::OK)
    }

    fn get_event_indicators(
        &mut self,
        _context: &ModelContext<Self>,
        indicators: &mut [f64],
    ) -> Result<bool, Fmi3Error> {
        assert!(!indicators.is_empty());
        // Event indicator for ground contact
        indicators[0] = if self.h == 0.0 && self.v == 0.0 {
            1.0 // Special case: stopped ball
        } else {
            self.h // Height as event indicator
        };
        Ok(true)
    }
}

// Export the FMU with full C API
fmi_export::export_fmu!(BouncingBall);

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
    }

    #[test]
    fn test_event_detection() {
        use fmi::EventFlags;

        let mut model = BouncingBall::new();
        let context = ModelContext::default();
        let mut event_flags = EventFlags::default();

        // No collision when above ground
        model.h = 1.0;
        model.v = -1.0;
        model.event_update(&context, &mut event_flags).unwrap();
        assert!(!event_flags.values_of_continuous_states_changed);

        // Collision when at ground and moving down
        model.h = 0.0;
        model.v = -1.0;
        model.event_update(&context, &mut event_flags).unwrap();
        assert!(event_flags.values_of_continuous_states_changed);
        assert!(model.v > 0.0); // Velocity should be reversed
    }

    #[test]
    fn test_event_indicators() {
        let mut model = BouncingBall::new();
        let context = ModelContext::default();
        let mut indicators = vec![0.0];

        model.h = 1.0;
        model.get_event_indicators(&context, &mut indicators).unwrap();
        assert_eq!(indicators[0], 1.0);
    }
}
