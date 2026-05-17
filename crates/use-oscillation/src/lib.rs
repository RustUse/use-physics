#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Oscillation-specific scalar helpers.

use core::f64::consts::TAU;

pub mod prelude;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn harmonic_phase(amplitude: f64, angular_frequency: f64, time: f64, phase: f64) -> Option<f64> {
    if !all_finite(&[amplitude, angular_frequency, time, phase])
        || amplitude < 0.0
        || angular_frequency < 0.0
        || time < 0.0
    {
        return None;
    }

    finite(angular_frequency.mul_add(time, phase))
}

/// Computes period from frequency using `T = 1 / f`.
///
/// Returns `None` when `frequency` is less than or equal to zero, when the input is not finite,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_oscillation::period_from_frequency;
///
/// assert_eq!(period_from_frequency(2.0), Some(0.5));
/// assert_eq!(period_from_frequency(0.0), None);
/// ```
#[must_use]
pub fn period_from_frequency(frequency: f64) -> Option<f64> {
    if !positive_finite(frequency) {
        return None;
    }

    finite(1.0 / frequency)
}

/// Computes frequency from period using `f = 1 / T`.
///
/// Returns `None` when `period` is less than or equal to zero, when the input is not finite, or
/// when the computed result is not finite.
#[must_use]
pub fn frequency_from_period(period: f64) -> Option<f64> {
    if !positive_finite(period) {
        return None;
    }

    finite(1.0 / period)
}

/// Computes angular frequency from frequency using `ω = 2πf`.
#[must_use]
pub fn angular_frequency_from_frequency(frequency: f64) -> Option<f64> {
    if !nonnegative_finite(frequency) {
        return None;
    }

    finite(TAU * frequency)
}

/// Computes frequency from angular frequency using `f = ω / 2π`.
#[must_use]
pub fn frequency_from_angular_frequency(angular_frequency: f64) -> Option<f64> {
    if !nonnegative_finite(angular_frequency) {
        return None;
    }

    finite(angular_frequency / TAU)
}

/// Computes angular frequency from period using `ω = 2π / T`.
///
/// # Examples
///
/// ```rust
/// use core::f64::consts::TAU;
/// use use_oscillation::angular_frequency_from_period;
///
/// let angular_frequency = angular_frequency_from_period(1.0).unwrap();
///
/// assert!((angular_frequency - TAU).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn angular_frequency_from_period(period: f64) -> Option<f64> {
    if !positive_finite(period) {
        return None;
    }

    finite(TAU / period)
}

/// Computes period from angular frequency using `T = 2π / ω`.
#[must_use]
pub fn period_from_angular_frequency(angular_frequency: f64) -> Option<f64> {
    if !positive_finite(angular_frequency) {
        return None;
    }

    finite(TAU / angular_frequency)
}

/// Computes displacement for simple harmonic motion using `x(t) = A * cos(ωt + φ)`.
///
/// # Examples
///
/// ```rust
/// use use_oscillation::displacement;
///
/// let value = displacement(2.0, 1.0, 0.0, 0.0).unwrap();
///
/// assert!((value - 2.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn displacement(amplitude: f64, angular_frequency: f64, time: f64, phase: f64) -> Option<f64> {
    let angle = harmonic_phase(amplitude, angular_frequency, time, phase)?;

    finite(amplitude * angle.cos())
}

/// Computes velocity for simple harmonic motion using `v(t) = -Aω * sin(ωt + φ)`.
#[must_use]
pub fn velocity(amplitude: f64, angular_frequency: f64, time: f64, phase: f64) -> Option<f64> {
    let angle = harmonic_phase(amplitude, angular_frequency, time, phase)?;

    finite(-amplitude * angular_frequency * angle.sin())
}

/// Computes acceleration for simple harmonic motion using `a(t) = -Aω² * cos(ωt + φ)`.
#[must_use]
pub fn acceleration(amplitude: f64, angular_frequency: f64, time: f64, phase: f64) -> Option<f64> {
    let angle = harmonic_phase(amplitude, angular_frequency, time, phase)?;

    finite(-amplitude * angular_frequency.powi(2) * angle.cos())
}

