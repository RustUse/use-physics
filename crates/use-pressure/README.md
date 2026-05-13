# use-pressure

Pressure and hydrostatic pressure helpers for `RustUse`.

## Install

```toml
[dependencies]
use-pressure = "0.0.1"
```

## Foundation

`use-pressure` provides small `f64`-first helpers for pressure calculations.

## Example

```rust
use use_pressure::{hydrostatic_pressure, pressure};

assert_eq!(pressure(100.0, 4.0)?, 25.0);
assert_eq!(hydrostatic_pressure(1000.0, 10.0, 2.0), 20_000.0);
# Ok::<(), use_pressure::PressureError>(())
```

## When to use directly

Choose `use-pressure` when you only need reusable pressure formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Compressible fluid models are out of scope.

## Status

`use-pressure` is a pre-1.0 crate with a deliberately small API.
