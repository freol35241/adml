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

use std::ffi::c_void;

/// Van der Pol oscillator model
#[repr(C)]
pub struct VanDerPol {
    /// Position-like state variable
    pub x0: f64,
    /// Velocity-like state variable
    pub x1: f64,
    /// Damping parameter (μ)
    pub mu: f64,
    /// Current simulation time
    pub time: f64,
}

impl Default for VanDerPol {
    fn default() -> Self {
        Self::new()
    }
}

impl VanDerPol {
    /// Create a new Van der Pol oscillator with default parameters
    pub fn new() -> Self {
        Self {
            x0: 2.0,  // Default initial position
            x1: 0.0,  // Default initial velocity
            mu: 1.0,  // Default damping parameter
            time: 0.0,
        }
    }

    /// Get the number of continuous states
    pub fn get_number_of_continuous_states(&self) -> usize {
        2
    }

    /// Get continuous states [x0, x1]
    pub fn get_continuous_states(&self) -> Vec<f64> {
        vec![self.x0, self.x1]
    }

    /// Set continuous states from [x0, x1]
    pub fn set_continuous_states(&mut self, states: &[f64]) {
        if states.len() >= 2 {
            self.x0 = states[0];
            self.x1 = states[1];
        }
    }

    /// Compute derivatives
    ///
    /// dx0/dt = x1
    /// dx1/dt = μ * (1 - x0²) * x1 - x0
    pub fn get_derivatives(&self) -> Vec<f64> {
        let der_x0 = self.x1;
        let der_x1 = self.mu * (1.0 - self.x0 * self.x0) * self.x1 - self.x0;
        vec![der_x0, der_x1]
    }

    /// Perform a simple Euler integration step
    pub fn do_step(&mut self, dt: f64) {
        let derivatives = self.get_derivatives();
        self.x0 += derivatives[0] * dt;
        self.x1 += derivatives[1] * dt;
        self.time += dt;
    }

    /// Calculate total energy (not conserved for Van der Pol)
    pub fn total_energy(&self) -> f64 {
        0.5 * self.x0 * self.x0 + 0.5 * self.x1 * self.x1
    }
}

// FMI 3.0 C API functions

#[no_mangle]
pub extern "C" fn fmi3_vanderpol_create() -> *mut c_void {
    let model = Box::new(VanDerPol::new());
    Box::into_raw(model) as *mut c_void
}

#[no_mangle]
pub extern "C" fn fmi3_vanderpol_free(instance: *mut c_void) {
    if !instance.is_null() {
        unsafe {
            let _ = Box::from_raw(instance as *mut VanDerPol);
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3_vanderpol_get_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: *mut f64,
) -> i32 {
    if instance.is_null() || value.is_null() {
        return -1;
    }

    let model = unsafe { &*(instance as *const VanDerPol) };

    unsafe {
        match value_reference {
            0 => *value = model.time,  // time
            1 => *value = model.x0,    // x0
            2 => *value = model.x1,    // der(x0) = x1
            3 => *value = model.x1,    // x1
            4 => *value = model.mu * (1.0 - model.x0 * model.x0) * model.x1 - model.x0,  // der(x1)
            5 => *value = model.mu,    // mu
            _ => return -1,
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn fmi3_vanderpol_set_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: f64,
) -> i32 {
    if instance.is_null() {
        return -1;
    }

    let model = unsafe { &mut *(instance as *mut VanDerPol) };

    match value_reference {
        1 => model.x0 = value,    // x0
        3 => model.x1 = value,    // x1
        5 => model.mu = value,    // mu
        _ => return -1,
    }

    0
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
        assert_eq!(model.time, 0.0);
    }

    #[test]
    fn test_derivatives() {
        let model = VanDerPol::new();
        let derivatives = model.get_derivatives();
        assert_eq!(derivatives.len(), 2);
        assert_eq!(derivatives[0], 0.0); // x1 = 0
        // μ * (1 - x0²) * x1 - x0 = 1.0 * (1 - 4) * 0 - 2.0 = -2.0
        assert_eq!(derivatives[1], -2.0);
    }

    #[test]
    fn test_state_operations() {
        let mut model = VanDerPol::new();

        assert_eq!(model.get_number_of_continuous_states(), 2);

        let states = model.get_continuous_states();
        assert_eq!(states.len(), 2);
        assert_eq!(states[0], 2.0);
        assert_eq!(states[1], 0.0);

        model.set_continuous_states(&[1.0, 0.5]);
        assert_eq!(model.x0, 1.0);
        assert_eq!(model.x1, 0.5);
    }

    #[test]
    fn test_nonlinear_damping() {
        let mut model = VanDerPol::new();
        model.mu = 2.0;

        // Test that damping term changes sign based on x0
        model.x0 = 0.5;
        model.x1 = 1.0;
        let der1 = model.get_derivatives()[1];

        model.x0 = 2.0;
        let der2 = model.get_derivatives()[1];

        // For small x0, damping is negative (energy input)
        // For large x0, damping is positive (energy dissipation)
        assert!(der1 > der2);
    }
}
