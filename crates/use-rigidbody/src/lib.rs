#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Scalar rigid-body mechanics helpers.

pub mod prelude;

/// Mass and rotational inertia for a scalar rigid body.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassProperties {
    pub mass: f64,
    pub moment_of_inertia: f64,
}

impl MassProperties {
    /// Creates mass properties when both values are finite and non-negative.
    #[must_use]
    pub fn new(mass: f64, moment_of_inertia: f64) -> Option<Self> {
        if !is_nonnegative_finite(mass) || !is_nonnegative_finite(moment_of_inertia) {
            return None;
        }

        Some(Self {
            mass,
            moment_of_inertia,
        })
    }

    /// Creates mass properties for a point mass using `I = mr²`.
    #[must_use]
    pub fn point_mass(mass: f64, radius: f64) -> Option<Self> {
        let moment_of_inertia = point_mass_moment_of_inertia(mass, radius)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a solid disk using `I = 0.5mr²`.
    #[must_use]
    pub fn solid_disk(mass: f64, radius: f64) -> Option<Self> {
        let moment_of_inertia = solid_disk_moment_of_inertia(mass, radius)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a thin ring using `I = mr²`.
    #[must_use]
    pub fn thin_ring(mass: f64, radius: f64) -> Option<Self> {
        let moment_of_inertia = thin_ring_moment_of_inertia(mass, radius)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a solid sphere using `I = (2 / 5)mr²`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rigidbody::MassProperties;
    ///
    /// let props = MassProperties::solid_sphere(5.0, 2.0).unwrap();
    ///
    /// assert_eq!(props.moment_of_inertia, 8.0);
    /// ```
    #[must_use]
    pub fn solid_sphere(mass: f64, radius: f64) -> Option<Self> {
        let moment_of_inertia = solid_sphere_moment_of_inertia(mass, radius)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a hollow sphere using `I = (2 / 3)mr²`.
    #[must_use]
    pub fn hollow_sphere(mass: f64, radius: f64) -> Option<Self> {
        let moment_of_inertia = hollow_sphere_moment_of_inertia(mass, radius)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a rod about its center using `I = (1 / 12)mL²`.
    #[must_use]
    pub fn rod_about_center(mass: f64, length: f64) -> Option<Self> {
        let moment_of_inertia = rod_moment_of_inertia_about_center(mass, length)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Creates mass properties for a rod about one end using `I = (1 / 3)mL²`.
    #[must_use]
    pub fn rod_about_end(mass: f64, length: f64) -> Option<Self> {
        let moment_of_inertia = rod_moment_of_inertia_about_end(mass, length)?;

        Self::new(mass, moment_of_inertia)
    }

    /// Applies the parallel-axis theorem using `I = I_cm + md²`.
    #[must_use]
    pub fn shifted_by_parallel_axis(self, distance: f64) -> Option<Self> {
        let moment_of_inertia =
            parallel_axis_moment_of_inertia(self.moment_of_inertia, self.mass, distance)?;

        Self::new(self.mass, moment_of_inertia)
    }
}

/// A one-dimensional rigid body with scalar translational and rotational state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RigidBody1D {
    pub mass_properties: MassProperties,
    pub position: f64,
    pub velocity: f64,
    pub angle: f64,
    pub angular_velocity: f64,
}

impl RigidBody1D {
    /// Creates a rigid body when the state values are finite.
    #[must_use]
    pub const fn new(
        mass_properties: MassProperties,
        position: f64,
        velocity: f64,
        angle: f64,
        angular_velocity: f64,
    ) -> Option<Self> {
        if !position.is_finite()
            || !velocity.is_finite()
            || !angle.is_finite()
            || !angular_velocity.is_finite()
        {
            return None;
        }

        Some(Self {
            mass_properties,
            position,
            velocity,
            angle,
            angular_velocity,
        })
    }

    /// Returns the body's mass.
    #[must_use]
    pub const fn mass(&self) -> f64 {
        self.mass_properties.mass
    }

    /// Returns the body's moment of inertia.
    #[must_use]
    pub const fn moment_of_inertia(&self) -> f64 {
        self.mass_properties.moment_of_inertia
    }

    /// Computes linear momentum using `p = mv`.
    #[must_use]
    pub fn linear_momentum(&self) -> Option<f64> {
        linear_momentum(self.mass(), self.velocity)
    }

    /// Computes angular momentum using `L = Iω`.
    #[must_use]
    pub fn angular_momentum(&self) -> Option<f64> {
        angular_momentum(self.moment_of_inertia(), self.angular_velocity)
    }

    /// Computes translational kinetic energy using `KE = 0.5mv²`.
    #[must_use]
    pub fn linear_kinetic_energy(&self) -> Option<f64> {
        linear_kinetic_energy(self.mass(), self.velocity)
    }

    /// Computes rotational kinetic energy using `KE_rot = 0.5Iω²`.
    #[must_use]
    pub fn rotational_kinetic_energy(&self) -> Option<f64> {
        rotational_kinetic_energy(self.moment_of_inertia(), self.angular_velocity)
    }

    /// Computes total kinetic energy as translational plus rotational energy.
    #[must_use]
    pub fn total_kinetic_energy(&self) -> Option<f64> {
        total_kinetic_energy(
            self.mass(),
            self.velocity,
            self.moment_of_inertia(),
            self.angular_velocity,
        )
    }

    /// Returns a copy with updated velocity after a linear impulse.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rigidbody::{MassProperties, RigidBody1D};
    ///
    /// let props = MassProperties::new(2.0, 4.0).unwrap();
    /// let body = RigidBody1D::new(props, 10.0, 3.0, 1.0, 5.0).unwrap();
    ///
    /// assert_eq!(body.with_impulse(4.0).unwrap().velocity, 5.0);
    /// ```
    #[must_use]
    pub fn with_impulse(self, impulse: f64) -> Option<Self> {
        let velocity = velocity_after_impulse(self.mass(), self.velocity, impulse)?;

        Self::new(
            self.mass_properties,
            self.position,
            velocity,
            self.angle,
            self.angular_velocity,
        )
    }

    /// Returns a copy with updated angular velocity after an angular impulse.
    #[must_use]
    pub fn with_angular_impulse(self, angular_impulse: f64) -> Option<Self> {
        let angular_velocity = angular_velocity_after_angular_impulse(
            self.moment_of_inertia(),
            self.angular_velocity,
            angular_impulse,
        )?;

        Self::new(
            self.mass_properties,
            self.position,
            self.velocity,
            self.angle,
            angular_velocity,
        )
    }

    /// Advances position and angle kinematically using the current velocities.
    ///
    /// This helper updates:
    ///
    /// - `position += velocity * time`
    /// - `angle += angular_velocity * time`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rigidbody::{MassProperties, RigidBody1D};
    ///
    /// let props = MassProperties::new(2.0, 4.0).unwrap();
    /// let body = RigidBody1D::new(props, 10.0, 3.0, 1.0, 5.0).unwrap();
    /// let advanced = body.advanced_kinematically(2.0).unwrap();
    ///
    /// assert_eq!(advanced.position, 16.0);
    /// assert_eq!(advanced.angle, 11.0);
    /// ```
    #[must_use]
    pub fn advanced_kinematically(self, time: f64) -> Option<Self> {
        if !is_nonnegative_finite(time) {
            return None;
        }

        let position = finite_result(self.velocity.mul_add(time, self.position))?;
        let angle = finite_result(self.angular_velocity.mul_add(time, self.angle))?;

        Self::new(
            self.mass_properties,
            position,
            self.velocity,
            angle,
            self.angular_velocity,
        )
    }
}

/// Computes the one-dimensional center of mass using `x_cm = Σ(m_i * x_i) / Σm_i`.
///
/// Returns `None` when the slices have different lengths, are empty, contain invalid inputs, or
/// when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::center_of_mass_1d;
///
/// assert_eq!(center_of_mass_1d(&[1.0, 3.0], &[0.0, 10.0]), Some(7.5));
/// ```
#[must_use]
pub fn center_of_mass_1d(masses: &[f64], positions: &[f64]) -> Option<f64> {
    if masses.len() != positions.len() || masses.is_empty() {
        return None;
    }

    let (weighted_position_sum, total_mass) = masses.iter().zip(positions).try_fold(
        (0.0, 0.0),
        |(weighted_sum, total_mass), (mass, position)| {
            if !is_nonnegative_finite(*mass) || !position.is_finite() {
                return None;
            }

            let weighted_sum = finite_result((*mass).mul_add(*position, weighted_sum))?;
            let total_mass = finite_result(total_mass + *mass)?;

            Some((weighted_sum, total_mass))
        },
    )?;

    if total_mass <= 0.0 {
        return None;
    }

    finite_result(weighted_position_sum / total_mass)
}

/// Computes the sum of non-negative masses.
///
/// Returns `Some(0.0)` for an empty slice and `None` when any mass or the result is invalid.
#[must_use]
pub fn combined_mass(masses: &[f64]) -> Option<f64> {
    masses.iter().try_fold(0.0, |sum, mass| {
        if !is_nonnegative_finite(*mass) {
            return None;
        }

        finite_result(sum + *mass)
    })
}

/// Computes reduced mass using `μ = (m1 * m2) / (m1 + m2)`.
#[must_use]
pub fn reduced_mass(mass_a: f64, mass_b: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass_a) || !is_nonnegative_finite(mass_b) {
        return None;
    }

    let total_mass = finite_result(mass_a + mass_b)?;
    if total_mass <= 0.0 {
        return None;
    }

    finite_result((mass_a * mass_b) / total_mass)
}

/// Computes point-mass moment of inertia using `I = mr²`.
#[must_use]
pub fn point_mass_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 1.0)
}

/// Computes solid-disk moment of inertia using `I = 0.5mr²`.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::solid_disk_moment_of_inertia;
///
/// assert_eq!(solid_disk_moment_of_inertia(2.0, 3.0), Some(9.0));
/// ```
#[must_use]
pub fn solid_disk_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 0.5)
}

