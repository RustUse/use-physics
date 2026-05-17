#[cfg(feature = "particle")]
#[test]
fn facade_reexports_particle_helpers() {
    use use_physics::{
        Particle, ParticleFamily, ParticleKind, antiparticle, charge, particle, rest_mass_mev_c2,
    };

    let electron = Particle::new(ParticleKind::Electron);

    assert_eq!(charge(ParticleKind::Electron).thirds, -3);
    assert_eq!(electron.family(), ParticleFamily::Lepton);
    assert_eq!(
        antiparticle(ParticleKind::Electron),
        Some(ParticleKind::Positron)
    );
    assert_eq!(
        particle::family(ParticleKind::Photon),
        use_physics::ParticleFamily::GaugeBoson
    );
    assert_eq!(rest_mass_mev_c2(ParticleKind::Photon), Some(0.0));
}
