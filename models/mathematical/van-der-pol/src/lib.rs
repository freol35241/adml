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
use fmi_export::fmi3::{Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

/// Van der Pol oscillator model
///
/// The Van der Pol oscillator is a non-conservative oscillator with non-linear damping.
/// It evolves in time according to the second-order differential equation:
/// d²x/dt² - μ(1 - x²)dx/dt + x = 0
///
/// This is implemented as a system of first-order ODEs:
/// - der(x0) = x1
/// - der(x1) = μ(1 - x0²)x1 - x0
#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct VanDerPol {
    /// Damping parameter μ
    #[variable(causality = Parameter, start = 1.0, initial = Exact)]
    pub mu: f64,

    /// State variable x0 (position-like)
    #[variable(causality = Output, start = 2.0, initial = Exact)]
    pub x0: f64,

    /// State variable x1 (velocity-like)
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub x1: f64,

    /// Derivative of x0
    #[variable(causality = Local, derivative = x0, initial = Calculated)]
    der_x0: f64,

    /// Derivative of x1
    #[variable(causality = Local, derivative = x1, initial = Calculated)]
    der_x1: f64,
}

impl UserModel for VanDerPol {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        self.der_x0 = self.x1;
        self.der_x1 = self.mu * (1.0 - self.x0 * self.x0) * self.x1 - self.x0;
        Ok(Fmi3Res::OK)
    }

    adml_solver::euler_cs_step!(0.0001);
}

fmi_export::export_fmu!(VanDerPol);

impl VanDerPol {
    /// Create a new Van der Pol oscillator with default parameters
    pub fn new() -> Self {
        Self {
            mu: 1.0,
            x0: 2.0,
            x1: 0.0,
            der_x0: 0.0,
            der_x1: 0.0,
        }
    }

    /// Calculate total energy (not conserved for Van der Pol)
    pub fn total_energy(&self) -> f64 {
        0.5 * self.x0 * self.x0 + 0.5 * self.x1 * self.x1
    }

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let der_x0 = self.x1;
        let der_x1 = self.mu * (1.0 - self.x0 * self.x0) * self.x1 - self.x0;
        self.x0 += der_x0 * time_step;
        self.x1 += der_x1 * time_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = VanDerPol::new();
        assert_eq!(model.x0, 2.0);
        assert_eq!(model.x1, 0.0);
        assert_eq!(model.mu, 1.0);
    }

    #[test]
    fn test_derivative_calculation() {
        let model = VanDerPol::new();

        // Manually calculate what the derivatives should be
        let der_x0 = model.x1; // = 0.0
        assert_eq!(der_x0, 0.0);

        // der(x1) = μ * ((1 - x0²) * x1) - x0
        //         = 1.0 * ((1 - 4.0) * 0.0) - 2.0
        //         = -2.0
        let der_x1 = model.mu * (1.0 - model.x0 * model.x0) * model.x1 - model.x0;
        assert_eq!(der_x1, -2.0);
    }

    #[test]
    fn test_nonlinear_damping() {
        let mut model = VanDerPol::new();
        model.mu = 2.0;

        // Test that damping term changes sign based on x0
        model.x0 = 0.5;
        model.x1 = 1.0;
        let der1 = model.mu * (1.0 - model.x0 * model.x0) * model.x1 - model.x0;

        model.x0 = 2.0;
        let der2 = model.mu * (1.0 - model.x0 * model.x0) * model.x1 - model.x0;

        // For small x0, damping is negative (energy input)
        // For large x0, damping is positive (energy dissipation)
        assert!(der1 > der2);
    }

    #[test]
    fn test_do_step() {
        let mut model = VanDerPol::new();

        let _initial_x0 = model.x0;
        let initial_x1 = model.x1;
        let dt = 0.1;

        model.do_step(0.0, dt);

        // After one Euler step:
        // x0_new = x0 + x1*dt = 2.0 + 0.0*0.1 = 2.0
        assert!((model.x0 - 2.0).abs() < 1e-10);

        // x1_new = x1 + (-2.0)*0.1 = 0.0 - 0.2 = -0.2
        assert!((model.x1 - (-0.2)).abs() < 1e-10);
        assert!(model.x1 < initial_x1);
    }

    #[test]
    fn test_energy_calculation() {
        let mut model = VanDerPol::new();
        model.x0 = 1.0;
        model.x1 = 1.0;

        let energy = model.total_energy();
        assert!((energy - 1.0).abs() < 1e-10); // 0.5*1² + 0.5*1² = 1.0
    }
}
