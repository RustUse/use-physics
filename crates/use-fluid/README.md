# use-fluid

Small fluid mechanics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-fluid = "0.0.1"
```

## Foundation

`use-fluid` provides small fluid mechanics helpers for buoyancy, hydrostatic pressure, flow rate, continuity, Bernoulli-style relations, viscosity, Reynolds number, and simple drag calculations.

Inputs are expected to be SI-style numeric values:

- kilograms per cubic meter for density
- cubic meters for volume
- meters per second squared for acceleration
- newtons for force
- pascals for pressure
- meters for depth, height, and characteristic length
- square meters for area
- cubic meters per second for volumetric flow rate
- kilograms per second for mass flow rate
- pascal-seconds for dynamic viscosity
- square meters per second for kinematic viscosity

This crate does not define a full unit system.

More general units and constants belong in the top-level `use-units` and `use-constants` sets.

Material databases belong in top-level `use-materials`.

This crate is not a CFD solver.

## Example

```rust
use use_fluid::{Fluid, PipeFlow, dynamic_pressure};

let water = Fluid::with_dynamic_viscosity(1000.0, 0.001).unwrap();
let flow = PipeFlow::new(2.0, 3.0).unwrap();

assert_eq!(flow.volumetric_flow_rate(), Some(6.0));
assert_eq!(flow.mass_flow_rate(water.density), Some(6000.0));
assert_eq!(dynamic_pressure(water.density, 3.0), Some(4500.0));
```

## When to use directly

Choose `use-fluid` when you only need reusable fluid mechanics formulas without a full units or simulation stack.

## Scope

- APIs stay `f64`-first and do not define a full unit system.
- Simple scalar helpers only; CFD, turbulence modeling, and material datasets are out of scope.
- Broader units, constants, and materials belong in the top-level `RustUse` sets.

## Status

`use-fluid` is a pre-1.0 crate with a deliberately small API.
