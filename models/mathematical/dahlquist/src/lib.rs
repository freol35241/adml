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

// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]

pub use fmu_from_struct::prelude::*;

/// Dahlquist test equation model
///
/// This implements a simple first-order linear ODE: der(x) = -k * x
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct Dahlquist {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    /// Decay constant k
    pub k: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "1.0")]
    /// State variable x
    pub x: f64,

    /// FMU runtime information (optional)
    pub fmu_info: FmuInfo,
}

impl FmuFunctions for Dahlquist {
    fn exit_initialization_mode(&mut self) {
        // Nothing special needed for initialization
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Euler integration: x = x + dx/dt * dt
        // where dx/dt = -k * x
        let der_x = -self.k * self.x;
        self.x += der_x * time_step;
    }
}

impl Dahlquist {
    /// Create a new Dahlquist model with default parameters
    pub fn new() -> Self {
        Self {
            k: 1.0,
            x: 1.0,
            fmu_info: FmuInfo::default(),
        }
    }

    /// Get the analytical solution at time t from initial value x0
    pub fn analytical_solution(x0: f64, k: f64, t: f64) -> f64 {
        x0 * (-k * t).exp()
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