/// Computes thin-ring moment of inertia using `I = mr²`.
#[must_use]
pub fn thin_ring_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    point_mass_moment_of_inertia(mass, radius)
}

/// Computes solid-sphere moment of inertia using `I = (2 / 5)mr²`.
#[must_use]
pub fn solid_sphere_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 2.0 / 5.0)
}

/// Computes hollow-sphere moment of inertia using `I = (2 / 3)mr²`.
#[must_use]
pub fn hollow_sphere_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 2.0 / 3.0)
}

/// Computes rod moment of inertia about its center using `I = (1 / 12)mL²`.
#[must_use]
pub fn rod_moment_of_inertia_about_center(mass: f64, length: f64) -> Option<f64> {
    scaled_square_measure(mass, length, 1.0 / 12.0)
}

/// Computes rod moment of inertia about one end using `I = (1 / 3)mL²`.
#[must_use]
pub fn rod_moment_of_inertia_about_end(mass: f64, length: f64) -> Option<f64> {
    scaled_square_measure(mass, length, 1.0 / 3.0)
}

/// Applies the parallel-axis theorem using `I = I_cm + md²`.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::parallel_axis_moment_of_inertia;
///
/// assert_eq!(parallel_axis_moment_of_inertia(4.0, 2.0, 3.0), Some(22.0));
/// ```
#[must_use]
pub fn parallel_axis_moment_of_inertia(
    center_moment_of_inertia: f64,
    mass: f64,
    distance: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(center_moment_of_inertia)
        || !is_nonnegative_finite(mass)
        || !is_nonnegative_finite(distance)
    {
        return None;
    }

    finite_result((mass * distance).mul_add(distance, center_moment_of_inertia))
}

