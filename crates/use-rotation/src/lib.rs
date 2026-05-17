#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::f64::consts::TAU;

pub mod prelude;

/// A rotating body with scalar moment of inertia and angular velocity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotatingBody {
    pub moment_of_inertia: f64,
    pub angular_velocity: f64,
}

impl RotatingBody {
    /// Creates a rotating body when `moment_of_inertia` is non-negative and both values are finite.
    #[must_use]
    pub const fn new(moment_of_inertia: f64, angular_velocity: f64) -> Option<Self> {
        if !moment_of_inertia.is_finite()
            || moment_of_inertia < 0.0
            || !angular_velocity.is_finite()
        {
            return None;
        }

        Some(Self {
            moment_of_inertia,
            angular_velocity,
        })
    }

    /// Computes angular momentum using `L = Iω`.
    #[must_use]
    pub fn angular_momentum(&self) -> Option<f64> {
        angular_momentum(self.moment_of_inertia, self.angular_velocity)
    }

    /// Computes rotational kinetic energy using `KE_rot = 0.5 * I * ω²`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rotation::RotatingBody;
    ///
    /// let body = RotatingBody::new(4.0, 5.0).unwrap();
    ///
    /// assert_eq!(body.rotational_kinetic_energy(), Some(50.0));
    /// ```
    #[must_use]
    pub fn rotational_kinetic_energy(&self) -> Option<f64> {
        rotational_kinetic_energy(self.moment_of_inertia, self.angular_velocity)
    }

    /// Computes angular acceleration from applied torque using `α = τ / I`.
    #[must_use]
    pub fn angular_acceleration_from_torque(&self, torque: f64) -> Option<f64> {
        angular_acceleration_from_torque(torque, self.moment_of_inertia)
    }
}

/// A scalar angular position and angular velocity pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularState {
    pub angular_position: f64,
    pub angular_velocity: f64,
}

impl AngularState {
    /// Creates an angular state when both values are finite.
    #[must_use]
    pub const fn new(angular_position: f64, angular_velocity: f64) -> Option<Self> {
        if !angular_position.is_finite() || !angular_velocity.is_finite() {
            return None;
        }

        Some(Self {
            angular_position,
            angular_velocity,
        })
    }

    /// Advances the state under constant angular acceleration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rotation::AngularState;
    ///
    /// let next = AngularState::new(1.0, 2.0)
    ///     .unwrap()
    ///     .advanced_by_constant_acceleration(3.0, 4.0)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     next,
    ///     AngularState {
    ///         angular_position: 33.0,
    ///         angular_velocity: 14.0,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn advanced_by_constant_acceleration(
        &self,
        angular_acceleration: f64,
        time: f64,
    ) -> Option<Self> {
        let displacement = angular_displacement(self.angular_velocity, angular_acceleration, time)?;
        let angular_velocity =
            final_angular_velocity(self.angular_velocity, angular_acceleration, time)?;
        let angular_position = finite_result(self.angular_position + displacement)?;

        Some(Self {
            angular_position,
            angular_velocity,
        })
    }
}

/// Converts degrees to radians.
///
/// Returns `None` when the input or result is not finite.
#[must_use]
pub fn radians_from_degrees(degrees: f64) -> Option<f64> {
    if !degrees.is_finite() {
        return None;
    }

    finite_result(degrees.to_radians())
}

/// Converts radians to degrees.
///
/// Returns `None` when the input or result is not finite.
#[must_use]
pub fn degrees_from_radians(radians: f64) -> Option<f64> {
    if !radians.is_finite() {
        return None;
    }

    finite_result(radians.to_degrees())
}

/// Converts radians to revolutions.
///
/// Returns `None` when the input or result is not finite.
#[must_use]
pub fn revolutions_from_radians(radians: f64) -> Option<f64> {
    if !radians.is_finite() {
        return None;
    }

    finite_result(radians / TAU)
}

/// Converts revolutions to radians.
///
/// Returns `None` when the input or result is not finite.
#[must_use]
pub fn radians_from_revolutions(revolutions: f64) -> Option<f64> {
    if !revolutions.is_finite() {
        return None;
    }

    finite_result(revolutions * TAU)
}

/// Computes angular velocity using `ω = Δθ / t`.
///
/// Returns `None` when `time` is less than or equal to zero, when any input is not finite, or
/// when the computed angular velocity is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::angular_velocity;
///
/// assert_eq!(angular_velocity(10.0, 2.0), Some(5.0));
/// ```
#[must_use]
pub fn angular_velocity(angular_displacement: f64, time: f64) -> Option<f64> {
    if !angular_displacement.is_finite() || !is_positive_finite(time) {
        return None;
    }

    finite_result(angular_displacement / time)
}

