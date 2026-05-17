#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small particle classification and metadata helpers.

pub mod prelude;

/// Broad family groupings for the supported particles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleFamily {
    /// Leptons such as electrons, muons, taus, and neutrinos.
    Lepton,
    /// Quarks and antiquarks.
    Quark,
    /// Gauge bosons such as photons and gluons.
    GaugeBoson,
    /// Scalar bosons such as the Higgs boson.
    ScalarBoson,
    /// Baryons such as protons and neutrons.
    Baryon,
    /// Mesons such as pions.
    Meson,
}

/// Spin-statistics classification for the supported particles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleStatistics {
    /// A fermion with half-integer spin.
    Fermion,
    /// A boson with integer spin.
    Boson,
}

/// Identifies a supported particle kind.
///
/// The enum is intentionally small and practical rather than exhaustive.
///
/// # Examples
///
/// ```rust
/// use use_particle::ParticleKind;
///
/// let electron = ParticleKind::Electron;
///
/// assert_eq!(electron, ParticleKind::Electron);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleKind {
    /// The electron.
    Electron,
    /// The positron.
    Positron,
    /// The muon.
    Muon,
    /// The antimuon.
    Antimuon,
    /// The tau lepton.
    Tau,
    /// The antitau.
    Antitau,

    /// The electron neutrino.
    ElectronNeutrino,
    /// The electron antineutrino.
    ElectronAntineutrino,
    /// The muon neutrino.
    MuonNeutrino,
    /// The muon antineutrino.
    MuonAntineutrino,
    /// The tau neutrino.
    TauNeutrino,
    /// The tau antineutrino.
    TauAntineutrino,

    /// The up quark.
    UpQuark,
    /// The anti-up quark.
    AntiUpQuark,
    /// The down quark.
    DownQuark,
    /// The anti-down quark.
    AntiDownQuark,
    /// The charm quark.
    CharmQuark,
    /// The anti-charm quark.
    AntiCharmQuark,
    /// The strange quark.
    StrangeQuark,
    /// The anti-strange quark.
    AntiStrangeQuark,
    /// The top quark.
    TopQuark,
    /// The anti-top quark.
    AntiTopQuark,
    /// The bottom quark.
    BottomQuark,
    /// The anti-bottom quark.
    AntiBottomQuark,

    /// The photon.
    Photon,
    /// The gluon.
    Gluon,
    /// The positively charged `W` boson.
    WPlusBoson,
    /// The negatively charged `W` boson.
    WMinusBoson,
    /// The neutral `Z` boson.
    ZBoson,
    /// The Higgs boson.
    HiggsBoson,

    /// The proton.
    Proton,
    /// The antiproton.
    Antiproton,
    /// The neutron.
    Neutron,
    /// The antineutron.
    Antineutron,

    /// The positively charged pion.
    PionPlus,
    /// The negatively charged pion.
    PionMinus,
    /// The neutral pion.
    PionZero,
}

/// An exact electric charge expressed in thirds of the elementary charge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementaryCharge {
    /// Charge measured in thirds of the elementary charge.
    pub thirds: i8,
}

impl ElementaryCharge {
    /// Creates a charge from thirds of the elementary charge.
    #[must_use]
    pub const fn new_thirds(thirds: i8) -> Self {
        Self { thirds }
    }

    /// Returns a neutral charge.
    #[must_use]
    pub const fn neutral() -> Self {
        Self::new_thirds(0)
    }

    /// Returns a `+1e` charge.
    #[must_use]
    pub const fn positive_one() -> Self {
        Self::new_thirds(3)
    }

    /// Returns a `-1e` charge.
    #[must_use]
    pub const fn negative_one() -> Self {
        Self::new_thirds(-3)
    }

    /// Returns the charge in elementary-charge units.
    #[must_use]
    pub fn as_elementary_units(self) -> f64 {
        f64::from(self.thirds) / 3.0
    }

    /// Returns `true` when the charge is neutral.
    #[must_use]
    pub const fn is_neutral(self) -> bool {
        self.thirds == 0
    }