/// Computes the center moment from a shifted moment using `I_cm = I - md²`.
#[must_use]
pub fn center_moment_from_parallel_axis(
    shifted_moment_of_inertia: f64,
    mass: f64,
    distance: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(shifted_moment_of_inertia)
        || !is_nonnegative_finite(mass)
        || !is_nonnegative_finite(distance)
    {
        return None;
    }

    nonnegative_finite_result((mass * distance).mul_add(-distance, shifted_moment_of_inertia))
}

/// Computes linear momentum using `p = mv`.
#[must_use]
pub fn linear_momentum(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !velocity.is_finite() {
        return None;
    }

    finite_result(mass * velocity)
}

/// Computes translational kinetic energy using `KE = 0.5mv²`.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::linear_kinetic_energy;
///
/// assert_eq!(linear_kinetic_energy(2.0, 3.0), Some(9.0));
/// ```
#[must_use]
pub fn linear_kinetic_energy(mass: f64, velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !velocity.is_finite() {
        return None;
    }

    nonnegative_finite_result(0.5 * mass * velocity * velocity)
}

/// Computes final velocity after an impulse using `v_final = v_initial + J / m`.
#[must_use]
pub fn velocity_after_impulse(mass: f64, initial_velocity: f64, impulse: f64) -> Option<f64> {
    if !is_positive_finite(mass) || !initial_velocity.is_finite() || !impulse.is_finite() {
        return None;
    }

    finite_result(initial_velocity + impulse / mass)
}

