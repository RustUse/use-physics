#![allow(clippy::float_cmp)]

use use_nuclear::{
    ATOMIC_MASS_UNIT_MEV_C2, DecayLaw, NuclideNumbers, activity,
    binding_energy_mev_from_mass_defect_u,
};

fn main() -> Result<(), &'static str> {
    let decay_law = DecayLaw::from_half_life(10.0).ok_or("expected valid half-life")?;
    let remaining = decay_law
        .remaining_quantity(100.0, 10.0)
        .ok_or("expected valid remaining quantity")?;
    let helium = NuclideNumbers::new(4, 2).ok_or("expected valid nuclide numbers")?;

    assert!((remaining - 50.0).abs() < 1.0e-12);
    assert_eq!(activity(2.0, 10.0), Some(20.0));
    assert_eq!(helium.proton_count(), 2);
    assert_eq!(helium.neutron_count(), 2);
    assert_eq!(
        binding_energy_mev_from_mass_defect_u(1.0),
        Some(ATOMIC_MASS_UNIT_MEV_C2),
    );

    Ok(())
}
