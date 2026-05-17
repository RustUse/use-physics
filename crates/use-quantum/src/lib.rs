#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small quantum physics scalar helpers.

pub mod prelude;

/// Planck constant `h`, in joule seconds.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;

/// Reduced Planck constant `hbar`, in joule seconds.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const REDUCED_PLANCK_CONSTANT: f64 = 1.054_571_817e-34;

/// Speed of light in vacuum, in meters per second.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Elementary charge, in coulombs.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Electron rest mass, in kilograms.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31;

/// Bohr radius `a0`, in meters.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11;

/// Hydrogen Rydberg energy magnitude, in electron volts.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const RYDBERG_ENERGY_EV: f64 = 13.605_693_122_994;

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn principal_squared(principal: u32) -> Option<f64> {
    if principal == 0 {
        return None;
    }

    let principal = f64::from(principal);
    finite_result(principal * principal)
}

fn momentum_magnitude_from_mass_velocity(mass: f64, velocity: f64) -> Option<f64> {
    if !is_positive_finite(mass) || !velocity.is_finite() {
        return None;
    }

    let speed = velocity.abs();
    if speed == 0.0 {
        return None;
    }

    let momentum = mass * speed;
    (momentum.is_finite() && momentum > 0.0).then_some(momentum)
}

/// Computes photon energy from frequency using `E = h * f`.
///
/// Returns `None` when `frequency` is negative, when the input is not finite, or when the
/// computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::{PLANCK_CONSTANT, photon_energy_from_frequency};
///
/// let energy = photon_energy_from_frequency(1.0).ok_or("expected valid frequency")?;
///
/// assert_eq!(energy, PLANCK_CONSTANT);
/// # Ok::<(), &'static str>(())
/// ```
#[must_use]
pub fn photon_energy_from_frequency(frequency: f64) -> Option<f64> {
    if !is_nonnegative_finite(frequency) {
        return None;
    }

    finite_result(PLANCK_CONSTANT * frequency)
}

/// Computes photon energy from wavelength using `E = h * c / lambda`.
///
/// Returns `None` when `wavelength` is not positive and finite, or when the computed result is
/// not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::photon_energy_from_wavelength;
///
/// let energy = photon_energy_from_wavelength(500.0e-9).ok_or("expected valid wavelength")?;
///
/// assert!(energy > 0.0);
/// # Ok::<(), &'static str>(())
/// ```
#[must_use]
pub fn photon_energy_from_wavelength(wavelength: f64) -> Option<f64> {
    if !is_positive_finite(wavelength) {
        return None;
    }

    finite_result((PLANCK_CONSTANT * SPEED_OF_LIGHT) / wavelength)
}

/// Computes frequency from photon energy using `f = E / h`.
///
/// Returns `None` when `energy` is negative, when the input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn frequency_from_photon_energy(energy: f64) -> Option<f64> {
    if !is_nonnegative_finite(energy) {
        return None;
    }

    finite_result(energy / PLANCK_CONSTANT)
}

/// Computes wavelength from photon energy using `lambda = h * c / E`.
///
/// Returns `None` when `energy` is not positive and finite, or when the computed result is not
/// finite.
#[must_use]
pub fn wavelength_from_photon_energy(energy: f64) -> Option<f64> {
    if !is_positive_finite(energy) {
        return None;
    }

    finite_result((PLANCK_CONSTANT * SPEED_OF_LIGHT) / energy)
}

/// Computes photon momentum from wavelength using `p = h / lambda`.
///
/// Returns `None` when `wavelength` is not positive and finite, or when the computed result is
/// not finite.
#[must_use]
pub fn photon_momentum_from_wavelength(wavelength: f64) -> Option<f64> {
    if !is_positive_finite(wavelength) {
        return None;
    }

    finite_result(PLANCK_CONSTANT / wavelength)
}

/// Computes photon momentum from energy using `p = E / c`.
///
/// Returns `None` when `energy` is negative, when the input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn photon_momentum_from_energy(energy: f64) -> Option<f64> {
    if !is_nonnegative_finite(energy) {
        return None;
    }

    finite_result(energy / SPEED_OF_LIGHT)
}

/// Converts joules to electron volts using `eV = J / e`.
///
/// Returns `None` when `joules` is negative, when the input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn joules_to_electron_volts(joules: f64) -> Option<f64> {
    if !is_nonnegative_finite(joules) {
        return None;
    }

    finite_result(joules / ELEMENTARY_CHARGE)
}

