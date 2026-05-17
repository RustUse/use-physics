#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Scalar helpers for one-dimensional collisions.

pub mod prelude;

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn normalized_nonnegative(value: f64) -> Option<f64> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }

    Some(if value == 0.0 { 0.0 } else { value })
}

fn combined_mass(mass_a: f64, mass_b: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass_a) || !is_nonnegative_finite(mass_b) {
        return None;
    }

    let total_mass = mass_a + mass_b;
    is_positive_finite(total_mass).then_some(total_mass)
}

fn momentum_from_mass_velocity(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !velocity.is_finite() {
        return None;
    }

    finite_result(mass * velocity)
}

fn total_momentum_1d(mass_a: f64, velocity_a: f64, mass_b: f64, velocity_b: f64) -> Option<f64> {
    let momentum_a = momentum_from_mass_velocity(mass_a, velocity_a)?;
    let momentum_b = momentum_from_mass_velocity(mass_b, velocity_b)?;

    finite_result(momentum_a + momentum_b)
}

/// Computes the signed relative velocity between two one-dimensional bodies.
///
/// Formula: `v_rel = v_a - v_b`
#[must_use]
pub fn relative_velocity(velocity_a: f64, velocity_b: f64) -> Option<f64> {
    if !velocity_a.is_finite() || !velocity_b.is_finite() {
        return None;
    }

    finite_result(velocity_a - velocity_b)
}

/// Computes the relative speed between two one-dimensional bodies.
///
/// Formula: `speed_rel = |v_a - v_b|`
#[must_use]
pub fn relative_speed(velocity_a: f64, velocity_b: f64) -> Option<f64> {
    let relative = relative_velocity(velocity_a, velocity_b)?;

    normalized_nonnegative(relative.abs())
}

/// Computes the coefficient of restitution from approach and separation speeds.
///
/// Formula: `e = separation_speed / approach_speed`
///
/// Returns `None` when `approach_speed` is less than or equal to zero, when
/// `separation_speed` is negative, when any input is not finite, when the computed value is not
/// finite, or when the result is greater than `1.0`.
///
/// # Examples
///
/// ```rust
/// use use_collision::coefficient_of_restitution;
///
/// assert_eq!(coefficient_of_restitution(10.0, 8.0), Some(0.8));
/// assert_eq!(coefficient_of_restitution(10.0, 0.0), Some(0.0));
/// ```
#[must_use]
pub fn coefficient_of_restitution(approach_speed: f64, separation_speed: f64) -> Option<f64> {
    if !is_positive_finite(approach_speed) || !is_nonnegative_finite(separation_speed) {
        return None;
    }

    let coefficient = separation_speed / approach_speed;
    if !coefficient.is_finite() || coefficient > 1.0 {
        return None;
    }

    normalized_nonnegative(coefficient)
}

/// Computes separation speed from an approach speed and restitution coefficient.
///
/// Formula: `separation_speed = e * approach_speed`
#[must_use]
pub fn separation_speed_from_restitution(
    approach_speed: f64,
    coefficient_of_restitution: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(approach_speed) || !is_valid_restitution(coefficient_of_restitution) {
        return None;
    }

    normalized_nonnegative(coefficient_of_restitution * approach_speed)
}

/// Returns `true` when a restitution coefficient is finite and within `[0.0, 1.0]`.
#[must_use]
pub fn is_valid_restitution(coefficient_of_restitution: f64) -> bool {
    coefficient_of_restitution.is_finite() && (0.0..=1.0).contains(&coefficient_of_restitution)
}

/// Returns whether a valid restitution coefficient is effectively perfectly elastic.
///
/// This returns `Some(true)` when `abs(e - 1.0) <= tolerance`.
#[must_use]
pub fn is_perfectly_elastic(coefficient_of_restitution: f64, tolerance: f64) -> Option<bool> {
    if !is_valid_restitution(coefficient_of_restitution) || !is_nonnegative_finite(tolerance) {
        return None;
    }

    Some((coefficient_of_restitution - 1.0).abs() <= tolerance)
}

/// Returns whether a valid restitution coefficient is effectively perfectly inelastic.
///
/// This returns `Some(true)` when `abs(e) <= tolerance`.
#[must_use]
pub fn is_perfectly_inelastic(coefficient_of_restitution: f64, tolerance: f64) -> Option<bool> {
    if !is_valid_restitution(coefficient_of_restitution) || !is_nonnegative_finite(tolerance) {
        return None;
    }

    Some(coefficient_of_restitution.abs() <= tolerance)
}

