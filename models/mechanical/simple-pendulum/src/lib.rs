//! Simple Pendulum Model
//!
//! This model implements the classical simple pendulum - a point mass suspended
//! from a fixed point by a massless, rigid rod, free to swing in a vertical plane.
//!
//! # Physics
//!
//! The simple pendulum is governed by the following differential equations:
//!
//! dθ/dt = ω
//! dω/dt = -(g/L) * sin(θ) - (b/m) * ω
//!
//! where:
//! - θ (theta): Angular displacement from vertical equilibrium [rad]
//! - ω (omega): Angular velocity [rad/s]
//! - g: Gravitational acceleration [m/s²]
//! - L: Length of pendulum [m]
//! - b: Damping coefficient [kg/s]
//! - m: Mass of bob [kg]
//!
//! # Small-Angle Approximation
//!
//! For small angles (θ << 1 rad), sin(θ) ≈ θ, yielding a linear system with
//! analytical solutions. The undamped natural frequency is ω₀ = sqrt(g/L).
//!
//! # Features
//!
//! - Nonlinear dynamics (full sin(θ) term)
//! - Optional damping
//! - Energy calculation (kinetic + potential)
//! - Valid for large-angle oscillations and continuous rotation

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(non_snake_case)]

pub use fmu_from_struct::prelude::*;

/// Simple pendulum model with nonlinear dynamics
///
/// Models a point mass suspended from a fixed pivot, swinging under gravity
/// with optional damping.
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct SimplePendulum {
    // === Parameters (settable via FMI) ===
    /// Gravitational acceleration [m/s²]
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "9.81")]
    pub g: f64,

    /// Pendulum length [m]
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub L: f64,

    /// Mass of pendulum bob [kg]
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "1.0")]
    pub m: f64,

    /// Damping coefficient [kg/s]
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "0.0")]
    pub b: f64,

    // === State Variables (outputs, read-only via FMI) ===
    /// Angular position [rad], measured from vertical downward equilibrium
    /// Positive values indicate counterclockwise displacement
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.1")]
    pub theta: f64,

    /// Angular velocity [rad/s]
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    pub omega: f64,

    // === Derived Outputs (calculated after integration) ===
    /// Total mechanical energy [J]
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    pub energy: f64,

    /// Kinetic energy [J]
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    pub KE: f64,

    /// Potential energy [J], reference at lowest point
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    pub PE: f64,

    // === Internal (not exposed to FMI) ===
    pub fmu_info: FmuInfo,
}

impl FmuFunctions for SimplePendulum {
    fn exit_initialization_mode(&mut self) {
        // Calculate initial derived outputs
        self.update_derived_outputs();
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Calculate derivatives
        let der_theta = self.omega;
        let der_omega = -(self.g / self.L) * self.theta.sin() - (self.b / self.m) * self.omega;

        // Euler integration
        self.theta += der_theta * time_step;
        self.omega += der_omega * time_step;

        // Update derived outputs after integration
        self.update_derived_outputs();
    }
}

impl SimplePendulum {
    /// Create a new simple pendulum with default parameters
    pub fn new() -> Self {
        let mut pendulum = Self {
            g: 9.81,
            L: 1.0,
            m: 1.0,
            b: 0.0,
            theta: 0.1,
            omega: 0.0,
            energy: 0.0,
            KE: 0.0,
            PE: 0.0,
            fmu_info: FmuInfo::default(),
        };
        pendulum.update_derived_outputs();
        pendulum
    }

    /// Update derived outputs (energy calculations)
    ///
    /// This should be called after manually modifying state variables (theta, omega)
    /// to ensure derived outputs are recalculated.
    pub fn update_derived_outputs(&mut self) {
        // Kinetic energy: KE = (1/2) * m * (L * ω)²
        self.KE = 0.5 * self.m * (self.L * self.omega).powi(2);

        // Potential energy: PE = m * g * L * (1 - cos(θ))
        // Reference is at lowest point (θ = 0 is vertical down)
        self.PE = self.m * self.g * self.L * (1.0 - self.theta.cos());

        // Total mechanical energy
        self.energy = self.KE + self.PE;
    }

