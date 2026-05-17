# use-elasticity

Small scalar elasticity and mechanics-of-materials helpers for `RustUse`.

## Install

```toml
[dependencies]
use-elasticity = "0.0.1"
```

## Foundation

`use-elasticity` provides small scalar helpers for stress, strain, Young's modulus, shear modulus,
bulk modulus, Poisson's ratio, axial deformation, stiffness, and elastic energy.

Inputs are expected to be SI-style numeric values:

- newtons for force
- square meters for area
- pascals for stress and elastic moduli
- meters for length, displacement, deformation, and height
- cubic meters for volume
- joules for energy
- joules per cubic meter for energy density

Material property catalogs belong in top-level `use-materials`.
Unit abstractions belong in top-level `use-units`.
Structural equilibrium helpers belong in `use-statics`.
Simulation belongs in top-level `use-simulation`.

## Example

```rust
use use_elasticity::{ElasticBar, ElasticMaterial, normal_stress};

let Some(material) = ElasticMaterial::with_poisson_ratio(260.0, 0.3) else {
    unreachable!();
};
let Some(bar) = ElasticBar::new(10.0, 2.0, 1_000.0) else {
    unreachable!();
};

assert_eq!(normal_stress(100.0, 2.0), Some(50.0));
assert!(matches!(material.shear_modulus(), Some(value) if (value - 100.0).abs() < 1.0e-12));
assert_eq!(bar.deformation_under_force(100.0), Some(0.5));
```

## When to use directly

Choose `use-elasticity` when you need reusable scalar elasticity relations without a larger
materials or simulation layer.

## Scope

- This crate covers small scalar elasticity helpers only.
- It is not a material database, finite element solver, structural analysis engine, fracture
  mechanics package, plasticity model, or simulation framework.
- It does not expand into fatigue analysis, beam theory, or design-code compliance workflows.

## Status

`use-elasticity` is a pre-1.0 crate with a deliberately small API.
