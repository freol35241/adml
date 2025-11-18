# Single-Zone 1R1C Thermal RC Model

A simplified thermal network model for building/house heating systems using lumped parameter analysis.

## Model Description

This model represents a building as a simple thermal system with one thermal zone (the interior space). It uses an electrical-thermal analogy where thermal resistance and capacitance behave like resistors and capacitors in an RC circuit.

### Background

RC thermal models are widely used in building energy simulation and HVAC control because they:
- Capture essential thermal dynamics with minimal parameters
- Enable fast simulation suitable for real-time control
- Provide intuitive physical interpretation
- Scale well to multi-zone buildings

This single-zone model is the simplest useful thermal building model, suitable for small buildings or as a starting point for more complex multi-zone models.

## Mathematical Formulation

The model implements the following thermal energy balance:

$$C_{th} \frac{dT_{indoor}}{dt} = Q_{heat} - Q_{loss}$$

where the heat loss through the building envelope is:

$$Q_{loss} = \frac{T_{indoor} - T_{ambient}}{R_{th}}$$

Combining these:

$$\frac{dT_{indoor}}{dt} = \frac{Q_{heat}}{C_{th}} - \frac{T_{indoor} - T_{ambient}}{R_{th} \cdot C_{th}}$$

### Physical Interpretation

- **$R_{th}$**: Thermal resistance representing the insulation quality of walls, roof, windows, etc.
- **$C_{th}$**: Thermal capacitance representing the heat storage capacity of air, walls, furniture, etc.
- **$Q_{heat}$**: Heat input from the heating system (furnace, heat pump, etc.)
- **$Q_{loss}$**: Heat loss to the environment through the building envelope
- **$T_{indoor}$**: Indoor air temperature (state variable)
- **$T_{ambient}$**: Outdoor/ambient temperature (external input)

### Key Properties

**Time Constant:**
$$\tau = R_{th} \cdot C_{th}$$

The time constant represents how quickly the building responds to changes. After time $\tau$, the temperature reaches approximately 63.2% of its final steady-state value.

**Steady-State Temperature:**

When $\frac{dT_{indoor}}{dt} = 0$:

$$T_{indoor,ss} = T_{ambient} + Q_{heat} \cdot R_{th}$$

**Analytical Solution:**

For constant $Q_{heat}$ and $T_{ambient}$:

$$T_{indoor}(t) = T_{ss} + (T_0 - T_{ss}) e^{-t/\tau}$$

where $T_0$ is the initial temperature and $T_{ss}$ is the steady-state temperature.

### Assumptions

- Single thermal zone (uniform indoor temperature)
- Constant thermal properties ($R_{th}$, $C_{th}$)
- Linear heat transfer (valid for typical temperature ranges)
- No solar gains, internal gains, or infiltration (can be added to $Q_{heat}$)
- Well-mixed air (no stratification)

## Parameters

| Name | Symbol | Type | Default | Units | Description |
|------|--------|------|---------|-------|-------------|
| R_th | $R_{th}$ | Real | 0.01 | K/W | Thermal resistance (insulation quality) |
| C_th | $C_{th}$ | Real | 10,000,000 | J/K | Thermal capacitance (thermal mass) |
| T_ambient | $T_{ambient}$ | Real | 0.0 | °C | Outdoor/ambient temperature |
| Q_heat | $Q_{heat}$ | Real | 5000.0 | W | Heating power input |

### Typical Parameter Values

**Small well-insulated house (~100 m²):**
- $R_{th}$ ≈ 0.01 K/W (good insulation)
- $C_{th}$ ≈ 10,000,000 J/K (≈10 MJ/K)
- $\tau$ ≈ 100,000 s ≈ 28 hours
- $Q_{heat}$ ≈ 5,000 W (5 kW heating capacity)

**Poorly insulated building:**
- $R_{th}$ ≈ 0.003 K/W (poor insulation)
- Requires more heating power for same indoor temperature

**Heavy thermal mass building (concrete):**
- $C_{th}$ ≈ 50,000,000 J/K
- Slower temperature response, better thermal stability

## State Variables and Outputs

