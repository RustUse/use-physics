# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["force", "gravity"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin, mirrors the boundaries of the concrete crates, and exposes each enabled crate under a matching module such as `use_physics::gravity`.

## Example

```rust
# #[cfg(all(feature = "force", feature = "gravity"))]
# fn main() {
use use_physics::{escape_velocity, force};

let applied_force = force(10.0, 2.0);
let escape = escape_velocity(5.972e24, 6.371e6).unwrap();

assert_eq!(applied_force, 20.0);
assert!(escape > 11_000.0);
# }
# #[cfg(not(all(feature = "force", feature = "gravity")))]
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
