# use-power

Average, mechanical, and electrical power helpers for `RustUse`.

## Install

```toml
[dependencies]
use-power = "0.0.1"
```

## Foundation

`use-power` provides small `f64`-first helpers for common power calculations.

## Example

```rust
use use_power::{average_power, electrical_power};

assert_eq!(average_power(120.0, 6.0)?, 20.0);
assert_eq!(electrical_power(12.0, 2.0), 24.0);
# Ok::<(), use_power::PowerError>(())
```

## When to use directly

Choose `use-power` when you only need reusable power formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Power electronics models are out of scope.

## Status

`use-power` is a pre-1.0 crate with a deliberately small API.
