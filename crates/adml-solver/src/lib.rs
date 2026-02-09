//! Reusable numerical solvers for ADML FMU models.
//!
//! Provides generic forward Euler and symplectic Euler solvers that work with
//! any model implementing `UserModel + ModelGetSetStates` from fmi-export.
//!
//! # Usage
//!
//! Models define their equations once in `calculate_values()`, then use a macro
//! to generate the `do_step` method with micro-stepping:
//!
//! ```ignore
//! impl UserModel for MyModel {
//!     type LoggingCategory = DefaultLoggingCategory;
//!
//!     fn calculate_values(&mut self, _context: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
//!         self.der_x = -self.k * self.x;
//!         Ok(Fmi3Res::OK)
//!     }
//!
//!     adml_solver::euler_cs_step!(0.001); // 1ms fixed step
//! }
//! ```

use fmi::fmi3::Fmi3Error;
use fmi_export::fmi3::{CSDoStepResult, Context, ModelGetSetStates, UserModel};

/// Perform forward Euler integration with micro-stepping.
///
/// Integrates the model's continuous states from `current_communication_point`
/// to `current_communication_point + communication_step_size` using fixed-step
/// forward Euler with step size `fixed_step`.
///
/// The model's `calculate_values()` is called at each micro-step to update
/// derivatives before integration. This is the same function used by ME mode,
/// so equations are defined only once.
///
/// For models with `NUM_STATES == 0`, this simply advances time.
pub fn euler_step<M: UserModel + ModelGetSetStates>(
    model: &mut M,
    context: &mut dyn Context<M>,
    current_communication_point: f64,
    communication_step_size: f64,
    fixed_step: f64,
) -> Result<CSDoStepResult, Fmi3Error> {
    let t_end = current_communication_point + communication_step_size;

    if M::NUM_STATES == 0 {
        context.set_time(t_end);
        return Ok(CSDoStepResult::completed(t_end));
    }

    let mut x = vec![0.0; M::NUM_STATES];
    let mut dx = vec![0.0; M::NUM_STATES];
    let mut t = current_communication_point;

    while t_end - t > f64::EPSILON * t_end.abs().max(1.0) {
        let dt = (t_end - t).min(fixed_step);

        // Update derivatives at current state
        model.calculate_values(context)?;
        model.get_continuous_states(&mut x)?;
        model.get_continuous_state_derivatives(&mut dx)?;

        // Forward Euler: x_{n+1} = x_n + dt * dx/dt
        for i in 0..M::NUM_STATES {
            x[i] += dx[i] * dt;
        }

        model.set_continuous_states(&x)?;
        t += dt;
        context.set_time(t);
    }

    Ok(CSDoStepResult::completed(t_end))
}

/// Perform symplectic (semi-implicit) Euler integration with micro-stepping.
///
/// This solver assumes the state vector is organized as pairs of
/// (position, velocity) variables, where odd-indexed derivatives depend on
/// even-indexed states (positions) and even-indexed derivatives are simply
/// the velocities (odd-indexed states).
///
/// The integration order is:
/// 1. Compute derivatives at current state via `calculate_values()`
/// 2. Update velocities (odd-indexed states) using current derivatives
/// 3. Update positions (even-indexed states) using the **new** velocities
///
/// This preserves energy for conservative systems, unlike forward Euler.
///
/// # State ordering convention
///
/// States must be ordered as: `[pos_0, vel_0, pos_1, vel_1, ...]`
/// where `der(pos_i) = vel_i` and `der(vel_i) = f(pos_i, vel_i, ...)`.
pub fn symplectic_euler_step<M: UserModel + ModelGetSetStates>(
    model: &mut M,
    context: &mut dyn Context<M>,
    current_communication_point: f64,
    communication_step_size: f64,
    fixed_step: f64,
) -> Result<CSDoStepResult, Fmi3Error> {
    let t_end = current_communication_point + communication_step_size;

    if M::NUM_STATES == 0 {
        context.set_time(t_end);
        return Ok(CSDoStepResult::completed(t_end));
    }

    assert!(
        M::NUM_STATES % 2 == 0,
        "symplectic Euler requires an even number of states (position/velocity pairs)"
    );

    let mut x = vec![0.0; M::NUM_STATES];
    let mut dx = vec![0.0; M::NUM_STATES];
    let mut t = current_communication_point;

    while t_end - t > f64::EPSILON * t_end.abs().max(1.0) {
        let dt = (t_end - t).min(fixed_step);

        // Update derivatives at current state
        model.calculate_values(context)?;
        model.get_continuous_states(&mut x)?;
        model.get_continuous_state_derivatives(&mut dx)?;

        // Symplectic Euler: update velocities first, then positions
        for pair in 0..(M::NUM_STATES / 2) {
            let pos_idx = pair * 2;
            let vel_idx = pair * 2 + 1;

            // Step 1: Update velocity using current acceleration
            x[vel_idx] += dx[vel_idx] * dt;

            // Step 2: Update position using NEW velocity
            x[pos_idx] += x[vel_idx] * dt;
        }

        model.set_continuous_states(&x)?;
        t += dt;
        context.set_time(t);
    }

    Ok(CSDoStepResult::completed(t_end))
}

