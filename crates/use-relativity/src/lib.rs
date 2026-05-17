#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small special relativity scalar helpers.

/// Re-exports for ergonomic glob imports.
pub mod prelude;

/// Speed of light in vacuum, in meters per second.
///
/// This crate keeps the value locally as a convenience for scalar special relativity helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

const SPEED_OF_LIGHT_SQUARED: f64 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_subluminal_velocity(velocity: f64) -> bool {
    velocity.is_finite() && velocity.abs() < SPEED_OF_LIGHT
}

fn signed_beta(velocity: f64) -> Option<f64> {
    if !is_subluminal_velocity(velocity) {
        return None;
    }

    let beta = velocity / SPEED_OF_LIGHT;
    if beta.abs() >= 1.0 {
        return None;
    }

    finite(beta)
}

fn gamma_from_signed_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || beta.abs() >= 1.0 {
        return None;
    }

    let one_minus_beta_squared = (-beta).mul_add(beta, 1.0);
    if !one_minus_beta_squared.is_finite() || one_minus_beta_squared <= 0.0 {
        return None;
    }

    let gamma = one_minus_beta_squared.sqrt().recip();
    if gamma < 1.0 {
        return None;
    }

    finite(gamma)
}

fn signed_speed_from_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || beta.abs() >= 1.0 {
        return None;
    }

    let velocity = beta * SPEED_OF_LIGHT;
    if velocity.abs() >= SPEED_OF_LIGHT {
        return None;
    }

    finite(velocity)
}

/// Computes the dimensionless speed ratio `β = v / c`.
///
/// The `speed` input is treated as a magnitude in meters per second.
///
/// Returns `None` when `speed` is negative, not finite, greater than or equal to the speed of
/// light, or when the computed ratio is not finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, beta};
///
/// assert_eq!(beta(SPEED_OF_LIGHT * 0.5), Some(0.5));
/// ```
#[must_use]
pub fn beta(speed: f64) -> Option<f64> {
    if !is_subluminal_speed(speed) {
        return None;
    }

    let beta = speed / SPEED_OF_LIGHT;
    if !(0.0..1.0).contains(&beta) {
        return None;
    }

    finite(beta)
}

/// Computes the speed magnitude `v = βc` in meters per second.
///
/// Returns `None` when `beta` is negative, not finite, greater than or equal to `1.0`, or when
/// the computed speed is not finite.
#[must_use]
pub fn speed_from_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || !(0.0..1.0).contains(&beta) {
        return None;
    }

    let speed = beta * SPEED_OF_LIGHT;
    if speed >= SPEED_OF_LIGHT {
        return None;
    }

    finite(speed)
}

/// Returns `true` when `speed` is finite, non-negative, and strictly less than the speed of light.
#[must_use]
pub fn is_subluminal_speed(speed: f64) -> bool {
    is_nonnegative_finite(speed) && speed < SPEED_OF_LIGHT
}

/// Computes the Lorentz factor `γ = 1 / sqrt(1 - β²)` from a non-negative `beta` magnitude.
///
/// Returns `None` when `beta` is negative, not finite, greater than or equal to `1.0`, or when
/// the computed factor is not finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::lorentz_factor_from_beta;
///
/// assert_eq!(lorentz_factor_from_beta(0.0), Some(1.0));
/// ```
#[must_use]
pub fn lorentz_factor_from_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || !(0.0..1.0).contains(&beta) {
        return None;
    }

    gamma_from_signed_beta(beta)
}

/// Computes the Lorentz factor `γ` from a speed magnitude in meters per second.
///
/// This helper delegates to [`beta`] and then to [`lorentz_factor_from_beta`].
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, lorentz_factor};
///
/// assert!((lorentz_factor(SPEED_OF_LIGHT * 0.6).unwrap() - 1.25).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn lorentz_factor(speed: f64) -> Option<f64> {
    beta(speed).and_then(lorentz_factor_from_beta)
}

/// Computes dilated coordinate time `t = γτ` from proper time `τ`.
///
/// Returns `None` when `proper_time` is negative or not finite, when `speed` is invalid, or when
/// the computed time is not finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, dilated_time};
///
/// assert!((dilated_time(10.0, SPEED_OF_LIGHT * 0.6).unwrap() - 12.5).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn dilated_time(proper_time: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(proper_time) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    finite(gamma * proper_time)
}

