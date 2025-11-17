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

use std::ffi::c_void;

const V_MIN: f64 = 0.1; // Minimum velocity threshold
const EVENT_EPSILON: f64 = 1e-10;

/// Bouncing ball model
#[repr(C)]
pub struct BouncingBall {
    /// Height above ground (m)
    pub h: f64,
    /// Vertical velocity (m/s)
    pub v: f64,
    /// Gravitational acceleration (m/s²), typically negative
    pub g: f64,
    /// Coefficient of restitution (0 < e < 1)
    pub e: f64,
    /// Current simulation time
    pub time: f64,
}

impl Default for BouncingBall {
    fn default() -> Self {
        Self::new()
    }
}

impl BouncingBall {
    /// Create a new bouncing ball with default parameters
    pub fn new() -> Self {
        Self {
            h: 1.0,      // Start at 1 meter height
            v: 0.0,      // Start from rest
            g: -9.81,    // Earth gravity
            e: 0.7,      // Coefficient of restitution
            time: 0.0,
        }
    }

    /// Get the number of continuous states
    pub fn get_number_of_continuous_states(&self) -> usize {
        2
    }

    /// Get continuous states [h, v]
    pub fn get_continuous_states(&self) -> Vec<f64> {
        vec![self.h, self.v]
    }

    /// Set continuous states from [h, v]
    pub fn set_continuous_states(&mut self, states: &[f64]) {
        if states.len() >= 2 {
            self.h = states[0];
            self.v = states[1];
        }
    }

    /// Compute derivatives
    ///
    /// dh/dt = v
    /// dv/dt = g
    pub fn get_derivatives(&self) -> Vec<f64> {
        vec![self.v, self.g]
    }

    /// Check for collision event
    ///
    /// Returns true if the ball has hit the ground
    pub fn check_collision(&self) -> bool {
        self.h <= 0.0 && self.v < 0.0
    }

    /// Get event indicator for collision detection
    ///
    /// Zero crossing indicates collision
    pub fn get_event_indicator(&self) -> f64 {
        // Add hysteresis for better stability
        if self.h > -EVENT_EPSILON && self.h <= 0.0 && self.v > 0.0 {
            -EVENT_EPSILON
        } else {
            self.h
        }
    }

    /// Handle collision event
    ///
    /// Reverses velocity with energy loss and checks stopping condition
    pub fn handle_collision(&mut self) -> bool {
        if self.check_collision() {
            // Reverse velocity with coefficient of restitution
            self.v = -self.v * self.e;
            self.h = f64::EPSILON; // Slightly above ground

            // Check if velocity is too small - stop bouncing
            if self.v < V_MIN {
                self.v = 0.0;
                self.g = 0.0; // Stop gravity to keep ball at rest
                return true; // Stopped bouncing
            }

            return false; // Still bouncing
        }
        false
    }

    /// Perform a simple Euler integration step with event handling
    pub fn do_step(&mut self, dt: f64) -> bool {
        // Simple Euler integration
        let derivatives = self.get_derivatives();
        self.h += derivatives[0] * dt;
        self.v += derivatives[1] * dt;
        self.time += dt;

        // Check and handle collision
        self.handle_collision()
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

// FMI 3.0 C API functions

#[no_mangle]
pub extern "C" fn fmi3_bouncingball_create() -> *mut c_void {
    let model = Box::new(BouncingBall::new());
    Box::into_raw(model) as *mut c_void
}

#[no_mangle]
pub extern "C" fn fmi3_bouncingball_free(instance: *mut c_void) {
    if !instance.is_null() {
        unsafe {
            let _ = Box::from_raw(instance as *mut BouncingBall);
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3_bouncingball_get_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: *mut f64,
) -> i32 {
    if instance.is_null() || value.is_null() {
        return -1;
    }

    let model = unsafe { &*(instance as *const BouncingBall) };

    unsafe {
        match value_reference {
            0 => *value = model.time,  // time
            1 => *value = model.h,     // h
            2 => *value = model.v,     // der(h) = v
            3 => *value = model.v,     // v
            4 => *value = model.g,     // der(v) = g
            5 => *value = model.g,     // g
            6 => *value = model.e,     // e
            7 => *value = V_MIN,       // v_min (constant)
            _ => return -1,
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn fmi3_bouncingball_set_float64(
    instance: *mut c_void,
    value_reference: u32,
    value: f64,
) -> i32 {
    if instance.is_null() {
        return -1;
    }

    let model = unsafe { &mut *(instance as *mut BouncingBall) };

    match value_reference {
        1 => model.h = value,     // h
        3 => model.v = value,     // v
        5 => model.g = value,     // g
        6 => model.e = value,     // e
        7 => return -1,           // v_min is constant
        _ => return -1,
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_initial_values() {
        let model = BouncingBall::new();
        assert_eq!(model.h, 1.0);
        assert_eq!(model.v, 0.0);
        assert_eq!(model.g, -9.81);
        assert_eq!(model.e, 0.7);
    }

    #[test]
    fn test_derivatives() {
        let model = BouncingBall::new();
        let derivatives = model.get_derivatives();
        assert_eq!(derivatives.len(), 2);
        assert_eq!(derivatives[0], 0.0);   // v = 0
        assert_eq!(derivatives[1], -9.81); // g = -9.81
    }

    #[test]
    fn test_state_operations() {
        let mut model = BouncingBall::new();

        assert_eq!(model.get_number_of_continuous_states(), 2);

        let states = model.get_continuous_states();
        assert_eq!(states.len(), 2);
        assert_eq!(states[0], 1.0);
        assert_eq!(states[1], 0.0);

        model.set_continuous_states(&[2.0, -1.0]);
        assert_eq!(model.h, 2.0);
        assert_eq!(model.v, -1.0);
    }

    #[test]
    fn test_collision_detection() {
        let mut model = BouncingBall::new();

        // No collision when above ground
        model.h = 1.0;
        model.v = -1.0;
        assert!(!model.check_collision());

        // No collision when at ground but moving up
        model.h = 0.0;
        model.v = 1.0;
        assert!(!model.check_collision());

        // Collision when at/below ground and moving down
        model.h = 0.0;
        model.v = -1.0;
        assert!(model.check_collision());
    }

    #[test]
    fn test_collision_handling() {
        let mut model = BouncingBall::new();
        model.h = 0.0;
        model.v = -2.0;
        model.e = 0.8;

        let stopped = model.handle_collision();

        assert!(!stopped);
        assert_relative_eq!(model.v, 1.6, epsilon = 1e-10); // -(-2.0) * 0.8
        assert!(model.h > 0.0); // Moved slightly above ground
    }

    #[test]
    fn test_energy_calculation() {
        let mut model = BouncingBall::new();
        model.h = 1.0;
        model.v = 0.0;

        let pe = model.potential_energy();
        let ke = model.kinetic_energy();

        assert_relative_eq!(pe, 9.81, epsilon = 1e-10);
        assert_relative_eq!(ke, 0.0, epsilon = 1e-10);
    }
}