/// Generates a `do_step` method that uses forward Euler integration
/// with micro-stepping at the given fixed step size.
///
/// Place this inside your `impl UserModel for MyModel` block.
/// The model must also implement `ModelGetSetStates` (auto-generated
/// by the `FmuModel` derive macro from `#[variable(derivative = x)]`).
///
/// # Example
///
/// ```ignore
/// impl UserModel for Dahlquist {
///     type LoggingCategory = DefaultLoggingCategory;
///     fn calculate_values(&mut self, _ctx: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
///         self.der_x = -self.k * self.x;
///         Ok(Fmi3Res::OK)
///     }
///     adml_solver::euler_cs_step!(0.001);
/// }
/// ```
#[macro_export]
macro_rules! euler_cs_step {
    ($fixed_step:expr) => {
        fn do_step(
            &mut self,
            context: &mut dyn fmi_export::fmi3::Context<Self>,
            current_communication_point: f64,
            communication_step_size: f64,
            _no_set_fmu_state_prior_to_current_point: bool,
        ) -> Result<fmi_export::fmi3::CSDoStepResult, fmi::fmi3::Fmi3Error> {
            $crate::euler_step(
                self,
                context,
                current_communication_point,
                communication_step_size,
                $fixed_step,
            )
        }
    };
}

/// Generates a `do_step` method that uses symplectic (semi-implicit) Euler
/// integration with micro-stepping at the given fixed step size.
///
/// This is ideal for Hamiltonian systems (pendulums, springs, orbital mechanics)
/// where energy conservation is important.
///
/// States must be ordered as `[pos_0, vel_0, pos_1, vel_1, ...]`.
///
/// # Example
///
/// ```ignore
/// impl UserModel for SimplePendulum {
///     type LoggingCategory = DefaultLoggingCategory;
///     fn calculate_values(&mut self, _ctx: &dyn Context<Self>) -> Result<Fmi3Res, Fmi3Error> {
///         self.der_theta = self.omega;
///         self.der_omega = -(self.g / self.L) * self.theta.sin();
///         Ok(Fmi3Res::OK)
///     }
///     adml_solver::symplectic_euler_cs_step!(0.001);
/// }
/// ```
#[macro_export]
macro_rules! symplectic_euler_cs_step {
    ($fixed_step:expr) => {
        fn do_step(
            &mut self,
            context: &mut dyn fmi_export::fmi3::Context<Self>,
            current_communication_point: f64,
            communication_step_size: f64,
            _no_set_fmu_state_prior_to_current_point: bool,
        ) -> Result<fmi_export::fmi3::CSDoStepResult, fmi::fmi3::Fmi3Error> {
            $crate::symplectic_euler_step(
                self,
                context,
                current_communication_point,
                communication_step_size,
                $fixed_step,
            )
        }
    };
}
