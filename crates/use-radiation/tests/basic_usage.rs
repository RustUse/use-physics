#![allow(clippy::float_cmp)]

use use_radiation::{Dose, RadiationBeam, Shield, absorbed_dose, equivalent_dose};

#[test]
fn radiation_helpers_cover_beam_shield_and_dose_workflow() -> Result<(), &'static str> {
    let beam = RadiationBeam::new(12.0, 3.0).ok_or("expected valid beam")?;
    let shield = Shield::new(core::f64::consts::LN_2, 1.0).ok_or("expected valid shield")?;
    let absorbed = absorbed_dose(20.0, 4.0).ok_or("expected absorbed dose")?;
    let dose = Dose::new(absorbed).ok_or("expected dose")?;

    assert_eq!(beam.intensity(), Some(4.0));
    assert_eq!(beam.photon_flux_density(2.0), Some(2.0));
    assert!(
        (shield
            .attenuated_intensity(80.0)
            .ok_or("expected attenuated intensity")?
            - 40.0)
            .abs()
            < 1.0e-12
    );
    assert_eq!(dose.equivalent(20.0), equivalent_dose(5.0, 20.0));

    Ok(())
}
