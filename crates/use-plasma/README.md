# use-plasma

Small scalar plasma physics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-plasma = "0.0.1"
```

## Foundation

`use-plasma` provides small scalar plasma physics helpers for plasma frequency, Debye length,
Debye number, thermal speed, gyrofrequency, gyroradius, plasma pressure, magnetic pressure,
plasma beta, Alfven speed, charge density, and quasi-neutrality checks.

Inputs are expected to be SI-style numeric values:

- particles per cubic meter for number density
- kelvin for temperature
- teslas for magnetic flux density
- kilograms for mass
- kilograms per cubic meter for mass density
- meters for Debye length and gyroradius
- radians per second for angular frequency
- hertz for frequency
- pascals for pressure
- coulombs per cubic meter for charge density

The crate keeps only a few local convenience constants. Broader physical constants belong in the
top-level `use-constants` set. Unit abstractions belong in the top-level `use-units` set.
Electromagnetic field helpers belong in `use-electromagnetism`, and particle metadata belongs in
`use-particle`.

## Example

```rust
use use_plasma::{ElectronPlasma, alfven_speed};

# fn main() -> Result<(), &'static str> {
let plasma = ElectronPlasma::new(1.0e18, 10_000.0).ok_or("expected valid plasma")?;

assert!(plasma.debye_length().is_some_and(|value| value > 0.0));
assert!(plasma.plasma_frequency().is_some_and(|value| value > 0.0));
assert!(alfven_speed(0.01, 1.0e-10).is_some_and(|value| value >= 0.0));
# Ok(())
# }
```

## When to use directly

Choose `use-plasma` when you need reusable scalar plasma formulas without bringing in a larger
simulation stack or a broader plasma modeling framework.

## Scope

- APIs stay dependency-free, `f64`-first, and intentionally small.
- This crate is not a plasma simulation engine, particle-in-cell framework,
  magnetohydrodynamics solver, radiation transport tool, or materials database.
- Detailed collisional transport models, plasma chemistry, and reaction networks are out of scope.

## Status

`use-plasma` is a pre-1.0 crate with a deliberately small scalar API.
