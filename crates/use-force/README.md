# use-force

Force, weight, and impulse helpers for `RustUse`.

## Install

```toml
[dependencies]
use-force = "0.0.1"
```

## Foundation

`use-force` provides small `f64`-first helpers for force, weight, and impulse calculations.

## Example

```rust
use use_force::{force, impulse};

assert_eq!(force(10.0, 2.0), 20.0);
assert_eq!(impulse(2.0, 1.0, 4.0), 6.0);
```

## When to use directly

Choose `use-force` when you only need reusable force formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Rigid-body simulation models are out of scope.

## Status

`use-force` is a pre-1.0 crate with a deliberately small API.
