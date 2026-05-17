# use-electromagnetism

Small scalar helpers for combined electric and magnetic field relations in `RustUse`.

## Install

```toml
[dependencies]
use-electromagnetism = "0.0.1"
```

## Foundation

`use-electromagnetism` provides small scalar helpers for combined electric and magnetic field relations, Lorentz-force convenience calculations, velocity selectors, cyclotron motion, field energy density, Poynting magnitude, and simple electromagnetic constant relationships.

Inputs are expected to be SI-style numeric values:

- coulombs for charge
- volts per meter for electric field
- meters per second for velocity and speed
- teslas for magnetic flux density
- newtons for force
- kilograms for mass
- joules per cubic meter for energy density
- watts per square meter for Poynting magnitude

The crate keeps only a few local convenience constants. Broader physical constants belong in the top-level `use-constants` set.

## Example

```rust
use use_electromagnetism::{
    ElectromagneticField, lorentz_force_scalar, velocity_selector_speed,
};

let field = ElectromagneticField::new(10.0, 2.0).unwrap();

assert_eq!(velocity_selector_speed(20.0, 4.0), Some(5.0));
assert_eq!(
    lorentz_force_scalar(1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2),
    Some(16.0)
);
assert!(field.energy_density().unwrap() > 0.0);
```

## When to use directly

Choose `use-electromagnetism` when you need small reusable scalar helpers that combine electric and magnetic field relations in one place.

Use `use-electricity` for electricity-specific scalar helpers. Use `use-magnetism` for magnetism-specific scalar helpers.

## Scope

- APIs stay `f64`-first and do not define a full unit system.
- This crate is not a full vector calculus, field theory, Maxwell-equation solver, vector-field engine, optics system, wave system, or signal-processing crate.
- General constants belong in `use-constants`.
- Unit abstractions belong in `use-units`.
- Wave, signal, and optics abstractions belong in the top-level `use-wave`, `use-signal`, and `use-optics` sets.

## Status

`use-electromagnetism` is a pre-1.0 crate with a deliberately small scalar API.