/// Computes simple-harmonic acceleration from displacement using `a = -ω²x`.
#[must_use]
pub fn acceleration_from_displacement(displacement: f64, angular_frequency: f64) -> Option<f64> {
    if !displacement.is_finite() || !nonnegative_finite(angular_frequency) {
        return None;
    }

    finite(-angular_frequency.powi(2) * displacement)
}

/// Computes the maximum speed using `v_max = Aω`.
#[must_use]
pub fn max_speed(amplitude: f64, angular_frequency: f64) -> Option<f64> {
    if !nonnegative_finite(amplitude) || !nonnegative_finite(angular_frequency) {
        return None;
    }

    finite(amplitude * angular_frequency)
}

/// Computes the maximum acceleration using `a_max = Aω²`.
#[must_use]
pub fn max_acceleration(amplitude: f64, angular_frequency: f64) -> Option<f64> {
    if !nonnegative_finite(amplitude) || !nonnegative_finite(angular_frequency) {
        return None;
    }

    finite(amplitude * angular_frequency.powi(2))
}

/// Computes spring angular frequency using `ω = sqrt(k / m)`.
#[must_use]
pub fn spring_angular_frequency(spring_constant: f64, mass: f64) -> Option<f64> {
    if !nonnegative_finite(spring_constant) || !positive_finite(mass) {
        return None;
    }

    finite((spring_constant / mass).sqrt())
}

/// Computes spring period using `T = 2π * sqrt(m / k)`.
///
/// # Examples
///
/// ```rust
/// use core::f64::consts::PI;
/// use use_oscillation::spring_period;
///
/// let period = spring_period(8.0, 2.0).unwrap();
///
/// assert!((period - PI).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn spring_period(spring_constant: f64, mass: f64) -> Option<f64> {
    if !positive_finite(spring_constant) || !positive_finite(mass) {
        return None;
    }

    finite(TAU * (mass / spring_constant).sqrt())
}

/// Computes spring frequency in cycles per second.
#[must_use]
pub fn spring_frequency(spring_constant: f64, mass: f64) -> Option<f64> {
    let period = spring_period(spring_constant, mass)?;

    frequency_from_period(period)
}

/// Computes spring constant from mass and period using `k = 4π²m / T²`.
#[must_use]
pub fn spring_constant_from_period(mass: f64, period: f64) -> Option<f64> {
    if !nonnegative_finite(mass) || !positive_finite(period) {
        return None;
    }

    finite(TAU.powi(2) * mass / period.powi(2))
}

/// Computes mass from spring constant and period using `m = kT² / 4π²`.
#[must_use]
pub fn mass_from_spring_period(spring_constant: f64, period: f64) -> Option<f64> {
    if !nonnegative_finite(spring_constant) || !positive_finite(period) {
        return None;
    }

    finite(spring_constant * period.powi(2) / TAU.powi(2))
}

/// Computes the small-angle simple pendulum period using `T = 2π * sqrt(L / g)`.
///
/// # Examples
///
/// ```rust
/// use core::f64::consts::TAU;
/// use use_oscillation::simple_pendulum_period;
///
/// let period = simple_pendulum_period(9.806_65, 9.806_65).unwrap();
///
/// assert!((period - TAU).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn simple_pendulum_period(length: f64, gravitational_acceleration: f64) -> Option<f64> {
    if !length.is_finite() || length < 0.0 || !positive_finite(gravitational_acceleration) {
        return None;
    }

    finite(TAU * (length / gravitational_acceleration).sqrt())
}

/// Computes small-angle simple pendulum frequency in cycles per second.
#[must_use]
pub fn simple_pendulum_frequency(length: f64, gravitational_acceleration: f64) -> Option<f64> {
    let period = simple_pendulum_period(length, gravitational_acceleration)?;

    frequency_from_period(period)
}

/// Computes small-angle simple pendulum angular frequency using `ω = sqrt(g / L)`.
#[must_use]
pub fn simple_pendulum_angular_frequency(
    length: f64,
    gravitational_acceleration: f64,
) -> Option<f64> {
    if !positive_finite(length) || !positive_finite(gravitational_acceleration) {
        return None;
    }

    finite((gravitational_acceleration / length).sqrt())
}

