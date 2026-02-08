//! Single-Zone 1R1C Thermal RC Model
//!
//! This model implements a simplified building thermal model using an
//! electrical circuit analogy (RC network).
//!
//! # Physics
//!
//! The model represents a single thermal zone with:
//! - One thermal resistance (R) representing the building envelope [K/W]
//! - One thermal capacitance (C) representing the thermal mass [J/K]
//!
//! The governing equation is:
//!
//! C * dT_indoor/dt = (T_outdoor - T_indoor) / R + Q_heating
//!
//! where:
//! - T_indoor: Indoor temperature [°C]
//! - T_outdoor: Outdoor ambient temperature [°C]
//! - R: Thermal resistance of building envelope [K/W]
//! - C: Thermal capacitance of building [J/K]
//! - Q_heating: Heating power input [W]
//!
//! # Steady-State Solution
//!
//! At steady state (dT/dt = 0):
//! T_indoor = T_outdoor + Q_heating * R
//!
//! # Time Constant
//!
//! The thermal time constant is τ = R * C [s], which characterizes
//! how quickly the building responds to temperature changes.

#![allow(non_snake_case)]

use fmi::fmi3::{Fmi3Error, Fmi3Res};
use fmi_export::fmi3::{CSDoStepResult, Context, DefaultLoggingCategory, UserModel};
use fmi_export::FmuModel;

/// Single-zone 1R1C thermal model
///
/// Models a building's thermal behavior as an RC circuit.
#[derive(FmuModel, Default, Debug)]
#[model(co_simulation = true, user_model = false)]
pub struct RcThermalSingleZone {
    // === Parameters (settable via FMI) ===
    /// Thermal resistance of building envelope [K/W]
    #[variable(causality = Parameter, start = 0.01, initial = Exact)]
    pub R: f64,

    /// Thermal capacitance of building [J/K]
    #[variable(causality = Parameter, start = 1000000.0, initial = Exact)]
    pub C: f64,

    /// Outdoor ambient temperature [°C]
    #[variable(causality = Parameter, start = 0.0, initial = Exact)]
    pub T_outdoor: f64,

    /// Heating power input [W]
    #[variable(causality = Parameter, start = 0.0, initial = Exact)]
    pub Q_heating: f64,

    // === State Variables (outputs) ===
    /// Indoor temperature [°C]
    #[variable(causality = Output, start = 20.0, initial = Exact)]
    pub T_indoor: f64,

    /// Heat flow through envelope [W] (positive = heat loss)
    #[variable(causality = Output, start = 0.0, initial = Calculated)]
    pub Q_envelope: f64,

    /// Net heat flow into zone [W]
    #[variable(causality = Output, start = 0.0, initial = Calculated)]
    pub Q_net: f64,

    /// Derivative of indoor temperature
    #[variable(causality = Local, derivative = T_indoor, initial = Calculated)]
    pub der_T_indoor: f64,
}

impl UserModel for RcThermalSingleZone {
    type LoggingCategory = DefaultLoggingCategory;

    fn configurate(&mut self, _context: &dyn Context<Self>) -> Result<(), Fmi3Error> {
        // Calculate initial derived outputs
        self.update_derived_outputs();
        Ok(())
    }

    fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
        self.update_derived_outputs();
        Ok(Fmi3Res::OK)
    }

    fn do_step(
        &mut self,
        context: &mut dyn Context<Self>,
        current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Result<CSDoStepResult, Fmi3Error> {
        // Calculate heat flows
        self.update_derived_outputs();

        // Euler integration: dT/dt = Q_net / C
        self.der_T_indoor = self.Q_net / self.C;
        self.T_indoor += self.der_T_indoor * communication_step_size;

        let target_time = current_communication_point + communication_step_size;
        context.set_time(target_time);
        Ok(CSDoStepResult::completed(target_time))
    }
}

fmi_export::export_fmu!(RcThermalSingleZone);

impl RcThermalSingleZone {
    /// Create a new RC thermal model with default parameters
    pub fn new() -> Self {
        let mut model = Self {
            R: 0.01,
            C: 1_000_000.0,
            T_outdoor: 0.0,
            Q_heating: 0.0,
            T_indoor: 20.0,
            Q_envelope: 0.0,
            Q_net: 0.0,
            der_T_indoor: 0.0,
        };
        model.update_derived_outputs();
        model
    }

    /// Update derived outputs (heat flow calculations)
    fn update_derived_outputs(&mut self) {
        // Heat flow through envelope: positive means heat loss to outside
        self.Q_envelope = (self.T_indoor - self.T_outdoor) / self.R;

        // Net heat flow into zone
        self.Q_net = self.Q_heating - self.Q_envelope;

        // Temperature derivative
        self.der_T_indoor = self.Q_net / self.C;
    }

    /// Calculate thermal time constant τ = R * C [s]
    pub fn time_constant(&self) -> f64 {
        self.R * self.C
    }

