#![allow(clippy::float_cmp)]

#[cfg(feature = "radiation")]
#[test]
fn facade_reexports_radiation_workflow() -> Result<(), &'static str> {
    use use_physics::prelude::{
        Dose, RadiationBeam, RadiationKind, Shield, default_radiation_weighting_factor,
    };

    let beam = RadiationBeam::new(10.0, 2.0).ok_or("expected beam")?;
    let shield = Shield::new(core::f64::consts::LN_2, 1.0).ok_or("expected shield")?;
    let dose = Dose::new(2.0).ok_or("expected dose")?;
    let weighting =
        default_radiation_weighting_factor(RadiationKind::Gamma).ok_or("expected weighting")?;

    assert_eq!(beam.intensity(), Some(5.0));
    assert_eq!(beam.photon_flux(2.0), Some(5.0));
    assert!(
        (shield
            .attenuated_intensity(100.0)
            .ok_or("expected attenuation")?
            - 50.0)
            .abs()
            < 1.0e-12
    );
    assert_eq!(dose.equivalent(weighting), Some(2.0));
    assert_eq!(
        use_physics::RADIATION_SPEED_OF_LIGHT,
        use_physics::radiation::SPEED_OF_LIGHT
    );
    assert_eq!(
        use_physics::RADIATION_JOULES_PER_MEV,
        use_physics::radiation::JOULES_PER_MEV
    );

    Ok(())
}
