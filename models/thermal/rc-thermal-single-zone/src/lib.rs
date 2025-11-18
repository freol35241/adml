//! Single-Zone 1R1C Thermal RC Model
//!
//! A simplified thermal model of a building/house represented as a lumped parameter system:
//! - One thermal resistance (R_th): insulation/walls to outside environment
//! - One thermal capacitance (C_th): thermal mass of air + structure
//! - One heating source (Q_heat): controllable heat input
//! - Ambient temperature (T_ambient): outdoor temperature
//!
//! ## Mathematical Model
//!
//! The model implements the thermal energy balance equation:
//!
//! ```text
//! C_th * dT_indoor/dt = Q_heat - Q_loss
//!
//! where: Q_loss = (T_indoor - T_ambient) / R_th
//! ```
//!
//! Rearranged as:
//!
//! ```text
//! dT_indoor/dt = Q_heat/C_th - (T_indoor - T_ambient)/(R_th * C_th)
//! ```
//!
//! ## Physical Interpretation
//!
//! - **Time constant**: τ = R_th * C_th (seconds)
//! - **Steady-state**: T_indoor = T_ambient + Q_heat * R_th (when dT/dt = 0)
//! - **Heat loss**: Q_loss = (T_indoor - T_ambient) / R_th (Watts)
//!
//! ## Typical Values
//!
//! For a well-insulated small house (~100 m²):
//! - R_th ≈ 0.01 K/W (good insulation)
//! - C_th ≈ 10,000,000 J/K (thermal mass)
//! - τ ≈ 100,000 s ≈ 28 hours
//! - Q_heat ≈ 5,000 W (5 kW heater)

// Allow clippy lints for generated code from fmu_from_struct derive macro
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ptr_offset_with_cast)]
// Allow non_snake_case for physics notation (R_th, C_th, T_indoor, etc.)
#![allow(non_snake_case)]

pub use fmu_from_struct::prelude::*;

/// Single-zone 1R1C thermal RC model
///
/// Represents a simplified building thermal system with:
/// - Thermal resistance (insulation)
/// - Thermal capacitance (thermal mass)
/// - Heating input
/// - Heat loss to ambient
///
/// # Equations
///
/// The model implements:
/// - State equation: `dT_indoor/dt = Q_heat/C_th - (T_indoor - T_ambient)/(R_th * C_th)`
/// - Heat loss: `Q_loss = (T_indoor - T_ambient) / R_th`
///
/// # Parameters
///
/// - `R_th`: Thermal resistance [K/W]
/// - `C_th`: Thermal capacitance [J/K]
/// - `T_ambient`: Ambient/outdoor temperature [°C]
/// - `Q_heat`: Heating power input [W]
///
/// # State Variables
///
/// - `T_indoor`: Indoor temperature [°C]
/// - `Q_loss`: Heat loss through walls [W] (derived)
/// - `dT_dt`: Rate of temperature change [K/s] (derived)
#[derive(Fmu, Default, Debug, Clone)]
#[fmu_from_struct(fmi_version = 3)]
pub struct RCThermalSingleZone {
    // === Parameters (can be set via FMI before simulation) ===
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "0.01")]
    /// Thermal resistance (insulation quality)
    /// Units: K/W (Kelvin per Watt)
    /// Typical range: 0.001 - 0.1 K/W
    pub R_th: f64,

    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "10000000.0")]
    /// Thermal capacitance (thermal mass)
    /// Units: J/K (Joules per Kelvin)
    /// Typical range: 1e6 - 1e8 J/K
    pub C_th: f64,

    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "0.0")]
    /// Ambient/outdoor temperature
    /// Units: °C (degrees Celsius)
    pub T_ambient: f64,

    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = "5000.0")]
    /// Heating power input
    /// Units: W (Watts)
    /// Typical range: 0 - 20000 W
    pub Q_heat: f64,

    // === State Variables / Outputs (read-only via FMI) ===
    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "20.0")]
    /// Indoor temperature (state variable)
    /// Units: °C (degrees Celsius)
    pub T_indoor: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    /// Heat loss through walls (derived output)
    /// Units: W (Watts)
    pub Q_loss: f64,

    #[fmu_from_struct(output)]
    #[fmu_from_struct(start_value = "0.0")]
    /// Rate of temperature change (derived output)
    /// Units: K/s (Kelvin per second)
    pub dT_dt: f64,

    // === Internal variables (not exposed via FMI) ===
    /// Current simulation time
    time: f64,

    /// FMU runtime information
    pub fmu_info: FmuInfo,
}