    /// Returns `true` when the charge is positive.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.thirds > 0
    }

    /// Returns `true` when the charge is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.thirds < 0
    }
}

/// A spin value expressed as doubled units of `hbar`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spin {
    /// Spin measured in doubled units of `hbar`.
    pub doubled: i8,
}

impl Spin {
    /// Creates a spin from doubled units of `hbar`.
    #[must_use]
    pub const fn new_doubled(doubled: i8) -> Self {
        Self { doubled }
    }

    /// Returns spin `0`.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new_doubled(0)
    }

    /// Returns spin `1/2`.
    #[must_use]
    pub const fn half() -> Self {
        Self::new_doubled(1)
    }

    /// Returns spin `1`.
    #[must_use]
    pub const fn one() -> Self {
        Self::new_doubled(2)
    }

    /// Returns the spin in units of `hbar`.
    #[must_use]
    pub fn as_units_of_hbar(self) -> f64 {
        f64::from(self.doubled) / 2.0
    }

    /// Returns `true` when the spin is an integer multiple of `hbar`.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.doubled % 2 == 0
    }

    /// Returns `true` when the spin is a half-integer multiple of `hbar`.
    #[must_use]
    pub const fn is_half_integer(self) -> bool {
        !self.is_integer()
    }
}

/// A lightweight particle wrapper that delegates to the free helper functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Particle {
    /// The underlying particle kind.
    pub kind: ParticleKind,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl Particle {
    /// Creates a new `Particle` from `kind`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_particle::{Particle, ParticleKind};
    ///
    /// let electron = Particle::new(ParticleKind::Electron);
    ///
    /// assert_eq!(electron.kind, ParticleKind::Electron);
    /// ```
    #[must_use]
    pub const fn new(kind: ParticleKind) -> Self {
        Self { kind }
    }

    /// Returns the particle family.
    #[must_use]
    pub const fn family(&self) -> ParticleFamily {
        family(self.kind)
    }

    /// Returns the exact charge.
    #[must_use]
    pub const fn charge(&self) -> ElementaryCharge {
        charge(self.kind)
    }

    /// Returns the spin.
    #[must_use]
    pub const fn spin(&self) -> Spin {
        spin(self.kind)
    }

    /// Returns the particle statistics.
    #[must_use]
    pub const fn statistics(&self) -> ParticleStatistics {
        statistics(self.kind)
    }

    /// Returns the antiparticle when it is modeled by this crate.
    #[must_use]
    pub const fn antiparticle(&self) -> Option<Self> {
        match antiparticle(self.kind) {
            Some(kind) => Some(Self::new(kind)),
            None => None,
        }
    }

    /// Returns the approximate rest mass in `MeV/c^2`.
    #[must_use]
    pub const fn rest_mass_mev_c2(&self) -> Option<f64> {
        rest_mass_mev_c2(self.kind)
    }

    /// Returns `true` when this particle is represented as an antiparticle variant.
    #[must_use]
    pub const fn is_antiparticle(&self) -> bool {
        is_antiparticle(self.kind)
    }

    /// Returns `true` when the particle is its own antiparticle.
    #[must_use]
    pub const fn is_self_antiparticle(&self) -> bool {
        is_self_antiparticle(self.kind)
    }
}

/// Returns the exact charge in thirds of the elementary charge.
#[must_use]
pub const fn charge_thirds(kind: ParticleKind) -> i8 {
    match kind {
        ParticleKind::Electron
        | ParticleKind::Muon
        | ParticleKind::Tau
        | ParticleKind::WMinusBoson
        | ParticleKind::Antiproton
        | ParticleKind::PionMinus => -3,
        ParticleKind::Positron
        | ParticleKind::Antimuon
        | ParticleKind::Antitau
        | ParticleKind::WPlusBoson
        | ParticleKind::Proton
        | ParticleKind::PionPlus => 3,
        ParticleKind::UpQuark | ParticleKind::CharmQuark | ParticleKind::TopQuark => 2,
        ParticleKind::AntiUpQuark | ParticleKind::AntiCharmQuark | ParticleKind::AntiTopQuark => -2,
        ParticleKind::DownQuark | ParticleKind::StrangeQuark | ParticleKind::BottomQuark => -1,
        ParticleKind::AntiDownQuark
        | ParticleKind::AntiStrangeQuark
        | ParticleKind::AntiBottomQuark => 1,
        ParticleKind::ElectronNeutrino
        | ParticleKind::ElectronAntineutrino
        | ParticleKind::MuonNeutrino
        | ParticleKind::MuonAntineutrino
        | ParticleKind::TauNeutrino
        | ParticleKind::TauAntineutrino
        | ParticleKind::Photon
        | ParticleKind::Gluon
        | ParticleKind::ZBoson
        | ParticleKind::HiggsBoson
        | ParticleKind::Neutron
        | ParticleKind::Antineutron
        | ParticleKind::PionZero => 0,
    }
}