/// Computes angular acceleration using `α = (ω_final - ω_initial) / t`.
///
/// Returns `None` when `time` is less than or equal to zero, when any input is not finite, or
/// when the computed angular acceleration is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::angular_acceleration;
///
/// assert_eq!(angular_acceleration(2.0, 10.0, 4.0), Some(2.0));
/// ```
#[must_use]
pub fn angular_acceleration(
    initial_angular_velocity: f64,
    final_angular_velocity: f64,
    time: f64,
) -> Option<f64> {
    if !initial_angular_velocity.is_finite()
        || !final_angular_velocity.is_finite()
        || !is_positive_finite(time)
    {
        return None;
    }

    finite_result((final_angular_velocity - initial_angular_velocity) / time)
}

/// Computes final angular velocity using `ω_final = ω_initial + αt`.
///
/// Returns `None` when `time` is negative, when any input is not finite, or when the computed
/// angular velocity is not finite.
#[must_use]
pub fn final_angular_velocity(
    initial_angular_velocity: f64,
    angular_acceleration: f64,
    time: f64,
) -> Option<f64> {
    if !initial_angular_velocity.is_finite()
        || !angular_acceleration.is_finite()
        || !is_nonnegative_finite(time)
    {
        return None;
    }

    finite_result(angular_acceleration.mul_add(time, initial_angular_velocity))
}

/// Computes angular displacement using `θ = ω_initial * t + 0.5 * α * t²`.
///
/// Returns `None` when `time` is negative, when any input is not finite, or when the computed
/// angular displacement is not finite.
#[must_use]
pub fn angular_displacement(
    initial_angular_velocity: f64,
    angular_acceleration: f64,
    time: f64,
) -> Option<f64> {
    if !initial_angular_velocity.is_finite()
        || !angular_acceleration.is_finite()
        || !is_nonnegative_finite(time)
    {
        return None;
    }

    let acceleration_term = 0.5 * angular_acceleration * time * time;

    finite_result(initial_angular_velocity.mul_add(time, acceleration_term))
}

/// Computes squared final angular velocity using `ω_final² = ω_initial² + 2αθ`.
///
/// Returns `None` when any input is not finite, when the computed squared value is negative, or
/// when the computed squared value is not finite.
#[must_use]
pub fn final_angular_velocity_squared(
    initial_angular_velocity: f64,
    angular_acceleration: f64,
    angular_displacement: f64,
) -> Option<f64> {
    if !initial_angular_velocity.is_finite()
        || !angular_acceleration.is_finite()
        || !angular_displacement.is_finite()
    {
        return None;
    }

    let squared = initial_angular_velocity.mul_add(
        initial_angular_velocity,
        2.0 * angular_acceleration * angular_displacement,
    );

    if !squared.is_finite() || squared < 0.0 {
        return None;
    }

    Some(squared)
}

/// Computes final angular velocity using `ω_final = sqrt(ω_initial² + 2αθ)`.
///
/// Returns `None` when the squared value is negative, when any input is not finite, or when the
/// computed angular velocity is not finite.
#[must_use]
pub fn final_angular_velocity_from_displacement(
    initial_angular_velocity: f64,
    angular_acceleration: f64,
    angular_displacement: f64,
) -> Option<f64> {
    let squared = final_angular_velocity_squared(
        initial_angular_velocity,
        angular_acceleration,
        angular_displacement,
    )?;

    finite_result(squared.sqrt())
}

/// Computes tangential speed using `v = ωr`.
///
/// Returns `None` when `radius` is negative, when any input is not finite, or when the computed
/// tangential speed is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::tangential_speed;
///
/// assert_eq!(tangential_speed(3.0, 2.0), Some(6.0));
/// ```
#[must_use]
pub fn tangential_speed(angular_velocity: f64, radius: f64) -> Option<f64> {
    if !angular_velocity.is_finite() || !is_nonnegative_finite(radius) {
        return None;
    }

    finite_result(angular_velocity * radius)
}

/// Computes angular velocity from tangential speed using `ω = v / r`.
///
/// Returns `None` when `radius` is less than or equal to zero, when any input is not finite, or
/// when the computed angular velocity is not finite.
#[must_use]
pub fn angular_velocity_from_tangential_speed(tangential_speed: f64, radius: f64) -> Option<f64> {
    if !tangential_speed.is_finite() || !is_positive_finite(radius) {
        return None;
    }

    finite_result(tangential_speed / radius)
}

