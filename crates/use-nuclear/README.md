# use-nuclear

Small nuclear physics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-nuclear = "0.0.1"
```

## Foundation

`use-nuclear` provides small scalar helpers for radioactive decay, activity, mass defect,
binding energy, and simple nuclide number relationships.

Inputs are numeric SI-style values unless a function name explicitly refers to `MeV` or atomic
mass units. The crate stays dependency-free and does not define a full unit system.

## Example

```rust
use use_nuclear::{
    activity, binding_energy_mev_from_mass_defect_u, DecayLaw, NuclideNumbers,
    ATOMIC_MASS_UNIT_MEV_C2,
};

# fn main() -> Result<(), &'static str> {
let decay_law = DecayLaw::from_half_life(10.0).ok_or("expected valid half-life")?;
let remaining = decay_law
  .remaining_quantity(100.0, 10.0)
  .ok_or("expected valid remaining quantity")?;
let helium = NuclideNumbers::new(4, 2).ok_or("expected valid nuclide numbers")?;

assert!((remaining - 50.0).abs() < 1.0e-12);
assert_eq!(activity(2.0, 10.0), Some(20.0));
assert_eq!(helium.neutron_count(), 2);
assert_eq!(
    binding_energy_mev_from_mass_defect_u(1.0),
    Some(ATOMIC_MASS_UNIT_MEV_C2),
);
# Ok(())
# }
```

## When to use directly

Choose `use-nuclear` when you need reusable scalar formulas for decay, activity, or nuclear
energy relationships without pulling in a larger chemistry or units model.

## Scope

- Small scalar helpers for radioactive decay, activity, mass-energy equivalence, and simple
  nuclide-number relationships.
- General constants belong in the top-level `use-constants` set.
- Unit abstractions belong in the top-level `use-units` set.
- Element symbols, atomic masses, and periodic-table concepts belong in `use-chemistry`.
- Isotope databases, reactor simulation, radiation transport, dosimetry, and radiation safety
  tooling are out of scope.

## Status

`use-nuclear` is a pre-1.0 crate with a deliberately small API.