/// Returns the exact charge for `kind`.
///
/// # Examples
///
/// ```rust
/// use use_particle::{ParticleKind, charge};
///
/// assert_eq!(charge(ParticleKind::Electron).thirds, -3);
/// assert_eq!(charge(ParticleKind::UpQuark).thirds, 2);
/// ```
#[must_use]
pub const fn charge(kind: ParticleKind) -> ElementaryCharge {
    ElementaryCharge::new_thirds(charge_thirds(kind))
}

/// Returns the charge in elementary-charge units.
#[must_use]
pub fn charge_in_elementary_units(kind: ParticleKind) -> f64 {
    charge(kind).as_elementary_units()
}

/// Returns the spin for `kind`.
///
/// # Examples
///
/// ```rust
/// use use_particle::{ParticleKind, Spin, spin};
///
/// assert_eq!(spin(ParticleKind::Electron), Spin::half());
/// assert_eq!(spin(ParticleKind::Photon), Spin::one());
/// ```
#[must_use]
pub const fn spin(kind: ParticleKind) -> Spin {
    match kind {
        ParticleKind::Electron
        | ParticleKind::Positron
        | ParticleKind::Muon
        | ParticleKind::Antimuon
        | ParticleKind::Tau
        | ParticleKind::Antitau
        | ParticleKind::ElectronNeutrino
        | ParticleKind::ElectronAntineutrino
        | ParticleKind::MuonNeutrino
        | ParticleKind::MuonAntineutrino
        | ParticleKind::TauNeutrino
        | ParticleKind::TauAntineutrino
        | ParticleKind::UpQuark
        | ParticleKind::AntiUpQuark
        | ParticleKind::DownQuark
        | ParticleKind::AntiDownQuark
        | ParticleKind::CharmQuark
        | ParticleKind::AntiCharmQuark
        | ParticleKind::StrangeQuark
        | ParticleKind::AntiStrangeQuark
        | ParticleKind::TopQuark
        | ParticleKind::AntiTopQuark
        | ParticleKind::BottomQuark
        | ParticleKind::AntiBottomQuark
        | ParticleKind::Proton
        | ParticleKind::Antiproton
        | ParticleKind::Neutron
        | ParticleKind::Antineutron => Spin::half(),
        ParticleKind::Photon
        | ParticleKind::Gluon
        | ParticleKind::WPlusBoson
        | ParticleKind::WMinusBoson
        | ParticleKind::ZBoson => Spin::one(),
        ParticleKind::HiggsBoson
        | ParticleKind::PionPlus
        | ParticleKind::PionMinus
        | ParticleKind::PionZero => Spin::zero(),
    }
}

/// Returns the particle statistics implied by the modeled spin.
#[must_use]
pub const fn statistics(kind: ParticleKind) -> ParticleStatistics {
    if spin(kind).is_half_integer() {
        ParticleStatistics::Fermion
    } else {
        ParticleStatistics::Boson
    }
}