/// Computes tangential acceleration using `a_t = αr`.
///
/// Returns `None` when `radius` is negative, when any input is not finite, or when the computed
/// tangential acceleration is not finite.
#[must_use]
pub fn tangential_acceleration(angular_acceleration: f64, radius: f64) -> Option<f64> {
    if !angular_acceleration.is_finite() || !is_nonnegative_finite(radius) {
        return None;
    }

    finite_result(angular_acceleration * radius)
}

/// Computes centripetal acceleration using `a_c = ω²r`.
///
/// Returns `None` when `radius` is negative, when any input is not finite, or when the computed
/// acceleration is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::centripetal_acceleration_from_angular_velocity;
///
/// assert_eq!(centripetal_acceleration_from_angular_velocity(3.0, 2.0), Some(18.0));
/// ```
#[must_use]
pub fn centripetal_acceleration_from_angular_velocity(
    angular_velocity: f64,
    radius: f64,
) -> Option<f64> {
    if !angular_velocity.is_finite() || !is_nonnegative_finite(radius) {
        return None;
    }

    let acceleration = angular_velocity * angular_velocity * radius;
    if acceleration < 0.0 {
        return None;
    }

    finite_result(acceleration)
}

/// Computes centripetal acceleration using `a_c = v² / r`.
///
/// Returns `None` when `radius` is less than or equal to zero, when any input is not finite, or
/// when the computed acceleration is not finite.
#[must_use]
pub fn centripetal_acceleration_from_tangential_speed(
    tangential_speed: f64,
    radius: f64,
) -> Option<f64> {
    if !tangential_speed.is_finite() || !is_positive_finite(radius) {
        return None;
    }

    let acceleration = tangential_speed * tangential_speed / radius;
    if acceleration < 0.0 {
        return None;
    }

    finite_result(acceleration)
}

/// Computes point-mass moment of inertia using `I = mr²`.
///
/// Returns `None` when `mass` or `radius` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn point_mass_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 1.0)
}

/// Computes solid-disk moment of inertia using `I = 0.5mr²`.
///
/// Returns `None` when `mass` or `radius` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::solid_disk_moment_of_inertia;
///
/// assert_eq!(solid_disk_moment_of_inertia(2.0, 3.0), Some(9.0));
/// ```
#[must_use]
pub fn solid_disk_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 0.5)
}

/// Computes thin-ring moment of inertia using `I = mr²`.
///
/// Returns `None` when `mass` or `radius` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn thin_ring_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    point_mass_moment_of_inertia(mass, radius)
}

/// Computes solid-sphere moment of inertia using `I = (2 / 5)mr²`.
///
/// Returns `None` when `mass` or `radius` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn solid_sphere_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 2.0 / 5.0)
}

/// Computes hollow-sphere moment of inertia using `I = (2 / 3)mr²`.
///
/// Returns `None` when `mass` or `radius` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn hollow_sphere_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    scaled_square_measure(mass, radius, 2.0 / 3.0)
}

/// Computes rod moment of inertia about its center using `I = (1 / 12)mL²`.
///
/// Returns `None` when `mass` or `length` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn rod_moment_of_inertia_about_center(mass: f64, length: f64) -> Option<f64> {
    scaled_square_measure(mass, length, 1.0 / 12.0)
}

/// Computes rod moment of inertia about one end using `I = (1 / 3)mL²`.
///
/// Returns `None` when `mass` or `length` is negative, when any input is not finite, or when the
/// computed moment of inertia is not finite.
#[must_use]
pub fn rod_moment_of_inertia_about_end(mass: f64, length: f64) -> Option<f64> {
    scaled_square_measure(mass, length, 1.0 / 3.0)
}

/// Computes angular momentum using `L = Iω`.
///
/// Returns `None` when `moment_of_inertia` is negative, when any input is not finite, or when the
/// computed angular momentum is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::angular_momentum;
///
/// assert_eq!(angular_momentum(4.0, 5.0), Some(20.0));
/// ```
#[must_use]
pub fn angular_momentum(moment_of_inertia: f64, angular_velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(moment_of_inertia) || !angular_velocity.is_finite() {
        return None;
    }

    finite_result(moment_of_inertia * angular_velocity)
}

/// Computes angular velocity from angular momentum using `ω = L / I`.
///
/// Returns `None` when `moment_of_inertia` is less than or equal to zero, when any input is not
/// finite, or when the computed angular velocity is not finite.
#[must_use]
pub fn angular_velocity_from_angular_momentum(
    angular_momentum: f64,
    moment_of_inertia: f64,
) -> Option<f64> {
    if !angular_momentum.is_finite() || !is_positive_finite(moment_of_inertia) {
        return None;
    }

    finite_result(angular_momentum / moment_of_inertia)
}

