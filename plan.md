# ADML Terminal Ontology Plan

## Background

rust-fmi (fmi-export v0.2 / fmi-schema v0.7) now supports FMI 3.0 terminals via:

- **Struct-level** `#[terminal(matching_rule = "...", terminal_kind = "...")]` — marks a struct as a terminal definition
- **Field-level** `#[terminal(name = "...")]` — names a child terminal
- **Field-level** `#[child(prefix = "...")]` — includes a child struct's variables with a prefix
- **`TerminalProvider` trait** — auto-generated, produces `fmi_schema::fmi3::Terminal` structs
- **Automatic packaging** — `cargo fmi` embeds `terminalsAndIcons/terminalsAndIcons.xml` in the FMU

## Problem

ADML's 6 models expose flat variables with no semantic grouping. There is no way for a tool to know that the RC Thermal model's `T_indoor` and `Q_heating` form a thermal interface, or that the bouncing ball's `h` and `v` form a translational mechanical interface. This prevents automatic connection and interoperability between ADML models and third-party FMUs.

## Proposed Ontology

Create an `adml-ontology` crate (`crates/adml-ontology/`) defining reusable terminal types following physical domain conventions (inspired by Modelica connectors). Use reverse domain notation for `terminalKind` per FMI 3.0.2 recommendations.

### Terminal Types

#### 1. Thermal Heat Port
```rust
/// Thermal interface: temperature + heat flow
#[derive(FmuModel, Default, Debug)]
#[terminal(matching_rule = "plug", terminal_kind = "org.adml.thermal.heatport")]
pub struct HeatPort {
    /// Temperature [K or °C]
    #[variable(causality = Output, start = 293.15, initial = Exact)]
    pub T: f64,

    /// Heat flow rate [W] (positive = into component)
    #[variable(causality = Output, start = 0.0, initial = Calculated)]
    pub Q_flow: f64,
}
```

#### 2. Mechanical Translational Flange
```rust
/// 1D translational interface: position + velocity
#[derive(FmuModel, Default, Debug)]
#[terminal(matching_rule = "plug", terminal_kind = "org.adml.mechanical.translational")]
pub struct TranslationalFlange {
    /// Position [m]
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub s: f64,

    /// Velocity [m/s]
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub v: f64,
}
```

#### 3. Mechanical Rotational Flange
```rust
/// 1D rotational interface: angle + angular velocity
#[derive(FmuModel, Default, Debug)]
#[terminal(matching_rule = "plug", terminal_kind = "org.adml.mechanical.rotational")]
pub struct RotationalFlange {
    /// Angle [rad]
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub phi: f64,

    /// Angular velocity [rad/s]
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub omega: f64,
}
```

#### 4. Real Signal (causal, non-physical)
```rust
/// Scalar real signal for causal connections
#[derive(FmuModel, Default, Debug)]
#[terminal(matching_rule = "plug", terminal_kind = "org.adml.signal.real")]
pub struct RealSignal {
    /// Signal value [–]
    #[variable(causality = Output, start = 0.0, initial = Exact)]
    pub value: f64,
}
```

### How Models Would Use This

**RC Thermal (before):**
```rust
pub struct RcThermalSingleZone {
    #[variable(causality = Parameter, start = 0.01, initial = Exact)]
    pub R: f64,
    #[variable(causality = Parameter, start = 1000000.0, initial = Exact)]
    pub C: f64,
    #[variable(causality = Parameter, start = 0.0, initial = Exact)]
    pub T_outdoor: f64,
    #[variable(causality = Parameter, start = 0.0, initial = Exact)]
    pub Q_heating: f64,
    #[variable(causality = Output, start = 20.0, initial = Exact)]
    pub T_indoor: f64,
    // ...
}
```

**RC Thermal (after):**
```rust
pub struct RcThermalSingleZone {
    #[variable(causality = Parameter, start = 0.01, initial = Exact)]
    pub R: f64,
    #[variable(causality = Parameter, start = 1000000.0, initial = Exact)]
    pub C: f64,

    /// Outdoor boundary condition
    #[child]
    #[terminal(name = "outdoor")]
    pub outdoor: HeatPort,    // outdoor.T, outdoor.Q_flow

    /// Indoor zone thermal port
    #[child]
    #[terminal(name = "indoor")]
    pub indoor: HeatPort,     // indoor.T, indoor.Q_flow

    #[variable(causality = Local, derivative = indoor.T, initial = Calculated)]
    der_T_indoor: f64,
}
```