impl FmuFunctions for RCThermalSingleZone {
    fn exit_initialization_mode(&mut self) {
        // Calculate initial derived outputs
        self.update_derived_outputs();
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Calculate heat loss: Q_loss = (T_indoor - T_ambient) / R_th
        let q_loss = (self.T_indoor - self.T_ambient) / self.R_th;

        // Calculate rate of temperature change:
        // dT_indoor/dt = Q_heat/C_th - Q_loss/C_th
        let der_t_indoor = self.Q_heat / self.C_th - q_loss / self.C_th;

        // Euler integration: T_indoor(t+dt) = T_indoor(t) + dT_indoor/dt * dt
        self.T_indoor += der_t_indoor * time_step;

        // Update derived outputs
        self.Q_loss = (self.T_indoor - self.T_ambient) / self.R_th;
        self.dT_dt = der_t_indoor;

        // Update time
        self.time += time_step;
    }
}

// === Helper Methods (for testing and validation) ===
impl RCThermalSingleZone {
    /// Create a new RCThermalSingleZone model with default parameters
    pub fn new() -> Self {
        let mut model = Self {
            R_th: 0.01,
            C_th: 10_000_000.0,
            T_ambient: 0.0,
            Q_heat: 5000.0,
            T_indoor: 20.0,
            Q_loss: 0.0,
            dT_dt: 0.0,
            time: 0.0,
            fmu_info: FmuInfo::default(),
        };
        model.update_derived_outputs();
        model
    }

    /// Update derived output variables
    fn update_derived_outputs(&mut self) {
        self.Q_loss = (self.T_indoor - self.T_ambient) / self.R_th;
        self.dT_dt = self.Q_heat / self.C_th - self.Q_loss / self.C_th;
    }

    /// Calculate the time constant (tau) of the system
    ///
    /// The time constant represents how quickly the building temperature responds
    /// to changes. After time τ, the system reaches ~63.2% of its final value.
    ///
    /// Returns: time constant in seconds
    pub fn time_constant(&self) -> f64 {
        self.R_th * self.C_th
    }

    /// Calculate steady-state indoor temperature
    ///
    /// At steady state (dT/dt = 0), the indoor temperature is:
    /// T_indoor_ss = T_ambient + Q_heat * R_th
    ///
    /// Returns: steady-state temperature in °C
    pub fn steady_state_temperature(&self) -> f64 {
        self.T_ambient + self.Q_heat * self.R_th
    }

    /// Calculate analytical solution for indoor temperature
    ///
    /// For constant Q_heat and T_ambient, the solution is:
    /// T_indoor(t) = T_ss + (T_0 - T_ss) * exp(-t/τ)
    ///
    /// where:
    /// - T_ss = T_ambient + Q_heat * R_th (steady-state temperature)
    /// - T_0 = initial indoor temperature
    /// - τ = R_th * C_th (time constant)
    ///
    /// # Arguments
    ///
    /// * `r_th` - Thermal resistance [K/W]
    /// * `c_th` - Thermal capacitance [J/K]
    /// * `t_ambient` - Ambient temperature [°C]
    /// * `q_heat` - Heating power [W]
    /// * `t_initial` - Initial indoor temperature [°C]
    /// * `t` - Time [s]
    ///
    /// # Returns
    ///
    /// Indoor temperature at time t [°C]
    pub fn analytical_solution(
        r_th: f64,
        c_th: f64,
        t_ambient: f64,
        q_heat: f64,
        t_initial: f64,
        t: f64,
    ) -> f64 {
        let tau = r_th * c_th;
        let t_ss = t_ambient + q_heat * r_th;
        t_ss + (t_initial - t_ss) * (-t / tau).exp()
    }