    /// Analytical solution for small-angle, undamped pendulum
    ///
    /// For small angles and no damping, the pendulum behaves as a simple harmonic
    /// oscillator with angular frequency ω₀ = sqrt(g/L).
    ///
    /// Solution: θ(t) = θ₀ * cos(ω₀ * t) + (ω₀/ω₀) * sin(ω₀ * t)
    ///          ω(t) = -θ₀ * ω₀ * sin(ω₀ * t) + ω₀ * cos(ω₀ * t)
    ///
    /// # Arguments
    /// * `theta_0` - Initial angular position [rad]
    /// * `omega_0` - Initial angular velocity [rad/s]
    /// * `g` - Gravitational acceleration [m/s²]
    /// * `L` - Pendulum length [m]
    /// * `t` - Time [s]
    ///
    /// # Returns
    /// Tuple of (theta, omega) at time t
    pub fn analytical_solution(theta_0: f64, omega_0: f64, g: f64, L: f64, t: f64) -> (f64, f64) {
        let omega_n = (g / L).sqrt(); // Natural frequency
        let theta = theta_0 * (omega_n * t).cos() + (omega_0 / omega_n) * (omega_n * t).sin();
        let omega = -theta_0 * omega_n * (omega_n * t).sin() + omega_0 * (omega_n * t).cos();
        (theta, omega)
    }

    /// Calculate the period of small-angle oscillations
    ///
    /// For small angles, T = 2π * sqrt(L/g)
    pub fn small_angle_period(L: f64, g: f64) -> f64 {
        2.0 * std::f64::consts::PI * (L / g).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let pendulum = SimplePendulum::new();
        assert_eq!(pendulum.g, 9.81);
        assert_eq!(pendulum.L, 1.0);
        assert_eq!(pendulum.m, 1.0);
        assert_eq!(pendulum.b, 0.0);
        assert_eq!(pendulum.theta, 0.1);
        assert_eq!(pendulum.omega, 0.0);
    }

    #[test]
    fn test_equilibrium() {
        let mut pendulum = SimplePendulum::new();
        pendulum.theta = 0.0;
        pendulum.omega = 0.0;

        // At equilibrium, pendulum should remain at rest
        pendulum.do_step(0.0, 0.01);

        assert!((pendulum.theta).abs() < 1e-10);
        assert!((pendulum.omega).abs() < 1e-10);
    }

    #[test]
    fn test_energy_conservation_undamped() {
        let mut pendulum = SimplePendulum::new();
        pendulum.theta = 0.2; // Small angle
        pendulum.omega = 0.0;
        pendulum.b = 0.0; // No damping
        pendulum.update_derived_outputs();

        let initial_energy = pendulum.energy;

        // Simulate for one period
        let dt = 0.001; // Small time step for accuracy
        let period = SimplePendulum::small_angle_period(pendulum.L, pendulum.g);
        let steps = (period / dt) as usize;

        for _ in 0..steps {
            pendulum.do_step(0.0, dt);
        }

        // Energy should be approximately conserved (within numerical error)
        let energy_change = (pendulum.energy - initial_energy).abs();
        let relative_error = energy_change / initial_energy;

        // Allow for ~5% error due to Euler integration
        assert!(
            relative_error < 0.05,
            "Energy changed by {:.2}% (expected < 5%)",
            relative_error * 100.0
        );
    }

    #[test]
    fn test_energy_dissipation_damped() {
        let mut pendulum = SimplePendulum::new();
        pendulum.theta = 0.2;
        pendulum.omega = 0.0;
        pendulum.b = 0.1; // With damping
        pendulum.update_derived_outputs();

        let initial_energy = pendulum.energy;

        // Simulate for some time
        let dt = 0.01;
        for _ in 0..1000 {
            pendulum.do_step(0.0, dt);
        }

        // Energy should decrease with damping
        assert!(
            pendulum.energy < initial_energy,
            "Energy should decrease with damping"
        );
    }

    #[test]
    fn test_period() {
        // For L=1m, g=9.81m/s², period should be ~2.006s
        let period = SimplePendulum::small_angle_period(1.0, 9.81);
        let expected_period = 2.006;

        assert!(
            (period - expected_period).abs() < 0.01,
            "Period calculation incorrect"
        );
    }
}