/// Computes pendulum length from period using `L = g * (T / 2π)²`.
#[must_use]
pub fn pendulum_length_from_period(period: f64, gravitational_acceleration: f64) -> Option<f64> {
    if !positive_finite(period) || !nonnegative_finite(gravitational_acceleration) {
        return None;
    }

    finite(gravitational_acceleration * (period / TAU).powi(2))
}

/// Computes spring potential energy using `U = 0.5 * k * x²`.
#[must_use]
pub fn spring_potential_energy(spring_constant: f64, displacement: f64) -> Option<f64> {
    if !spring_constant.is_finite() || spring_constant < 0.0 || !displacement.is_finite() {
        return None;
    }

    finite(0.5 * spring_constant * displacement.powi(2))
}

/// Computes oscillator total energy using `E = 0.5 * k * A²`.
#[must_use]
pub fn oscillator_total_energy(spring_constant: f64, amplitude: f64) -> Option<f64> {
    if !nonnegative_finite(spring_constant) || !nonnegative_finite(amplitude) {
        return None;
    }

    finite(0.5 * spring_constant * amplitude.powi(2))
}

/// Computes kinetic energy from total energy and potential energy using `KE = E_total - U`.
#[must_use]
pub fn kinetic_energy_from_total_and_potential(
    total_energy: f64,
    potential_energy: f64,
) -> Option<f64> {
    if !nonnegative_finite(total_energy) || !nonnegative_finite(potential_energy) {
        return None;
    }

    let kinetic_energy = finite(total_energy - potential_energy)?;
    if kinetic_energy < 0.0 {
        return None;
    }

    Some(kinetic_energy)
}

/// Computes damping ratio using `ζ = c / (2 * sqrt(mk))`.
///
/// # Examples
///
/// ```rust
/// use use_oscillation::damping_ratio;
///
/// assert_eq!(damping_ratio(4.0, 2.0, 8.0), Some(0.5));
/// ```
#[must_use]
pub fn damping_ratio(damping_coefficient: f64, mass: f64, spring_constant: f64) -> Option<f64> {
    if !nonnegative_finite(damping_coefficient)
        || !positive_finite(mass)
        || !positive_finite(spring_constant)
    {
        return None;
    }

    let denominator = finite(2.0 * (mass * spring_constant).sqrt())?;

    finite(damping_coefficient / denominator)
}

/// Computes the critical damping coefficient using `c_critical = 2 * sqrt(mk)`.
#[must_use]
pub fn critical_damping_coefficient(mass: f64, spring_constant: f64) -> Option<f64> {
    if !nonnegative_finite(mass) || !nonnegative_finite(spring_constant) {
        return None;
    }

    finite(2.0 * (mass * spring_constant).sqrt())
}

/// Computes the damped angular frequency for an underdamped oscillator.
#[must_use]
pub fn damped_angular_frequency(
    undamped_angular_frequency: f64,
    damping_ratio: f64,
) -> Option<f64> {
    if !nonnegative_finite(undamped_angular_frequency)
        || !nonnegative_finite(damping_ratio)
        || damping_ratio >= 1.0
    {
        return None;
    }

    let damping_term = damping_ratio.mul_add(-damping_ratio, 1.0);

    finite(undamped_angular_frequency * damping_term.sqrt())
}

/// Returns `true` when the damping ratio represents an underdamped system.
#[must_use]
pub fn is_underdamped(damping_ratio: f64) -> bool {
    damping_ratio.is_finite() && (0.0..1.0).contains(&damping_ratio)
}

/// Returns `true` when the damping ratio is within `tolerance` of critical damping.
#[must_use]
pub fn is_critically_damped(damping_ratio: f64, tolerance: f64) -> Option<bool> {
    if !damping_ratio.is_finite() || !nonnegative_finite(tolerance) {
        return None;
    }

    Some((damping_ratio - 1.0).abs() <= tolerance)
}

/// Returns `true` when the damping ratio represents an overdamped system.
#[must_use]
pub fn is_overdamped(damping_ratio: f64) -> bool {
    damping_ratio.is_finite() && damping_ratio > 1.0
}