/// Computes proper time `τ = t / γ` from dilated coordinate time `t`.
///
/// Returns `None` when `dilated_time` is negative or not finite, when `speed` is invalid, or when
/// the computed proper time is not finite.
#[must_use]
pub fn proper_time(dilated_time: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(dilated_time) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    finite(dilated_time / gamma)
}

/// Computes contracted length `L = L0 / γ` from proper length `L0`.
///
/// Returns `None` when `proper_length` is negative or not finite, when `speed` is invalid, or
/// when the computed contracted length is not finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, contracted_length};
///
/// assert!((contracted_length(10.0, SPEED_OF_LIGHT * 0.6).unwrap() - 8.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn contracted_length(proper_length: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(proper_length) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    finite(proper_length / gamma)
}

/// Computes proper length `L0 = Lγ` from a contracted length `L`.
///
/// Returns `None` when `contracted_length` is negative or not finite, when `speed` is invalid,
/// or when the computed proper length is not finite.
#[must_use]
pub fn proper_length(contracted_length: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(contracted_length) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    finite(contracted_length * gamma)
}

/// Computes rest energy `E0 = mc²` in joules.
///
/// Returns `None` when `mass` is negative or not finite, or when the computed energy is not
/// finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, rest_energy};
///
/// assert!((rest_energy(1.0).unwrap() - (SPEED_OF_LIGHT * SPEED_OF_LIGHT)).abs() < 1.0e-3);
/// ```
#[must_use]
pub fn rest_energy(mass: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) {
        return None;
    }

    finite(mass * SPEED_OF_LIGHT_SQUARED)
}

/// Computes rest mass `m = E0 / c²` from rest energy in joules.
///
/// Returns `None` when `rest_energy` is negative or not finite, or when the computed mass is not
/// finite.
#[must_use]
pub fn mass_from_rest_energy(rest_energy: f64) -> Option<f64> {
    if !is_nonnegative_finite(rest_energy) {
        return None;
    }

    finite(rest_energy / SPEED_OF_LIGHT_SQUARED)
}

/// Computes total relativistic energy `E = γmc²` in joules.
///
/// Returns `None` when `mass` is negative or not finite, when `speed` is invalid, or when the
/// computed energy is not finite.
#[must_use]
pub fn total_energy(mass: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    finite(gamma * mass * SPEED_OF_LIGHT_SQUARED)
}

/// Computes relativistic kinetic energy `KE = (γ - 1)mc²` in joules.
///
/// Returns `None` when `mass` is negative or not finite, when `speed` is invalid, or when the
/// computed energy is negative or not finite.
#[must_use]
pub fn relativistic_kinetic_energy(mass: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) {
        return None;
    }

    let gamma = lorentz_factor(speed)?;
    let kinetic_energy = (gamma - 1.0) * mass * SPEED_OF_LIGHT_SQUARED;
    if kinetic_energy < 0.0 {
        return None;
    }

    finite(kinetic_energy)
}

/// Computes relativistic momentum `p = γmv`.
///
/// Returns `None` when `mass` is negative or not finite, when `velocity` is not finite or has a
/// magnitude greater than or equal to the speed of light, or when the computed momentum is not
/// finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, relativistic_momentum};
///
/// let expected = 1.25 * SPEED_OF_LIGHT * 0.6;
///
/// assert!((relativistic_momentum(1.0, SPEED_OF_LIGHT * 0.6).unwrap() - expected).abs() < 1.0e-3);
/// ```
#[must_use]
pub fn relativistic_momentum(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !is_subluminal_velocity(velocity) {
        return None;
    }

    let gamma = gamma_from_signed_beta(signed_beta(velocity)?)?;
    finite(gamma * mass * velocity)
}

/// Computes rest mass from relativistic momentum and velocity using `m = p / (γv)`.
///
/// Returns `None` when `velocity` is zero, when `velocity` is not finite or has a magnitude
/// greater than or equal to the speed of light, when `momentum` is not finite, or when the
/// computed rest mass is negative or not finite.
#[must_use]
pub fn rest_mass_from_momentum_speed(momentum: f64, velocity: f64) -> Option<f64> {
    if !momentum.is_finite() || !is_subluminal_velocity(velocity) || velocity == 0.0 {
        return None;
    }

    let gamma = gamma_from_signed_beta(signed_beta(velocity)?)?;
    let mass = momentum / (gamma * velocity);
    if mass < 0.0 {
        return None;
    }

    finite(mass)
}