/// Computes impulse from a velocity change using `J = m(v_final - v_initial)`.
#[must_use]
pub fn impulse_from_velocity_change(
    mass: f64,
    initial_velocity: f64,
    final_velocity: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(mass) || !initial_velocity.is_finite() || !final_velocity.is_finite()
    {
        return None;
    }

    finite_result(mass * (final_velocity - initial_velocity))
}

/// Computes angular momentum using `L = Iω`.
#[must_use]
pub fn angular_momentum(moment_of_inertia: f64, angular_velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(moment_of_inertia) || !angular_velocity.is_finite() {
        return None;
    }

    finite_result(moment_of_inertia * angular_velocity)
}

/// Computes rotational kinetic energy using `KE_rot = 0.5Iω²`.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::rotational_kinetic_energy;
///
/// assert_eq!(rotational_kinetic_energy(4.0, 5.0), Some(50.0));
/// ```
#[must_use]
pub fn rotational_kinetic_energy(moment_of_inertia: f64, angular_velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(moment_of_inertia) || !angular_velocity.is_finite() {
        return None;
    }

    nonnegative_finite_result(0.5 * moment_of_inertia * angular_velocity * angular_velocity)
}

/// Computes final angular velocity after an angular impulse using `ω_final = ω_initial + J / I`.
#[must_use]
pub fn angular_velocity_after_angular_impulse(
    moment_of_inertia: f64,
    initial_angular_velocity: f64,
    angular_impulse: f64,
) -> Option<f64> {
    if !is_positive_finite(moment_of_inertia)
        || !initial_angular_velocity.is_finite()
        || !angular_impulse.is_finite()
    {
        return None;
    }

    finite_result(initial_angular_velocity + angular_impulse / moment_of_inertia)
}

/// Computes angular impulse from an angular velocity change using `J = I(ω_final - ω_initial)`.
#[must_use]
pub fn angular_impulse_from_angular_velocity_change(
    moment_of_inertia: f64,
    initial_angular_velocity: f64,
    final_angular_velocity: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(moment_of_inertia)
        || !initial_angular_velocity.is_finite()
        || !final_angular_velocity.is_finite()
    {
        return None;
    }

    finite_result(moment_of_inertia * (final_angular_velocity - initial_angular_velocity))
}

/// Computes total kinetic energy using `KE_total = 0.5mv² + 0.5Iω²`.
///
/// # Examples
///
/// ```rust
/// use use_rigidbody::total_kinetic_energy;
///
/// assert_eq!(total_kinetic_energy(2.0, 3.0, 4.0, 5.0), Some(59.0));
/// ```
#[must_use]
pub fn total_kinetic_energy(
    mass: f64,
    linear_velocity: f64,
    moment_of_inertia: f64,
    angular_velocity: f64,
) -> Option<f64> {
    let linear = linear_kinetic_energy(mass, linear_velocity)?;
    let rotational = rotational_kinetic_energy(moment_of_inertia, angular_velocity)?;

    nonnegative_finite_result(linear + rotational)
}

