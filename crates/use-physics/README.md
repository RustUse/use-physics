# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["force", "energy"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin and mirrors the boundaries of the concrete crates.

## Example

```rust
# #[cfg(all(feature = "force", feature = "energy"))]
# fn main() {
use use_physics::{force, kinetic_energy};

let applied_force = force(10.0, 2.0);
let energy = kinetic_energy(2.0, 3.0);

assert_eq!(applied_force, 20.0);
assert_eq!(energy, 9.0);
# }
# #[cfg(not(all(feature = "force", feature = "energy")))]
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
