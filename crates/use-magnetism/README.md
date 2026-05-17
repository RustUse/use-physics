# use-magnetism

Magnetism-specific scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-magnetism = "0.0.1"
```

## Foundation

`use-magnetism` provides small magnetism-specific scalar helpers for magnetic force, magnetic flux, magnetic flux density, solenoid and loop fields, magnetic pressure, and simple magnetic field primitives.

Inputs are expected to be SI-style numeric values:

- teslas for magnetic flux density
- webers for magnetic flux
- square meters for area
- coulombs for charge
- meters per second for velocity
- amperes for current
- meters for length, radius, and distance
- newtons for force
- joules per cubic meter for energy density
- pascals for magnetic pressure

The crate does not define a full unit system.

More general units and constants belong in the top-level `use-units` and `use-constants` sets. Vector operations should live in or compose with `use-vector`. Combined electric and magnetic relations should live in a separate `use-electromagnetism` crate.

## Example

```rust
use std::f64::consts::FRAC_PI_2;

use use_magnetism::{MagneticField, magnetic_field_inside_solenoid, magnetic_flux};

assert_eq!(magnetic_flux(2.0, 3.0, 0.0), Some(6.0));
assert!(magnetic_field_inside_solenoid(1_000.0, 2.0, 0.5).unwrap() > 0.0);

let field = MagneticField::new(3.0).expect("valid field");
assert_eq!(field.force_on_charge(1.0, 2.0, FRAC_PI_2), Some(6.0));
```

## When to use directly

Choose `use-magnetism` when you need reusable scalar magnetism formulas without bringing in a broader unit, vector, or electromagnetism system.

## Scope

- APIs stay `f64`-first and deliberately small.
- The crate keeps only one local convenience constant for magnetism-specific helpers.
- Full unit systems, waveform models, signal processing, and general vector operations are out of scope.

## Status

`use-magnetism` is a pre-1.0 crate with a deliberately small API.
