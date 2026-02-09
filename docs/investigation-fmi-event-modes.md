# Investigation: FMI Event Modes in ADML Bouncing Ball

## Summary

The ADML bouncing ball model does **not** use FMI 3.0 event modes. All collision
handling is done inline within the custom `do_step()` implementation. The
`fmi-export` crate (v0.1.1) **does** support the full FMI 3.0 event mode
protocol, so the infrastructure is available but unused.

## Current ADML Implementation

**File:** `models/mechanical/bouncing-ball/src/lib.rs`

The bouncing ball declares zero event indicators and handles collisions entirely
inside `do_step()` via a 1ms fixed micro-stepping loop:

```rust
// Line 97-107
if self.h <= 0.0 && self.v < 0.0 {
    self.h = f64::MIN_POSITIVE;
    self.v = -self.v * self.e;
    if self.v < self.v_min {
        self.v = 0.0;
        self.h = 0.0;
        self.stopped = true;
        break;
    }
}
```

- `MAX_EVENT_INDICATORS = 0` (no fields with `event_indicator = true`)
- `event_update()` not overridden (uses default no-op)
- `get_event_indicators()` not overridden (uses default no-op)
- `CSDoStepResult::event_handling_needed` always `false`

## Reference-FMUs Implementation

**Source:** https://github.com/modelica/Reference-FMUs — `BouncingBall/model.c`

The reference implementation cleanly separates event detection from event
handling using the FMI event mode protocol:

### Event Indicator (`getEventIndicators`)

```c
// 1 event indicator: z[0] = h (with hysteresis)
Status getEventIndicators(ModelInstance *comp, double z[], size_t nz) {
    if (M(h) > -EVENT_EPSILON && M(h) <= 0 && M(v) > 0) {
        z[0] = -EVENT_EPSILON;  // hysteresis to prevent chattering
    } else {
        z[0] = M(h);
    }
    return OK;
}
```

The integrator (internal for CS, external for ME) monitors `z[0]` for sign
changes. A negative-going zero-crossing triggers an event.

### Event Update (`eventUpdate`)

```c
Status eventUpdate(ModelInstance *comp) {
    if (M(h) <= 0 && M(v) < 0) {
        M(h) = DBL_MIN;
        M(v) = -M(v) * M(e);
        if (M(v) < V_MIN) {
            M(v) = 0;
            M(g) = 0;  // Note: sets gravity to 0
        }
        comp->valuesOfContinuousStatesChanged = true;
    } else {
        comp->valuesOfContinuousStatesChanged = false;
    }
    comp->nominalsOfContinuousStatesChanged = false;
    comp->terminateSimulation  = false;
    comp->nextEventTimeDefined = false;
    return OK;
}
```

### Configuration

```c
#define MAX_EVENT_INDICATORS 1
#define EVENT_UPDATE
```

### Model Exchange Flow

The external integrator follows this sequence on event detection:

1. Integrator calls `fmi3GetEventIndicators()` at each step
2. Detects sign change in `z[0]` (height crosses zero)
3. Calls `fmi3EnterEventMode()`
4. Calls `fmi3UpdateDiscreteStates()` → triggers `eventUpdate()`
5. Reads back flags: `valuesOfContinuousStatesChanged = true`
6. Calls `fmi3EnterContinuousTimeMode()`
7. Reinitializes integrator with new continuous states

### Co-Simulation Flow

The internal fixed-step solver (`cosimulation.c`) handles this automatically:

1. Micro-steps with `FIXED_SOLVER_STEP = 1e-3`
2. Calls `getEventIndicators()` after each step
3. Compares current `z[]` with previous `prez[]` for sign changes
4. On sign change: calls `eventUpdate()`, reinitializes states

## What `fmi-export` Supports

The `fmi-export` 0.1.1 crate provides the full FMI 3.0 event mode API:

### UserModel Trait Methods (Available for Override)

```rust
// src/fmi3/traits/mod.rs

fn event_update(
    &mut self,
    _context: &dyn Context<Self>,
    event_flags: &mut EventFlags,
) -> Result<Fmi3Res, Fmi3Error>;

fn get_event_indicators(
    &mut self,
    _context: &dyn Context<Self>,
    indicators: &mut [f64],
) -> Result<bool, Fmi3Error>;
```

### Generated FFI Functions

- `fmi3EnterEventMode` → transitions to `ModelState::EventMode`
- `fmi3UpdateDiscreteStates` → calls `model.event_update()`
- `fmi3GetEventIndicators` → calls `model.get_event_indicators()`
- `fmi3GetNumberOfEventIndicators` → returns `M::MAX_EVENT_INDICATORS`
- `fmi3EnterContinuousTimeMode` → transitions back to continuous mode

### Derive Macro Support

Fields can be annotated with `#[variable(event_indicator = true)]` to set
`MAX_EVENT_INDICATORS`.

### CSDoStepResult

```rust
pub struct CSDoStepResult {
    pub event_handling_needed: bool,  // Signal CS master to enter event mode
    pub terminate_simulation: bool,
    pub early_return: bool,
    pub last_successful_time: f64,
}
```

### Known Limitation

There is a TODO in `common.rs:93-94` where Co-Simulation event mode is
hardcoded off after initialization:

```rust
//TODO support event mode switch
let event_mode_used = false;
```

This affects the initial state transition only. The `enter_event_mode()` function
itself works and can be called later during simulation.

## Comparison

| Aspect | ADML | Reference-FMUs |
|--------|------|----------------|
| Event indicators | 0 | 1 (`h` with hysteresis) |
| Event detection | Inline check in `do_step()` | Zero-crossing on indicator sign change |
| Event handling | Manual in step loop | Separate `eventUpdate()` callback |
| `valuesOfContinuousStatesChanged` | Never set | Set `true` after bounce |
| Stop behavior | `v=0, h=0, stopped=true` | `v=0, g=0` |
| ME event protocol | Not used | Full enter/update/leave cycle |
| CS event signaling | Never signals | Internal solver detects and handles |
| Hysteresis | None | `EVENT_EPSILON = 1e-10` |

## Consequences of Current Approach

1. **Model Exchange is incomplete:** An external ME integrator has no event
   indicators to monitor, so it cannot detect bounces. The ME interface is
   generated by `fmi-export` but events will never fire.

2. **Co-Simulation accuracy:** Events are caught only at 1ms fixed-step
   boundaries. The exact bounce time is not located — the ball may penetrate
   slightly below `h=0` before the collision check fires on the next step. The
   `h < 0 → h = 0` snap on line 123 masks this.

3. **No event signaling to master:** `CSDoStepResult::event_handling_needed` is
   always `false`, so a co-simulation master cannot know that a discrete state
   change occurred during the step.

## Potential Improvement

The bouncing ball could be refactored to use proper event modes:

1. Mark `h` as an event indicator (or add a dedicated indicator field)
2. Implement `get_event_indicators()` returning `h` with hysteresis
3. Implement `event_update()` for velocity reversal and stopping logic
4. In `do_step()`, detect zero-crossings on the indicator, return
   `event_handling_needed: true` at the crossing time
5. This would make ME work correctly and give CS masters event visibility

The `fmi-export` infrastructure is ready — the model just needs to use it.
