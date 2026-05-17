# use-radiation

Small radiation physics scalar helpers for `RustUse`.

## Install

```toml
[dependencies]
use-radiation = "0.0.1"
```

## Foundation

`use-radiation` provides small scalar helpers for radiation intensity, fluence,
absorbed dose, equivalent and effective dose, attenuation, half-value layers,
photon flux, and simple radiation-type classification.

It is not a radiation safety tool, medical dosimetry package, shielding
certification tool, transport solver, Monte Carlo engine, or regulatory
compliance library.

Inputs are expected to be SI-style numeric values:

- watts for power
- square meters for area
- joules for energy
- kilograms for mass
- gray for absorbed dose
- sieverts for equivalent and effective dose
- seconds for time
- meters for distance and thickness
- inverse meters for linear attenuation coefficient
- square meters per kilogram for mass attenuation coefficient
- kilograms per cubic meter for density

General constants belong in the top-level `use-constants` set.
Unit abstractions belong in the top-level `use-units` set.
Broader photon quantum relations belong in `use-quantum`.
Radioactive decay helpers belong in `use-nuclear`.

## Example

```rust
use use_radiation::{
    Dose, RadiationBeam, RadiationKind, Shield, default_radiation_weighting_factor,
};

# fn main() -> Result<(), &'static str> {
let beam = RadiationBeam::new(10.0, 2.0).ok_or("expected beam")?;
let shield = Shield::new(core::f64::consts::LN_2, 1.0).ok_or("expected shield")?;
let dose = Dose::new(2.0).ok_or("expected dose")?;
let weighting = default_radiation_weighting_factor(RadiationKind::Gamma)
    .ok_or("expected weighting")?;

assert_eq!(beam.intensity(), Some(5.0));
assert!((shield.attenuated_intensity(100.0).ok_or("expected attenuation")? - 50.0).abs() < 1.0e-12);
assert_eq!(dose.equivalent(weighting), Some(2.0));
# Ok(())
# }
```

## When to use directly

Choose `use-radiation` when you only need reusable scalar radiation formulas.

## Scope

- APIs stay `f64`-first and dependency-free.
- The crate focuses on compact scalar helpers instead of transport or safety workflows.
- Decay chains, isotope catalogs, neutron energy weighting models, and unit systems are out of scope.

## Status

`use-radiation` is a pre-1.0 crate with a deliberately small API.
