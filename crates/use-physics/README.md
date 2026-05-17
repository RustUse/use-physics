# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["gravity", "momentum", "fluid", "electricity", "magnetism", "particle", "nuclear", "work"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin, mirrors the boundaries of the concrete crates, and exposes each enabled crate under a matching module such as `use_physics::gravity`, `use_physics::momentum`, `use_physics::fluid`, `use_physics::electricity`, `use_physics::magnetism`, `use_physics::particle`, `use_physics::nuclear`, or `use_physics::work`.

When focused crates would otherwise collide at the root, the facade keeps explicit aliases or module boundaries. For example, enabling both `force` and `momentum` preserves `use-force`'s `impulse` export and re-exports the force-time helper from `use-momentum` as `momentum_impulse`. Enabling both `pressure` and `fluid` keeps `use-pressure`'s `hydrostatic_pressure` export at the root and re-exports the fluid-specific helper as `fluid_hydrostatic_pressure`. Likewise, the full `use-work` surface stays available under `use_physics::work` while the existing root `work` export continues to come from `use-energy`.

## Example

```rust
# #[cfg(feature = "magnetism")]
# fn main() {
use use_physics::{MagneticField, magnetic_flux};

assert_eq!(magnetic_flux(2.0, 3.0, 0.0), Some(6.0));
assert_eq!(
	MagneticField::new(3.0)
		.and_then(|field| field.energy_density())
		.map(|value| value > 0.0),
	Some(true)
);

# }
# #[cfg(not(feature = "magnetism"))]
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
