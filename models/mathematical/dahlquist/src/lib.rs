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

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::{
    fmi3::{DefaultLoggingCategory, ModelContext, UserModel},
    FmuModel,
};

/// Dahlquist FMU model implementing der(x) = -k * x
///
/// This is a simple first-order linear ODE that demonstrates basic
/// Model Exchange and Co-Simulation capabilities.
#[derive(FmuModel, Default, Debug)]
#[model()]
pub struct Dahlquist {
    /// The state variable
    #[variable(causality = Output, variability = Continuous, state, start = 1.0, initial = Exact)]
    pub x: f64,

    /// The derivative of x, calculated as der(x) = -k * x
    #[variable(causality = Local, variability = Continuous, derivative = x, initial = Calculated)]
    der_x: f64,

    /// The parameter k (decay constant)
    #[variable(causality = Parameter, variability = Fixed, start = 1.0, initial = Exact)]
    pub k: f64,
}

impl Dahlquist {
    /// Create a new Dahlquist model with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the analytical solution at time t from initial value x0
    pub fn analytical_solution(x0: f64, k: f64, t: f64) -> f64 {
        x0 * (-k * t).exp()
    }
}

impl UserModel for Dahlquist {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &ModelContext<Self>) -> Result<Fmi3Res, Fmi3Error> {
        // Calculate the derivative: der(x) = -k * x
        self.der_x = -self.k * self.x;
        Ok(Fmi3Res::OK)
    }
}

// Export the FMU with full C API
fmi_export::export_fmu!(Dahlquist);

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
        let mut model = Dahlquist::new();
        let context = ModelContext::default();

        model.calculate_values(&context).unwrap();

        // der(x) = -k * x = -1.0 * 1.0 = -1.0
        assert_eq!(model.der_x, -1.0);
    }

    #[test]
    fn test_analytical_solution() {
        assert_eq!(Dahlquist::analytical_solution(1.0, 1.0, 0.0), 1.0);

        let t1_value = Dahlquist::analytical_solution(1.0, 1.0, 1.0);
        let expected = (-1.0f64).exp();
        assert!((t1_value - expected).abs() < 1e-10);
    }

    #[test]
    fn test_parameter_modification() {
        let mut model = Dahlquist::new();
        let context = ModelContext::default();

        model.k = 2.0;
        model.calculate_values(&context).unwrap();

        // der(x) = -k * x = -2.0 * 1.0 = -2.0
        assert_eq!(model.der_x, -2.0);
    }
}
