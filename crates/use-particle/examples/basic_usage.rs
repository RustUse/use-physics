use use_particle::{Particle, ParticleFamily, ParticleKind, antiparticle, charge, spin};

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-12
}

fn main() {
    let electron = Particle::new(ParticleKind::Electron);

    assert_eq!(electron.family(), ParticleFamily::Lepton);
    assert_eq!(charge(ParticleKind::Electron).thirds, -3);
    assert_eq!(spin(ParticleKind::Photon).doubled, 2);
    assert_eq!(
        antiparticle(ParticleKind::Electron),
        Some(ParticleKind::Positron)
    );
    assert!(approx_eq(electron.charge().as_elementary_units(), -1.0,));
}