/// Converts electron volts to joules using `J = eV * e`.
///
/// Returns `None` when `electron_volts` is negative, when the input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn electron_volts_to_joules(electron_volts: f64) -> Option<f64> {
    if !is_nonnegative_finite(electron_volts) {
        return None;
    }

    finite_result(electron_volts * ELEMENTARY_CHARGE)
}

/// Computes de Broglie wavelength from momentum magnitude using `lambda = h / p`.
///
/// Returns `None` when `momentum` is not positive and finite, or when the computed result is not
/// finite.
#[must_use]
pub fn de_broglie_wavelength(momentum: f64) -> Option<f64> {
    if !is_positive_finite(momentum) {
        return None;
    }

    finite_result(PLANCK_CONSTANT / momentum)
}

/// Computes de Broglie wavelength from mass and velocity using `lambda = h / (m * |v|)`.
///
/// Returns `None` when `mass` is not positive and finite, when `velocity` is zero or not finite,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::{PLANCK_CONSTANT, de_broglie_wavelength_from_mass_velocity};
///
/// let wavelength =
///     de_broglie_wavelength_from_mass_velocity(2.0, 3.0).ok_or("expected valid inputs")?;
///
/// assert!((wavelength - (PLANCK_CONSTANT / 6.0)).abs() < 1.0e-12);
/// # Ok::<(), &'static str>(())
/// ```
#[must_use]
pub fn de_broglie_wavelength_from_mass_velocity(mass: f64, velocity: f64) -> Option<f64> {
    de_broglie_wavelength(momentum_magnitude_from_mass_velocity(mass, velocity)?)
}

/// Computes momentum magnitude from a de Broglie wavelength using `p = h / lambda`.
///
/// Returns `None` when `wavelength` is not positive and finite, or when the computed result is
/// not finite.
#[must_use]
pub fn momentum_from_de_broglie_wavelength(wavelength: f64) -> Option<f64> {
    photon_momentum_from_wavelength(wavelength)
}

/// Computes angular frequency from energy using `omega = E / hbar`.
///
/// Returns `None` when `energy` is negative, when the input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn angular_frequency_from_energy(energy: f64) -> Option<f64> {
    if !is_nonnegative_finite(energy) {
        return None;
    }

    finite_result(energy / REDUCED_PLANCK_CONSTANT)
}

/// Computes energy from angular frequency using `E = hbar * omega`.
///
/// Returns `None` when `angular_frequency` is negative, when the input is not finite, or when
/// the computed result is not finite.
#[must_use]
pub fn energy_from_angular_frequency(angular_frequency: f64) -> Option<f64> {
    if !is_nonnegative_finite(angular_frequency) {
        return None;
    }

    finite_result(REDUCED_PLANCK_CONSTANT * angular_frequency)
}

fn minimum_conjugate_uncertainty(uncertainty: f64) -> Option<f64> {
    if !is_positive_finite(uncertainty) {
        return None;
    }

    finite_result(REDUCED_PLANCK_CONSTANT / (2.0 * uncertainty))
}

/// Computes the minimum position uncertainty estimate from `delta x >= hbar / (2 * delta p)`.
///
/// Returns `None` when `momentum_uncertainty` is not positive and finite, or when the computed
/// result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::{REDUCED_PLANCK_CONSTANT, minimum_position_uncertainty};
///
/// let position_uncertainty =
///     minimum_position_uncertainty(REDUCED_PLANCK_CONSTANT).ok_or("expected valid input")?;
///
/// assert!((position_uncertainty - 0.5).abs() < 1.0e-12);
/// # Ok::<(), &'static str>(())
/// ```
#[must_use]
pub fn minimum_position_uncertainty(momentum_uncertainty: f64) -> Option<f64> {
    minimum_conjugate_uncertainty(momentum_uncertainty)
}

/// Computes the minimum momentum uncertainty estimate from `delta p >= hbar / (2 * delta x)`.
///
/// Returns `None` when `position_uncertainty` is not positive and finite, or when the computed
/// result is not finite.
#[must_use]
pub fn minimum_momentum_uncertainty(position_uncertainty: f64) -> Option<f64> {
    minimum_conjugate_uncertainty(position_uncertainty)
}