/// Computes quality factor from damping ratio using `Q = 1 / (2ζ)`.
///
/// # Examples
///
/// ```rust
/// use use_oscillation::quality_factor_from_damping_ratio;
///
/// assert_eq!(quality_factor_from_damping_ratio(0.25), Some(2.0));
/// ```
#[must_use]
pub fn quality_factor_from_damping_ratio(damping_ratio: f64) -> Option<f64> {
    if !positive_finite(damping_ratio) {
        return None;
    }

    finite(1.0 / (2.0 * damping_ratio))
}

/// Computes damping ratio from quality factor using `ζ = 1 / (2Q)`.
#[must_use]
pub fn damping_ratio_from_quality_factor(quality_factor: f64) -> Option<f64> {
    if !positive_finite(quality_factor) {
        return None;
    }

    finite(1.0 / (2.0 * quality_factor))
}

/// Computes the natural resonance angular frequency of a spring-mass oscillator.
#[must_use]
pub fn resonance_angular_frequency_natural(spring_constant: f64, mass: f64) -> Option<f64> {
    spring_angular_frequency(spring_constant, mass)
}

/// A simple scalar harmonic oscillator state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleHarmonicOscillator {
    pub amplitude: f64,
    pub angular_frequency: f64,
    pub phase: f64,
}

impl SimpleHarmonicOscillator {
    /// Creates a simple harmonic oscillator from amplitude, angular frequency, and phase.
    #[must_use]
    pub fn new(amplitude: f64, angular_frequency: f64, phase: f64) -> Option<Self> {
        if !nonnegative_finite(amplitude)
            || !nonnegative_finite(angular_frequency)
            || !phase.is_finite()
        {
            return None;
        }

        Some(Self {
            amplitude,
            angular_frequency,
            phase,
        })
    }

    /// Computes displacement at `time`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_oscillation::SimpleHarmonicOscillator;
    ///
    /// let oscillator = SimpleHarmonicOscillator::new(2.0, 1.0, 0.0).unwrap();
    ///
    /// assert!((oscillator.displacement(0.0).unwrap() - 2.0).abs() < 1.0e-12);
    /// ```
    #[must_use]
    pub fn displacement(&self, time: f64) -> Option<f64> {
        displacement(self.amplitude, self.angular_frequency, time, self.phase)
    }

    /// Computes velocity at `time`.
    #[must_use]
    pub fn velocity(&self, time: f64) -> Option<f64> {
        velocity(self.amplitude, self.angular_frequency, time, self.phase)
    }

    /// Computes acceleration at `time`.
    #[must_use]
    pub fn acceleration(&self, time: f64) -> Option<f64> {
        acceleration(self.amplitude, self.angular_frequency, time, self.phase)
    }

    /// Computes the oscillation period.
    #[must_use]
    pub fn period(&self) -> Option<f64> {
        period_from_angular_frequency(self.angular_frequency)
    }

    /// Computes the oscillation frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<f64> {
        frequency_from_angular_frequency(self.angular_frequency)
    }

    /// Computes maximum speed.
    #[must_use]
    pub fn max_speed(&self) -> Option<f64> {
        max_speed(self.amplitude, self.angular_frequency)
    }

    /// Computes maximum acceleration.
    #[must_use]
    pub fn max_acceleration(&self) -> Option<f64> {
        max_acceleration(self.amplitude, self.angular_frequency)
    }
}

/// A spring-mass oscillator state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringOscillator {
    pub spring_constant: f64,
    pub mass: f64,
}

impl SpringOscillator {
    /// Creates a spring oscillator from spring constant and mass.
    #[must_use]
    pub fn new(spring_constant: f64, mass: f64) -> Option<Self> {
        if !nonnegative_finite(spring_constant) || !positive_finite(mass) {
            return None;
        }

        Some(Self {
            spring_constant,
            mass,
        })
    }

    /// Computes angular frequency.
    #[must_use]
    pub fn angular_frequency(&self) -> Option<f64> {
        spring_angular_frequency(self.spring_constant, self.mass)
    }

    /// Computes period.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::f64::consts::PI;
    /// use use_oscillation::SpringOscillator;
    ///
    /// let oscillator = SpringOscillator::new(8.0, 2.0).unwrap();
    ///
    /// assert!((oscillator.period().unwrap() - PI).abs() < 1.0e-12);
    /// ```
    #[must_use]
    pub fn period(&self) -> Option<f64> {
        spring_period(self.spring_constant, self.mass)
    }