/// Returns the broad particle family for `kind`.
///
/// # Examples
///
/// ```rust
/// use use_particle::{ParticleFamily, ParticleKind, family};
///
/// assert_eq!(family(ParticleKind::Electron), ParticleFamily::Lepton);
/// assert_eq!(family(ParticleKind::Proton), ParticleFamily::Baryon);
/// ```
#[must_use]
pub const fn family(kind: ParticleKind) -> ParticleFamily {
    match kind {
        ParticleKind::Electron
        | ParticleKind::Positron
        | ParticleKind::Muon
        | ParticleKind::Antimuon
        | ParticleKind::Tau
        | ParticleKind::Antitau
        | ParticleKind::ElectronNeutrino
        | ParticleKind::ElectronAntineutrino
        | ParticleKind::MuonNeutrino
        | ParticleKind::MuonAntineutrino
        | ParticleKind::TauNeutrino
        | ParticleKind::TauAntineutrino => ParticleFamily::Lepton,
        ParticleKind::UpQuark
        | ParticleKind::AntiUpQuark
        | ParticleKind::DownQuark
        | ParticleKind::AntiDownQuark
        | ParticleKind::CharmQuark
        | ParticleKind::AntiCharmQuark
        | ParticleKind::StrangeQuark
        | ParticleKind::AntiStrangeQuark
        | ParticleKind::TopQuark
        | ParticleKind::AntiTopQuark
        | ParticleKind::BottomQuark
        | ParticleKind::AntiBottomQuark => ParticleFamily::Quark,
        ParticleKind::Photon
        | ParticleKind::Gluon
        | ParticleKind::WPlusBoson
        | ParticleKind::WMinusBoson
        | ParticleKind::ZBoson => ParticleFamily::GaugeBoson,
        ParticleKind::HiggsBoson => ParticleFamily::ScalarBoson,
        ParticleKind::Proton
        | ParticleKind::Antiproton
        | ParticleKind::Neutron
        | ParticleKind::Antineutron => ParticleFamily::Baryon,
        ParticleKind::PionPlus | ParticleKind::PionMinus | ParticleKind::PionZero => {
            ParticleFamily::Meson
        },
    }
}

/// Returns `true` when `kind` is a lepton.
#[must_use]
pub const fn is_lepton(kind: ParticleKind) -> bool {
    matches!(family(kind), ParticleFamily::Lepton)
}

/// Returns `true` when `kind` is a quark.
#[must_use]
pub const fn is_quark(kind: ParticleKind) -> bool {
    matches!(family(kind), ParticleFamily::Quark)
}

/// Returns `true` when `kind` is a boson.
#[must_use]
pub const fn is_boson(kind: ParticleKind) -> bool {
    matches!(statistics(kind), ParticleStatistics::Boson)
}

/// Returns `true` when `kind` is a baryon.
#[must_use]
pub const fn is_baryon(kind: ParticleKind) -> bool {
    matches!(family(kind), ParticleFamily::Baryon)
}

/// Returns `true` when `kind` is a meson.
#[must_use]
pub const fn is_meson(kind: ParticleKind) -> bool {
    matches!(family(kind), ParticleFamily::Meson)
}

/// Returns `true` when `kind` is a fermion.
#[must_use]
pub const fn is_fermion(kind: ParticleKind) -> bool {
    matches!(statistics(kind), ParticleStatistics::Fermion)
}

