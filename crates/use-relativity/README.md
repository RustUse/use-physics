# use-relativity

Small special relativity scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-relativity = "0.0.1"
```

## Foundation

`use-relativity` provides small special relativity scalar helpers for Lorentz factor, time dilation,
length contraction, mass-energy relations, relativistic momentum, rapidity, velocity addition,
and simple longitudinal Doppler calculations.

Inputs are expected to be SI-style numeric values:

- meters per second for speed and velocity
- seconds for time
- meters for length
- kilograms for mass
- joules for energy
- kilogram meters per second for momentum
- hertz for frequency

Positive beta in the Doppler helpers means the source is approaching the observer.

This crate keeps `SPEED_OF_LIGHT` locally for convenience. Broader physical constants belong in the
top-level `use-constants` set.

Unit abstractions belong in the top-level `use-units` set.

## Example

```rust
use use_relativity::{RelativisticBody, SPEED_OF_LIGHT, beta, dilated_time, velocity_addition};

assert_eq!(beta(SPEED_OF_LIGHT * 0.5), Some(0.5));
assert!((dilated_time(10.0, SPEED_OF_LIGHT * 0.6).unwrap() - 12.5).abs() < 1.0e-12);
assert!((velocity_addition(SPEED_OF_LIGHT * 0.5, SPEED_OF_LIGHT * 0.5).unwrap()
    - (SPEED_OF_LIGHT * 0.8))
    .abs()
    < 1.0e-3);

let body = RelativisticBody::new(1.0, SPEED_OF_LIGHT * 0.6).unwrap();

assert!(body.total_energy().unwrap() > body.rest_energy().unwrap());
```

## When to use directly

Choose `use-relativity` when you need small scalar special relativity helpers without a full unit,
geometry, or simulation system.

## Scope

- APIs stay `f64`-first and focus on scalar special relativity helpers.
- General relativity, tensor calculus, curved spacetime, and numerical simulation are out of scope.
- Full constants systems and unit abstractions belong in `use-constants` and `use-units`.

## Status

`use-relativity` is a pre-1.0 crate with a deliberately small API.
