# use-thermodynamics

Ideal gas and heat-energy helpers for `RustUse`.

## Install

```toml
[dependencies]
use-thermodynamics = "0.0.1"
```

## Foundation

`use-thermodynamics` provides small `f64`-first helpers for ideal gas and heat-energy calculations.

## Example

```rust
use use_thermodynamics::{celsius_to_kelvin, heat_energy, ideal_gas_pressure};

let pressure = ideal_gas_pressure(2.0, 300.0, 3.0)?;

assert!((pressure - 1_662.892_523_630_648).abs() < 1.0e-12);
assert_eq!(celsius_to_kelvin(0.0), 273.15);
assert_eq!(heat_energy(2.0, 4.0, 5.0), 40.0);
# Ok::<(), use_thermodynamics::ThermodynamicsError>(())
```

## When to use directly

Choose `use-thermodynamics` when you only need reusable thermodynamics formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Phase transitions and property tables are out of scope.

## Status

`use-thermodynamics` is a pre-1.0 crate with a deliberately small API.
