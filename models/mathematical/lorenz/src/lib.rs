//! Lorenz System
//!
//! The famous chaotic attractor discovered by Edward Lorenz in 1963.
//! This system models atmospheric convection and exhibits chaotic behavior
//! for certain parameter values.
//!
//! The system is defined by three coupled differential equations:
//!
//! dx/dt = σ(y - x)
//! dy/dt = x(ρ - z) - y
//! dz/dt = xy - βz
//!
//! where:
//! - σ (sigma) is the Prandtl number
//! - ρ (rho) is the Rayleigh number
//! - β (beta) is a geometric factor
//!
//! Classic chaotic parameters: σ=10, ρ=28, β=8/3

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

/// Lorenz system model
///
/// The Lorenz system is a three-dimensional dynamical system that exhibits
/// chaotic behavior for certain parameter values. It was originally derived
/// as a simplified model of atmospheric convection.
///
/// The system is implemented as:
/// - der(x) = sigma * (y - x)
/// - der(y) = x * (rho - z) - y
/// - der(z) = x * y - beta * z
#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct Lorenz {
    /// Prandtl number (sigma) - ratio of momentum diffusivity to thermal diffusivity
    #[variable(causality = Parameter, start = 10.0, initial = Exact)]
    pub sigma: f64,

    /// Rayleigh number (rho) - ratio of buoyancy to viscous forces
    #[variable(causality = Parameter, start = 28.0, initial = Exact)]
    pub rho: f64,

    /// Geometric factor (beta)
    #[variable(causality = Parameter, start = 2.6666666666666665, initial = Exact)]
    pub beta: f64,

    /// State variable x
    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub x: f64,

    /// State variable y
    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub y: f64,

    /// State variable z
    #[variable(causality = Output, start = 1.0, initial = Exact)]
    pub z: f64,

    /// Derivative of x
    #[variable(causality = Local, derivative = x, initial = Calculated)]
    der_x: f64,

    /// Derivative of y
    #[variable(causality = Local, derivative = y, initial = Calculated)]
    der_y: f64,

    /// Derivative of z
    #[variable(causality = Local, derivative = z, initial = Calculated)]
    der_z: f64,
}

impl UserModel for Lorenz {
    type LoggingCategory = DefaultLoggingCategory;

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        self.der_x = self.sigma * (self.y - self.x);
        self.der_y = self.x * (self.rho - self.z) - self.y;
        self.der_z = self.x * self.y - self.beta * self.z;
        Ok(Fmi3Res::OK)
    }

    adml_solver::euler_cs_step!(0.0001);
}

fmi_export::export_fmu!(Lorenz);