/// Computes the minimum energy uncertainty estimate from `delta E >= hbar / (2 * delta t)`.
///
/// Returns `None` when `time_uncertainty` is not positive and finite, or when the computed
/// result is not finite.
#[must_use]
pub fn minimum_energy_uncertainty(time_uncertainty: f64) -> Option<f64> {
    minimum_conjugate_uncertainty(time_uncertainty)
}

/// Computes the minimum time uncertainty estimate from `delta t >= hbar / (2 * delta E)`.
///
/// Returns `None` when `energy_uncertainty` is not positive and finite, or when the computed
/// result is not finite.
#[must_use]
pub fn minimum_time_uncertainty(energy_uncertainty: f64) -> Option<f64> {
    minimum_conjugate_uncertainty(energy_uncertainty)
}

/// Computes the hydrogen-like Bohr orbit radius for `Z = 1` using `r_n = a0 * n^2`.
///
/// Returns `None` when `n == 0` or when the computed result is not finite.
#[must_use]
pub fn bohr_orbit_radius(n: u32) -> Option<f64> {
    finite_result(BOHR_RADIUS * principal_squared(n)?)
}

/// Computes the hydrogen energy level in electron volts using `E_n = -Ry / n^2`.
///
/// Returns `None` when `n == 0` or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::{RYDBERG_ENERGY_EV, hydrogen_energy_level_ev};
///
/// assert_eq!(hydrogen_energy_level_ev(1), Some(-RYDBERG_ENERGY_EV));
/// ```
#[must_use]
pub fn hydrogen_energy_level_ev(n: u32) -> Option<f64> {
    finite_result(-RYDBERG_ENERGY_EV / principal_squared(n)?)
}

/// Computes the absolute transition energy between two hydrogen energy levels in electron volts.
///
/// Returns `None` when either quantum number is zero or when the computed result is not finite.
/// Returns `Some(0.0)` when the levels are equal.
#[must_use]
pub fn hydrogen_transition_energy_ev(initial_n: u32, final_n: u32) -> Option<f64> {
    if initial_n == 0 || final_n == 0 {
        return None;
    }

    if initial_n == final_n {
        return Some(0.0);
    }

    let initial = hydrogen_energy_level_ev(initial_n)?;
    let final_ = hydrogen_energy_level_ev(final_n)?;

    finite_result((final_ - initial).abs())
}

/// Computes the photon wavelength for a hydrogen transition in meters.
///
/// Returns `None` when either quantum number is zero, when the transition energy is zero, or
/// when any intermediate conversion is not finite.
///
/// # Examples
///
/// ```rust
/// use use_quantum::hydrogen_transition_wavelength;
///
/// let wavelength =
///     hydrogen_transition_wavelength(2, 1).ok_or("expected valid transition")?;
///
/// assert!(wavelength.is_finite() && wavelength > 0.0);
/// # Ok::<(), &'static str>(())
/// ```
#[must_use]
pub fn hydrogen_transition_wavelength(initial_n: u32, final_n: u32) -> Option<f64> {
    let transition_energy_ev = hydrogen_transition_energy_ev(initial_n, final_n)?;
    if transition_energy_ev == 0.0 {
        return None;
    }

    wavelength_from_photon_energy(electron_volts_to_joules(transition_energy_ev)?)
}

/// Quantum numbers for a single-electron atomic-state style validation helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumNumbers {
    /// Principal quantum number `n`.
    pub principal: u32,
    /// Azimuthal quantum number `l`.
    pub azimuthal: u32,
    /// Magnetic quantum number `m_l`.
    pub magnetic: i32,
    /// Twice the spin projection. `1` means `+1/2`, `-1` means `-1/2`.
    pub spin_twice: i8,
}

/// Returns `true` when `n >= 1`.
#[must_use]
pub const fn is_valid_principal_quantum_number(n: u32) -> bool {
    n >= 1
}

/// Returns `true` when `n >= 1` and `l < n`.
#[must_use]
pub const fn is_valid_azimuthal_quantum_number(n: u32, l: u32) -> bool {
    is_valid_principal_quantum_number(n) && l < n
}

/// Returns `true` when `-l <= m_l <= l`.
#[must_use]
pub fn is_valid_magnetic_quantum_number(l: u32, m_l: i32) -> bool {
    let l = i64::from(l);
    let magnetic = i64::from(m_l);

    (-l..=l).contains(&magnetic)
}