fn scaled_square_measure(primary: f64, measure: f64, factor: f64) -> Option<f64> {
    if !is_nonnegative_finite(primary) || !is_nonnegative_finite(measure) {
        return None;
    }

    nonnegative_finite_result(factor * primary * measure * measure)
}

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(normalize_zero(value))
}

fn nonnegative_finite_result(value: f64) -> Option<f64> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }

    Some(normalize_zero(value))
}

fn normalize_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

#[cfg(test)]
mod tests {
    use super::{
        MassProperties, RigidBody1D, angular_impulse_from_angular_velocity_change,
        angular_momentum, angular_velocity_after_angular_impulse, center_moment_from_parallel_axis,
        center_of_mass_1d, combined_mass, hollow_sphere_moment_of_inertia,
        impulse_from_velocity_change, linear_kinetic_energy, linear_momentum,
        parallel_axis_moment_of_inertia, point_mass_moment_of_inertia, reduced_mass,
        rod_moment_of_inertia_about_center, rod_moment_of_inertia_about_end,
        rotational_kinetic_energy, solid_disk_moment_of_inertia, solid_sphere_moment_of_inertia,
        thin_ring_moment_of_inertia, total_kinetic_energy, velocity_after_impulse,
    };

    const EPSILON: f64 = 1.0e-12;

