#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small nuclear physics helpers.

pub mod prelude;

const SPEED_OF_LIGHT_SQUARED: f64 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;

/// Natural logarithm of `2` for half-life conversions and decay-law helpers.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const LN_2: f64 = std::f64::consts::LN_2;

/// Speed of light in vacuum in meters per second.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Joules in one mega-electron-volt.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const JOULES_PER_MEV: f64 = 1.602_176_634e-13;

/// Atomic mass unit energy equivalent in mega-electron-volts per `c^2`.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ATOMIC_MASS_UNIT_MEV_C2: f64 = 931.494_102_42;

/// Simple exponential-decay law parameterized by a decay constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecayLaw {
    /// Decay constant in inverse time units.
    pub decay_constant: f64,
}

impl DecayLaw {
    /// Creates a decay law from a non-negative, finite decay constant.
    #[must_use]
    pub fn from_decay_constant(decay_constant: f64) -> Option<Self> {
        if is_non_negative_finite(decay_constant) {
            Some(Self { decay_constant })
        } else {
            None
        }
    }

    /// Creates a decay law from a positive, finite half-life.
    #[must_use]
    pub fn from_half_life(half_life: f64) -> Option<Self> {
        Self::from_decay_constant(decay_constant_from_half_life(half_life)?)
    }

    /// Returns the half-life for this decay law.
    #[must_use]
    pub fn half_life(&self) -> Option<f64> {
        half_life_from_decay_constant(self.decay_constant)
    }

    /// Returns the mean lifetime for this decay law.
    #[must_use]
    pub fn mean_lifetime(&self) -> Option<f64> {
        mean_lifetime(self.decay_constant)
    }

    /// Returns the remaining fraction after the given elapsed time.
    #[must_use]
    pub fn remaining_fraction(&self, time: f64) -> Option<f64> {
        remaining_fraction_from_decay_constant(self.decay_constant, time)
    }

    /// Returns the remaining quantity after the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_nuclear::DecayLaw;
    ///
    /// # fn main() -> Result<(), &'static str> {
    /// let decay_law = DecayLaw::from_half_life(10.0).ok_or("expected valid half-life")?;
    /// let remaining = decay_law
    ///     .remaining_quantity(100.0, 10.0)
    ///     .ok_or("expected valid remaining quantity")?;
    ///
    /// assert!((remaining - 50.0).abs() < 1.0e-12);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn remaining_quantity(&self, initial_quantity: f64, time: f64) -> Option<f64> {
        remaining_quantity_from_decay_constant(initial_quantity, self.decay_constant, time)
    }

    /// Returns the decayed quantity after the given elapsed time.
    #[must_use]
    pub fn decayed_quantity(&self, initial_quantity: f64, time: f64) -> Option<f64> {
        decayed_quantity_from_decay_constant(initial_quantity, self.decay_constant, time)
    }

    /// Returns the activity for the given number of nuclei.
    #[must_use]
    pub fn activity(&self, number_of_nuclei: f64) -> Option<f64> {
        activity(self.decay_constant, number_of_nuclei)
    }
}

/// Mass and atomic numbers for a nuclide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NuclideNumbers {
    /// Mass number `A`, equal to total nucleons.
    pub mass_number: u32,
    /// Atomic number `Z`, equal to total protons.
    pub atomic_number: u32,
}

impl NuclideNumbers {
    /// Creates validated nuclide numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_nuclear::NuclideNumbers;
    ///
    /// let helium = NuclideNumbers::new(4, 2);
    ///
    /// assert_eq!(helium, Some(NuclideNumbers { mass_number: 4, atomic_number: 2 }));
    /// ```
    #[must_use]
    pub const fn new(mass_number: u32, atomic_number: u32) -> Option<Self> {
        if is_valid_nuclide_numbers(mass_number, atomic_number) {
            Some(Self {
                mass_number,
                atomic_number,
            })
        } else {
            None
        }
    }

    /// Returns the proton count for this nuclide.
    #[must_use]
    pub const fn proton_count(&self) -> u32 {
        self.atomic_number
    }

    /// Returns the neutron count for this nuclide.
    #[must_use]
    pub const fn neutron_count(&self) -> u32 {
        self.mass_number - self.atomic_number
    }

    /// Returns the total nucleon count for this nuclide.
    #[must_use]
    pub const fn nucleon_count(&self) -> u32 {
        self.mass_number
    }
}

