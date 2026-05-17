# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["gravity", "momentum", "electricity"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin, mirrors the boundaries of the concrete crates, and exposes each enabled crate under a matching module such as `use_physics::gravity`, `use_physics::momentum`, or `use_physics::electricity`.

When focused crates would otherwise collide at the root, the facade keeps explicit aliases. For example, enabling both `force` and `momentum` preserves `use-force`'s `impulse` export and re-exports the force-time helper from `use-momentum` as `momentum_impulse`.

## Example

```rust
# #[cfg(all(feature = "gravity", feature = "momentum"))]
# fn main() {
use use_physics::{escape_velocity, recoil_velocity};

let escape = escape_velocity(5.972e24, 6.371e6).unwrap();
let recoil = recoil_velocity(1.0, 10.0, 5.0);

assert!(escape > 11_000.0);
assert_eq!(recoil, Some(-2.0));
# }
# #[cfg(not(all(feature = "gravity", feature = "momentum")))]
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
