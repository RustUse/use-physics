# Changelog

## Unreleased

### Added

- Added the initial `use-physics` multi-crate workspace scaffold.
- Added focused crates for motion, force, energy, power, pressure, density, and thermodynamics.
- Added the `use-physics` facade crate with feature-gated re-exports.
- Added the `use-electricity` focused crate for charge, simple circuits, and Coulomb-force helpers.
- Added the `use-magnetism` focused crate for magnetic flux, force, field, and pressure helpers, and exposed it through the `use-physics` facade.
- Added the `use-work` focused crate for mechanical work and work-energy helpers.
- Added the `use-rigidbody` focused crate for rigid-body mass properties, inertia, impulse response, and simple scalar state helpers, and exposed it through the `use-physics` facade.

### Tooling

- Added the RustUse baseline for formatting, linting, CI, dependency policy, and release automation.