/// Computes a decay constant from a half-life.
///
/// Formula: `lambda = ln(2) / t_half`.
///
/// Returns `None` when `half_life` is not positive and finite, or when the computed decay
/// constant is not finite.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::{decay_constant_from_half_life, LN_2};
///
/// # fn main() -> Result<(), &'static str> {
/// let decay_constant =
///     decay_constant_from_half_life(10.0).ok_or("expected valid decay constant")?;
///
/// assert!((decay_constant - (LN_2 / 10.0)).abs() < 1.0e-12);
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn decay_constant_from_half_life(half_life: f64) -> Option<f64> {
    if !is_positive_finite(half_life) {
        return None;
    }

    finite_non_negative(LN_2 / half_life)
}

/// Computes a half-life from a decay constant.
///
/// Formula: `t_half = ln(2) / lambda`.
#[must_use]
pub fn half_life_from_decay_constant(decay_constant: f64) -> Option<f64> {
    if !is_positive_finite(decay_constant) {
        return None;
    }

    finite_non_negative(LN_2 / decay_constant)
}

/// Computes the mean lifetime from a decay constant.
///
/// Formula: `tau = 1 / lambda`.
#[must_use]
pub fn mean_lifetime(decay_constant: f64) -> Option<f64> {
    if !is_positive_finite(decay_constant) {
        return None;
    }

    finite_non_negative(1.0 / decay_constant)
}

/// Computes the remaining fraction from a decay constant and elapsed time.
///
/// Formula: `N / N0 = exp(-lambda * t)`.
#[must_use]
pub fn remaining_fraction_from_decay_constant(decay_constant: f64, time: f64) -> Option<f64> {
    if !is_non_negative_finite(decay_constant) || !is_non_negative_finite(time) {
        return None;
    }

    finite_unit_interval((-(decay_constant * time)).exp())
}

/// Computes the remaining fraction from a half-life and elapsed time.
///
/// Formula: `N / N0 = 2^(-t / t_half)`.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::remaining_fraction_from_half_life;
///
/// # fn main() -> Result<(), &'static str> {
/// let remaining_fraction = remaining_fraction_from_half_life(10.0, 10.0)
///     .ok_or("expected valid remaining fraction")?;
///
/// assert!((remaining_fraction - 0.5).abs() < 1.0e-12);
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn remaining_fraction_from_half_life(half_life: f64, time: f64) -> Option<f64> {
    if !is_positive_finite(half_life) || !is_non_negative_finite(time) {
        return None;
    }

    finite_unit_interval((-(time / half_life)).exp2())
}

/// Computes the remaining quantity from an initial quantity, decay constant, and elapsed time.
///
/// Formula: `N = N0 * exp(-lambda * t)`.
#[must_use]
pub fn remaining_quantity_from_decay_constant(
    initial_quantity: f64,
    decay_constant: f64,
    time: f64,
) -> Option<f64> {
    if !is_non_negative_finite(initial_quantity) {
        return None;
    }

    finite_non_negative(
        initial_quantity * remaining_fraction_from_decay_constant(decay_constant, time)?,
    )
}

/// Computes the remaining quantity from an initial quantity, half-life, and elapsed time.
#[must_use]
pub fn remaining_quantity_from_half_life(
    initial_quantity: f64,
    half_life: f64,
    time: f64,
) -> Option<f64> {
    if !is_non_negative_finite(initial_quantity) {
        return None;
    }

    finite_non_negative(initial_quantity * remaining_fraction_from_half_life(half_life, time)?)
}

/// Computes the decayed fraction from a decay constant and elapsed time.
///
/// Formula: `fraction_decayed = 1 - exp(-lambda * t)`.
#[must_use]
pub fn decayed_fraction_from_decay_constant(decay_constant: f64, time: f64) -> Option<f64> {
    finite_unit_interval(1.0 - remaining_fraction_from_decay_constant(decay_constant, time)?)
}

/// Computes the decayed quantity from an initial quantity, decay constant, and elapsed time.
///
/// Formula: `N_decayed = N0 * (1 - exp(-lambda * t))`.
#[must_use]
pub fn decayed_quantity_from_decay_constant(
    initial_quantity: f64,
    decay_constant: f64,
    time: f64,
) -> Option<f64> {
    if !is_non_negative_finite(initial_quantity) {
        return None;
    }

    finite_non_negative(
        initial_quantity * decayed_fraction_from_decay_constant(decay_constant, time)?,
    )
}

