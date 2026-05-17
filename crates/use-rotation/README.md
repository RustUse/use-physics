# use-rotation

Rotational motion, angular momentum, and moment-of-inertia helpers for `RustUse`.

## Install

```toml
[dependencies]
use-rotation = "0.0.1"
```

## Foundation

`use-rotation` provides small `f64`-first helpers for rotational motion and rotational dynamics.

Inputs are expected to be SI-style numeric values:

- radians for angular displacement
- radians per second for angular velocity
- radians per second squared for angular acceleration
- meters for radius and length
- kilograms for mass
- kilogram square meters for moment of inertia
- kilogram square meters per second for angular momentum
- joules for rotational kinetic energy
- newton-meters for torque

The crate does not define a full unit system.

Vector operations should live in or compose with `use-vector`.

Torque-specific scalar helpers belong in `use-torque`.

## Example

```rust
use use_rotation::{
    AngularState, RotatingBody, angular_velocity, rotational_kinetic_energy,
    solid_disk_moment_of_inertia, tangential_speed,
};

assert_eq!(angular_velocity(10.0, 2.0), Some(5.0));
assert_eq!(tangential_speed(3.0, 2.0), Some(6.0));
assert_eq!(solid_disk_moment_of_inertia(2.0, 3.0), Some(9.0));
assert_eq!(rotational_kinetic_energy(4.0, 5.0), Some(50.0));

let body = RotatingBody::new(4.0, 5.0).unwrap();
assert_eq!(body.angular_momentum(), Some(20.0));

let state = AngularState::new(1.0, 2.0)
    .unwrap()
    .advanced_by_constant_acceleration(3.0, 4.0)
    .unwrap();

assert_eq!(state.angular_position, 33.0);
assert_eq!(state.angular_velocity, 14.0);
```

## When to use directly

Choose `use-rotation` when you need small, reusable helpers for scalar angular motion, angular momentum, rotational energy, centripetal relations, and common moments of inertia.

## Scope

- APIs stay `f64`-first and focus on scalar rotational helpers.
- Higher-dimensional vector operations should live in or compose with `use-vector`.
- Broad torque-specific helper APIs belong in `use-torque`.
- Full rigid-body dynamics, tensors, and simulation engines are out of scope.

## Status

`use-rotation` is a pre-1.0 crate with a deliberately small API.