    /// Calculate total energy stored in the thermal mass relative to ambient
    ///
    /// E = C_th * (T_indoor - T_ambient)
    ///
    /// Returns: stored energy in Joules
    pub fn stored_energy(&self) -> f64 {
        self.C_th * (self.T_indoor - self.T_ambient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_initialization() {
        let model = RCThermalSingleZone::new();

        // Test default parameter values
        assert_eq!(model.R_th, 0.01);
        assert_eq!(model.C_th, 10_000_000.0);
        assert_eq!(model.T_ambient, 0.0);
        assert_eq!(model.Q_heat, 5000.0);

        // Test default state
        assert_eq!(model.T_indoor, 20.0);

        // Test initial time
        assert_eq!(model.time, 0.0);

        // Test derived outputs are calculated
        let expected_q_loss = (20.0 - 0.0) / 0.01;
        assert_relative_eq!(model.Q_loss, expected_q_loss, epsilon = 1e-6);
    }

    #[test]
    fn test_time_constant() {
        let model = RCThermalSingleZone::new();
        let tau = model.time_constant();

        assert_eq!(tau, 0.01 * 10_000_000.0);
        assert_eq!(tau, 100_000.0); // ~28 hours
    }

    #[test]
    fn test_steady_state_temperature() {
        let model = RCThermalSingleZone::new();
        let t_ss = model.steady_state_temperature();

        // T_ss = T_ambient + Q_heat * R_th = 0.0 + 5000.0 * 0.01 = 50.0°C
        assert_eq!(t_ss, 50.0);
    }

    #[test]
    fn test_steady_state_temperature_with_ambient() {
        let mut model = RCThermalSingleZone::new();
        model.T_ambient = 10.0;

        let t_ss = model.steady_state_temperature();

        // T_ss = 10.0 + 5000.0 * 0.01 = 60.0°C
        assert_eq!(t_ss, 60.0);
    }

    #[test]
    fn test_heat_loss_calculation() {
        let model = RCThermalSingleZone::new();

        // Q_loss = (T_indoor - T_ambient) / R_th
        // Q_loss = (20.0 - 0.0) / 0.01 = 2000.0 W
        assert_eq!(model.Q_loss, 2000.0);
    }

    #[test]
    fn test_derivative_calculation() {
        let model = RCThermalSingleZone::new();

        // dT/dt = Q_heat/C_th - Q_loss/C_th
        // dT/dt = 5000/10e6 - 2000/10e6 = 0.0005 - 0.0002 = 0.0003 K/s
        let expected_der = 5000.0 / 10_000_000.0 - 2000.0 / 10_000_000.0;
        assert_relative_eq!(model.dT_dt, expected_der, epsilon = 1e-10);
        assert_relative_eq!(model.dT_dt, 0.0003, epsilon = 1e-10);
    }

    #[test]
    fn test_one_step() {
        let mut model = RCThermalSingleZone::new();

        let initial_t = model.T_indoor;
        let dt = 1.0; // 1 second

        model.do_step(0.0, dt);

        // After one step: T_new = T_old + dT/dt * dt
        // T_new = 20.0 + 0.0003 * 1.0 = 20.0003
        assert_relative_eq!(model.T_indoor, 20.0003, epsilon = 1e-10);
        assert!(model.T_indoor > initial_t); // Should be warming up
    }

    #[test]
    fn test_no_heating_cools_down() {
        let mut model = RCThermalSingleZone::new();
        model.Q_heat = 0.0;
        model.T_indoor = 20.0;
        model.T_ambient = 0.0;
        model.update_derived_outputs();

        let initial_t = model.T_indoor;

        model.do_step(0.0, 1.0);

        // Without heating, indoor should cool down toward ambient
        assert!(model.T_indoor < initial_t);
    }

    #[test]
    fn test_heating_warms_up() {
        let mut model = RCThermalSingleZone::new();
        model.Q_heat = 10_000.0; // Strong heating
        model.T_indoor = 0.0;
        model.T_ambient = 0.0;
        model.update_derived_outputs();

        let initial_t = model.T_indoor;

        model.do_step(0.0, 1.0);

        // With heating and starting cold, should warm up
        assert!(model.T_indoor > initial_t);
    }

    #[test]
    fn test_equilibrium_no_change() {
        let mut model = RCThermalSingleZone::new();

        // Set T_indoor to steady-state temperature
        model.T_indoor = model.steady_state_temperature();
        model.update_derived_outputs();

        // At steady state, Q_heat should equal Q_loss
        assert_relative_eq!(model.Q_heat, model.Q_loss, epsilon = 1e-6);

        // dT/dt should be zero
        assert_relative_eq!(model.dT_dt, 0.0, epsilon = 1e-10);

        // Taking a step should not change temperature (much)
        let initial_t = model.T_indoor;
        model.do_step(0.0, 0.1);

        assert_relative_eq!(model.T_indoor, initial_t, epsilon = 1e-8);
    }

    #[test]
    fn test_analytical_solution_at_t_zero() {
        let t_initial = 20.0;
        let result = RCThermalSingleZone::analytical_solution(
            0.01,
            10_000_000.0,
            0.0,
            5000.0,
            t_initial,
            0.0,
        );

        // At t=0, should return initial temperature
        assert_eq!(result, t_initial);
    }

    #[test]
    fn test_analytical_solution_at_infinity() {
        // At very large time, should approach steady state
        let t_very_large = 1_000_000.0; // Much larger than tau=100,000s
        let result = RCThermalSingleZone::analytical_solution(
            0.01,
            10_000_000.0,
            0.0,
            5000.0,
            20.0,
            t_very_large,
        );

        let t_ss = 0.0 + 5000.0 * 0.01; // 50.0°C
        assert_relative_eq!(result, t_ss, epsilon = 0.01); // Within 0.01°C
    }

    #[test]
    fn test_stored_energy() {
        let model = RCThermalSingleZone::new();

        // E = C_th * (T_indoor - T_ambient)
        // E = 10e6 * (20 - 0) = 200,000,000 J
        let expected_energy = 10_000_000.0 * 20.0;
        assert_eq!(model.stored_energy(), expected_energy);
    }

    #[test]
    fn test_parameters_affect_time_constant() {
        let mut model1 = RCThermalSingleZone::new();
        let mut model2 = RCThermalSingleZone::new();

        model1.R_th = 0.01;
        model1.C_th = 10_000_000.0;

        model2.R_th = 0.02; // Double the resistance
        model2.C_th = 10_000_000.0;

        let tau1 = model1.time_constant();
        let tau2 = model2.time_constant();

        // Doubling R_th should double time constant
        assert_eq!(tau2, 2.0 * tau1);
    }
}