/// Computes kinetic energy from mass and one-dimensional velocity.
///
/// Formula: `KE = 0.5 * m * v²`
#[must_use]
pub fn kinetic_energy(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !velocity.is_finite() {
        return None;
    }

    normalized_nonnegative(0.5 * mass * velocity * velocity)
}

/// Computes the total kinetic energy of two one-dimensional bodies.
#[must_use]
pub fn total_kinetic_energy_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    let energy_a = kinetic_energy(mass_a, velocity_a)?;
    let energy_b = kinetic_energy(mass_b, velocity_b)?;

    normalized_nonnegative(energy_a + energy_b)
}

/// Computes the kinetic energy lost between an initial and final state.
///
/// Formula: `loss = KE_initial - KE_final`
#[must_use]
pub fn kinetic_energy_loss(initial_kinetic_energy: f64, final_kinetic_energy: f64) -> Option<f64> {
    if !is_nonnegative_finite(initial_kinetic_energy)
        || !is_nonnegative_finite(final_kinetic_energy)
        || final_kinetic_energy > initial_kinetic_energy
    {
        return None;
    }

    normalized_nonnegative(initial_kinetic_energy - final_kinetic_energy)
}

/// Computes the fraction of kinetic energy lost between two states.
///
/// Formula: `loss_fraction = (KE_initial - KE_final) / KE_initial`
#[must_use]
pub fn kinetic_energy_loss_fraction(
    initial_kinetic_energy: f64,
    final_kinetic_energy: f64,
) -> Option<f64> {
    let invalid_inputs =
        !is_positive_finite(initial_kinetic_energy) || !is_nonnegative_finite(final_kinetic_energy);

    if invalid_inputs || final_kinetic_energy > initial_kinetic_energy {
        return None;
    }

    normalized_nonnegative((initial_kinetic_energy - final_kinetic_energy) / initial_kinetic_energy)
}

/// Computes the final velocities of a one-dimensional collision from masses, initial velocities,
/// and a coefficient of restitution.
///
/// Formulas:
///
/// - `v_a' = (m_a*v_a + m_b*v_b - m_b*e*(v_a - v_b)) / (m_a + m_b)`
/// - `v_b' = (m_a*v_a + m_b*v_b + m_a*e*(v_a - v_b)) / (m_a + m_b)`
///
/// # Examples
///
/// ```rust
/// use use_collision::collision_final_velocities_1d;
///
/// let (final_a, final_b) = collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0, 1.0).unwrap();
///
/// assert!((final_a + 1.0).abs() < 1.0e-12);
/// assert!((final_b - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn collision_final_velocities_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
    coefficient_of_restitution: f64,
) -> Option<(f64, f64)> {
    if !velocity_a.is_finite()
        || !velocity_b.is_finite()
        || !is_valid_restitution(coefficient_of_restitution)
    {
        return None;
    }

    let total_mass = combined_mass(mass_a, mass_b)?;
    let momentum_sum = total_momentum_1d(mass_a, velocity_a, mass_b, velocity_b)?;
    let relative = relative_velocity(velocity_a, velocity_b)?;
    let restitution_term_a = finite_result(mass_b * coefficient_of_restitution * relative)?;
    let restitution_term_b = finite_result(mass_a * coefficient_of_restitution * relative)?;
    let final_velocity_a = finite_result((momentum_sum - restitution_term_a) / total_mass)?;
    let final_velocity_b = finite_result((momentum_sum + restitution_term_b) / total_mass)?;

    Some((final_velocity_a, final_velocity_b))
}

/// Computes the final velocities of a perfectly elastic one-dimensional collision.
///
/// This delegates to [`collision_final_velocities_1d`] with `e = 1.0`.
///
/// # Examples
///
/// ```rust
/// use use_collision::elastic_collision_final_velocities_1d;
///
/// let (final_a, final_b) = elastic_collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0).unwrap();
///
/// assert!((final_a + 1.0).abs() < 1.0e-12);
/// assert!((final_b - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn elastic_collision_final_velocities_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<(f64, f64)> {
    collision_final_velocities_1d(mass_a, velocity_a, mass_b, velocity_b, 1.0)
}

