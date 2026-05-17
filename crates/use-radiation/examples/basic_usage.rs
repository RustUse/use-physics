use use_radiation::{
    Dose, RadiationBeam, RadiationKind, Shield, default_radiation_weighting_factor,
};

fn main() -> Result<(), &'static str> {
    let beam = RadiationBeam::new(10.0, 2.0).ok_or("expected valid beam")?;
    let shield = Shield::new(core::f64::consts::LN_2, 1.0).ok_or("expected valid shield")?;
    let dose = Dose::new(2.0).ok_or("expected valid dose")?;
    let weighting = default_radiation_weighting_factor(RadiationKind::Gamma)
        .ok_or("expected gamma weighting")?;

    assert_eq!(beam.intensity(), Some(5.0));
    assert_eq!(beam.photon_flux(2.0), Some(5.0));
    assert!(
        (shield
            .transmitted_fraction()
            .ok_or("expected transmitted fraction")?
            - 0.5)
            .abs()
            < 1.0e-12
    );
    assert_eq!(dose.equivalent(weighting), Some(2.0));

    Ok(())
}
