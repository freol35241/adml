//! Van der Pol Oscillator
//!
//! A classic nonlinear oscillator with nonlinear damping:
//!
//! dx0/dt = x1
//! dx1/dt = μ * (1 - x0²) * x1 - x0
//!
//! where μ is the damping parameter. For μ > 0, the system exhibits
//! a stable limit cycle. The Van der Pol oscillator is important in
//! studying self-sustaining oscillations in various physical systems.

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::{
    fmi3::{DefaultLoggingCategory, ModelContext, UserModel},
    FmuModel,
};

/// Van der Pol oscillator FMU model
///
/// The Van der Pol oscillator is a non-conservative oscillator with non-linear damping.
/// It evolves in time according to the second-order differential equation:
/// d²x/dt² - μ(1 - x²)dx/dt + x = 0
///
/// This is implemented as a system of first-order ODEs:
/// - der(x[0]) = x[1]
/// - der(x[1]) = μ(1 - x[0]²)x[1] - x[0]
#[derive(FmuModel, Default, Debug)]
#[model()]
pub struct VanDerPol {
    /// State variables [x0, x1] where x0 is position-like and x1 is velocity-like
    #[variable(causality = Output, variability = Continuous, state, start = [2.0, 0.0], initial = Exact)]
    pub x: [f64; 2],

    /// Derivatives [der(x0), der(x1)]
    #[variable(causality = Local, variability = Continuous, derivative = x, initial = Calculated)]
    der_x: [f64; 2],

    /// Damping parameter μ
    #[variable(causality = Parameter, variability = Fixed, start = 1.0, initial = Exact)]
    pub mu: f64,
}

impl VanDerPol {
    /// Create a new Van der Pol oscillator with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Get x0 (position-like variable)
    pub fn x0(&self) -> f64 {
        self.x[0]
    }

    /// Get x1 (velocity-like variable)
    pub fn x1(&self) -> f64 {
        self.x[1]
    }

    /// Set x0 (position-like variable)
    pub fn set_x0(&mut self, value: f64) {
        self.x[0] = value;
    }

    /// Set x1 (velocity-like variable)
    pub fn set_x1(&mut self, value: f64) {
        self.x[1] = value;
    }

    /// Calculate total energy (not conserved for Van der Pol)
    pub fn total_energy(&self) -> f64 {
        0.5 * self.x[0] * self.x[0] + 0.5 * self.x[1] * self.x[1]
    }
}

impl UserModel for VanDerPol {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &ModelContext<Self>) -> Result<Fmi3Res, Fmi3Error> {
        // Calculate the derivatives according to Van der Pol equations:
        // der(x[0]) = x[1]
        self.der_x[0] = self.x[1];

        // der(x[1]) = mu * ((1 - x[0]²) * x[1]) - x[0]
        self.der_x[1] = self.mu * ((1.0 - self.x[0] * self.x[0]) * self.x[1]) - self.x[0];

        Ok(Fmi3Res::OK)
    }
}

// Export the FMU with full C API
fmi_export::export_fmu!(VanDerPol);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = VanDerPol::new();
        assert_eq!(model.x[0], 2.0);
        assert_eq!(model.x[1], 0.0);
        assert_eq!(model.mu, 1.0);
    }

    #[test]
    fn test_derivative_calculation() {
        let mut model = VanDerPol::new();
        let context = ModelContext::default();

        model.calculate_values(&context).unwrap();

        // der(x[0]) = x[1] = 0.0
        assert_eq!(model.der_x[0], 0.0);

        // der(x[1]) = μ * ((1 - x[0]²) * x[1]) - x[0]
        //           = 1.0 * ((1 - 4.0) * 0.0) - 2.0
        //           = -2.0
        assert_eq!(model.der_x[1], -2.0);
    }

    #[test]
    fn test_nonlinear_damping() {
        let mut model = VanDerPol::new();
        let context = ModelContext::default();
        model.mu = 2.0;

        // Test that damping term changes sign based on x0
        model.x[0] = 0.5;
        model.x[1] = 1.0;
        model.calculate_values(&context).unwrap();
        let der1 = model.der_x[1];

        model.x[0] = 2.0;
        model.calculate_values(&context).unwrap();
        let der2 = model.der_x[1];

        // For small x0, damping is negative (energy input)
        // For large x0, damping is positive (energy dissipation)
        assert!(der1 > der2);
    }

    #[test]
    fn test_accessors() {
        let mut model = VanDerPol::new();

        assert_eq!(model.x0(), 2.0);
        assert_eq!(model.x1(), 0.0);

        model.set_x0(1.5);
        model.set_x1(0.5);

        assert_eq!(model.x0(), 1.5);
        assert_eq!(model.x1(), 0.5);
    }
}
