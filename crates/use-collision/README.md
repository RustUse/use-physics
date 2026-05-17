# use-collision

One-dimensional collision, restitution, impulse, and kinetic-energy helpers for `RustUse`.

## Install

```toml
[dependencies]
use-collision = "0.0.1"
```

## Foundation

`use-collision` provides small, dependency-free helpers for scalar one-dimensional collision calculations.

Inputs are expected to be SI-style numeric values:

- kilograms for mass
- meters per second for velocity
- kilogram meters per second for momentum
- joules for kinetic energy
- newton-seconds for impulse

The coefficient of restitution is modeled as a scalar in `[0.0, 1.0]`.

## Example

```rust
use use_collision::{Collision1D, CollisionBody1D};

let body_a = CollisionBody1D::new(1.0, 1.0).unwrap();
let body_b = CollisionBody1D::new(1.0, -1.0).unwrap();
let collision = Collision1D::new(body_a, body_b, 1.0).unwrap();

assert_eq!(collision.final_velocities(), Some((-1.0, 1.0)));
assert_eq!(collision.kinetic_energy_loss(), Some(0.0));
```

## When to use directly

Choose `use-collision` when you need small, reusable helpers for one-dimensional collision outcomes, restitution, collision impulse, and kinetic-energy changes.

## Scope

- The crate focuses on scalar collision relations, coefficient of restitution, kinetic-energy changes, and impulse.
- Momentum and broader impulse utilities belong in `use-momentum`.
- Vector operations belong in `use-vector`.
- Simulation loops belong in top-level `use-simulation`.
- Rigid-body engines, contact solvers, vector mechanics, and game-physics systems are out of scope.

## Status

`use-collision` is a pre-1.0 crate with a deliberately small API.
