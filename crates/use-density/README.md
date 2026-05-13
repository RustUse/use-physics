# use-density

Density, mass, and volume helpers for `RustUse`.

## Install

```toml
[dependencies]
use-density = "0.0.1"
```

## Foundation

`use-density` provides small `f64`-first helpers for density relationships.

## Example

```rust
use use_density::{density, mass, volume};

assert_eq!(density(10.0, 2.0)?, 5.0);
assert_eq!(mass(5.0, 2.0), 10.0);
assert_eq!(volume(10.0, 5.0)?, 2.0);
# Ok::<(), use_density::DensityError>(())
```

## When to use directly

Choose `use-density` when you only need reusable density formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Material property tables are out of scope.

## Status

`use-density` is a pre-1.0 crate with a deliberately small API.
