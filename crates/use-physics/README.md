# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["gravity", "momentum", "electricity", "particle", "work"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin, mirrors the boundaries of the concrete crates, and exposes each enabled crate under a matching module such as `use_physics::gravity`, `use_physics::momentum`, `use_physics::electricity`, `use_physics::particle`, or `use_physics::work`.

When focused crates would otherwise collide at the root, the facade keeps explicit aliases or module boundaries. For example, enabling both `force` and `momentum` preserves `use-force`'s `impulse` export and re-exports the force-time helper from `use-momentum` as `momentum_impulse`. Likewise, the full `use-work` surface stays available under `use_physics::work` while the existing root `work` export continues to come from `use-energy`.

## Example

```rust
# #[cfg(feature = "work")]
# fn main() {
use use_physics::{spring_work, work::work_at_angle_degrees};

assert_eq!(spring_work(100.0, 0.5, 0.0), Some(12.5));
assert!((work_at_angle_degrees(10.0, 2.0, 60.0).unwrap() - 10.0).abs() < 1e-12);

# }
# #[cfg(not(feature = "work"))]
# fn main() {}
```

## When to use directly

Choose `use-physics` when you want one dependency and one import surface. Prefer the focused crates directly when you only need one physics domain.

## Scope

- The facade stays close to the focused crate APIs.
- Feature flags map directly to the focused crates in this workspace.
- Units systems and symbolic algebra are out of scope.

## Status

`use-physics` is a pre-1.0 crate with a deliberately small facade over focused helpers.
