# use-particle

Small particle classification and metadata helpers for `RustUse`.

## Install

```toml
[dependencies]
use-particle = "0.0.1"
```

## Foundation

`use-particle` provides small, dependency-free primitives for classifying common particles,
working with exact electric charge in thirds of the elementary charge, and looking up simple
particle metadata such as spin, statistics, and approximate rest mass.

## Example

```rust
use use_particle::{Particle, ParticleFamily, ParticleKind, antiparticle, charge};

let electron = Particle::new(ParticleKind::Electron);

assert_eq!(electron.family(), ParticleFamily::Lepton);
assert_eq!(charge(ParticleKind::Electron).thirds, -3);
assert_eq!(antiparticle(ParticleKind::Electron), Some(ParticleKind::Positron));
```

## When to use directly

Choose `use-particle` when you need lightweight particle metadata without pulling in a broader
physics or chemistry model.

## Scope

- The crate stays intentionally small and does not try to be a complete particle physics database.
- Atomic structure, periodic table data, isotopes, electron shells, and chemistry concepts belong
  in `use-chemistry`.
- General constants and unit systems belong in `use-constants` and `use-units`.
- Rest masses are approximate metadata for practical examples and should not be treated as
  precision reference data.

## Status

`use-particle` is a pre-1.0 crate with a deliberately small API.