/// Computes elapsed time from a remaining fraction and decay constant.
///
/// Formula: `t = -ln(f) / lambda`.
#[must_use]
pub fn elapsed_time_from_remaining_fraction(
    decay_constant: f64,
    remaining_fraction: f64,
) -> Option<f64> {
    if !is_positive_finite(decay_constant)
        || !remaining_fraction.is_finite()
        || remaining_fraction <= 0.0
        || remaining_fraction > 1.0
    {
        return None;
    }

    finite_non_negative(normalize_zero(-remaining_fraction.ln() / decay_constant))
}

/// Computes activity from a decay constant and number of nuclei.
///
/// Formula: `A = lambda * N`.
///
/// Activity is returned in becquerels when `decay_constant` is expressed per second.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::activity;
///
/// assert_eq!(activity(2.0, 10.0), Some(20.0));
/// ```
#[must_use]
pub fn activity(decay_constant: f64, number_of_nuclei: f64) -> Option<f64> {
    if !is_non_negative_finite(decay_constant) || !is_non_negative_finite(number_of_nuclei) {
        return None;
    }

    finite_non_negative(decay_constant * number_of_nuclei)
}

/// Computes the number of nuclei from activity and decay constant.
///
/// Formula: `N = A / lambda`.
#[must_use]
pub fn nuclei_from_activity(activity: f64, decay_constant: f64) -> Option<f64> {
    if !is_non_negative_finite(activity) || !is_positive_finite(decay_constant) {
        return None;
    }

    finite_non_negative(activity / decay_constant)
}

/// Computes specific activity from activity and mass.
///
/// Formula: `specific activity = A / m`.
#[must_use]
pub fn specific_activity(activity: f64, mass: f64) -> Option<f64> {
    if !is_non_negative_finite(activity) || !is_positive_finite(mass) {
        return None;
    }

    finite_non_negative(activity / mass)
}

/// Computes energy in joules from a mass defect in kilograms.
///
/// Formula: `E = delta_m * c^2`.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::{energy_from_mass_defect_kg, SPEED_OF_LIGHT};
///
/// # fn main() -> Result<(), &'static str> {
/// let energy = energy_from_mass_defect_kg(1.0).ok_or("expected valid energy")?;
///
/// assert!((energy - (SPEED_OF_LIGHT * SPEED_OF_LIGHT)).abs() < 1.0e-12);
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn energy_from_mass_defect_kg(mass_defect_kg: f64) -> Option<f64> {
    if !is_non_negative_finite(mass_defect_kg) {
        return None;
    }

    finite_non_negative(mass_defect_kg * SPEED_OF_LIGHT_SQUARED)
}

/// Computes mass defect in kilograms from energy in joules.
///
/// Formula: `delta_m = E / c^2`.
#[must_use]
pub fn mass_defect_kg_from_energy(energy_joules: f64) -> Option<f64> {
    if !is_non_negative_finite(energy_joules) {
        return None;
    }

    finite_non_negative(energy_joules / SPEED_OF_LIGHT_SQUARED)
}

/// Converts energy in joules to mega-electron-volts.
#[must_use]
pub fn joules_to_mev(joules: f64) -> Option<f64> {
    if !is_non_negative_finite(joules) {
        return None;
    }

    finite_non_negative(joules / JOULES_PER_MEV)
}

/// Converts energy in mega-electron-volts to joules.
#[must_use]
pub fn mev_to_joules(mev: f64) -> Option<f64> {
    if !is_non_negative_finite(mev) {
        return None;
    }

    finite_non_negative(mev * JOULES_PER_MEV)
}

/// Computes binding energy in mega-electron-volts from a mass defect in atomic mass units.
///
/// Formula: `E(MeV) = delta_m(u) * 931.49410242`.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::{binding_energy_mev_from_mass_defect_u, ATOMIC_MASS_UNIT_MEV_C2};
///
/// # fn main() -> Result<(), &'static str> {
/// let binding_energy =
///     binding_energy_mev_from_mass_defect_u(1.0).ok_or("expected valid binding energy")?;
///
/// assert!((binding_energy - ATOMIC_MASS_UNIT_MEV_C2).abs() < 1.0e-12);
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn binding_energy_mev_from_mass_defect_u(mass_defect_u: f64) -> Option<f64> {
    if !is_non_negative_finite(mass_defect_u) {
        return None;
    }

    finite_non_negative(mass_defect_u * ATOMIC_MASS_UNIT_MEV_C2)
}

