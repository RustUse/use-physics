# use-momentum

Linear momentum, impulse, and recoil helpers for `RustUse`.

## Install

```toml
[dependencies]
use-momentum = "0.0.1"
```

## Foundation

`use-momentum` provides small `f64`-first helpers for linear momentum and impulse calculations.

Inputs are expected to be SI-style numeric values:

- kilograms for mass
- meters per second for velocity
- kilogram meters per second for momentum
- newton seconds for impulse
- seconds for time

The crate does not define a full unit system.

## Example

```rust
use use_momentum::{MovingMass, impulse, momentum, recoil_velocity};

assert_eq!(momentum(2.0, 3.0), Some(6.0));

assert_eq!(impulse(10.0, 2.0), Some(20.0));

assert_eq!(recoil_velocity(1.0, 10.0, 5.0), Some(-2.0));
assert_eq!(MovingMass::new(2.0, 3.0).unwrap().momentum(), Some(6.0));
```

## When to use directly

Choose `use-momentum` when you need small, reusable helpers for scalar momentum, impulse, and recoil.

## Scope

- APIs stay `f64`-first and focus on one-dimensional scalar helpers.
- One-dimensional collision outcomes and restitution helpers belong in `use-collision`.
- Higher-dimensional vector operations should live in or compose with `use-vector`.
- Full rigid-body simulation and broader physics engines are out of scope.

## Status

`use-momentum` is a pre-1.0 crate with a deliberately small API.