/// Computes total energy from rest mass and momentum using `E = sqrt((pc)² + (mc²)²)`.
///
/// Returns `None` when `rest_mass` is negative or not finite, when `momentum` is not finite, or
/// when the computed energy is not finite.
#[must_use]
pub fn energy_momentum_relation(rest_mass: f64, momentum: f64) -> Option<f64> {
    if !is_nonnegative_finite(rest_mass) || !momentum.is_finite() {
        return None;
    }

    let momentum_term = momentum * SPEED_OF_LIGHT;
    let rest_energy = rest_mass * SPEED_OF_LIGHT_SQUARED;
    let energy_squared = momentum_term.mul_add(momentum_term, rest_energy * rest_energy);
    if !energy_squared.is_finite() || energy_squared < 0.0 {
        return None;
    }

    finite(energy_squared.sqrt())
}

/// Computes rapidity `φ = atanh(β)` from a signed `beta`.
///
/// Returns `None` when `beta` is not finite, has an absolute value greater than or equal to
/// `1.0`, or when the computed rapidity is not finite.
#[must_use]
pub fn rapidity_from_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || beta.abs() >= 1.0 {
        return None;
    }

    finite(beta.atanh())
}

/// Computes `β = tanh(φ)` from rapidity.
///
/// Returns `None` when `rapidity` is not finite or when the computed beta is not finite.
#[must_use]
pub fn beta_from_rapidity(rapidity: f64) -> Option<f64> {
    if !rapidity.is_finite() {
        return None;
    }

    let beta = rapidity.tanh();
    if beta.abs() >= 1.0 {
        return None;
    }

    finite(beta)
}

/// Computes signed velocity `v = c * tanh(φ)` from rapidity.
///
/// Returns `None` when `rapidity` is not finite or when the computed velocity is not finite.
#[must_use]
pub fn speed_from_rapidity(rapidity: f64) -> Option<f64> {
    beta_from_rapidity(rapidity).and_then(signed_speed_from_beta)
}

/// Computes relativistic velocity addition `u = (v + w) / (1 + vw / c²)`.
///
/// Returns `None` when either velocity is not finite or has a magnitude greater than or equal to
/// the speed of light, when the denominator is zero, when the computed result is not finite, or
/// when the result has a magnitude greater than or equal to the speed of light.
///
/// # Examples
///
/// ```rust
/// use use_relativity::{SPEED_OF_LIGHT, velocity_addition};
///
/// let expected = SPEED_OF_LIGHT * 0.8;
///
/// assert!((velocity_addition(SPEED_OF_LIGHT * 0.5, SPEED_OF_LIGHT * 0.5).unwrap() - expected).abs() < 1.0e-3);
/// ```
#[must_use]
pub fn velocity_addition(velocity_a: f64, velocity_b: f64) -> Option<f64> {
    if !is_subluminal_velocity(velocity_a) || !is_subluminal_velocity(velocity_b) {
        return None;
    }

    let denominator = 1.0 + ((velocity_a * velocity_b) / SPEED_OF_LIGHT_SQUARED);
    if !denominator.is_finite() || denominator == 0.0 {
        return None;
    }

    let velocity = (velocity_a + velocity_b) / denominator;
    if velocity.abs() >= SPEED_OF_LIGHT {
        return None;
    }

    finite(velocity)
}

/// Computes the longitudinal relativistic Doppler factor `D = sqrt((1 + β) / (1 - β))`.
///
/// Positive `beta` means the source is approaching the observer. Negative `beta` means the source
/// is receding.
///
/// Returns `None` when `beta` is not finite, when `beta <= -1.0` or `beta >= 1.0`, or when the
/// computed factor is not finite.
#[must_use]
pub fn doppler_factor_longitudinal_from_beta(beta: f64) -> Option<f64> {
    if !beta.is_finite() || beta <= -1.0 || beta >= 1.0 {
        return None;
    }

    let numerator = 1.0 + beta;
    let denominator = 1.0 - beta;
    if numerator <= 0.0 || denominator <= 0.0 {
        return None;
    }

    finite((numerator / denominator).sqrt())
}