/// Computes binding energy per nucleon.
///
/// Formula: `E_per_nucleon = E_binding / A`.
#[must_use]
pub fn binding_energy_per_nucleon(binding_energy_mev: f64, nucleon_count: u32) -> Option<f64> {
    if !is_non_negative_finite(binding_energy_mev) || nucleon_count == 0 {
        return None;
    }

    finite_non_negative(binding_energy_mev / f64::from(nucleon_count))
}

/// Computes neutron count from mass number and atomic number.
///
/// Formula: `N = A - Z`.
///
/// # Examples
///
/// ```rust
/// use use_nuclear::neutron_count;
///
/// assert_eq!(neutron_count(4, 2), Some(2));
/// ```
#[must_use]
pub const fn neutron_count(mass_number: u32, atomic_number: u32) -> Option<u32> {
    mass_number.checked_sub(atomic_number)
}

/// Computes nucleon count from proton and neutron counts.
///
/// Formula: `A = Z + N`.
#[must_use]
pub const fn nucleon_count(proton_count: u32, neutron_count: u32) -> Option<u32> {
    proton_count.checked_add(neutron_count)
}

/// Validates mass and atomic numbers for a simple nuclide representation.
#[must_use]
pub const fn is_valid_nuclide_numbers(mass_number: u32, atomic_number: u32) -> bool {
    mass_number != 0 && atomic_number != 0 && atomic_number <= mass_number
}

fn is_non_negative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn finite_non_negative(value: f64) -> Option<f64> {
    if value.is_finite() && value >= 0.0 {
        Some(normalize_zero(value))
    } else {
        None
    }
}

fn finite_unit_interval(value: f64) -> Option<f64> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Some(normalize_zero(value))
    } else {
        None
    }
}

