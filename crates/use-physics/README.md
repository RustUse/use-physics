# use-physics

Feature-gated facade for the focused `RustUse` physics crates.

## Install

```toml
[dependencies]
use-physics = { version = "0.0.1", default-features = false, features = ["gravity", "orbit", "momentum", "oscillation", "relativity", "quantum", "plasma", "electricity", "magnetism", "electromagnetism", "particle", "nuclear", "radiation", "work"] }
```

## Foundation

`use-physics` re-exports focused `f64`-first physics helpers behind opt-in features. The facade stays thin, mirrors the boundaries of the concrete crates, and exposes each enabled crate under a matching module such as `use_physics::gravity`, `use_physics::orbit`, `use_physics::momentum`, `use_physics::oscillation`, `use_physics::relativity`, `use_physics::quantum`, `use_physics::plasma`, `use_physics::electricity`, `use_physics::magnetism`, `use_physics::electromagnetism`, `use_physics::particle`, `use_physics::nuclear`, `use_physics::radiation`, or `use_physics::work`.

When focused crates would otherwise collide at the root, the facade keeps explicit aliases or module boundaries. For example, enabling both `force` and `momentum` preserves `use-force`'s `impulse` export and re-exports the force-time helper from `use-momentum` as `momentum_impulse`. Enabling both `pressure` and `fluid` keeps `use-pressure`'s `hydrostatic_pressure` export at the root and re-exports the fluid-specific helper as `fluid_hydrostatic_pressure`. Enabling `orbit` alongside `gravity` or `force` keeps the existing root `GRAVITATIONAL_CONSTANT`, `STANDARD_GRAVITY`, and `circular_orbital_period` behavior, while the overlapping orbit variants remain available under `use_physics::orbit`. Enabling `oscillation` keeps `use-motion`'s root `displacement` export and `use-work`'s root `spring_potential_energy` export unchanged while exposing the oscillation variants as `oscillation_displacement` and `oscillation_spring_potential_energy`. Enabling `electromagnetism` alongside `magnetism` or `nuclear` keeps the existing `VACUUM_PERMEABILITY` and `SPEED_OF_LIGHT` exports at the root and re-exports the electromagnetism versions as `ELECTROMAGNETISM_VACUUM_PERMEABILITY` and `ELECTROMAGNETISM_SPEED_OF_LIGHT`. Enabling `relativity` alongside `electromagnetism` or `nuclear` keeps the existing root `SPEED_OF_LIGHT` export and re-exports the relativity version as `RELATIVITY_SPEED_OF_LIGHT`. Enabling `quantum` alongside a crate that already owns the root `SPEED_OF_LIGHT` export keeps that existing root behavior and re-exports the quantum version as `QUANTUM_SPEED_OF_LIGHT`. Enabling `plasma` alongside `quantum`, `electromagnetism`, or `magnetism` keeps the established root `ELEMENTARY_CHARGE`, `ELECTRON_MASS`, `VACUUM_PERMITTIVITY`, `VACUUM_PERMEABILITY`, and `magnetic_pressure` exports and re-exports the plasma variants as `PLASMA_ELEMENTARY_CHARGE`, `PLASMA_ELECTRON_MASS`, `PLASMA_VACUUM_PERMITTIVITY`, `PLASMA_VACUUM_PERMEABILITY`, and `plasma_magnetic_pressure`. Enabling `radiation` keeps the focused crate available under `use_physics::radiation`, exposes the overlapping convenience constants as `RADIATION_SPEED_OF_LIGHT` and `RADIATION_JOULES_PER_MEV`, and leaves the overlapping photon-constant and photon-energy helpers on the namespaced `use_physics::radiation` module. Likewise, the full `use-work` surface stays available under `use_physics::work` while the existing root `work` export continues to come from `use-energy`.

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
