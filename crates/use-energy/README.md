# use-energy

Work and mechanical energy helpers for `RustUse`.

## Install

```toml
[dependencies]
use-energy = "0.0.1"
```

## Foundation

`use-energy` provides small `f64`-first helpers for work, kinetic energy, and potential energy.

## Example

```rust
use use_energy::{kinetic_energy, work};

assert_eq!(kinetic_energy(2.0, 3.0), 9.0);
assert_eq!(work(5.0, 10.0), 50.0);
```

## When to use directly

Choose `use-energy` when you only need reusable energy formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Field solvers and advanced thermodynamics are out of scope.

## Status

`use-energy` is a pre-1.0 crate with a deliberately small API.