/// Computes observed longitudinal Doppler-shifted frequency `f_observed = f_emitted * D`.
///
/// Positive `beta` means the source is approaching the observer.
///
/// Returns `None` when `emitted_frequency` is negative or not finite, when `beta` is invalid, or
/// when the computed frequency is not finite.
///
/// # Examples
///
/// ```rust
/// use use_relativity::observed_frequency_longitudinal;
///
/// assert!((observed_frequency_longitudinal(100.0, 0.6).unwrap() - 200.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn observed_frequency_longitudinal(emitted_frequency: f64, beta: f64) -> Option<f64> {
    if !is_nonnegative_finite(emitted_frequency) {
        return None;
    }

    let doppler_factor = doppler_factor_longitudinal_from_beta(beta)?;
    finite(emitted_frequency * doppler_factor)
}

/// A body with scalar rest mass and signed velocity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelativisticBody {
    /// Rest mass in kilograms.
    pub rest_mass: f64,
    /// Signed velocity in meters per second.
    pub velocity: f64,
}

impl RelativisticBody {
    /// Creates a relativistic body when `rest_mass` is finite and non-negative and `velocity` is
    /// finite with a magnitude strictly less than the speed of light.
    #[must_use]
    pub fn new(rest_mass: f64, velocity: f64) -> Option<Self> {
        if !is_nonnegative_finite(rest_mass) || !is_subluminal_velocity(velocity) {
            return None;
        }

        Some(Self {
            rest_mass,
            velocity,
        })
    }

    /// Computes the speed ratio magnitude `β` for the body's current velocity.
    #[must_use]
    pub fn beta(&self) -> Option<f64> {
        beta(self.velocity.abs())
    }

    /// Computes the Lorentz factor `γ` for the body's current speed magnitude.
    #[must_use]
    pub fn lorentz_factor(&self) -> Option<f64> {
        lorentz_factor(self.velocity.abs())
    }

    /// Computes the body's rest energy in joules.
    #[must_use]
    pub fn rest_energy(&self) -> Option<f64> {
        rest_energy(self.rest_mass)
    }

    /// Computes the body's total relativistic energy in joules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_relativity::{RelativisticBody, SPEED_OF_LIGHT};
    ///
    /// let body = RelativisticBody::new(1.0, SPEED_OF_LIGHT * 0.6).unwrap();
    /// let expected = 1.25 * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    ///
    /// assert!((body.total_energy().unwrap() - expected).abs() < 1.0e-3);
    /// ```
    #[must_use]
    pub fn total_energy(&self) -> Option<f64> {
        total_energy(self.rest_mass, self.velocity.abs())
    }

    /// Computes the body's relativistic kinetic energy in joules.
    #[must_use]
    pub fn kinetic_energy(&self) -> Option<f64> {
        relativistic_kinetic_energy(self.rest_mass, self.velocity.abs())
    }

