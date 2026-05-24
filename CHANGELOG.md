# Changelog

## Unreleased

## [0.0.3](https://github.com/RustUse/use-physics/compare/use-physics-v0.0.2...use-physics-v0.0.3) - 2026-05-24

### Changed

- Publish use-elasticity v0.0.2; update refs
- Initialize use-physics workspace and tooling

### Changed

- Published the physics-owned `use-elasticity` line as `0.0.2` and updated the `use-physics`
	facade to depend on it, taking over the crate name after the materials elasticity helper moved to
	`use-material-elasticity`.

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
