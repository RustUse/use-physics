# use-gravity

Gravity-specific helpers for `RustUse`.

## Install

```toml
[dependencies]
use-gravity = "0.0.1"
```

## Foundation

`use-gravity` provides small `f64`-first helpers for gravitational force, gravitational acceleration, orbital velocity, escape velocity, orbital period, and gravitational potential energy.

Inputs are expected to be SI-style numeric values:

- kilograms for mass
- meters for distance, radius, and height
- seconds for time
- newtons for force
- joules for energy

## Example

```rust
use use_gravity::{GravityBody, escape_velocity, gravitational_force};

let earth = GravityBody::new(5.972e24, 6.371e6).unwrap();
let force = gravitational_force(1.0, 1.0, 1.0).unwrap();
let escape = escape_velocity(5.972e24, 6.371e6).unwrap();

assert_eq!(force, use_gravity::GRAVITATIONAL_CONSTANT);
assert!(earth.surface_gravity().unwrap() > 9.8);
assert!(escape > 11_000.0);
```

## When to use directly

Choose `use-gravity` when you only need reusable gravity and orbit formulas.

## Scope

- APIs stay `f64`-first and do not define a full unit system.
- The crate keeps only a couple of convenience constants locally.
- Broader constants and units belong in the top-level `use-constants` and `use-units` sets.

## Status

`use-gravity` is a pre-1.0 crate with a deliberately small API.