/// Returns the modeled antiparticle for `kind`.
///
/// Self-conjugate particles such as the photon return themselves.
///
/// # Examples
///
/// ```rust
/// use use_particle::{ParticleKind, antiparticle};
///
/// assert_eq!(antiparticle(ParticleKind::Electron), Some(ParticleKind::Positron));
/// assert_eq!(antiparticle(ParticleKind::Photon), Some(ParticleKind::Photon));
/// ```
#[must_use]
#[allow(clippy::unnecessary_wraps)]
pub const fn antiparticle(kind: ParticleKind) -> Option<ParticleKind> {
    match kind {
        ParticleKind::Electron => Some(ParticleKind::Positron),
        ParticleKind::Positron => Some(ParticleKind::Electron),
        ParticleKind::Muon => Some(ParticleKind::Antimuon),
        ParticleKind::Antimuon => Some(ParticleKind::Muon),
        ParticleKind::Tau => Some(ParticleKind::Antitau),
        ParticleKind::Antitau => Some(ParticleKind::Tau),
        ParticleKind::ElectronNeutrino => Some(ParticleKind::ElectronAntineutrino),
        ParticleKind::ElectronAntineutrino => Some(ParticleKind::ElectronNeutrino),
        ParticleKind::MuonNeutrino => Some(ParticleKind::MuonAntineutrino),
        ParticleKind::MuonAntineutrino => Some(ParticleKind::MuonNeutrino),
        ParticleKind::TauNeutrino => Some(ParticleKind::TauAntineutrino),
        ParticleKind::TauAntineutrino => Some(ParticleKind::TauNeutrino),
        ParticleKind::UpQuark => Some(ParticleKind::AntiUpQuark),
        ParticleKind::AntiUpQuark => Some(ParticleKind::UpQuark),
        ParticleKind::DownQuark => Some(ParticleKind::AntiDownQuark),
        ParticleKind::AntiDownQuark => Some(ParticleKind::DownQuark),
        ParticleKind::CharmQuark => Some(ParticleKind::AntiCharmQuark),
        ParticleKind::AntiCharmQuark => Some(ParticleKind::CharmQuark),
        ParticleKind::StrangeQuark => Some(ParticleKind::AntiStrangeQuark),
        ParticleKind::AntiStrangeQuark => Some(ParticleKind::StrangeQuark),
        ParticleKind::TopQuark => Some(ParticleKind::AntiTopQuark),
        ParticleKind::AntiTopQuark => Some(ParticleKind::TopQuark),
        ParticleKind::BottomQuark => Some(ParticleKind::AntiBottomQuark),
        ParticleKind::AntiBottomQuark => Some(ParticleKind::BottomQuark),
        ParticleKind::Photon
        | ParticleKind::Gluon
        | ParticleKind::ZBoson
        | ParticleKind::HiggsBoson => Some(kind),
        ParticleKind::WPlusBoson => Some(ParticleKind::WMinusBoson),
        ParticleKind::WMinusBoson => Some(ParticleKind::WPlusBoson),
        ParticleKind::Proton => Some(ParticleKind::Antiproton),
        ParticleKind::Antiproton => Some(ParticleKind::Proton),
        ParticleKind::Neutron => Some(ParticleKind::Antineutron),
        ParticleKind::Antineutron => Some(ParticleKind::Neutron),
        ParticleKind::PionPlus => Some(ParticleKind::PionMinus),
        ParticleKind::PionMinus => Some(ParticleKind::PionPlus),
        ParticleKind::PionZero => Some(ParticleKind::PionZero),
    }
}

/// Returns `true` when `kind` is represented as an antiparticle variant.
#[must_use]
pub const fn is_antiparticle(kind: ParticleKind) -> bool {
    matches!(
        kind,
        ParticleKind::Positron
            | ParticleKind::Antimuon
            | ParticleKind::Antitau
            | ParticleKind::ElectronAntineutrino
            | ParticleKind::MuonAntineutrino
            | ParticleKind::TauAntineutrino
            | ParticleKind::AntiUpQuark
            | ParticleKind::AntiDownQuark
            | ParticleKind::AntiCharmQuark
            | ParticleKind::AntiStrangeQuark
            | ParticleKind::AntiTopQuark
            | ParticleKind::AntiBottomQuark
            | ParticleKind::WMinusBoson
            | ParticleKind::Antiproton
            | ParticleKind::Antineutron
            | ParticleKind::PionMinus
    )
}

/// Returns `true` when `kind` is its own antiparticle.
#[must_use]
pub const fn is_self_antiparticle(kind: ParticleKind) -> bool {
    matches!(
        kind,
        ParticleKind::Photon
            | ParticleKind::Gluon
            | ParticleKind::ZBoson
            | ParticleKind::HiggsBoson
            | ParticleKind::PionZero
    )
}

