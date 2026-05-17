# use-rigidbody

Rigid-body mass properties and scalar mechanics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-rigidbody = "0.0.1"
```

## Foundation

`use-rigidbody` provides small `f64`-first helpers for scalar rigid-body mechanics primitives:
mass properties, center of mass, moments of inertia, kinetic energy, angular momentum, simple
impulse response, and small rigid-body state helpers.

Inputs are expected to be SI-style numeric values:

- kilograms for mass
- meters for position, radius, length, and distance
- meters per second for velocity
- radians for angle
- radians per second for angular velocity
- kilogram square meters for moment of inertia
- kilogram meters per second for linear momentum
- kilogram square meters per second for angular momentum
- joules for kinetic energy
- newton-seconds for impulse

Vector mechanics should compose with `use-vector`.

Simulation loops belong in top-level `use-simulation`.

Collision-specific helpers belong in `use-collision`.

Rotation-specific helpers belong in `use-rotation`.

Torque-specific helpers belong in `use-torque`.

## Example

```rust
use use_rigidbody::{MassProperties, RigidBody1D, total_kinetic_energy};

let props = MassProperties::solid_sphere(5.0, 2.0).unwrap();
let body = RigidBody1D::new(props, 10.0, 3.0, 1.0, 5.0).unwrap();

assert_eq!(body.linear_momentum(), Some(15.0));
assert_eq!(body.rotational_kinetic_energy(), Some(100.0));
assert_eq!(total_kinetic_energy(5.0, 3.0, 8.0, 5.0), Some(122.5));
```

## When to use directly

Choose `use-rigidbody` when you need small, reusable scalar helpers for rigid-body mass
properties, inertia, momentum, kinetic energy, and simple kinematic state updates.

## Scope

- APIs stay `f64`-first and focus on scalar rigid-body mechanics primitives.
- This crate is not a physics engine, collision detector, contact solver, constraint solver, game
  physics package, or simulation framework.
- Vector rigid-body dynamics, joints, broad phase, narrow phase, and simulation loops are out of
  scope.

## Status

`use-rigidbody` is a pre-1.0 crate with a deliberately small API.