    /// Calculate steady-state indoor temperature
    ///
    /// At steady state: T_indoor = T_outdoor + Q_heating * R
    pub fn steady_state_temperature(&self) -> f64 {
        self.T_outdoor + self.Q_heating * self.R
    }

    /// Analytical solution for step response
    ///
    /// For a step change in heating power:
    /// T(t) = T_ss + (T_0 - T_ss) * exp(-t / τ)
    ///
    /// where:
    /// - T_ss is the steady-state temperature
    /// - T_0 is the initial temperature
    /// - τ = R * C is the time constant
    pub fn analytical_step_response(
        T_0: f64,
        T_outdoor: f64,
        Q_heating: f64,
        R: f64,
        C: f64,
        t: f64,
    ) -> f64 {
        let tau = R * C;
        let T_ss = T_outdoor + Q_heating * R;
        T_ss + (T_0 - T_ss) * (-t / tau).exp()
    }

    /// Calculate stored thermal energy [J] relative to 0°C reference
    pub fn stored_energy(&self) -> f64 {
        self.C * self.T_indoor
    }

    /// Analytical solution with parameter-first signature
    ///
    /// This provides a convenient static interface for tests:
    /// T(t) = T_ss + (T_0 - T_ss) * exp(-t / τ)
    pub fn analytical_solution(
        R: f64,
        C: f64,
        T_outdoor: f64,
        Q_heating: f64,
        T_0: f64,
        t: f64,
    ) -> f64 {
        Self::analytical_step_response(T_0, T_outdoor, Q_heating, R, C, t)
    }

    /// Perform a single Euler integration step (for testing without FMI context)
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        self.update_derived_outputs();
        self.T_indoor += self.der_T_indoor * time_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let model = RcThermalSingleZone::new();
        assert_eq!(model.R, 0.01);
        assert_eq!(model.C, 1_000_000.0);
        assert_eq!(model.T_outdoor, 0.0);
        assert_eq!(model.Q_heating, 0.0);
        assert_eq!(model.T_indoor, 20.0);
    }

    #[test]
    fn test_time_constant() {
        let model = RcThermalSingleZone::new();
        let tau = model.time_constant();
        assert!((tau - 10_000.0).abs() < 1e-6); // R * C = 0.01 * 1e6 = 10000
    }

    #[test]
    fn test_steady_state() {
        let mut model = RcThermalSingleZone::new();
        model.T_outdoor = 5.0;
        model.Q_heating = 1000.0;

        let T_ss = model.steady_state_temperature();
        // T_ss = T_outdoor + Q_heating * R = 5 + 1000 * 0.01 = 15°C
        assert!((T_ss - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_cooling_without_heating() {
        let mut model = RcThermalSingleZone::new();
        model.T_indoor = 20.0;
        model.T_outdoor = 0.0;
        model.Q_heating = 0.0;

        let initial_temp = model.T_indoor;

        // Simulate for some time
        let dt = 10.0; // 10 second steps
        for _ in 0..100 {
            model.do_step(0.0, dt);
        }

        // Temperature should decrease towards outdoor temperature
        assert!(model.T_indoor < initial_temp);
        assert!(model.T_indoor > model.T_outdoor);
    }

    #[test]
    fn test_heating_response() {
        let mut model = RcThermalSingleZone::new();
        model.T_indoor = 15.0;
        model.T_outdoor = 0.0;
        model.Q_heating = 2000.0;

        // Steady state should be T_outdoor + Q * R = 0 + 2000 * 0.01 = 20°C
        // Since T_indoor (15) < T_ss (20), temperature should rise
        let initial_temp = model.T_indoor;

        let dt = 10.0;
        for _ in 0..100 {
            model.do_step(0.0, dt);
        }

        assert!(model.T_indoor > initial_temp);
    }

    #[test]
    fn test_analytical_step_response() {
        let R = 0.01;
        let C = 1_000_000.0;

        // At t=0, should equal initial temperature
        let T_0 = RcThermalSingleZone::analytical_step_response(20.0, 0.0, 1000.0, R, C, 0.0);
        assert!((T_0 - 20.0).abs() < 1e-10);

        // At t=∞, should approach steady state
        let T_inf = RcThermalSingleZone::analytical_step_response(20.0, 0.0, 1000.0, R, C, 1e10);
        let T_ss = 0.0 + 1000.0 * R; // = 10.0
        assert!((T_inf - T_ss).abs() < 1e-6);
    }

    #[test]
    fn test_heat_flow_calculation() {
        let mut model = RcThermalSingleZone::new();
        model.T_indoor = 20.0;
        model.T_outdoor = 0.0;
        model.Q_heating = 0.0;
        model.update_derived_outputs();

        // Q_envelope = (T_indoor - T_outdoor) / R = 20 / 0.01 = 2000 W
        assert!((model.Q_envelope - 2000.0).abs() < 1e-6);

        // Q_net = Q_heating - Q_envelope = 0 - 2000 = -2000 W
        assert!((model.Q_net - (-2000.0)).abs() < 1e-6);
    }
}