/// Returns an approximate rest mass in `MeV/c^2` for `kind`.
///
/// This metadata is intended for practical examples rather than precision reference work.
#[must_use]
pub const fn rest_mass_mev_c2(kind: ParticleKind) -> Option<f64> {
    match kind {
        ParticleKind::Electron | ParticleKind::Positron => Some(0.511),
        ParticleKind::Muon | ParticleKind::Antimuon => Some(105.658),
        ParticleKind::Tau | ParticleKind::Antitau => Some(1_776.86),
        ParticleKind::ElectronNeutrino
        | ParticleKind::ElectronAntineutrino
        | ParticleKind::MuonNeutrino
        | ParticleKind::MuonAntineutrino
        | ParticleKind::TauNeutrino
        | ParticleKind::TauAntineutrino
        | ParticleKind::UpQuark
        | ParticleKind::AntiUpQuark
        | ParticleKind::DownQuark
        | ParticleKind::AntiDownQuark
        | ParticleKind::CharmQuark
        | ParticleKind::AntiCharmQuark
        | ParticleKind::StrangeQuark
        | ParticleKind::AntiStrangeQuark
        | ParticleKind::TopQuark
        | ParticleKind::AntiTopQuark
        | ParticleKind::BottomQuark
        | ParticleKind::AntiBottomQuark => None,
        ParticleKind::Photon | ParticleKind::Gluon => Some(0.0),
        ParticleKind::WPlusBoson | ParticleKind::WMinusBoson => Some(80_379.0),
        ParticleKind::ZBoson => Some(91_188.0),
        ParticleKind::HiggsBoson => Some(125_250.0),
        ParticleKind::Proton | ParticleKind::Antiproton => Some(938.272),
        ParticleKind::Neutron | ParticleKind::Antineutron => Some(939.565),
        ParticleKind::PionPlus | ParticleKind::PionMinus => Some(139.570),
        ParticleKind::PionZero => Some(134.977),
    }
}

