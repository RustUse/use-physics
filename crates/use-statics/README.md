# use-statics

Small statics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-statics = "0.0.1"
```

## Foundation

`use-statics` provides small helpers for force balance, moment balance, equilibrium checks,
static friction, inclined planes, and simple support reactions.

Inputs are expected to be SI-style numeric values:

- newtons for force and load
- meters for span, position, and moment arm
- newton-meters for moment
- kilograms for mass
- radians for incline angle
- newtons per meter for distributed load

The crate stays narrow on purpose. It is not a structural analysis engine, finite element solver,
truss solver, frame solver, rigid-body physics engine, or simulation framework.

Vector operations should live in or compose with `use-vector`. Torque-specific helpers belong in
`use-torque`. Rigid-body mechanics belong in `use-rigidbody`. Simulation belongs in top-level
`use-simulation`.

## Example

```rust
use use_statics::{
    CantileverReaction, Force2D, StaticSystem2D, cantilever_end_point_load_reaction,
    simply_supported_point_load_reactions,
};

let Some(system) = StaticSystem2D::new(
    vec![
        Force2D::new(100.0, 0.0).unwrap(),
        Force2D::new(-100.0, 0.0).unwrap(),
    ],
    vec![0.0],
) else {
    panic!("valid system should construct");
};

let Some((left, right)) = simply_supported_point_load_reactions(10.0, 100.0, 5.0) else {
    panic!("valid point load should produce reactions");
};

let Some(cantilever) = cantilever_end_point_load_reaction(2.0, 50.0) else {
    panic!("valid cantilever load should produce a reaction");
};

assert_eq!(system.is_equilibrium(0.0), Some(true));
assert_eq!((left, right), (50.0, 50.0));
assert_eq!(
    cantilever,
    CantileverReaction {
        vertical_reaction: 50.0,
        fixed_end_moment: 100.0,
    }
);
```

## When to use directly

Choose `use-statics` when you want small, direct helpers for introductory or utility-level statics
work without pulling in a broader mechanics surface.

## Scope

- APIs stay dependency-free and `f64`-first.
- Helpers focus on scalar formulas and simple 2D force and moment balances.
- Arbitrary beam loading, truss analysis, frame analysis, FEA, CAD, and design-code compliance are
  out of scope.
- Broad vector algebra belongs in `use-vector` or companion math crates.

## Status

`use-statics` is a pre-1.0 crate with a deliberately small API.
