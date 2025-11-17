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

use std::ffi::c_void;

/// Model state structure
#[repr(C)]
pub struct Dahlquist {
    /// State variable x
    pub x: f64,
    /// Parameter k (decay constant)
    pub k: f64,
    /// Time
    pub time: f64,
}

impl Default for Dahlquist {
    fn default() -> Self {
        Self::new()
    }
}

impl Dahlquist {
    /// Create a new Dahlquist model with default parameters
    pub fn new() -> Self {
        Self {
            x: 1.0,
            k: 1.0,
            time: 0.0,
        }
    }

    /// Get the number of continuous states
    pub fn get_number_of_continuous_states(&self) -> usize {
        1
    }

    /// Get continuous states
    pub fn get_continuous_states(&self) -> Vec<f64> {
        vec![self.x]
    }

    /// Set continuous states
    pub fn set_continuous_states(&mut self, states: &[f64]) {
        if !states.is_empty() {
            self.x = states[0];
        }
    }

    /// Compute derivatives: dx/dt = -k * x
    pub fn get_derivatives(&self) -> Vec<f64> {
        vec![-self.k * self.x]
    }

    /// Perform a simple Euler integration step
    pub fn do_step(&mut self, dt: f64) {
        let derivatives = self.get_derivatives();
        self.x += derivatives[0] * dt;
        self.time += dt;
    }

    /// Get the analytical solution at time t
    pub fn analytical_solution(&self, t: f64) -> f64 {
        self.x * (-self.k * t).exp()
    }
}

// FMI 3.0 C API functions
// These would be exported as extern "C" functions in a real FMU

#[no_mangle]
pub extern "C" fn fmi3_dahlquist_create() -> *mut c_void {
    let model = Box::new(Dahlquist::new());
    Box::into_raw(model) as *mut c_void
}

#[no_mangle]
pub extern "C" fn fmi3_dahlquist_free(instance: *mut c_void) {
    if !instance.is_null() {
        unsafe {
            let _ = Box::from_raw(instance as *mut Dahlquist);
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3_dahlquist_get_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: *mut f64,
) -> i32 {
    if instance.is_null() || value.is_null() {
        return -1;
    }

    let model = unsafe { &*(instance as *const Dahlquist) };

    unsafe {
        match value_reference {
            0 => *value = model.time,  // time
            1 => *value = model.x,     // x
            2 => *value = -model.k * model.x,  // der(x)
            3 => *value = model.k,     // k
            _ => return -1,
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn fmi3_dahlquist_set_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: f64,
) -> i32 {
    if instance.is_null() {
        return -1;
    }

    let model = unsafe { &mut *(instance as *mut Dahlquist) };

    match value_reference {
        1 => model.x = value,     // x
        3 => model.k = value,     // k
        _ => return -1,
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = Dahlquist::new();
        assert_eq!(model.x, 1.0);
        assert_eq!(model.k, 1.0);
        assert_eq!(model.time, 0.0);
    }

    #[test]
    fn test_derivatives() {
        let model = Dahlquist::new();
        let derivatives = model.get_derivatives();
        assert_eq!(derivatives.len(), 1);
        assert_eq!(derivatives[0], -1.0); // -k * x = -1.0 * 1.0
    }

    #[test]
    fn test_state_operations() {
        let mut model = Dahlquist::new();

        assert_eq!(model.get_number_of_continuous_states(), 1);

        let states = model.get_continuous_states();
        assert_eq!(states.len(), 1);
        assert_eq!(states[0], 1.0);

        model.set_continuous_states(&[2.0]);
        assert_eq!(model.x, 2.0);
    }
}