| Name | Symbol | Type | Initial | Units | Description |
|------|--------|------|---------|-------|-------------|
| T_indoor | $T_{indoor}$ | Real | 20.0 | °C | Indoor temperature (state variable) |
| Q_loss | $Q_{loss}$ | Real | - | W | Heat loss through building envelope (derived) |
| dT_dt | $\frac{dT}{dt}$ | Real | - | K/s | Rate of temperature change (derived) |

## FMI Interface

### Inputs
This model uses **parameters** rather than time-varying inputs for simplicity and FMI compatibility. Parameters remain constant during each simulation run but can be varied between runs.

### Outputs
- `T_indoor` : Indoor temperature (primary state variable)
- `Q_loss` : Heat loss to environment (derived output)
- `dT_dt` : Rate of temperature change (derived output)

### Parameters
All parameters (`R_th`, `C_th`, `T_ambient`, `Q_heat`) can be set before simulation starts.

## Usage

### Building the FMU

```bash
./scripts/build-fmu.sh models/thermal/rc-thermal-single-zone
```

The FMU file will be created at: `fmus/RcThermalSingleZone.fmu`

### Example Simulation (Python with FMPy)

```python
import fmpy
import matplotlib.pyplot as plt

# Simulate heating from cold start
result = fmpy.simulate_fmu(
    'fmus/RcThermalSingleZone.fmu',
    stop_time=200000,  # ~55 hours (2 time constants)
    step_size=10.0,     # 10 second steps
    output_interval=1000.0,  # Output every 1000s
    start_values={
        'R_th': 0.01,
        'C_th': 10000000.0,
        'T_ambient': 0.0,
        'Q_heat': 5000.0
    }
)

# Plot results
plt.figure(figsize=(12, 8))

plt.subplot(2, 1, 1)
plt.plot(result['time'] / 3600, result['T_indoor'], 'b-', label='Indoor Temperature')
plt.axhline(y=50.0, color='r', linestyle='--', label='Steady-State (50°C)')
plt.axhline(y=0.0, color='k', linestyle=':', label='Ambient (0°C)')
plt.xlabel('Time [hours]')
plt.ylabel('Temperature [°C]')
plt.legend()
plt.grid(True)
plt.title('Building Heating Response')

plt.subplot(2, 1, 2)
plt.plot(result['time'] / 3600, result['Q_heat'], 'g-', label='Heat Input')
plt.plot(result['time'] / 3600, result['Q_loss'], 'r-', label='Heat Loss')
plt.xlabel('Time [hours]')
plt.ylabel('Power [W]')
plt.legend()
plt.grid(True)
plt.title('Energy Balance')

plt.tight_layout()
plt.show()
```

### Example: Comparing Insulation Quality

```python
import fmpy
import matplotlib.pyplot as plt

# Simulate with poor insulation
result_poor = fmpy.simulate_fmu(
    'fmus/RcThermalSingleZone.fmu',
    stop_time=300000,
    step_size=10.0,
    output_interval=1000.0,
    start_values={'R_th': 0.005, 'Q_heat': 5000.0}
)

# Simulate with good insulation
result_good = fmpy.simulate_fmu(
    'fmus/RcThermalSingleZone.fmu',
    stop_time=300000,
    step_size=10.0,
    output_interval=1000.0,
    start_values={'R_th': 0.02, 'Q_heat': 5000.0}
)

plt.figure(figsize=(10, 6))
plt.plot(result_poor['time'] / 3600, result_poor['T_indoor'],
         'r-', label='Poor Insulation (R=0.005 K/W)')
plt.plot(result_good['time'] / 3600, result_good['T_indoor'],
         'g-', label='Good Insulation (R=0.02 K/W)')
plt.xlabel('Time [hours]')
plt.ylabel('Indoor Temperature [°C]')
plt.legend()
plt.grid(True)
plt.title('Effect of Insulation Quality on Indoor Temperature')
plt.show()
```

### Running Tests

```bash
# Rust tests (unit + physics validation)
cargo test -p adml-rc-thermal-single-zone

# Build FMU first
./scripts/build-fmu.sh models/thermal/rc-thermal-single-zone

# FMU integration tests (requires FMU to be built)
pytest testing/fmu-integration-tests/test_rc_thermal_single_zone_fmu.py -v
```

## Physics Validation

This model is validated against:

1. **Analytical Solution** - Compared to exact exponential solution: $T_{indoor}(t) = T_{ss} + (T_0 - T_{ss}) e^{-t/\tau}$
   - Tested with heating from cold start
   - Tested with cooling down (no heating)
   - Tested with various parameter combinations

2. **Energy Balance** - Verified that $Q_{heat} = Q_{loss}$ at steady state

3. **Time Constant Verification** - Confirmed that system reaches 63.2% of final value after time $\tau$

4. **Physical Properties** - Tested that:
   - Better insulation (higher $R_{th}$) → higher steady-state temperature
   - Larger thermal mass (higher $C_{th}$) → slower response
   - Without heating, temperature approaches ambient

5. **Convergence** - Solution converges with decreasing step size (first-order Euler integration)

See `tests/physics_tests.rs` for comprehensive test suite.

## Implementation Notes

### Numerical Method

Uses explicit Euler integration:
- First-order accurate: $O(\Delta t)$
- Requires reasonably small time steps (recommend $\Delta t \leq 10$ s for typical buildings)
- Unconditionally stable for this system (linear, dissipative)

For typical building parameters (τ ≈ 28 hours), step sizes of 1-10 seconds provide good accuracy.

### AI Implementation

This model was implemented by **Claude Sonnet 4.5** on 2025-11-18 as the first thermal model in the ADML library.

#### Implementation Strategy

1. **Scaffolding** - Used templates from `docs/AI_SCAFFOLDING.md`
2. **Physics Implementation** - Implemented energy balance equation with Euler integration
3. **Validation** - Compared against analytical solution for exponential response
4. **Testing Approach** - Comprehensive physics tests covering:
   - Analytical solution matching (multiple scenarios)
   - Steady-state behavior and energy balance
   - Time constant verification
   - Parameter sensitivity
   - Edge cases and boundary conditions

#### Challenges Encountered

**Input vs Parameter Handling:**
- Initially planned to make `T_ambient` and `Q_heat` as FMI inputs (time-varying)
- Discovered `fmu_from_struct` primarily supports parameters for Co-Simulation
- Solution: Implemented as parameters (constant per simulation run)
- Users can still vary these between runs or use external controllers

**Long Time Constants:**
- Building time constants are very large (hours to days)
- Required careful selection of simulation times for tests (multiple time constants)
- Solution: Automated calculation based on τ in tests

#### Verification Strategy

1. **Analytical comparison** - Primary validation method
2. **Energy balance** - Secondary check at steady state
3. **Physical properties** - Verified expected behaviors (insulation quality, thermal mass)
4. **Convergence testing** - Confirmed numerical accuracy with step size refinement

## Applications

This model is suitable for:

- **HVAC control development** - Testing heating controllers and algorithms
- **Energy analysis** - Estimating heating energy consumption
- **Model-predictive control (MPC)** - Fast simulation for optimization
- **Education** - Teaching building thermal dynamics
- **Baseline for complex models** - Foundation for multi-zone models

## Extensions and Future Work

Potential enhancements:
- **Multi-zone model** - Multiple rooms with inter-zone heat transfer
- **2R2C or higher-order models** - Separate envelope and interior thermal mass
- **Solar gains** - Add solar radiation input
- **Infiltration and ventilation** - Include air exchange heat loss
- **Nonlinear effects** - Temperature-dependent properties
- **Cooling mode** - Add air conditioning model

## References

1. ISO 52016-1:2017 - Energy performance of buildings - Calculation procedures
2. Bacher, P., & Madsen, H. (2011). "Identifying suitable models for the heat dynamics of buildings." *Energy and Buildings*, 43(7), 1511-1522.
3. Ramallo-González, A. P., & Coley, D. A. (2014). "Using self-adaptive optimisation methods to perform sequential optimisation for low-energy building design." *Energy and Buildings*, 81, 18-29.
4. Reynders, G., Diriken, J., & Saelens, D. (2014). "Quality of grey-box models and identified parameters as function of the accuracy of input and observation signals." *Energy and Buildings*, 82, 263-274.

## Version History

- **1.0.0** (2025-11-18): Initial implementation by Claude Sonnet 4.5
  - Single-zone 1R1C thermal network
  - Analytical solution validation
  - Comprehensive physics tests
  - FMI 3.0 Co-Simulation export

## License

Dual licensed under MIT and Apache 2.0 (see repository root).