/// Returns whether `kind` has nonzero rest mass when that metadata is modeled here.
#[must_use]
pub const fn has_rest_mass(kind: ParticleKind) -> Option<bool> {
    match kind {
        ParticleKind::Photon | ParticleKind::Gluon => Some(false),
        ParticleKind::Electron
        | ParticleKind::Positron
        | ParticleKind::Muon
        | ParticleKind::Antimuon
        | ParticleKind::Tau
        | ParticleKind::Antitau
        | ParticleKind::WPlusBoson
        | ParticleKind::WMinusBoson
        | ParticleKind::ZBoson
        | ParticleKind::HiggsBoson
        | ParticleKind::Proton
        | ParticleKind::Antiproton
        | ParticleKind::Neutron
        | ParticleKind::Antineutron
        | ParticleKind::PionPlus
        | ParticleKind::PionMinus
        | ParticleKind::PionZero => Some(true),
        ParticleKind::ElectronNeutrino
        | ParticleKind::ElectronAntineutrino
        | ParticleKind::MuonNeutrino
        | ParticleKind::MuonAntineutrino
        | ParticleKind::TauNeutrino
        | ParticleKind::TauAntineutrino
        | ParticleKind::UpQuark
        | ParticleKind::AntiUpQuark
        | ParticleKind::DownQuark
        | ParticleKind::AntiDownQuark
        | ParticleKind::CharmQuark
        | ParticleKind::AntiCharmQuark
        | ParticleKind::StrangeQuark
        | ParticleKind::AntiStrangeQuark
        | ParticleKind::TopQuark
        | ParticleKind::AntiTopQuark
        | ParticleKind::BottomQuark
        | ParticleKind::AntiBottomQuark => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ElementaryCharge, Particle, ParticleFamily, ParticleKind, ParticleStatistics, Spin,
        antiparticle, charge, charge_thirds, family, has_rest_mass, is_antiparticle, is_baryon,
        is_boson, is_fermion, is_lepton, is_meson, is_quark, is_self_antiparticle,
        rest_mass_mev_c2, spin, statistics,
    };

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn charge_helpers_cover_common_particles() {
        assert_eq!(charge_thirds(ParticleKind::Electron), -3);
        assert_eq!(charge_thirds(ParticleKind::Positron), 3);
        assert_eq!(charge_thirds(ParticleKind::UpQuark), 2);
        assert_eq!(charge_thirds(ParticleKind::DownQuark), -1);
        assert_eq!(charge_thirds(ParticleKind::Photon), 0);

        assert!(approx_eq(
            charge(ParticleKind::Electron).as_elementary_units(),
            -1.0
        ));
        assert!(approx_eq(
            charge(ParticleKind::UpQuark).as_elementary_units(),
            0.666_666_666_7,
        ));
    }

    #[test]
    fn spin_helpers_follow_statistics_rules() {
        assert_eq!(spin(ParticleKind::Electron), Spin::half());
        assert_eq!(spin(ParticleKind::Photon), Spin::one());
        assert_eq!(spin(ParticleKind::HiggsBoson), Spin::zero());

        assert_eq!(
            statistics(ParticleKind::Electron),
            ParticleStatistics::Fermion
        );
        assert_eq!(statistics(ParticleKind::Photon), ParticleStatistics::Boson);
        assert_eq!(
            statistics(ParticleKind::HiggsBoson),
            ParticleStatistics::Boson
        );
    }

    #[test]
    fn family_helpers_group_particle_kinds() {
        assert_eq!(family(ParticleKind::Electron), ParticleFamily::Lepton);
        assert_eq!(family(ParticleKind::UpQuark), ParticleFamily::Quark);
        assert_eq!(family(ParticleKind::Photon), ParticleFamily::GaugeBoson);
        assert_eq!(
            family(ParticleKind::HiggsBoson),
            ParticleFamily::ScalarBoson
        );
        assert_eq!(family(ParticleKind::Proton), ParticleFamily::Baryon);
        assert_eq!(family(ParticleKind::PionPlus), ParticleFamily::Meson);

        assert!(is_lepton(ParticleKind::Electron));
        assert!(is_quark(ParticleKind::UpQuark));
        assert!(is_boson(ParticleKind::Photon));
        assert!(is_baryon(ParticleKind::Proton));
        assert!(is_meson(ParticleKind::PionZero));
        assert!(is_fermion(ParticleKind::Electron));
        assert!(!is_fermion(ParticleKind::Photon));
    }

    #[test]
    fn antimatter_helpers_cover_pairs_and_self_conjugate_particles() {
        assert_eq!(
            antiparticle(ParticleKind::Electron),
            Some(ParticleKind::Positron)
        );
        assert_eq!(
            antiparticle(ParticleKind::Positron),
            Some(ParticleKind::Electron)
        );
        assert_eq!(
            antiparticle(ParticleKind::Proton),
            Some(ParticleKind::Antiproton)
        );
        assert_eq!(
            antiparticle(ParticleKind::Photon),
            Some(ParticleKind::Photon)
        );

        assert!(is_antiparticle(ParticleKind::Positron));
        assert!(!is_antiparticle(ParticleKind::Electron));
        assert!(is_self_antiparticle(ParticleKind::Photon));
        assert!(is_self_antiparticle(ParticleKind::PionZero));
        assert!(!is_self_antiparticle(ParticleKind::Electron));
    }

    #[test]
    fn rest_mass_metadata_is_small_and_practical() {
        assert!(matches!(
            rest_mass_mev_c2(ParticleKind::Electron),
            Some(mass) if approx_eq(mass, 0.511)
        ));
        assert_eq!(rest_mass_mev_c2(ParticleKind::Photon), Some(0.0));
        assert_eq!(rest_mass_mev_c2(ParticleKind::ElectronNeutrino), None);

        assert_eq!(has_rest_mass(ParticleKind::Photon), Some(false));
        assert_eq!(has_rest_mass(ParticleKind::Electron), Some(true));
    }

    #[test]
    fn particle_wrapper_delegates_to_free_functions() {
        let electron = Particle::new(ParticleKind::Electron);

        assert_eq!(electron.charge(), ElementaryCharge::negative_one());
        assert_eq!(electron.family(), ParticleFamily::Lepton);
        assert!(matches!(
            electron.antiparticle(),
            Some(Particle {
                kind: ParticleKind::Positron
            })
        ));
    }
}
