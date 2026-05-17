# use-momentum

Linear momentum, impulse, recoil, and one-dimensional collision helpers for `RustUse`.

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
use use_momentum::{MovingMass, final_velocity_after_sticking_collision, momentum, recoil_velocity};

assert_eq!(momentum(2.0, 3.0), Some(6.0));

let final_velocity = final_velocity_after_sticking_collision(2.0, 3.0, 4.0, -1.0).unwrap();
assert!((final_velocity - 0.333_333_333_333_333_3).abs() < 1.0e-12);

assert_eq!(recoil_velocity(1.0, 10.0, 5.0), Some(-2.0));
assert_eq!(MovingMass::new(2.0, 3.0).unwrap().momentum(), Some(6.0));
```

## When to use directly

Choose `use-momentum` when you need small, reusable helpers for scalar momentum, impulse, recoil, and basic one-dimensional collisions.

## Scope

- APIs stay `f64`-first and focus on one-dimensional scalar helpers.
- Higher-dimensional vector operations should live in or compose with `use-vector`.
- Full rigid-body simulation, restitution modeling, and broader physics engines are out of scope.

## Status

`use-momentum` is a pre-1.0 crate with a deliberately small API.