    /// Computes the body's relativistic momentum in kilogram meters per second.
    #[must_use]
    pub fn momentum(&self) -> Option<f64> {
        relativistic_momentum(self.rest_mass, self.velocity)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    const EPSILON: f64 = 1.0e-12;

    fn approx_eq(actual: f64, expected: f64) -> bool {
        let scale = expected.abs().max(1.0);
        (actual - expected).abs() <= EPSILON * scale
    }

    fn assert_option_approx_eq(actual: Option<f64>, expected: f64) {
        let value = actual.expect("expected Some(value)");
        assert!(
            approx_eq(value, expected),
            "expected {expected}, got {value}"
        );
    }

    #[test]
    fn beta_helpers_validate_speed_ranges() {
        assert_option_approx_eq(beta(SPEED_OF_LIGHT * 0.5), 0.5);
        assert_eq!(beta(0.0), Some(0.0));
        assert_eq!(beta(-1.0), None);
        assert_eq!(beta(SPEED_OF_LIGHT), None);

        assert_option_approx_eq(speed_from_beta(0.5), SPEED_OF_LIGHT * 0.5);
        assert_eq!(speed_from_beta(1.0), None);
        assert_eq!(speed_from_beta(-0.1), None);

        assert!(is_subluminal_speed(0.0));
        assert!(is_subluminal_speed(SPEED_OF_LIGHT * 0.5));
        assert!(!is_subluminal_speed(SPEED_OF_LIGHT));
        assert!(!is_subluminal_speed(f64::NAN));
    }

    #[test]
    fn lorentz_helpers_compute_expected_gamma() {
        assert_eq!(lorentz_factor_from_beta(0.0), Some(1.0));
        assert_option_approx_eq(lorentz_factor_from_beta(0.6), 1.25);
        assert_eq!(lorentz_factor_from_beta(1.0), None);

        assert_option_approx_eq(lorentz_factor(SPEED_OF_LIGHT * 0.6), 1.25);
    }

    #[test]
    fn time_dilation_helpers_compute_expected_values() {
        assert_option_approx_eq(dilated_time(10.0, SPEED_OF_LIGHT * 0.6), 12.5);
        assert_eq!(dilated_time(-10.0, SPEED_OF_LIGHT * 0.6), None);

        assert_option_approx_eq(proper_time(12.5, SPEED_OF_LIGHT * 0.6), 10.0);
        assert_eq!(proper_time(-12.5, SPEED_OF_LIGHT * 0.6), None);
    }

    #[test]
    fn length_helpers_compute_expected_values() {
        assert_option_approx_eq(contracted_length(10.0, SPEED_OF_LIGHT * 0.6), 8.0);
        assert_eq!(contracted_length(-10.0, SPEED_OF_LIGHT * 0.6), None);

        assert_option_approx_eq(proper_length(8.0, SPEED_OF_LIGHT * 0.6), 10.0);
    }

    #[test]
    fn mass_energy_helpers_compute_expected_values() {
        assert_option_approx_eq(rest_energy(1.0), SPEED_OF_LIGHT_SQUARED);
        assert_eq!(rest_energy(-1.0), None);

        assert_option_approx_eq(mass_from_rest_energy(SPEED_OF_LIGHT_SQUARED), 1.0);
        assert_option_approx_eq(
            total_energy(1.0, SPEED_OF_LIGHT * 0.6),
            1.25 * SPEED_OF_LIGHT_SQUARED,
        );
        assert_option_approx_eq(
            relativistic_kinetic_energy(1.0, SPEED_OF_LIGHT * 0.6),
            0.25 * SPEED_OF_LIGHT_SQUARED,
        );
    }

    #[test]
    fn momentum_helpers_compute_expected_values() {
        let expected_momentum = 1.25 * SPEED_OF_LIGHT * 0.6;

        assert_option_approx_eq(
            relativistic_momentum(1.0, SPEED_OF_LIGHT * 0.6),
            expected_momentum,
        );
        assert_option_approx_eq(
            relativistic_momentum(1.0, -SPEED_OF_LIGHT * 0.6),
            -expected_momentum,
        );
        assert_eq!(relativistic_momentum(-1.0, SPEED_OF_LIGHT * 0.6), None);

        assert_option_approx_eq(
            rest_mass_from_momentum_speed(expected_momentum, SPEED_OF_LIGHT * 0.6),
            1.0,
        );
        assert_eq!(rest_mass_from_momentum_speed(1.0, 0.0), None);

        assert_option_approx_eq(energy_momentum_relation(1.0, 0.0), SPEED_OF_LIGHT_SQUARED);
    }

    #[test]
    fn rapidity_helpers_compute_expected_values() {
        assert_eq!(rapidity_from_beta(0.0), Some(0.0));
        assert_eq!(beta_from_rapidity(0.0), Some(0.0));
        assert_eq!(speed_from_rapidity(0.0), Some(0.0));
    }

    #[test]
    fn velocity_addition_stays_subluminal() {
        assert_option_approx_eq(
            velocity_addition(SPEED_OF_LIGHT * 0.5, SPEED_OF_LIGHT * 0.5),
            SPEED_OF_LIGHT * 0.8,
        );
        assert_eq!(velocity_addition(SPEED_OF_LIGHT, 1.0), None);
    }

    #[test]
    fn doppler_helpers_compute_expected_values() {
        assert_eq!(doppler_factor_longitudinal_from_beta(0.0), Some(1.0));
        assert_option_approx_eq(doppler_factor_longitudinal_from_beta(0.6), 2.0);
        assert_eq!(doppler_factor_longitudinal_from_beta(1.0), None);

        assert_option_approx_eq(observed_frequency_longitudinal(100.0, 0.6), 200.0);
        assert_eq!(observed_frequency_longitudinal(-100.0, 0.6), None);
    }

    #[test]
    fn relativistic_body_validates_and_delegates() {
        let body = RelativisticBody::new(1.0, SPEED_OF_LIGHT * 0.6).expect("expected valid body");

        assert_option_approx_eq(body.lorentz_factor(), 1.25);
        assert_option_approx_eq(body.momentum(), 1.25 * SPEED_OF_LIGHT * 0.6);
        assert_eq!(RelativisticBody::new(-1.0, SPEED_OF_LIGHT * 0.6), None);
        assert_eq!(RelativisticBody::new(1.0, SPEED_OF_LIGHT), None);
    }
}