impl Lorenz {
    /// Create a new Lorenz system with classic chaotic parameters
    pub fn new() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            x: 1.0,
            y: 1.0,
            z: 1.0,
            der_x: 0.0,
            der_y: 0.0,
            der_z: 0.0,
        }
    }

    /// Create a new Lorenz system with custom parameters
    pub fn with_params(sigma: f64, rho: f64, beta: f64) -> Self {
        Self {
            sigma,
            rho,
            beta,
            x: 1.0,
            y: 1.0,
            z: 1.0,
            der_x: 0.0,
            der_y: 0.0,
            der_z: 0.0,
        }
    }

    /// Calculate the distance from the origin
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculate the equilibrium points for the current parameters
    /// Returns the three equilibrium points: origin and two symmetric points
    pub fn equilibrium_points(&self) -> [(f64, f64, f64); 3] {
        let origin = (0.0, 0.0, 0.0);

        if self.rho <= 1.0 {
            // Only origin is equilibrium for rho <= 1
            [origin, origin, origin]
        } else {
            // Two additional symmetric equilibrium points exist for rho > 1
            let c = ((self.rho - 1.0) * self.beta).sqrt();
            let eq_plus = (c, c, self.rho - 1.0);
            let eq_minus = (-c, -c, self.rho - 1.0);
            [origin, eq_plus, eq_minus]
        }
    }

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let der_x = self.sigma * (self.y - self.x);
        let der_y = self.x * (self.rho - self.z) - self.y;
        let der_z = self.x * self.y - self.beta * self.z;
        self.x += der_x * time_step;
        self.y += der_y * time_step;
        self.z += der_z * time_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = Lorenz::new();
        assert_eq!(model.x, 1.0);
        assert_eq!(model.y, 1.0);
        assert_eq!(model.z, 1.0);
        assert_eq!(model.sigma, 10.0);
        assert_eq!(model.rho, 28.0);
        assert!((model.beta - 8.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_derivative_calculation() {
        let model = Lorenz::new();

        // At (1, 1, 1) with sigma=10, rho=28, beta=8/3:
        // der(x) = 10 * (1 - 1) = 0
        let der_x = model.sigma * (model.y - model.x);
        assert!((der_x - 0.0).abs() < 1e-10);

        // der(y) = 1 * (28 - 1) - 1 = 27 - 1 = 26
        let der_y = model.x * (model.rho - model.z) - model.y;
        assert!((der_y - 26.0).abs() < 1e-10);

        // der(z) = 1 * 1 - (8/3) * 1 = 1 - 8/3 = -5/3
        let der_z = model.x * model.y - model.beta * model.z;
        assert!((der_z - (-5.0 / 3.0)).abs() < 1e-10);
    }

    #[test]
    fn test_equilibrium_at_origin() {
        let mut model = Lorenz::new();
        model.x = 0.0;
        model.y = 0.0;
        model.z = 0.0;

        // At origin, all derivatives should be zero
        let der_x = model.sigma * (model.y - model.x);
        let der_y = model.x * (model.rho - model.z) - model.y;
        let der_z = model.x * model.y - model.beta * model.z;

        assert!((der_x).abs() < 1e-10);
        assert!((der_y).abs() < 1e-10);
        assert!((der_z).abs() < 1e-10);
    }

    #[test]
    fn test_equilibrium_points() {
        let model = Lorenz::new();
        let eqs = model.equilibrium_points();

        // Check that the non-origin equilibrium points satisfy the equilibrium conditions
        for (eq_x, eq_y, eq_z) in &eqs[1..] {
            let der_x = model.sigma * (eq_y - eq_x);
            let der_y = eq_x * (model.rho - eq_z) - eq_y;
            let der_z = eq_x * eq_y - model.beta * eq_z;

            assert!(
                der_x.abs() < 1e-10,
                "der_x = {} at equilibrium ({}, {}, {})",
                der_x,
                eq_x,
                eq_y,
                eq_z
            );
            assert!(
                der_y.abs() < 1e-10,
                "der_y = {} at equilibrium ({}, {}, {})",
                der_y,
                eq_x,
                eq_y,
                eq_z
            );
            assert!(
                der_z.abs() < 1e-10,
                "der_z = {} at equilibrium ({}, {}, {})",
                der_z,
                eq_x,
                eq_y,
                eq_z
            );
        }
    }

    #[test]
    fn test_do_step() {
        let mut model = Lorenz::new();

        let initial_x = model.x;
        let initial_y = model.y;
        let initial_z = model.z;
        let dt = 0.001;

        model.do_step(0.0, dt);

        // x should stay approximately the same (der_x = 0)
        assert!((model.x - initial_x).abs() < 0.001);

        // y should increase (der_y = 26)
        assert!(model.y > initial_y);

        // z should decrease (der_z = -5/3)
        assert!(model.z < initial_z);
    }

    #[test]
    fn test_distance_from_origin() {
        let mut model = Lorenz::new();
        model.x = 3.0;
        model.y = 4.0;
        model.z = 0.0;

        let distance = model.distance_from_origin();
        assert!((distance - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_with_params() {
        let model = Lorenz::with_params(5.0, 15.0, 1.0);
        assert_eq!(model.sigma, 5.0);
        assert_eq!(model.rho, 15.0);
        assert_eq!(model.beta, 1.0);
    }
}
