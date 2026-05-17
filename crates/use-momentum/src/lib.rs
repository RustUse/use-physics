#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Linear momentum, impulse, and one-dimensional collision helpers.

pub mod prelude;

/// A moving mass with scalar velocity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingMass {
    pub mass: f64,
    pub velocity: f64,
}

impl MovingMass {
    /// Creates a moving mass when `mass` is non-negative and both values are finite.
    #[must_use]
    pub fn new(mass: f64, velocity: f64) -> Option<Self> {
        if !is_nonnegative_finite(mass) || !velocity.is_finite() {
            return None;
        }

        Some(Self { mass, velocity })
    }

    /// Computes linear momentum using `p = m * v`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_momentum::MovingMass;
    ///
    /// let moving_mass = MovingMass::new(2.0, 3.0).unwrap();
    ///
    /// assert_eq!(moving_mass.momentum(), Some(6.0));
    /// ```
    #[must_use]
    pub fn momentum(&self) -> Option<f64> {
        momentum(self.mass, self.velocity)
    }

    /// Computes kinetic energy using `0.5 * m * v^2`.
    #[must_use]
    pub fn kinetic_energy(&self) -> Option<f64> {
        finite_result(0.5 * self.mass * self.velocity * self.velocity)
    }
}

/// Computes linear momentum using `p = m * v`.
///
/// Returns `None` when `mass` is negative, when either input is not finite, or when the
/// computed momentum is not finite.
///
/// # Examples
///
/// ```rust
/// use use_momentum::momentum;
///
/// assert_eq!(momentum(2.0, 3.0), Some(6.0));
/// assert_eq!(momentum(2.0, -3.0), Some(-6.0));
/// ```
#[must_use]
pub fn momentum(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !velocity.is_finite() {
        return None;
    }

    finite_result(mass * velocity)
}

/// Computes velocity from momentum and mass using `v = p / m`.
///
/// Returns `None` when `mass` is less than or equal to zero, when either input is not finite,
/// or when the computed velocity is not finite.
#[must_use]
pub fn velocity_from_momentum(momentum: f64, mass: f64) -> Option<f64> {
    if !momentum.is_finite() || !is_positive_finite(mass) {
        return None;
    }

    finite_result(momentum / mass)
}

/// Computes mass from momentum and velocity using `m = p / v`.
///
/// Returns `None` when `velocity` is zero, when either input is not finite, when the computed
/// mass is negative, or when the computed mass is not finite.
#[must_use]
pub fn mass_from_momentum(momentum: f64, velocity: f64) -> Option<f64> {
    if !momentum.is_finite() || !velocity.is_finite() || velocity == 0.0 {
        return None;
    }

    let mass = momentum / velocity;
    if mass < 0.0 {
        return None;
    }

    finite_result(mass)
}

/// Computes impulse from force and elapsed time using `J = F * Δt`.
///
/// Returns `None` when `time` is negative, when either input is not finite, or when the
/// computed impulse is not finite.
///
/// # Examples
///
/// ```rust
/// use use_momentum::impulse;
///
/// assert_eq!(impulse(10.0, 2.0), Some(20.0));
/// assert_eq!(impulse(-10.0, 2.0), Some(-20.0));
/// ```
#[must_use]
pub fn impulse(force: f64, time: f64) -> Option<f64> {
    if !force.is_finite() || !time.is_finite() || time < 0.0 {
        return None;
    }

    finite_result(force * time)
}

/// Computes impulse from a change in momentum using `J = p_final - p_initial`.
///
/// Returns `None` when either input is not finite or when the computed impulse is not finite.
#[must_use]
pub fn impulse_from_momentum_change(initial_momentum: f64, final_momentum: f64) -> Option<f64> {
    if !initial_momentum.is_finite() || !final_momentum.is_finite() {
        return None;
    }

    finite_result(final_momentum - initial_momentum)
}

/// Computes average force from impulse and elapsed time using `F = J / Δt`.
///
/// Returns `None` when `time` is less than or equal to zero, when either input is not finite,
/// or when the computed force is not finite.
#[must_use]
pub fn average_force_from_impulse(impulse: f64, time: f64) -> Option<f64> {
    if !impulse.is_finite() || !is_positive_finite(time) {
        return None;
    }

    finite_result(impulse / time)
}