/// Computes the shared final velocity of a perfectly inelastic one-dimensional collision.
///
/// Formula: `v_final = (m_a*v_a + m_b*v_b) / (m_a + m_b)`
///
/// # Examples
///
/// ```rust
/// use use_collision::perfectly_inelastic_collision_velocity_1d;
///
/// let final_velocity = perfectly_inelastic_collision_velocity_1d(2.0, 3.0, 4.0, -1.0).unwrap();
///
/// assert!((final_velocity - 0.333_333_333_333_333_3).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn perfectly_inelastic_collision_velocity_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<f64> {
    if !velocity_a.is_finite() || !velocity_b.is_finite() {
        return None;
    }

    let total_mass = combined_mass(mass_a, mass_b)?;
    let total_momentum = total_momentum_1d(mass_a, velocity_a, mass_b, velocity_b)?;

    finite_result(total_momentum / total_mass)
}

/// Computes the final velocities of a perfectly inelastic one-dimensional collision.
///
/// This delegates to [`perfectly_inelastic_collision_velocity_1d`] and returns the same velocity
/// for both bodies.
#[must_use]
pub fn perfectly_inelastic_collision_final_velocities_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
) -> Option<(f64, f64)> {
    let final_velocity =
        perfectly_inelastic_collision_velocity_1d(mass_a, velocity_a, mass_b, velocity_b)?;

    Some((final_velocity, final_velocity))
}

/// Computes the collision impulse applied to body A.
///
/// Formula: `J_a = m_a * (v_a_final - v_a_initial)`
#[must_use]
pub fn collision_impulse_on_a(
    mass_a: f64,
    initial_velocity_a: f64,
    final_velocity_a: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(mass_a)
        || !initial_velocity_a.is_finite()
        || !final_velocity_a.is_finite()
    {
        return None;
    }

    finite_result(mass_a * (final_velocity_a - initial_velocity_a))
}

/// Computes the collision impulse applied to body B.
///
/// Formula: `J_b = m_b * (v_b_final - v_b_initial)`
#[must_use]
pub fn collision_impulse_on_b(
    mass_b: f64,
    initial_velocity_b: f64,
    final_velocity_b: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(mass_b)
        || !initial_velocity_b.is_finite()
        || !final_velocity_b.is_finite()
    {
        return None;
    }

    finite_result(mass_b * (final_velocity_b - initial_velocity_b))
}

/// Computes the impulses on both bodies for a one-dimensional collision.
///
/// This computes the final velocities with [`collision_final_velocities_1d`] and then returns the
/// impulse on A and the impulse on B.
///
/// # Examples
///
/// ```rust
/// use use_collision::collision_impulses_1d;
///
/// let (impulse_a, impulse_b) = collision_impulses_1d(1.0, 1.0, 1.0, -1.0, 1.0).unwrap();
///
/// assert!((impulse_a + 2.0).abs() < 1.0e-12);
/// assert!((impulse_b - 2.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn collision_impulses_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
    coefficient_of_restitution: f64,
) -> Option<(f64, f64)> {
    let (final_velocity_a, final_velocity_b) = collision_final_velocities_1d(
        mass_a,
        velocity_a,
        mass_b,
        velocity_b,
        coefficient_of_restitution,
    )?;
    let impulse_a = collision_impulse_on_a(mass_a, velocity_a, final_velocity_a)?;
    let impulse_b = collision_impulse_on_b(mass_b, velocity_b, final_velocity_b)?;

    Some((impulse_a, impulse_b))
}

/// Computes the total kinetic energy lost in a one-dimensional collision.
///
/// This computes the initial and final total kinetic energy and returns the non-negative loss.
///
/// # Examples
///
/// ```rust
/// use use_collision::collision_energy_loss_1d;
///
/// let loss = collision_energy_loss_1d(1.0, 1.0, 1.0, -1.0, 0.0).unwrap();
///
/// assert!((loss - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn collision_energy_loss_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
    coefficient_of_restitution: f64,
) -> Option<f64> {
    let initial_energy = total_kinetic_energy_1d(mass_a, velocity_a, mass_b, velocity_b)?;
    let (final_velocity_a, final_velocity_b) = collision_final_velocities_1d(
        mass_a,
        velocity_a,
        mass_b,
        velocity_b,
        coefficient_of_restitution,
    )?;
    let final_energy = total_kinetic_energy_1d(mass_a, final_velocity_a, mass_b, final_velocity_b)?;

    kinetic_energy_loss(initial_energy, final_energy)
}