/// Returns `true` when the spin projection is one of `-1` or `1`.
#[must_use]
pub const fn is_valid_spin_twice(spin_twice: i8) -> bool {
    matches!(spin_twice, -1 | 1)
}

/// Returns `true` when the supplied quantum-number combination is valid.
#[must_use]
pub fn is_valid_quantum_numbers(
    principal: u32,
    azimuthal: u32,
    magnetic: i32,
    spin_twice: i8,
) -> bool {
    is_valid_azimuthal_quantum_number(principal, azimuthal)
        && is_valid_magnetic_quantum_number(azimuthal, magnetic)
        && is_valid_spin_twice(spin_twice)
}

impl QuantumNumbers {
    /// Creates validated quantum numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_quantum::QuantumNumbers;
    ///
    /// let quantum_numbers = QuantumNumbers::new(2, 1, 0, 1);
    ///
    /// assert_eq!(
    ///     quantum_numbers,
    ///     Some(QuantumNumbers {
    ///         principal: 2,
    ///         azimuthal: 1,
    ///         magnetic: 0,
    ///         spin_twice: 1,
    ///     })
    /// );
    /// ```
    #[must_use]
    pub fn new(principal: u32, azimuthal: u32, magnetic: i32, spin_twice: i8) -> Option<Self> {
        is_valid_quantum_numbers(principal, azimuthal, magnetic, spin_twice).then_some(Self {
            principal,
            azimuthal,
            magnetic,
            spin_twice,
        })
    }

    /// Returns the spin projection in units of `hbar`.
    #[must_use]
    pub fn spin_projection(&self) -> f64 {
        f64::from(self.spin_twice) / 2.0
    }
}

/// A lightweight photon wrapper stored by energy in joules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Photon {
    /// Photon energy in joules.
    pub energy_joules: f64,
}

impl Photon {
    /// Creates a photon from non-negative finite energy in joules.
    #[must_use]
    pub fn from_energy_joules(energy_joules: f64) -> Option<Self> {
        is_nonnegative_finite(energy_joules).then_some(Self { energy_joules })
    }

    /// Creates a photon from frequency.
    #[must_use]
    pub fn from_frequency(frequency: f64) -> Option<Self> {
        Self::from_energy_joules(photon_energy_from_frequency(frequency)?)
    }

    /// Creates a photon from wavelength.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_quantum::Photon;
    ///
    /// let photon = Photon::from_wavelength(500.0e-9).ok_or("expected valid wavelength")?;
    ///
    /// assert!(photon.energy_joules() > 0.0);
    /// # Ok::<(), &'static str>(())
    /// ```
    #[must_use]
    pub fn from_wavelength(wavelength: f64) -> Option<Self> {
        Self::from_energy_joules(photon_energy_from_wavelength(wavelength)?)
    }

    /// Returns the photon energy in joules.
    #[must_use]
    pub const fn energy_joules(&self) -> f64 {
        self.energy_joules
    }

    /// Returns the photon energy in electron volts.
    #[must_use]
    pub fn energy_ev(&self) -> Option<f64> {
        joules_to_electron_volts(self.energy_joules)
    }

    /// Returns the photon frequency in hertz.
    #[must_use]
    pub fn frequency(&self) -> Option<f64> {
        frequency_from_photon_energy(self.energy_joules)
    }

    /// Returns the photon wavelength in meters.
    #[must_use]
    pub fn wavelength(&self) -> Option<f64> {
        wavelength_from_photon_energy(self.energy_joules)
    }

    /// Returns the photon momentum magnitude in kilogram meters per second.
    #[must_use]
    pub fn momentum(&self) -> Option<f64> {
        photon_momentum_from_energy(self.energy_joules)
    }
}

/// A lightweight matter-wave wrapper stored by momentum magnitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatterWave {
    /// Momentum magnitude in kilogram meters per second.
    pub momentum: f64,
}

impl MatterWave {
    /// Creates a matter wave from a positive finite momentum magnitude.
    #[must_use]
    pub fn from_momentum(momentum: f64) -> Option<Self> {
        is_positive_finite(momentum).then_some(Self { momentum })
    }