/// Computes the total momentum of a slice of momentum values.
///
/// Returns `Some(0.0)` for an empty slice. Returns `None` when any momentum value is not finite
/// or when the sum is not finite.
#[must_use]
pub fn total_momentum(momenta: &[f64]) -> Option<f64> {
    momenta.iter().try_fold(0.0, |sum, momentum| {
        if !momentum.is_finite() {
            return None;
        }

        finite_result(sum + *momentum)
    })
}

/// Computes the total momentum of two moving bodies using `p_total = m1v1 + m2v2`.
///
/// Returns `None` when either mass is negative, when any input is not finite, or when the total
/// momentum is not finite.
#[must_use]
pub fn two_body_total_momentum(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    let momentum_a = momentum(mass_a, velocity_a)?;
    let momentum_b = momentum(mass_b, velocity_b)?;

    finite_result(momentum_a + momentum_b)
}

/// Computes the shared final velocity for a perfectly inelastic one-dimensional collision.
///
/// Uses `v_final = (m1v1 + m2v2) / (m1 + m2)`.
///
/// Returns `None` when either mass is negative, when the total mass is less than or equal to
/// zero, when any input is not finite, or when the computed velocity is not finite.
///
/// # Examples
///
/// ```rust
/// use use_momentum::final_velocity_after_sticking_collision;
///
/// let final_velocity =
///     final_velocity_after_sticking_collision(2.0, 3.0, 4.0, -1.0).unwrap();
///
/// assert!((final_velocity - 0.333_333_333_333_333_3).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn final_velocity_after_sticking_collision(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    if !velocity_a.is_finite() || !velocity_b.is_finite() {
        return None;
    }

    let total_mass = combined_mass(mass_a, mass_b)?;
    let total_momentum = two_body_total_momentum(mass_a, velocity_a, mass_b, velocity_b)?;

    finite_result(total_momentum / total_mass)
}

/// Computes the final velocity of body A after a one-dimensional elastic collision.
///
/// Returns `None` when either mass is negative, when the total mass is less than or equal to
/// zero, when any input is not finite, or when the computed velocity is not finite.
#[must_use]
pub fn elastic_collision_velocity_a(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    elastic_collision_velocities(mass_a, velocity_a, mass_b, velocity_b).map(|(final_a, _)| final_a)
}

/// Computes the final velocity of body B after a one-dimensional elastic collision.
///
/// Returns `None` when either mass is negative, when the total mass is less than or equal to
/// zero, when any input is not finite, or when the computed velocity is not finite.
#[must_use]
pub fn elastic_collision_velocity_b(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    elastic_collision_velocities(mass_a, velocity_a, mass_b, velocity_b).map(|(_, final_b)| final_b)
}

/// Computes the final velocities of both bodies after a one-dimensional elastic collision.
///
/// Returns `None` when either mass is negative, when the total mass is less than or equal to
/// zero, when any input is not finite, or when either computed velocity is not finite.
///
/// # Examples
///
/// ```rust
/// use use_momentum::elastic_collision_velocities;
///
/// let (final_a, final_b) = elastic_collision_velocities(1.0, 1.0, 1.0, -1.0).unwrap();
///
/// assert!((final_a + 1.0).abs() < 1.0e-12);
/// assert!((final_b - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn elastic_collision_velocities(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<(f64, f64)> {
    if !velocity_a.is_finite() || !velocity_b.is_finite() {
        return None;
    }

    let total_mass = combined_mass(mass_a, mass_b)?;
    let coefficient_a = (mass_a - mass_b) / total_mass;
    let coupling_a = (2.0 * mass_b) / total_mass;
    let coefficient_b = (2.0 * mass_a) / total_mass;
    let coupling_b = (mass_b - mass_a) / total_mass;
    let final_a = coefficient_a.mul_add(velocity_a, coupling_a * velocity_b);
    let final_b = coefficient_b.mul_add(velocity_a, coupling_b * velocity_b);

    if !final_a.is_finite() || !final_b.is_finite() {
        return None;
    }

    Some((final_a, final_b))
}