/// Computes the fraction of kinetic energy lost in a one-dimensional collision.
#[must_use]
pub fn collision_energy_loss_fraction_1d(
    mass_a: f64,
    velocity_a: f64,
    mass_b: f64,
    velocity_b: f64,
    coefficient_of_restitution: f64,
) -> Option<f64> {
    let initial_energy = total_kinetic_energy_1d(mass_a, velocity_a, mass_b, velocity_b)?;
    let (final_velocity_a, final_velocity_b) = collision_final_velocities_1d(
        mass_a,
        velocity_a,
        mass_b,
        velocity_b,
        coefficient_of_restitution,
    )?;
    let final_energy = total_kinetic_energy_1d(mass_a, final_velocity_a, mass_b, final_velocity_b)?;

    kinetic_energy_loss_fraction(initial_energy, final_energy)
}

/// A one-dimensional body with scalar mass and velocity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionBody1D {
    pub mass: f64,
    pub velocity: f64,
}

impl CollisionBody1D {
    /// Creates a one-dimensional collision body when `mass` is non-negative and both values are
    /// finite.
    #[must_use]
    pub fn new(mass: f64, velocity: f64) -> Option<Self> {
        if !is_nonnegative_finite(mass) || !velocity.is_finite() {
            return None;
        }

        Some(Self { mass, velocity })
    }

    /// Computes kinetic energy for this body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_collision::CollisionBody1D;
    ///
    /// let body = CollisionBody1D::new(2.0, 3.0).unwrap();
    ///
    /// assert_eq!(body.kinetic_energy(), Some(9.0));
    /// ```
    #[must_use]
    pub fn kinetic_energy(&self) -> Option<f64> {
        kinetic_energy(self.mass, self.velocity)
    }

    /// Computes scalar momentum for this body using `p = m * v`.
    #[must_use]
    pub fn momentum(&self) -> Option<f64> {
        momentum_from_mass_velocity(self.mass, self.velocity)
    }
}

/// A one-dimensional collision configuration with two bodies and a restitution coefficient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision1D {
    pub body_a: CollisionBody1D,
    pub body_b: CollisionBody1D,
    pub coefficient_of_restitution: f64,
}

impl Collision1D {
    /// Creates a one-dimensional collision when the restitution coefficient is valid.
    #[must_use]
    pub fn new(
        body_a: CollisionBody1D,
        body_b: CollisionBody1D,
        coefficient_of_restitution: f64,
    ) -> Option<Self> {
        if !is_valid_restitution(coefficient_of_restitution) {
            return None;
        }

        Some(Self {
            body_a,
            body_b,
            coefficient_of_restitution,
        })
    }

    /// Computes the final velocities of both bodies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_collision::{Collision1D, CollisionBody1D};
    ///
    /// let body_a = CollisionBody1D::new(1.0, 1.0).unwrap();
    /// let body_b = CollisionBody1D::new(1.0, -1.0).unwrap();
    /// let collision = Collision1D::new(body_a, body_b, 1.0).unwrap();
    ///
    /// let (final_a, final_b) = collision.final_velocities().unwrap();
    ///
    /// assert!((final_a + 1.0).abs() < 1.0e-12);
    /// assert!((final_b - 1.0).abs() < 1.0e-12);
    /// ```
    #[must_use]
    pub fn final_velocities(&self) -> Option<(f64, f64)> {
        collision_final_velocities_1d(
            self.body_a.mass,
            self.body_a.velocity,
            self.body_b.mass,
            self.body_b.velocity,
            self.coefficient_of_restitution,
        )
    }

    /// Computes the final body states after the collision.
    #[must_use]
    pub fn final_bodies(&self) -> Option<(CollisionBody1D, CollisionBody1D)> {
        let (final_velocity_a, final_velocity_b) = self.final_velocities()?;
        let body_a = CollisionBody1D::new(self.body_a.mass, final_velocity_a)?;
        let body_b = CollisionBody1D::new(self.body_b.mass, final_velocity_b)?;

        Some((body_a, body_b))
    }

