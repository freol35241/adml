//! Dahlquist Test Equation
//!
//! A simple first-order ODE used to test numerical integrators:
//!
//! dx/dt = -k * x
//!
//! With k > 0, the analytical solution is: x(t) = x0 * exp(-k * t)
//!
//! This is a fundamental test case for ODE solvers, useful for studying
//! stability and accuracy of numerical integration methods.
//!
//! The Dahlquist test equation is particularly useful for analyzing stiff solvers.

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{CSDoStepResult, Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

/// Dahlquist test equation model
///
/// This implements a simple first-order linear ODE: der(x) = -k * x
#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct Dahlquist {
    /// Decay constant k
    #[variable(causality = Parameter, start = 1.0, initial = Exact)]
    pub k: f64,

    /// State variable x
    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub x: f64,

    /// Derivative of x (der_x = -k * x)
    #[variable(causality = Local, derivative = x, initial = Calculated)]
    der_x: f64,
}

impl UserModel for Dahlquist {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        self.der_x = -self.k * self.x;
        Ok(Fmi3Res::OK)
    }

    fn do_step(
        &mut self,
        context: &mut dyn Context<Self>,
        current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Result<CSDoStepResult, Fmi3Error> {
        // Calculate derivative
        self.der_x = -self.k * self.x;

        // Euler integration: x = x + dx/dt * dt
        self.x += self.der_x * communication_step_size;

        let target_time = current_communication_point + communication_step_size;
        context.set_time(target_time);
        Ok(CSDoStepResult::completed(target_time))
    }
}

fmi_export::export_fmu!(Dahlquist);

impl Dahlquist {
    /// Create a new Dahlquist model with default parameters
    pub fn new() -> Self {
        Self {
            k: 1.0,
            x: 1.0,
            der_x: -1.0,
        }
    }

    /// Get the analytical solution at time t from initial value x0
    pub fn analytical_solution(x0: f64, k: f64, t: f64) -> f64 {
        x0 * (-k * t).exp()
    }

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        self.der_x = -self.k * self.x;
        self.x += self.der_x * time_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = Dahlquist::new();
        assert_eq!(model.x, 1.0);
        assert_eq!(model.k, 1.0);
    }

    #[test]
    fn test_derivative_calculation() {
        let model = Dahlquist::new();

        // Manually calculate what one step should do
        let der_x = -model.k * model.x; // -1.0 * 1.0 = -1.0
        assert_eq!(der_x, -1.0);
    }

    #[test]
    fn test_analytical_solution() {
        assert_eq!(Dahlquist::analytical_solution(1.0, 1.0, 0.0), 1.0);

        let t1_value = Dahlquist::analytical_solution(1.0, 1.0, 1.0);
        let expected = (-1.0f64).exp();
        assert!((t1_value - expected).abs() < 1e-10);
    }

    #[test]
    fn test_do_step() {
        let mut model = Dahlquist::new();

        let initial_x = model.x;
        let dt = 0.1;

        model.do_step(0.0, dt);

        // After one Euler step: x_new = x + (-k*x)*dt = 1.0 + (-1.0)*0.1 = 0.9
        assert!((model.x - 0.9).abs() < 1e-10);
        assert!(model.x < initial_x); // Should decay
    }
}