/// Computes recoil velocity assuming the initial total momentum is zero.
///
/// Uses `v_recoil = -(projectile_mass * projectile_velocity) / body_mass`.
///
/// Returns `None` when `projectile_mass` is negative, when `body_mass` is less than or equal to
/// zero, when any input is not finite, or when the computed recoil velocity is not finite.
///
/// # Examples
///
/// ```rust
/// use use_momentum::recoil_velocity;
///
/// assert_eq!(recoil_velocity(1.0, 10.0, 5.0), Some(-2.0));
/// ```
#[must_use]
pub fn recoil_velocity(
    projectile_mass: f64,
    projectile_velocity: f64,
    body_mass: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(projectile_mass)
        || !projectile_velocity.is_finite()
        || !is_positive_finite(body_mass)
    {
        return None;
    }

    let projectile_momentum = momentum(projectile_mass, projectile_velocity)?;
    finite_result(-(projectile_momentum / body_mass))
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn combined_mass(mass_a: f64, mass_b: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass_a) || !is_nonnegative_finite(mass_b) {
        return None;
    }

    let total_mass = mass_a + mass_b;
    is_positive_finite(total_mass).then_some(total_mass)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        MovingMass, average_force_from_impulse, elastic_collision_velocities,
        final_velocity_after_sticking_collision, impulse, impulse_from_momentum_change,
        mass_from_momentum, momentum, recoil_velocity, total_momentum, two_body_total_momentum,
        velocity_from_momentum,
    };

    const EPSILON: f64 = 1.0e-10;

    fn assert_approx_eq(left: f64, right: f64) {
        assert!(
            (left - right).abs() < EPSILON,
            "left={left}, right={right}, diff={}",
            (left - right).abs()
        );
    }

    #[test]
    fn momentum_helpers_cover_common_cases() {
        assert_eq!(momentum(2.0, 3.0), Some(6.0));
        assert_eq!(momentum(2.0, -3.0), Some(-6.0));
        assert_eq!(momentum(-1.0, 3.0), None);

        assert_eq!(velocity_from_momentum(10.0, 2.0), Some(5.0));
        assert_eq!(velocity_from_momentum(10.0, 0.0), None);

        assert_eq!(mass_from_momentum(10.0, 2.0), Some(5.0));
        assert_eq!(mass_from_momentum(10.0, 0.0), None);
        assert_eq!(mass_from_momentum(-10.0, 2.0), None);
    }

    #[test]
    fn impulse_helpers_cover_common_cases() {
        assert_eq!(impulse(10.0, 2.0), Some(20.0));
        assert_eq!(impulse(-10.0, 2.0), Some(-20.0));
        assert_eq!(impulse(10.0, -1.0), None);

        assert_eq!(impulse_from_momentum_change(5.0, 12.0), Some(7.0));
        assert_eq!(average_force_from_impulse(20.0, 4.0), Some(5.0));
        assert_eq!(average_force_from_impulse(20.0, 0.0), None);
    }

    #[test]
    fn conservation_helpers_cover_common_cases() {
        assert_eq!(total_momentum(&[1.0, 2.0, 3.0]), Some(6.0));
        assert_eq!(total_momentum(&[]), Some(0.0));
        assert_eq!(two_body_total_momentum(2.0, 3.0, 4.0, -1.0), Some(2.0));
    }

    #[test]
    fn collision_helpers_cover_common_cases() {
        let final_velocity = final_velocity_after_sticking_collision(2.0, 3.0, 4.0, -1.0).unwrap();
        assert_approx_eq(final_velocity, 0.333_333_333_333_333_3);

        let (final_a, final_b) = elastic_collision_velocities(1.0, 1.0, 1.0, -1.0).unwrap();
        assert_approx_eq(final_a, -1.0);
        assert_approx_eq(final_b, 1.0);
    }

    #[test]
    fn recoil_and_moving_mass_cover_common_cases() {
        assert_eq!(recoil_velocity(1.0, 10.0, 5.0), Some(-2.0));
        assert_eq!(MovingMass::new(2.0, 3.0).unwrap().momentum(), Some(6.0));
        assert_eq!(MovingMass::new(-1.0, 3.0), None);
    }

    #[test]
    fn non_finite_inputs_are_rejected() {
        assert_eq!(momentum(f64::INFINITY, 1.0), None);
        assert_eq!(velocity_from_momentum(1.0, f64::NAN), None);
        assert_eq!(impulse(f64::NAN, 1.0), None);
        assert_eq!(total_momentum(&[1.0, f64::INFINITY]), None);
        assert_eq!(recoil_velocity(1.0, 10.0, f64::INFINITY), None);
    }

    #[test]
    fn moving_mass_computes_kinetic_energy() {
        assert_eq!(
            MovingMass::new(2.0, 3.0).unwrap().kinetic_energy(),
            Some(9.0)
        );
    }
}