    /// Computes the initial total kinetic energy.
    #[must_use]
    pub fn initial_kinetic_energy(&self) -> Option<f64> {
        total_kinetic_energy_1d(
            self.body_a.mass,
            self.body_a.velocity,
            self.body_b.mass,
            self.body_b.velocity,
        )
    }

    /// Computes the final total kinetic energy.
    #[must_use]
    pub fn final_kinetic_energy(&self) -> Option<f64> {
        let (final_velocity_a, final_velocity_b) = self.final_velocities()?;

        total_kinetic_energy_1d(
            self.body_a.mass,
            final_velocity_a,
            self.body_b.mass,
            final_velocity_b,
        )
    }

    /// Computes the total kinetic energy lost in the collision.
    #[must_use]
    pub fn kinetic_energy_loss(&self) -> Option<f64> {
        collision_energy_loss_1d(
            self.body_a.mass,
            self.body_a.velocity,
            self.body_b.mass,
            self.body_b.velocity,
            self.coefficient_of_restitution,
        )
    }

    /// Computes the fraction of kinetic energy lost in the collision.
    #[must_use]
    pub fn kinetic_energy_loss_fraction(&self) -> Option<f64> {
        collision_energy_loss_fraction_1d(
            self.body_a.mass,
            self.body_a.velocity,
            self.body_b.mass,
            self.body_b.velocity,
            self.coefficient_of_restitution,
        )
    }

    /// Computes the impulses applied to both bodies.
    #[must_use]
    pub fn impulses(&self) -> Option<(f64, f64)> {
        collision_impulses_1d(
            self.body_a.mass,
            self.body_a.velocity,
            self.body_b.mass,
            self.body_b.velocity,
            self.coefficient_of_restitution,
        )
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        Collision1D, CollisionBody1D, coefficient_of_restitution, collision_energy_loss_1d,
        collision_energy_loss_fraction_1d, collision_final_velocities_1d, collision_impulse_on_a,
        collision_impulse_on_b, collision_impulses_1d, elastic_collision_final_velocities_1d,
        is_perfectly_elastic, is_perfectly_inelastic, is_valid_restitution, kinetic_energy,
        kinetic_energy_loss, kinetic_energy_loss_fraction,
        perfectly_inelastic_collision_final_velocities_1d,
        perfectly_inelastic_collision_velocity_1d, relative_speed, relative_velocity,
        separation_speed_from_restitution, total_kinetic_energy_1d,
    };

    const EPSILON: f64 = 1.0e-12;

    fn assert_approx_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_option_approx_eq(actual: Option<f64>, expected: f64) {
        match actual {
            Some(value) => assert_approx_eq(value, expected),
            None => panic!("expected Some({expected}), got None"),
        }
    }

    fn assert_option_pair_approx_eq(actual: Option<(f64, f64)>, expected: (f64, f64)) {
        match actual {
            Some((value_a, value_b)) => {
                assert_approx_eq(value_a, expected.0);
                assert_approx_eq(value_b, expected.1);
            },
            None => panic!("expected Some(({},{}) ), got None", expected.0, expected.1),
        }
    }

    #[test]
    fn relative_velocity_and_speed_cover_signed_inputs() {
        assert_eq!(relative_velocity(5.0, 2.0), Some(3.0));
        assert_eq!(relative_velocity(2.0, 5.0), Some(-3.0));
        assert_eq!(relative_speed(2.0, 5.0), Some(3.0));
    }

    #[test]
    fn restitution_helpers_validate_common_cases() {
        assert_eq!(coefficient_of_restitution(10.0, 8.0), Some(0.8));
        assert_eq!(coefficient_of_restitution(10.0, 0.0), Some(0.0));
        assert_eq!(coefficient_of_restitution(0.0, 1.0), None);
        assert_eq!(coefficient_of_restitution(10.0, -1.0), None);
        assert_eq!(coefficient_of_restitution(10.0, 11.0), None);

        assert_eq!(separation_speed_from_restitution(10.0, 0.8), Some(8.0));
        assert_eq!(separation_speed_from_restitution(10.0, 1.2), None);

        assert!(is_valid_restitution(0.0));
        assert!(is_valid_restitution(1.0));
        assert!(!is_valid_restitution(-0.1));
        assert!(!is_valid_restitution(1.1));

        assert_eq!(is_perfectly_elastic(1.0, 0.0), Some(true));
        assert_eq!(is_perfectly_elastic(0.99, 0.02), Some(true));
        assert_eq!(is_perfectly_elastic(0.9, 0.02), Some(false));

        assert_eq!(is_perfectly_inelastic(0.0, 0.0), Some(true));
        assert_eq!(is_perfectly_inelastic(0.01, 0.02), Some(true));
        assert_eq!(is_perfectly_inelastic(0.1, 0.02), Some(false));
    }

