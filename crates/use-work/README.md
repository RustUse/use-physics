# use-work

Mechanical work helpers for `RustUse`.

## Install

```toml
[dependencies]
use-work = "0.0.1"
```

## Foundation

`use-work` provides small `f64`-first helpers for mechanical work, work at an angle, net work, spring work, gravitational work, friction work, and simple work-energy relationships.

Inputs are expected to be SI-style numeric values:

- newtons for force and friction force magnitude
- meters for displacement, height, and spring displacement
- joules for work and energy
- radians for angle helpers unless the function name says degrees
- newtons per meter for spring constant

The crate does not define a full unit system.

## Example

```rust
use use_work::{ConstantForceWork, work, work_at_angle_degrees};

let constant = ConstantForceWork::new(10.0, 2.0).unwrap();

assert_eq!(work(10.0, 2.0), Some(20.0));
assert_eq!(constant.work(), Some(20.0));
assert!((work_at_angle_degrees(10.0, 2.0, 60.0).unwrap() - 10.0).abs() < 1e-12);
```

## When to use directly

Choose `use-work` when you only need reusable scalar helpers for mechanical work calculations.

## Scope

- APIs stay `f64`-first and do not define a full unit system.
- Vector operations should live in or compose with `use-vector`.
- Broad energy helpers belong in `use-energy`.
- Broad power helpers belong in `use-power`.

## Status

`use-work` is a pre-1.0 crate with a deliberately small API.
