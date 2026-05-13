# use-motion

Basic kinematics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-motion = "0.0.1"
```

## Foundation

`use-motion` provides small `f64`-first helpers for common introductory kinematics calculations.

## Example

```rust
use use_motion::{average_speed, final_velocity};

let speed = average_speed(100.0, 10.0)?;

assert_eq!(speed, 10.0);
assert_eq!(final_velocity(2.0, 3.0, 4.0), 14.0);
# Ok::<(), use_motion::MotionError>(())
```

## When to use directly

Choose `use-motion` when you only need reusable motion formulas.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- Higher-level simulation models are out of scope.

## Status

`use-motion` is a pre-1.0 crate with a deliberately small API.