/// Computes rotational kinetic energy using `KE_rot = 0.5 * I * ω²`.
///
/// Returns `None` when `moment_of_inertia` is negative, when any input is not finite, or when the
/// computed kinetic energy is not finite.
///
/// # Examples
///
/// ```rust
/// use use_rotation::rotational_kinetic_energy;
///
/// assert_eq!(rotational_kinetic_energy(4.0, 5.0), Some(50.0));
/// ```
#[must_use]
pub fn rotational_kinetic_energy(moment_of_inertia: f64, angular_velocity: f64) -> Option<f64> {
    if !is_nonnegative_finite(moment_of_inertia) || !angular_velocity.is_finite() {
        return None;
    }

    let energy = 0.5 * moment_of_inertia * angular_velocity * angular_velocity;
    if energy < 0.0 {
        return None;
    }

    finite_result(energy)
}

/// Computes angular velocity from rotational kinetic energy using `ω = sqrt(2KE / I)`.
///
/// Returns the non-negative principal value when successful. Returns `None` when rotational
/// kinetic energy is negative, when `moment_of_inertia` is less than or equal to zero, when any
/// input is not finite, or when the computed angular velocity is not finite.
#[must_use]
pub fn angular_velocity_from_rotational_kinetic_energy(
    rotational_kinetic_energy: f64,
    moment_of_inertia: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(rotational_kinetic_energy) || !is_positive_finite(moment_of_inertia) {
        return None;
    }

    let squared = 2.0 * rotational_kinetic_energy / moment_of_inertia;
    if !squared.is_finite() || squared < 0.0 {
        return None;
    }

    finite_result(squared.sqrt())
}

/// Computes angular acceleration from torque using `α = τ / I`.
///
/// Returns `None` when `moment_of_inertia` is less than or equal to zero, when any input is not
/// finite, or when the computed angular acceleration is not finite.
///
/// For broader torque-specific scalar helpers such as lever-arm and balancing relations, prefer
/// `use-torque`.
#[must_use]
pub fn angular_acceleration_from_torque(torque: f64, moment_of_inertia: f64) -> Option<f64> {
    if !torque.is_finite() || !is_positive_finite(moment_of_inertia) {
        return None;
    }

    finite_result(torque / moment_of_inertia)
}