fn normalize_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        ATOMIC_MASS_UNIT_MEV_C2, DecayLaw, JOULES_PER_MEV, LN_2, NuclideNumbers, SPEED_OF_LIGHT,
        activity, binding_energy_mev_from_mass_defect_u, binding_energy_per_nucleon,
        decay_constant_from_half_life, decayed_fraction_from_decay_constant,
        decayed_quantity_from_decay_constant, elapsed_time_from_remaining_fraction,
        energy_from_mass_defect_kg, half_life_from_decay_constant, is_valid_nuclide_numbers,
        joules_to_mev, mass_defect_kg_from_energy, mean_lifetime, mev_to_joules, neutron_count,
        nuclei_from_activity, nucleon_count, remaining_fraction_from_decay_constant,
        remaining_fraction_from_half_life, remaining_quantity_from_decay_constant,
        remaining_quantity_from_half_life, specific_activity,
    };

    fn assert_approx_eq(left: f64, right: f64) {
        let tolerance = 1.0e-12 * left.abs().max(right.abs()).max(1.0);
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    fn assert_some_approx(actual: Option<f64>, expected: f64) {
        let Some(actual) = actual else {
            panic!("expected Some({expected})");
        };

        assert_approx_eq(actual, expected);
    }

    #[test]
    fn decay_constant_and_half_life_helpers_cover_common_cases() {
        assert_some_approx(decay_constant_from_half_life(10.0), LN_2 / 10.0);
        assert_eq!(decay_constant_from_half_life(0.0), None);
        assert_eq!(decay_constant_from_half_life(-1.0), None);

        assert_some_approx(half_life_from_decay_constant(LN_2 / 10.0), 10.0);
        assert_eq!(half_life_from_decay_constant(0.0), None);
    }

    #[test]
    fn mean_lifetime_requires_positive_decay_constant() {
        assert_eq!(mean_lifetime(2.0), Some(0.5));
        assert_eq!(mean_lifetime(0.0), None);
    }

    #[test]
    fn remaining_fraction_and_quantity_helpers_follow_decay_law() {
        assert_eq!(remaining_fraction_from_decay_constant(0.0, 10.0), Some(1.0));
        assert_some_approx(remaining_fraction_from_half_life(10.0, 10.0), 0.5);
        assert_some_approx(remaining_fraction_from_half_life(10.0, 20.0), 0.25);
        assert_eq!(remaining_fraction_from_half_life(0.0, 10.0), None);
        assert_eq!(remaining_fraction_from_half_life(10.0, -1.0), None);

        assert_some_approx(remaining_quantity_from_half_life(100.0, 10.0, 10.0), 50.0);
        assert_some_approx(
            remaining_quantity_from_decay_constant(100.0, LN_2 / 10.0, 10.0),
            50.0,
        );
    }

    #[test]
    fn decayed_fraction_and_elapsed_time_round_trip() {
        assert_some_approx(decayed_fraction_from_decay_constant(LN_2 / 10.0, 10.0), 0.5);
        assert_some_approx(
            decayed_quantity_from_decay_constant(100.0, LN_2 / 10.0, 10.0),
            50.0,
        );

        assert_some_approx(elapsed_time_from_remaining_fraction(LN_2 / 10.0, 0.5), 10.0);
        assert_eq!(elapsed_time_from_remaining_fraction(LN_2 / 10.0, 0.0), None);
        assert_eq!(elapsed_time_from_remaining_fraction(LN_2 / 10.0, 1.5), None);
    }

    #[test]
    fn activity_helpers_validate_inputs() {
        assert_eq!(activity(2.0, 10.0), Some(20.0));
        assert_eq!(activity(-1.0, 10.0), None);
        assert_eq!(activity(1.0, -10.0), None);

        assert_eq!(nuclei_from_activity(20.0, 2.0), Some(10.0));
        assert_eq!(nuclei_from_activity(20.0, 0.0), None);

        assert_eq!(specific_activity(100.0, 5.0), Some(20.0));
        assert_eq!(specific_activity(100.0, 0.0), None);
    }

    #[test]
    fn mass_energy_helpers_convert_between_common_units() {
        assert_some_approx(
            energy_from_mass_defect_kg(1.0),
            SPEED_OF_LIGHT * SPEED_OF_LIGHT,
        );
        assert_eq!(energy_from_mass_defect_kg(-1.0), None);

        assert_some_approx(
            mass_defect_kg_from_energy(SPEED_OF_LIGHT * SPEED_OF_LIGHT),
            1.0,
        );

        assert_some_approx(joules_to_mev(JOULES_PER_MEV), 1.0);
        assert_some_approx(mev_to_joules(1.0), JOULES_PER_MEV);

        assert_some_approx(
            binding_energy_mev_from_mass_defect_u(1.0),
            ATOMIC_MASS_UNIT_MEV_C2,
        );
        assert_eq!(binding_energy_per_nucleon(28.0, 4), Some(7.0));
        assert_eq!(binding_energy_per_nucleon(28.0, 0), None);
    }

    #[test]
    fn nuclear_composition_helpers_validate_counts() {
        assert_eq!(neutron_count(4, 2), Some(2));
        assert_eq!(neutron_count(2, 4), None);

        assert_eq!(nucleon_count(2, 2), Some(4));

        assert!(is_valid_nuclide_numbers(4, 2));
        assert!(!is_valid_nuclide_numbers(0, 0));
        assert!(!is_valid_nuclide_numbers(4, 0));
        assert!(!is_valid_nuclide_numbers(2, 4));
    }

    #[test]
    fn decay_law_methods_delegate_to_public_helpers() {
        assert_some_approx(
            DecayLaw::from_half_life(10.0).and_then(|decay_law| decay_law.remaining_fraction(10.0)),
            0.5,
        );
        assert_some_approx(
            DecayLaw::from_decay_constant(LN_2 / 10.0).and_then(|decay_law| decay_law.half_life()),
            10.0,
        );
        assert_eq!(DecayLaw::from_decay_constant(-1.0), None);
    }

    #[test]
    fn nuclide_numbers_methods_expose_component_counts() {
        let Some(helium) = NuclideNumbers::new(4, 2) else {
            panic!("expected helium nuclide numbers");
        };

        assert_eq!(helium.proton_count(), 2);
        assert_eq!(helium.neutron_count(), 2);
        assert_eq!(helium.nucleon_count(), 4);
        assert_eq!(NuclideNumbers::new(2, 4), None);
    }

    #[test]
    fn non_finite_inputs_return_none() {
        assert_eq!(decay_constant_from_half_life(f64::NAN), None);
        assert_eq!(
            remaining_fraction_from_decay_constant(1.0, f64::INFINITY),
            None
        );
        assert_eq!(activity(f64::INFINITY, 10.0), None);
        assert_eq!(energy_from_mass_defect_kg(f64::NAN), None);
    }
}
