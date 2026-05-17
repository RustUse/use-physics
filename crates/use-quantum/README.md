# use-quantum

Small quantum physics scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-quantum = "0.0.1"
```

## Foundation

`use-quantum` provides small quantum physics scalar helpers for photon energy, matter waves,
uncertainty estimates, simple Bohr-model relations, and quantum number validation.

Inputs are expected to be SI-style numeric values unless a function name explicitly says electron
volts.

The crate keeps only a few local convenience constants. Broader physical constants belong in the
top-level `use-constants` set. Unit abstractions belong in `use-units`. Wave abstractions belong
in `use-wave`. Particle classification belongs in `use-particle`.

## Example

```rust
use use_quantum::{Photon, QuantumNumbers, RYDBERG_ENERGY_EV, hydrogen_energy_level_ev};

let photon = Photon::from_wavelength(500.0e-9).ok_or("expected valid wavelength")?;
let quantum_numbers = QuantumNumbers::new(2, 1, 0, 1).ok_or("expected valid quantum numbers")?;

assert!(photon.energy_joules() > 0.0);
assert_eq!(quantum_numbers.spin_projection(), 0.5);
assert_eq!(hydrogen_energy_level_ev(1), Some(-RYDBERG_ENERGY_EV));
# Ok::<(), &'static str>(())
```

## When to use directly

Choose `use-quantum` when you only need small reusable scalar quantum formulas or validation
helpers.

## Scope

- APIs stay `f64`-first and unit-agnostic.
- This crate is not a quantum simulator, quantum computing library, Hilbert-space library, matrix
  mechanics library, optics system, wave system, unit system, or full atomic physics package.
- General constants belong in `use-constants`.
- Unit abstractions belong in `use-units`.
- Wave abstractions belong in `use-wave`.
- Particle classification belongs in `use-particle`.

## Status

`use-quantum` is a pre-1.0 crate with a deliberately small scalar API.
