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

// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

/// Van der Pol oscillator model
///
/// The Van der Pol oscillator is a non-conservative oscillator with non-linear damping.
/// It evolves in time according to the second-order differential equation:
/// d²x/dt² - μ(1 - x²)dx/dt + x = 0
///
/// This is implemented as a system of first-order ODEs:
/// - der(x0) = x1
/// - der(x1) = μ(1 - x0²)x1 - x0
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct VanDerPol {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    /// Damping parameter μ
    pub mu: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "2.0")]
    /// State variable x0 (position-like)
    pub x0: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    /// State variable x1 (velocity-like)
    pub x1: f64,

    /// FMU runtime information (optional)
    pub fmu_info: FmuInfo,
}

impl FmuFunctions for VanDerPol {
    fn exit_initialization_mode(&mut self) {
        // Nothing special needed for initialization
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Calculate derivatives according to Van der Pol equations:
        // der(x0) = x1
        let der_x0 = self.x1;

        // der(x1) = mu * (1 - x0²) * x1 - x0
        let der_x1 = self.mu * (1.0 - self.x0 * self.x0) * self.x1 - self.x0;

        // Euler integration: x_new = x + der(x) * dt
        self.x0 += der_x0 * time_step;
        self.x1 += der_x1 * time_step;
    }
}

impl VanDerPol {
    /// Create a new Van der Pol oscillator with default parameters
    pub fn new() -> Self {
        Self {
            mu: 1.0,
            x0: 2.0,
            x1: 0.0,
            fmu_info: FmuInfo::default(),
        }
    }

    /// Calculate total energy (not conserved for Van der Pol)
    pub fn total_energy(&self) -> f64 {
        0.5 * self.x0 * self.x0 + 0.5 * self.x1 * self.x1
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