fn scaled_square_measure(primary: f64, measure: f64, factor: f64) -> Option<f64> {
    if !is_nonnegative_finite(primary) || !is_nonnegative_finite(measure) {
        return None;
    }

    finite_result(factor * primary * measure * measure)
}

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        AngularState, RotatingBody, angular_acceleration, angular_acceleration_from_torque,
        angular_displacement, angular_momentum, angular_velocity,
        angular_velocity_from_angular_momentum, angular_velocity_from_rotational_kinetic_energy,
        angular_velocity_from_tangential_speed, centripetal_acceleration_from_angular_velocity,
        centripetal_acceleration_from_tangential_speed, degrees_from_radians,
        final_angular_velocity, final_angular_velocity_from_displacement,
        final_angular_velocity_squared, hollow_sphere_moment_of_inertia,
        point_mass_moment_of_inertia, radians_from_degrees, radians_from_revolutions,
        revolutions_from_radians, rod_moment_of_inertia_about_center,
        rod_moment_of_inertia_about_end, rotational_kinetic_energy, solid_disk_moment_of_inertia,
        solid_sphere_moment_of_inertia, tangential_acceleration, tangential_speed,
        thin_ring_moment_of_inertia,
    };
    use core::f64::consts::{PI, TAU};

    const EPSILON: f64 = 1.0e-12;

    fn assert_approx_eq(left: f64, right: f64) {
        assert!(
            (left - right).abs() <= EPSILON,
            "left={left}, right={right}"
        );
    }

    fn assert_some_approx_eq(value: Option<f64>, expected: f64) {
        assert_approx_eq(value.expect("expected Some value"), expected);
    }

    #[test]
    fn angular_conversions_cover_common_values() {
        assert_some_approx_eq(radians_from_degrees(180.0), PI);
        assert_some_approx_eq(degrees_from_radians(PI), 180.0);
        assert_some_approx_eq(revolutions_from_radians(2.0 * PI), 1.0);
        assert_some_approx_eq(radians_from_revolutions(1.0), TAU);
    }

    #[test]
    fn angular_velocity_requires_positive_time() {
        assert_eq!(angular_velocity(10.0, 2.0), Some(5.0));
        assert_eq!(angular_velocity(10.0, 0.0), None);
    }

    #[test]
    fn angular_acceleration_requires_positive_time() {
        assert_eq!(angular_acceleration(2.0, 10.0, 4.0), Some(2.0));
        assert_eq!(angular_acceleration(2.0, 10.0, 0.0), None);
    }

    #[test]
    fn final_angular_velocity_requires_nonnegative_time() {
        assert_eq!(final_angular_velocity(2.0, 3.0, 4.0), Some(14.0));
        assert_eq!(final_angular_velocity(2.0, 3.0, -1.0), None);
    }

    #[test]
    fn angular_displacement_requires_nonnegative_time() {
        assert_eq!(angular_displacement(2.0, 3.0, 4.0), Some(32.0));
        assert_eq!(angular_displacement(2.0, 3.0, -1.0), None);
    }

    #[test]
    fn displacement_based_kinematics_cover_common_values() {
        assert_eq!(final_angular_velocity_squared(2.0, 3.0, 4.0), Some(28.0));
        assert_some_approx_eq(
            final_angular_velocity_from_displacement(2.0, 3.0, 4.0),
            28.0_f64.sqrt(),
        );
    }

    #[test]
    fn tangential_and_centripetal_relations_cover_common_values() {
        assert_eq!(tangential_speed(3.0, 2.0), Some(6.0));
        assert_eq!(tangential_speed(3.0, -2.0), None);
        assert_eq!(angular_velocity_from_tangential_speed(6.0, 2.0), Some(3.0));
        assert_eq!(angular_velocity_from_tangential_speed(6.0, 0.0), None);
        assert_eq!(tangential_acceleration(3.0, 2.0), Some(6.0));
        assert_eq!(
            centripetal_acceleration_from_angular_velocity(3.0, 2.0),
            Some(18.0)
        );
        assert_eq!(
            centripetal_acceleration_from_tangential_speed(6.0, 2.0),
            Some(18.0)
        );
    }

    #[test]
    fn moment_of_inertia_helpers_cover_common_shapes() {
        assert_eq!(point_mass_moment_of_inertia(2.0, 3.0), Some(18.0));
        assert_eq!(solid_disk_moment_of_inertia(2.0, 3.0), Some(9.0));
        assert_eq!(thin_ring_moment_of_inertia(2.0, 3.0), Some(18.0));
        assert_eq!(solid_sphere_moment_of_inertia(5.0, 2.0), Some(8.0));
        assert_eq!(hollow_sphere_moment_of_inertia(3.0, 2.0), Some(8.0));
        assert_eq!(rod_moment_of_inertia_about_center(12.0, 2.0), Some(4.0));
        assert_eq!(rod_moment_of_inertia_about_end(3.0, 2.0), Some(4.0));
    }

    #[test]
    fn angular_momentum_helpers_cover_common_values() {
        assert_eq!(angular_momentum(4.0, 5.0), Some(20.0));
        assert_eq!(angular_momentum(-4.0, 5.0), None);
        assert_eq!(angular_velocity_from_angular_momentum(20.0, 4.0), Some(5.0));
        assert_eq!(angular_velocity_from_angular_momentum(20.0, 0.0), None);
    }

    #[test]
    fn rotational_energy_helpers_cover_common_values() {
        assert_eq!(rotational_kinetic_energy(4.0, 5.0), Some(50.0));
        assert_eq!(rotational_kinetic_energy(-4.0, 5.0), None);
        assert_eq!(
            angular_velocity_from_rotational_kinetic_energy(50.0, 4.0),
            Some(5.0)
        );
        assert_eq!(
            angular_velocity_from_rotational_kinetic_energy(-50.0, 4.0),
            None
        );
    }

    #[test]
    fn angular_acceleration_from_torque_requires_positive_inertia() {
        assert_eq!(angular_acceleration_from_torque(20.0, 4.0), Some(5.0));
        assert_eq!(angular_acceleration_from_torque(20.0, 0.0), None);
    }

    #[test]
    fn rotating_body_delegates_to_public_functions() {
        let body = RotatingBody::new(4.0, 5.0).expect("expected valid rotating body");

        assert_eq!(body.angular_momentum(), Some(20.0));
        assert_eq!(body.rotational_kinetic_energy(), Some(50.0));
        assert_eq!(RotatingBody::new(-4.0, 5.0), None);
    }

    #[test]
    fn angular_state_advances_with_constant_acceleration() {
        let next = AngularState::new(1.0, 2.0)
            .expect("expected valid angular state")
            .advanced_by_constant_acceleration(3.0, 4.0)
            .expect("expected advanced state");

        assert_eq!(
            next,
            AngularState {
                angular_position: 33.0,
                angular_velocity: 14.0,
            }
        );
        assert_eq!(AngularState::new(f64::NAN, 2.0), None);
    }
}