    fn assert_approx_eq(left: f64, right: f64) {
        let tolerance = EPSILON * left.abs().max(right.abs()).max(1.0);
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    fn assert_some_approx_eq(value: Option<f64>, expected: f64) {
        let Some(value) = value else {
            panic!("expected Some({expected})");
        };

        assert_approx_eq(value, expected);
    }

    #[test]
    fn center_of_mass_helpers_cover_common_cases() {
        assert_some_approx_eq(center_of_mass_1d(&[1.0, 1.0], &[0.0, 10.0]), 5.0);
        assert_some_approx_eq(center_of_mass_1d(&[1.0, 3.0], &[0.0, 10.0]), 7.5);
        assert_eq!(center_of_mass_1d(&[], &[]), None);
        assert_eq!(center_of_mass_1d(&[1.0], &[0.0, 1.0]), None);
        assert_eq!(center_of_mass_1d(&[-1.0], &[0.0]), None);
    }

    #[test]
    fn mass_helpers_cover_common_cases() {
        assert_some_approx_eq(combined_mass(&[1.0, 2.0, 3.0]), 6.0);
        assert_some_approx_eq(combined_mass(&[]), 0.0);
        assert_eq!(combined_mass(&[1.0, -2.0]), None);

        assert_some_approx_eq(reduced_mass(2.0, 2.0), 1.0);
        assert_some_approx_eq(reduced_mass(2.0, 6.0), 1.5);
        assert_eq!(reduced_mass(0.0, 0.0), None);
    }

    #[test]
    fn moment_of_inertia_helpers_cover_common_shapes() {
        assert_some_approx_eq(point_mass_moment_of_inertia(2.0, 3.0), 18.0);
        assert_some_approx_eq(solid_disk_moment_of_inertia(2.0, 3.0), 9.0);
        assert_some_approx_eq(thin_ring_moment_of_inertia(2.0, 3.0), 18.0);
        assert_some_approx_eq(solid_sphere_moment_of_inertia(5.0, 2.0), 8.0);
        assert_some_approx_eq(hollow_sphere_moment_of_inertia(3.0, 2.0), 8.0);
        assert_some_approx_eq(rod_moment_of_inertia_about_center(12.0, 2.0), 4.0);
        assert_some_approx_eq(rod_moment_of_inertia_about_end(3.0, 2.0), 4.0);
    }

    #[test]
    fn parallel_axis_helpers_validate_inputs() {
        assert_some_approx_eq(parallel_axis_moment_of_inertia(4.0, 2.0, 3.0), 22.0);
        assert_eq!(parallel_axis_moment_of_inertia(-4.0, 2.0, 3.0), None);

        assert_some_approx_eq(center_moment_from_parallel_axis(22.0, 2.0, 3.0), 4.0);
        assert_eq!(center_moment_from_parallel_axis(4.0, 2.0, 3.0), None);
    }

    #[test]
    fn linear_helpers_cover_common_values() {
        assert_some_approx_eq(linear_momentum(2.0, 3.0), 6.0);
        assert_some_approx_eq(linear_momentum(2.0, -3.0), -6.0);
        assert_eq!(linear_momentum(-2.0, 3.0), None);

        assert_some_approx_eq(linear_kinetic_energy(2.0, 3.0), 9.0);
        assert_some_approx_eq(linear_kinetic_energy(2.0, -3.0), 9.0);

        assert_some_approx_eq(velocity_after_impulse(2.0, 3.0, 4.0), 5.0);
        assert_eq!(velocity_after_impulse(0.0, 3.0, 4.0), None);

        assert_some_approx_eq(impulse_from_velocity_change(2.0, 3.0, 5.0), 4.0);
    }

    #[test]
    fn rotational_helpers_cover_common_values() {
        assert_some_approx_eq(angular_momentum(4.0, 5.0), 20.0);
        assert_some_approx_eq(angular_momentum(4.0, -5.0), -20.0);
        assert_eq!(angular_momentum(-4.0, 5.0), None);

        assert_some_approx_eq(rotational_kinetic_energy(4.0, 5.0), 50.0);

        assert_some_approx_eq(angular_velocity_after_angular_impulse(4.0, 5.0, 8.0), 7.0);
        assert_eq!(angular_velocity_after_angular_impulse(0.0, 5.0, 8.0), None);

        assert_some_approx_eq(
            angular_impulse_from_angular_velocity_change(4.0, 5.0, 7.0),
            8.0,
        );
    }

    #[test]
    fn total_energy_helper_combines_linear_and_rotational_energy() {
        assert_some_approx_eq(total_kinetic_energy(2.0, 3.0, 4.0, 5.0), 59.0);
    }

    #[test]
    fn mass_properties_validate_and_delegate_to_public_helpers() {
        let props = MassProperties::new(2.0, 4.0).expect("expected valid mass properties");
        assert_approx_eq(props.mass, 2.0);

        assert_eq!(MassProperties::new(-2.0, 4.0), None);
        assert_eq!(MassProperties::new(2.0, -4.0), None);

        let sphere = MassProperties::solid_sphere(5.0, 2.0).expect("expected solid sphere");
        assert_approx_eq(sphere.moment_of_inertia, 8.0);

        let rod = MassProperties::rod_about_center(12.0, 2.0).expect("expected rod about center");
        assert_approx_eq(rod.moment_of_inertia, 4.0);

        let shifted = props
            .shifted_by_parallel_axis(3.0)
            .expect("expected shifted properties");
        assert_approx_eq(shifted.moment_of_inertia, 22.0);
    }

    #[test]
    fn rigid_body_methods_delegate_to_public_helpers() {
        let props = MassProperties::new(2.0, 4.0).expect("expected valid mass properties");
        let body = RigidBody1D::new(props, 10.0, 3.0, 1.0, 5.0).expect("expected valid body");

        assert_approx_eq(body.mass(), 2.0);
        assert_approx_eq(body.moment_of_inertia(), 4.0);
        assert_some_approx_eq(body.linear_momentum(), 6.0);
        assert_some_approx_eq(body.angular_momentum(), 20.0);
        assert_some_approx_eq(body.linear_kinetic_energy(), 9.0);
        assert_some_approx_eq(body.rotational_kinetic_energy(), 50.0);
        assert_some_approx_eq(body.total_kinetic_energy(), 59.0);

        let with_impulse = body.with_impulse(4.0).expect("expected updated velocity");
        assert_approx_eq(with_impulse.velocity, 5.0);

        let with_angular_impulse = body
            .with_angular_impulse(8.0)
            .expect("expected updated angular velocity");
        assert_approx_eq(with_angular_impulse.angular_velocity, 7.0);

        let advanced = body
            .advanced_kinematically(2.0)
            .expect("expected advanced body state");
        assert_approx_eq(advanced.position, 16.0);
        assert_approx_eq(advanced.angle, 11.0);

        assert_eq!(RigidBody1D::new(props, f64::NAN, 3.0, 1.0, 5.0), None);
        assert_eq!(body.advanced_kinematically(-1.0), None);
    }
}