    #[test]
    fn kinetic_energy_helpers_cover_common_cases() {
        assert_eq!(kinetic_energy(2.0, 3.0), Some(9.0));
        assert_eq!(kinetic_energy(2.0, -3.0), Some(9.0));
        assert_eq!(kinetic_energy(-2.0, 3.0), None);

        assert_eq!(total_kinetic_energy_1d(2.0, 3.0, 4.0, 1.0), Some(11.0));

        assert_eq!(kinetic_energy_loss(10.0, 6.0), Some(4.0));
        assert_eq!(kinetic_energy_loss(6.0, 10.0), None);

        assert_eq!(kinetic_energy_loss_fraction(10.0, 6.0), Some(0.4));
        assert_eq!(kinetic_energy_loss_fraction(0.0, 0.0), None);
    }

    #[test]
    fn collision_velocity_helpers_cover_elastic_and_inelastic_cases() {
        assert_option_pair_approx_eq(
            elastic_collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0),
            (-1.0, 1.0),
        );

        assert_option_pair_approx_eq(
            collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0, 1.0),
            (-1.0, 1.0),
        );
        assert_option_pair_approx_eq(
            collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0, 0.0),
            (0.0, 0.0),
        );
        assert_eq!(
            collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0, 1.2),
            None
        );
        assert_eq!(
            collision_final_velocities_1d(-1.0, 1.0, 1.0, -1.0, 1.0),
            None
        );

        assert_eq!(
            perfectly_inelastic_collision_velocity_1d(1.0, 1.0, 1.0, -1.0),
            Some(0.0)
        );
        assert_option_approx_eq(
            perfectly_inelastic_collision_velocity_1d(2.0, 3.0, 4.0, -1.0),
            0.333_333_333_333_333_3,
        );

        assert_eq!(
            perfectly_inelastic_collision_final_velocities_1d(1.0, 1.0, 1.0, -1.0),
            Some((0.0, 0.0))
        );
    }

    #[test]
    fn impulse_and_energy_summary_helpers_cover_common_cases() {
        assert_eq!(collision_impulse_on_a(2.0, 3.0, 1.0), Some(-4.0));
        assert_eq!(collision_impulse_on_b(2.0, 1.0, 3.0), Some(4.0));

        assert_option_pair_approx_eq(collision_impulses_1d(1.0, 1.0, 1.0, -1.0, 1.0), (-2.0, 2.0));

        assert_option_approx_eq(collision_energy_loss_1d(1.0, 1.0, 1.0, -1.0, 1.0), 0.0);
        assert_option_approx_eq(collision_energy_loss_1d(1.0, 1.0, 1.0, -1.0, 0.0), 1.0);

        assert_option_approx_eq(
            collision_energy_loss_fraction_1d(1.0, 1.0, 1.0, -1.0, 0.0),
            1.0,
        );
    }

    #[test]
    fn simple_types_delegate_to_public_helpers() {
        let body = CollisionBody1D::new(2.0, 3.0).unwrap();
        assert_eq!(body.kinetic_energy(), Some(9.0));
        assert_eq!(body.momentum(), Some(6.0));
        assert_eq!(CollisionBody1D::new(-2.0, 3.0), None);

        let body_a = CollisionBody1D::new(1.0, 1.0).unwrap();
        let body_b = CollisionBody1D::new(1.0, -1.0).unwrap();
        let collision = Collision1D::new(body_a, body_b, 1.0).unwrap();

        assert_option_pair_approx_eq(collision.final_velocities(), (-1.0, 1.0));
        assert_option_approx_eq(collision.initial_kinetic_energy(), 1.0);
        assert_option_approx_eq(collision.final_kinetic_energy(), 1.0);
        assert_option_approx_eq(collision.kinetic_energy_loss(), 0.0);
        assert_option_pair_approx_eq(collision.impulses(), (-2.0, 2.0));
        assert_eq!(Collision1D::new(body_a, body_b, 1.2), None);
    }
}
