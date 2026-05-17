# use-electricity

Electricity-specific scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-electricity = "0.0.1"
```

## Foundation

`use-electricity` provides small helpers for charge, current, voltage, resistance, conductance, electrical power, electrical energy, Coulomb force, and simple series or parallel resistance calculations.

Inputs are expected to be SI-style numeric values:

- coulombs for charge
- amperes for current
- seconds for time
- volts for voltage
- ohms for resistance
- siemens for conductance
- watts for power
- joules for energy
- meters for distance
- newtons for force

The crate does not define a full unit system. More general units and constants belong in the top-level `use-units` and `use-constants` sets.

## Example

```rust
use use_electricity::{ElectricalLoad, power_from_voltage_current, voltage};

assert_eq!(voltage(2.0, 5.0), Some(10.0));
assert_eq!(power_from_voltage_current(10.0, 2.0), Some(20.0));

let load = ElectricalLoad::new(10.0, 5.0).expect("valid load");
assert_eq!(load.power(), Some(20.0));
```

## When to use directly

Choose `use-electricity` when you need reusable scalar electricity formulas without bringing in a broader circuit or unit system.

## Scope

- APIs stay `f64`-first and deliberately small.
- Circuit analysis is intentionally minimal and limited to simple scalar helpers.
- Broader constants, waveform modeling, signal processing, and measurement systems are out of scope.

## Status

`use-electricity` is a pre-1.0 crate with a deliberately small API.
