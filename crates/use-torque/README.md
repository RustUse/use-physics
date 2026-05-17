# use-torque

Torque-specific scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-torque = "0.0.1"
```

## Foundation

`use-torque` provides small scalar helpers for torque, lever-arm calculations, perpendicular force components, rotational equilibrium, angular acceleration from torque, and a few basic moment-of-inertia formulas.

Inputs are expected to be SI-style numeric values:

- newtons for force
- meters for lever arm and radius
- newton-meters for torque
- radians for angle helpers unless the function name says degrees
- kilograms for mass
- kilogram square meters for moment of inertia
- radians per second squared for angular acceleration

The crate does not define a full unit system. Vector operations should live in or compose with `use-vector`, and broader rotational motion should live in a separate `use-rotation` crate.

## Example

```rust
use use_torque::{is_rotational_equilibrium, torque, torque_at_angle_degrees};

let angled_torque = torque_at_angle_degrees(10.0, 2.0, 30.0).unwrap();

assert_eq!(torque(10.0, 2.0), Some(20.0));
assert!((angled_torque - 10.0).abs() < 1.0e-12);
assert_eq!(is_rotational_equilibrium(&[10.0, -10.0], 1.0e-6), Some(true));
```

## When to use directly

Choose `use-torque` when you only need reusable torque formulas and simple rotational balance helpers.

## Scope

- APIs stay dependency-free and `f64`-first.
- The crate does not define a full unit system.
- Vector math belongs in or alongside `use-vector`.
- Broader rotational motion, angular momentum, and rotational kinetic energy belong in a separate `use-rotation` crate.

## Status

`use-torque` is a pre-1.0 crate with a deliberately small API.