    /// Computes frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<f64> {
        spring_frequency(self.spring_constant, self.mass)
    }

    /// Computes total oscillator energy for a given amplitude.
    #[must_use]
    pub fn total_energy(&self, amplitude: f64) -> Option<f64> {
        oscillator_total_energy(self.spring_constant, amplitude)
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::{PI, TAU};

    use super::{
        SimpleHarmonicOscillator, SpringOscillator, acceleration, acceleration_from_displacement,
        angular_frequency_from_frequency, angular_frequency_from_period,
        critical_damping_coefficient, damped_angular_frequency, damping_ratio,
        damping_ratio_from_quality_factor, displacement, frequency_from_angular_frequency,
        frequency_from_period, is_critically_damped, is_overdamped, is_underdamped,
        kinetic_energy_from_total_and_potential, mass_from_spring_period, max_acceleration,
        max_speed, oscillator_total_energy, pendulum_length_from_period,
        period_from_angular_frequency, period_from_frequency, quality_factor_from_damping_ratio,
        resonance_angular_frequency_natural, simple_pendulum_angular_frequency,
        simple_pendulum_frequency, simple_pendulum_period, spring_angular_frequency,
        spring_constant_from_period, spring_frequency, spring_period, spring_potential_energy,
        velocity,
    };

    fn assert_approx_eq(left: f64, right: f64) {
        let delta = (left - right).abs();

        assert!(
            delta <= 1.0e-12,
            "left={left} right={right} delta={delta} tolerance=1e-12"
        );
    }

    #[test]
    fn period_and_frequency_helpers_cover_basic_cases() {
        assert_eq!(period_from_frequency(2.0), Some(0.5));
        assert_eq!(period_from_frequency(0.0), None);

        assert_eq!(frequency_from_period(0.5), Some(2.0));
        assert_eq!(frequency_from_period(0.0), None);

        assert_approx_eq(angular_frequency_from_frequency(1.0).unwrap(), TAU);
        assert_approx_eq(frequency_from_angular_frequency(TAU).unwrap(), 1.0);

        assert_approx_eq(angular_frequency_from_period(1.0).unwrap(), TAU);
        assert_approx_eq(period_from_angular_frequency(TAU).unwrap(), 1.0);
    }

    #[test]
    fn simple_harmonic_motion_helpers_cover_basic_cases() {
        assert_approx_eq(displacement(2.0, 1.0, 0.0, 0.0).unwrap(), 2.0);
        assert_approx_eq(velocity(2.0, 1.0, 0.0, 0.0).unwrap(), 0.0);
        assert_approx_eq(acceleration(2.0, 1.0, 0.0, 0.0).unwrap(), -2.0);

        assert_eq!(acceleration_from_displacement(2.0, 3.0), Some(-18.0));
        assert_eq!(max_speed(2.0, 3.0), Some(6.0));
        assert_eq!(max_acceleration(2.0, 3.0), Some(18.0));
    }

    #[test]
    fn spring_oscillator_helpers_cover_basic_cases() {
        assert_eq!(spring_angular_frequency(8.0, 2.0), Some(2.0));
        assert_eq!(spring_angular_frequency(8.0, 0.0), None);

        assert_approx_eq(spring_period(8.0, 2.0).unwrap(), PI);
        assert_eq!(spring_period(0.0, 2.0), None);

        assert_approx_eq(spring_frequency(8.0, 2.0).unwrap(), 1.0 / PI);
        assert_approx_eq(spring_constant_from_period(2.0, PI).unwrap(), 8.0);
        assert_approx_eq(mass_from_spring_period(8.0, PI).unwrap(), 2.0);
    }

    #[test]
    fn pendulum_helpers_cover_basic_cases() {
        assert_approx_eq(simple_pendulum_period(9.806_65, 9.806_65).unwrap(), TAU);
        assert_eq!(simple_pendulum_period(-1.0, 9.806_65), None);

        assert_approx_eq(
            simple_pendulum_frequency(9.806_65, 9.806_65).unwrap(),
            1.0 / TAU,
        );
        assert_approx_eq(
            simple_pendulum_angular_frequency(9.806_65, 9.806_65).unwrap(),
            1.0,
        );
        assert_approx_eq(
            pendulum_length_from_period(TAU, 9.806_65).unwrap(),
            9.806_65,
        );
    }

    #[test]
    fn energy_helpers_cover_basic_cases() {
        assert_eq!(spring_potential_energy(100.0, 0.5), Some(12.5));
        assert_eq!(spring_potential_energy(-100.0, 0.5), None);

        assert_eq!(oscillator_total_energy(100.0, 0.5), Some(12.5));
        assert_eq!(oscillator_total_energy(100.0, -0.5), None);

        assert_eq!(
            kinetic_energy_from_total_and_potential(12.5, 2.5),
            Some(10.0)
        );
        assert_eq!(kinetic_energy_from_total_and_potential(2.5, 12.5), None);
    }

    #[test]
    fn damping_helpers_cover_basic_cases() {
        assert_eq!(critical_damping_coefficient(2.0, 8.0), Some(8.0));
        assert_eq!(critical_damping_coefficient(-2.0, 8.0), None);

        assert_eq!(damping_ratio(4.0, 2.0, 8.0), Some(0.5));
        assert_eq!(damping_ratio(-4.0, 2.0, 8.0), None);

        assert_approx_eq(damped_angular_frequency(10.0, 0.6).unwrap(), 8.0);
        assert_eq!(damped_angular_frequency(10.0, 1.0), None);

        assert!(is_underdamped(0.5));
        assert!(!is_underdamped(1.0));
        assert!(is_overdamped(2.0));
        assert!(!is_overdamped(1.0));

        assert_eq!(is_critically_damped(1.0, 0.0), Some(true));
        assert_eq!(is_critically_damped(1.01, 0.02), Some(true));
        assert_eq!(is_critically_damped(1.1, 0.02), Some(false));
        assert_eq!(is_critically_damped(1.0, -1.0), None);
    }

    #[test]
    fn resonance_helpers_cover_basic_cases() {
        assert_eq!(quality_factor_from_damping_ratio(0.25), Some(2.0));
        assert_eq!(quality_factor_from_damping_ratio(0.0), None);

        assert_eq!(damping_ratio_from_quality_factor(2.0), Some(0.25));
        assert_eq!(damping_ratio_from_quality_factor(0.0), None);

        assert_eq!(resonance_angular_frequency_natural(8.0, 2.0), Some(2.0));
    }

    #[test]
    fn simple_harmonic_oscillator_type_delegates_to_helpers() {
        let oscillator = SimpleHarmonicOscillator::new(2.0, 1.0, 0.0).unwrap();

        assert_approx_eq(oscillator.displacement(0.0).unwrap(), 2.0);
        assert_approx_eq(oscillator.velocity(0.0).unwrap(), 0.0);
        assert_approx_eq(oscillator.acceleration(0.0).unwrap(), -2.0);
        assert_approx_eq(
            SimpleHarmonicOscillator::new(2.0, TAU, 0.0)
                .unwrap()
                .period()
                .unwrap(),
            1.0,
        );
        assert_eq!(SimpleHarmonicOscillator::new(-2.0, 1.0, 0.0), None);
    }

    #[test]
    fn spring_oscillator_type_delegates_to_helpers() {
        let oscillator = SpringOscillator::new(8.0, 2.0).unwrap();

        assert_eq!(oscillator.angular_frequency(), Some(2.0));
        assert_approx_eq(oscillator.period().unwrap(), PI);
        assert_eq!(oscillator.total_energy(0.5), Some(1.0));
        assert_eq!(SpringOscillator::new(8.0, 0.0), None);
    }

    #[test]
    fn helpers_reject_non_finite_inputs() {
        assert_eq!(period_from_frequency(f64::NAN), None);
        assert_eq!(displacement(1.0, 1.0, f64::INFINITY, 0.0), None);
        assert_eq!(spring_period(8.0, f64::NAN), None);
        assert_eq!(damping_ratio(4.0, 2.0, f64::INFINITY), None);
        assert_eq!(SimpleHarmonicOscillator::new(1.0, 1.0, f64::NAN), None);
        assert_eq!(SpringOscillator::new(f64::INFINITY, 1.0), None);
    }
}