**Bouncing Ball (after):**
```rust
pub struct BouncingBall {
    #[variable(causality = Parameter, start = -9.81, initial = Exact)]
    pub g: f64,
    #[variable(causality = Parameter, start = 0.7, initial = Exact)]
    pub e: f64,

    /// Ball position and velocity
    #[child]
    #[terminal(name = "ball")]
    pub ball: TranslationalFlange,   // ball.s, ball.v

    #[variable(causality = Local, derivative = ball.s, initial = Calculated)]
    der_h: f64,
    #[variable(causality = Local, derivative = ball.v, initial = Calculated)]
    der_v: f64,
    // ...
}
```

**Simple Pendulum (after):**
```rust
pub struct SimplePendulum {
    #[variable(causality = Parameter, start = 9.81, initial = Exact)]
    pub g: f64,
    // ...

    /// Pendulum shaft
    #[child]
    #[terminal(name = "shaft")]
    pub shaft: RotationalFlange,   // shaft.phi, shaft.omega

    // Energy outputs remain flat
    #[variable(causality = Output, initial = Calculated)]
    pub energy: f64,
    // ...
}
```

### Generated terminalsAndIcons.xml (RC Thermal example)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<fmiTerminalsAndIcons fmiVersion="3.0">
  <Terminals>
    <Terminal name="outdoor"
             terminalKind="org.adml.thermal.heatport"
             matchingRule="plug">
      <TerminalMemberVariable variableName="outdoor.T"
                             memberName="outdoor.T"
                             variableKind="signal"/>
      <TerminalMemberVariable variableName="outdoor.Q_flow"
                             memberName="outdoor.Q_flow"
                             variableKind="signal"/>
    </Terminal>
    <Terminal name="indoor"
             terminalKind="org.adml.thermal.heatport"
             matchingRule="plug">
      <TerminalMemberVariable variableName="indoor.T"
                             memberName="indoor.T"
                             variableKind="signal"/>
      <TerminalMemberVariable variableName="indoor.Q_flow"
                             memberName="indoor.Q_flow"
                             variableKind="signal"/>
    </Terminal>
  </Terminals>
</fmiTerminalsAndIcons>
```

## Implementation Steps

1. **Create `crates/adml-ontology/`** — new crate with terminal type definitions
2. **Add workspace dependency** — add `adml-ontology` to workspace Cargo.toml
3. **Refactor RC Thermal model** — use `HeatPort` terminals (proof of concept)
4. **Refactor Bouncing Ball** — use `TranslationalFlange`
5. **Refactor Simple Pendulum** — use `RotationalFlange`
6. **Mathematical models** — use `RealSignal` for Dahlquist/VanDerPol/Lorenz outputs
7. **Update tests** — adjust field access paths (e.g., `model.indoor.T` vs `model.T_indoor`)
8. **Update AGENTS.md** — document terminal usage for new models
9. **Verify FMU builds** — ensure `cargo fmi` produces valid terminalsAndIcons.xml

## Open Questions / Risks

1. **Derivative references to child fields** — `#[variable(causality = Local, derivative = indoor.T)]` may not work with dotted paths in the current derive macro. Need to verify. If not supported, we may need to keep derivative state fields flat and manually sync them with the terminal port fields.

2. **Variable name changes are breaking** — renaming `T_indoor` → `indoor.T` changes the FMU's variable names. This is acceptable for ADML since it's a young project, but worth noting.

3. **Input vs Output causality** — Physical ports in Modelica use acausal connections (effort + flow variables). FMI3 is inherently causal. The terminal types expose outputs; the connecting tool decides who reads what. This is the standard FMI3 approach.

4. **Mathematical models** — Wrapping Lorenz's x/y/z in signal terminals adds structure but may feel heavy for pure mathematical test cases. Could be optional.

5. **fmi-export version** — The `#[terminal]` and `#[child]` attributes are present in fmi-export 0.2.0 based on the derive macro source code, but need to verify they work correctly at runtime since there are no usage examples in the rust-fmi test suite.