    /// Creates a matter wave from mass and velocity magnitude.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_quantum::{MatterWave, PLANCK_CONSTANT};
    ///
    /// let wave = MatterWave::from_mass_velocity(2.0, 3.0).ok_or("expected valid inputs")?;
    ///
    /// assert!((wave.wavelength().ok_or("expected wavelength")? - (PLANCK_CONSTANT / 6.0)).abs() < 1.0e-12);
    /// # Ok::<(), &'static str>(())
    /// ```
    #[must_use]
    pub fn from_mass_velocity(mass: f64, velocity: f64) -> Option<Self> {
        Self::from_momentum(momentum_magnitude_from_mass_velocity(mass, velocity)?)
    }

    /// Returns the de Broglie wavelength in meters.
    #[must_use]
    pub fn wavelength(&self) -> Option<f64> {
        de_broglie_wavelength(self.momentum)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    fn approx_eq(left: f64, right: f64) -> bool {
        let scale = left.abs().max(right.abs()).max(1.0);
        (left - right).abs() <= 1.0e-12 * scale
    }

    fn assert_approx_eq(left: f64, right: f64) {
        assert!(
            approx_eq(left, right),
            "left={left:e} right={right:e} delta={:e}",
            (left - right).abs()
        );
    }

    fn assert_some_approx_eq(value: Option<f64>, expected: f64) {
        match value {
            Some(actual) => assert_approx_eq(actual, expected),
            None => panic!("expected Some({expected:e})"),
        }
    }

    #[test]
    fn photon_energy_helpers_cover_frequency_and_wavelength() {
        assert_eq!(photon_energy_from_frequency(1.0), Some(PLANCK_CONSTANT));
        assert_eq!(photon_energy_from_frequency(-1.0), None);

        assert_some_approx_eq(
            photon_energy_from_wavelength(SPEED_OF_LIGHT),
            PLANCK_CONSTANT,
        );
        assert_eq!(photon_energy_from_wavelength(0.0), None);
    }

    #[test]
    fn photon_frequency_and_wavelength_helpers_invert_energy() {
        assert_some_approx_eq(frequency_from_photon_energy(PLANCK_CONSTANT), 1.0);
        assert_eq!(frequency_from_photon_energy(-1.0), None);

        assert_some_approx_eq(
            wavelength_from_photon_energy(PLANCK_CONSTANT),
            SPEED_OF_LIGHT,
        );
        assert_eq!(wavelength_from_photon_energy(0.0), None);
    }

    #[test]
    fn photon_momentum_and_energy_conversion_helpers_work() {
        assert_some_approx_eq(photon_momentum_from_wavelength(PLANCK_CONSTANT), 1.0);
        assert_some_approx_eq(photon_momentum_from_energy(SPEED_OF_LIGHT), 1.0);

        assert_some_approx_eq(joules_to_electron_volts(ELEMENTARY_CHARGE), 1.0);
        assert_some_approx_eq(electron_volts_to_joules(1.0), ELEMENTARY_CHARGE);
    }

    #[test]
    fn matter_wave_helpers_cover_momentum_and_mass_velocity() {
        assert_some_approx_eq(de_broglie_wavelength(PLANCK_CONSTANT), 1.0);
        assert_eq!(de_broglie_wavelength(0.0), None);

        assert_some_approx_eq(
            de_broglie_wavelength_from_mass_velocity(2.0, 3.0),
            PLANCK_CONSTANT / 6.0,
        );
        assert_eq!(de_broglie_wavelength_from_mass_velocity(2.0, 0.0), None);
        assert_eq!(de_broglie_wavelength_from_mass_velocity(0.0, 3.0), None);

        assert_some_approx_eq(momentum_from_de_broglie_wavelength(PLANCK_CONSTANT), 1.0);
    }

    #[test]
    fn reduced_planck_and_uncertainty_helpers_work() {
        assert_some_approx_eq(angular_frequency_from_energy(REDUCED_PLANCK_CONSTANT), 1.0);
        assert_some_approx_eq(energy_from_angular_frequency(1.0), REDUCED_PLANCK_CONSTANT);

        assert_some_approx_eq(minimum_position_uncertainty(REDUCED_PLANCK_CONSTANT), 0.5);
        assert_eq!(minimum_position_uncertainty(0.0), None);

        assert_some_approx_eq(minimum_momentum_uncertainty(REDUCED_PLANCK_CONSTANT), 0.5);
        assert_eq!(minimum_momentum_uncertainty(0.0), None);

        assert_some_approx_eq(minimum_energy_uncertainty(REDUCED_PLANCK_CONSTANT), 0.5);
        assert_eq!(minimum_energy_uncertainty(0.0), None);

        assert_some_approx_eq(minimum_time_uncertainty(REDUCED_PLANCK_CONSTANT), 0.5);
        assert_eq!(minimum_time_uncertainty(0.0), None);
    }

    #[test]
    fn bohr_model_helpers_cover_levels_and_transitions() {
        assert_some_approx_eq(bohr_orbit_radius(1), BOHR_RADIUS);
        assert_some_approx_eq(bohr_orbit_radius(2), 4.0 * BOHR_RADIUS);
        assert_eq!(bohr_orbit_radius(0), None);

        assert_some_approx_eq(hydrogen_energy_level_ev(1), -RYDBERG_ENERGY_EV);
        assert_some_approx_eq(hydrogen_energy_level_ev(2), -RYDBERG_ENERGY_EV / 4.0);
        assert_eq!(hydrogen_energy_level_ev(0), None);

        assert_some_approx_eq(hydrogen_transition_energy_ev(2, 1), 10.204_269_842_245_5);
        assert_eq!(hydrogen_transition_energy_ev(1, 1), Some(0.0));
        assert_eq!(hydrogen_transition_energy_ev(0, 1), None);

        match hydrogen_transition_wavelength(2, 1) {
            Some(wavelength) => assert!(wavelength.is_finite() && wavelength > 0.0),
            None => panic!("expected a valid transition wavelength"),
        }
        assert_eq!(hydrogen_transition_wavelength(1, 1), None);
    }

    #[test]
    fn quantum_number_helpers_validate_expected_ranges() {
        assert!(is_valid_principal_quantum_number(1));
        assert!(!is_valid_principal_quantum_number(0));

        assert!(is_valid_azimuthal_quantum_number(1, 0));
        assert!(!is_valid_azimuthal_quantum_number(1, 1));

        assert!(is_valid_magnetic_quantum_number(1, -1));
        assert!(is_valid_magnetic_quantum_number(1, 0));
        assert!(is_valid_magnetic_quantum_number(1, 1));
        assert!(!is_valid_magnetic_quantum_number(1, 2));

        assert!(is_valid_spin_twice(1));
        assert!(is_valid_spin_twice(-1));
        assert!(!is_valid_spin_twice(0));

        assert!(is_valid_quantum_numbers(2, 1, 0, 1));
        assert!(!is_valid_quantum_numbers(2, 2, 0, 1));

        match QuantumNumbers::new(2, 1, 0, 1) {
            Some(quantum_numbers) => assert_eq!(quantum_numbers.spin_projection(), 0.5),
            None => panic!("expected valid quantum numbers"),
        }
        assert_eq!(QuantumNumbers::new(2, 2, 0, 1), None);
    }

    #[test]
    fn photon_wrapper_delegates_to_public_helpers() {
        match Photon::from_frequency(1.0) {
            Some(photon) => assert_eq!(photon.energy_joules(), PLANCK_CONSTANT),
            None => panic!("expected a valid photon from frequency"),
        }

        match Photon::from_wavelength(SPEED_OF_LIGHT) {
            Some(photon) => assert_some_approx_eq(photon.frequency(), 1.0),
            None => panic!("expected a valid photon from wavelength"),
        }

        match Photon::from_energy_joules(PLANCK_CONSTANT) {
            Some(photon) => assert_some_approx_eq(photon.wavelength(), SPEED_OF_LIGHT),
            None => panic!("expected a valid photon from energy"),
        }

        assert_eq!(Photon::from_energy_joules(-1.0), None);
    }

    #[test]
    fn matter_wave_wrapper_delegates_to_public_helpers() {
        match MatterWave::from_momentum(PLANCK_CONSTANT) {
            Some(wave) => assert_some_approx_eq(wave.wavelength(), 1.0),
            None => panic!("expected a valid matter wave from momentum"),
        }

        match MatterWave::from_mass_velocity(2.0, 3.0) {
            Some(wave) => assert_some_approx_eq(wave.wavelength(), PLANCK_CONSTANT / 6.0),
            None => panic!("expected a valid matter wave from mass and velocity"),
        }

        assert_eq!(MatterWave::from_momentum(0.0), None);
    }
}
