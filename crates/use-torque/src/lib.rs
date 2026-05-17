#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Torque-specific scalar helpers.

/// Re-exports for ergonomic glob imports.
pub mod prelude;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// Computes torque from a force and lever arm.
///
/// Formula: `τ = F * r`
///
/// Returns `None` when either input is not finite or when the computed result is not finite.
/// Negative forces and lever arms are allowed.
///
/// # Examples
///
/// ```rust
/// use use_torque::torque;
///
/// assert_eq!(torque(10.0, 2.0), Some(20.0));
/// ```
#[must_use]
pub fn torque(force: f64, lever_arm: f64) -> Option<f64> {
    if !force.is_finite() || !lever_arm.is_finite() {
        return None;
    }

    finite(force * lever_arm)
}

/// Computes torque when the applied force meets the lever arm at an angle in radians.
///
/// Formula: `τ = r * F * sin(theta)`
///
/// Returns `None` when any input is not finite or when the computed result is not finite.
/// Negative forces and lever arms are allowed.
///
/// # Examples
///
/// ```rust
/// use core::f64::consts::FRAC_PI_2;
/// use use_torque::torque_at_angle;
///
/// let applied = torque_at_angle(10.0, 2.0, FRAC_PI_2).unwrap();
///
/// assert!((applied - 20.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn torque_at_angle(force: f64, lever_arm: f64, angle_radians: f64) -> Option<f64> {
    if !force.is_finite() || !lever_arm.is_finite() || !angle_radians.is_finite() {
        return None;
    }

    finite(lever_arm * force * angle_radians.sin())
}

/// Computes torque when the applied force meets the lever arm at an angle in degrees.
///
/// This helper converts degrees to radians internally and delegates to [`torque_at_angle`].
#[must_use]
pub fn torque_at_angle_degrees(force: f64, lever_arm: f64, angle_degrees: f64) -> Option<f64> {
    if !angle_degrees.is_finite() {
        return None;
    }

    torque_at_angle(force, lever_arm, angle_degrees.to_radians())
}

/// Computes the force required to produce a known torque at a lever arm.
///
/// Formula: `F = τ / r`
///
/// Returns `None` when `lever_arm` is zero, when either input is not finite, or when the
/// computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_torque::force_from_torque;
///
/// assert_eq!(force_from_torque(20.0, 2.0), Some(10.0));
/// ```
#[must_use]
pub fn force_from_torque(applied_torque: f64, lever_arm: f64) -> Option<f64> {
    if !applied_torque.is_finite() || !lever_arm.is_finite() || lever_arm == 0.0 {
        return None;
    }

    finite(applied_torque / lever_arm)
}

/// Computes the lever arm required to produce a known torque from a force.
///
/// Formula: `r = τ / F`
///
/// Returns `None` when `force` is zero, when either input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn lever_arm_from_torque(applied_torque: f64, force: f64) -> Option<f64> {
    if !applied_torque.is_finite() || !force.is_finite() || force == 0.0 {
        return None;
    }

    finite(applied_torque / force)
}

/// Computes the component of a force that acts perpendicular to a lever arm.
///
/// Formula: `F_perp = F * sin(theta)`
///
/// Returns `None` when either input is not finite or when the computed result is not finite.
/// Negative forces are allowed.
#[must_use]
pub fn perpendicular_force_component(force: f64, angle_radians: f64) -> Option<f64> {
    if !force.is_finite() || !angle_radians.is_finite() {
        return None;
    }

    finite(force * angle_radians.sin())
}

/// Computes the perpendicular force component from an angle given in degrees.
///
/// This helper converts degrees to radians internally and delegates to
/// [`perpendicular_force_component`].
#[must_use]
pub fn perpendicular_force_component_degrees(force: f64, angle_degrees: f64) -> Option<f64> {
    if !angle_degrees.is_finite() {
        return None;
    }

    perpendicular_force_component(force, angle_degrees.to_radians())
}

/// Computes the perpendicular moment arm from a lever arm and angle in radians.
///
/// Formula: `r_perp = r * sin(theta)`
///
/// Returns `None` when either input is not finite or when the computed result is not finite.
/// Negative lever arms are allowed.
#[must_use]
pub fn moment_arm(lever_arm: f64, angle_radians: f64) -> Option<f64> {
    if !lever_arm.is_finite() || !angle_radians.is_finite() {
        return None;
    }

    finite(lever_arm * angle_radians.sin())
}

/// Computes the perpendicular moment arm from an angle given in degrees.
///
/// This helper converts degrees to radians internally and delegates to [`moment_arm`].
#[must_use]
pub fn moment_arm_degrees(lever_arm: f64, angle_degrees: f64) -> Option<f64> {
    if !angle_degrees.is_finite() {
        return None;
    }

    moment_arm(lever_arm, angle_degrees.to_radians())
}

/// Computes the sum of a slice of torque values.
///
/// Returns `Some(0.0)` for an empty slice, `None` when any input is not finite, or `None` when
/// the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_torque::net_torque;
///
/// assert_eq!(net_torque(&[10.0, -4.0, 2.0]), Some(8.0));
/// ```
#[must_use]
pub fn net_torque(torques: &[f64]) -> Option<f64> {
    if !all_finite(torques) {
        return None;
    }

    finite(torques.iter().copied().sum())
}

/// Converts force and lever-arm pairs into torque values.
///
/// Returns `Some(vec![])` for an empty slice or `None` when any pair is invalid.
#[must_use]
pub fn torques_from_force_lever_pairs(pairs: &[(f64, f64)]) -> Option<Vec<f64>> {
    pairs
        .iter()
        .map(|(force, lever_arm)| torque(*force, *lever_arm))
        .collect()
}

/// Computes the net torque for force and lever-arm pairs.
///
/// This helper delegates to [`torques_from_force_lever_pairs`] and then to [`net_torque`].
#[must_use]
pub fn net_torque_from_force_lever_pairs(pairs: &[(f64, f64)]) -> Option<f64> {
    let torques = torques_from_force_lever_pairs(pairs)?;

    net_torque(&torques)
}

/// Checks whether a torque system is in rotational equilibrium.
///
/// Returns `None` when `tolerance` is negative or not finite, when any torque is not finite, or
/// when the net torque cannot be computed as a finite value.
///
/// # Examples
///
/// ```rust
/// use use_torque::is_rotational_equilibrium;
///
/// assert_eq!(is_rotational_equilibrium(&[10.0, -10.0], 1.0e-6), Some(true));
/// ```
#[must_use]
pub fn is_rotational_equilibrium(torques: &[f64], tolerance: f64) -> Option<bool> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return None;
    }

    let total = net_torque(torques)?;

    Some(total.abs() <= tolerance)
}

/// Computes the balancing force needed at a lever arm to cancel a known torque.
///
/// Formula: `F_balance = -τ_known / r`
///
/// Returns `None` when `lever_arm` is zero, when either input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn balancing_force(known_torque: f64, lever_arm: f64) -> Option<f64> {
    if !known_torque.is_finite() || !lever_arm.is_finite() || lever_arm == 0.0 {
        return None;
    }

    finite(-known_torque / lever_arm)
}

/// Computes the balancing lever arm needed for a force to cancel a known torque.
///
/// Formula: `r_balance = -τ_known / F`
///
/// Returns `None` when `force` is zero, when either input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn balancing_lever_arm(known_torque: f64, force: f64) -> Option<f64> {
    if !known_torque.is_finite() || !force.is_finite() || force == 0.0 {
        return None;
    }

    finite(-known_torque / force)
}

/// Computes angular acceleration from torque and moment of inertia.
///
/// Formula: `α = τ / I`
///
/// Returns `None` when `moment_of_inertia` is less than or equal to zero, when either input is
/// not finite, or when the computed result is not finite. Broader angular-motion helpers belong
/// in a future or separate `use-rotation` crate.
///
/// # Examples
///
/// ```rust
/// use use_torque::angular_acceleration_from_torque;
///
/// assert_eq!(angular_acceleration_from_torque(20.0, 4.0), Some(5.0));
/// ```
#[must_use]
pub fn angular_acceleration_from_torque(
    applied_torque: f64,
    moment_of_inertia: f64,
) -> Option<f64> {
    if !applied_torque.is_finite() || !moment_of_inertia.is_finite() || moment_of_inertia <= 0.0 {
        return None;
    }

    finite(applied_torque / moment_of_inertia)
}

/// Computes the moment of inertia for a point mass.
///
/// Formula: `I = m * r²`
///
/// Returns `None` when `mass` or `radius` is negative, when either input is not finite, or when
/// the computed result is not finite.
#[must_use]
pub fn point_mass_moment_of_inertia(mass: f64, radius: f64) -> Option<f64> {
    if !mass.is_finite() || !radius.is_finite() || mass < 0.0 || radius < 0.0 {
        return None;
    }

    finite(mass * radius.powi(2))
}

/// Computes the moment of inertia of a uniform rod about its center.
///
/// Formula: `I = (1 / 12) * m * L²`
///
/// Returns `None` when `mass` or `length` is negative, when either input is not finite, or when
/// the computed result is not finite.
#[must_use]
pub fn rod_moment_of_inertia_about_center(mass: f64, length: f64) -> Option<f64> {
    if !mass.is_finite() || !length.is_finite() || mass < 0.0 || length < 0.0 {
        return None;
    }

    finite((mass * length.powi(2)) / 12.0)
}

/// Computes the moment of inertia of a uniform rod about one end.
///
/// Formula: `I = (1 / 3) * m * L²`
///
/// Returns `None` when `mass` or `length` is negative, when either input is not finite, or when
/// the computed result is not finite.
#[must_use]
pub fn rod_moment_of_inertia_about_end(mass: f64, length: f64) -> Option<f64> {
    if !mass.is_finite() || !length.is_finite() || mass < 0.0 || length < 0.0 {
        return None;
    }

    finite((mass * length.powi(2)) / 3.0)
}

/// A force applied at a lever arm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeverForce {
    /// Applied force in newtons.
    pub force: f64,
    /// Lever arm in meters.
    pub lever_arm: f64,
}

impl LeverForce {
    /// Creates a lever-force pair from finite values.
    ///
    /// Returns `None` when either input is not finite.
    #[must_use]
    pub const fn new(force: f64, lever_arm: f64) -> Option<Self> {
        if !force.is_finite() || !lever_arm.is_finite() {
            return None;
        }

        Some(Self { force, lever_arm })
    }

    /// Computes torque for this force and lever arm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_torque::LeverForce;
    ///
    /// let lever = LeverForce::new(10.0, 2.0).unwrap();
    ///
    /// assert_eq!(lever.torque(), Some(20.0));
    /// ```
    #[must_use]
    pub fn torque(&self) -> Option<f64> {
        torque(self.force, self.lever_arm)
    }

    /// Computes torque for this force and lever arm at an angle in radians.
    #[must_use]
    pub fn torque_at_angle(&self, angle_radians: f64) -> Option<f64> {
        torque_at_angle(self.force, self.lever_arm, angle_radians)
    }
}

/// A collection of torque values that can be analyzed as a system.
#[derive(Debug, Clone, PartialEq)]
pub struct TorqueSystem {
    /// Torque values in newton-meters.
    pub torques: Vec<f64>,
}

impl TorqueSystem {
    /// Creates a torque system from finite torque values.
    ///
    /// Returns `None` when any torque is not finite.
    #[must_use]
    pub fn new(torques: Vec<f64>) -> Option<Self> {
        all_finite(&torques).then_some(Self { torques })
    }

    /// Computes the system's net torque.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_torque::TorqueSystem;
    ///
    /// let system = TorqueSystem::new(vec![10.0, -4.0, 2.0]).unwrap();
    ///
    /// assert_eq!(system.net_torque(), Some(8.0));
    /// ```
    #[must_use]
    pub fn net_torque(&self) -> Option<f64> {
        net_torque(&self.torques)
    }

    /// Checks whether the system is in rotational equilibrium for the provided tolerance.
    #[must_use]
    pub fn is_equilibrium(&self, tolerance: f64) -> Option<bool> {
        is_rotational_equilibrium(&self.torques, tolerance)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use core::f64;
    use core::f64::consts::FRAC_PI_2;

    use super::{
        LeverForce, TorqueSystem, angular_acceleration_from_torque, balancing_force,
        balancing_lever_arm, force_from_torque, is_rotational_equilibrium, lever_arm_from_torque,
        moment_arm, moment_arm_degrees, net_torque, net_torque_from_force_lever_pairs,
        perpendicular_force_component, perpendicular_force_component_degrees,
        point_mass_moment_of_inertia, rod_moment_of_inertia_about_center,
        rod_moment_of_inertia_about_end, torque, torque_at_angle, torque_at_angle_degrees,
        torques_from_force_lever_pairs,
    };

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        let delta = (left - right).abs();

        assert!(
            delta <= tolerance,
            "left={left} right={right} delta={delta} tolerance={tolerance}"
        );
    }

    #[test]
    fn torque_helpers_handle_signed_inputs() {
        assert_eq!(torque(10.0, 2.0), Some(20.0));
        assert_eq!(torque(-10.0, 2.0), Some(-20.0));
        assert_eq!(torque(10.0, -2.0), Some(-20.0));
    }

    #[test]
    fn torque_angle_helpers_match_expected_values() {
        approx_eq(
            torque_at_angle(10.0, 2.0, FRAC_PI_2).unwrap(),
            20.0,
            1.0e-12,
        );
        approx_eq(torque_at_angle(10.0, 2.0, 0.0).unwrap(), 0.0, 1.0e-12);
        approx_eq(
            torque_at_angle_degrees(10.0, 2.0, 90.0).unwrap(),
            20.0,
            1.0e-12,
        );
        approx_eq(
            torque_at_angle_degrees(10.0, 2.0, 30.0).unwrap(),
            10.0,
            1.0e-12,
        );
    }

    #[test]
    fn inverse_helpers_validate_denominators() {
        assert_eq!(force_from_torque(20.0, 2.0), Some(10.0));
        assert_eq!(force_from_torque(20.0, 0.0), None);
        assert_eq!(lever_arm_from_torque(20.0, 10.0), Some(2.0));
        assert_eq!(lever_arm_from_torque(20.0, 0.0), None);
    }

    #[test]
    fn perpendicular_helpers_compute_expected_components() {
        approx_eq(
            perpendicular_force_component(10.0, FRAC_PI_2).unwrap(),
            10.0,
            1.0e-12,
        );
        approx_eq(
            perpendicular_force_component_degrees(10.0, 30.0).unwrap(),
            5.0,
            1.0e-12,
        );
        approx_eq(moment_arm(2.0, FRAC_PI_2).unwrap(), 2.0, 1.0e-12);
        approx_eq(moment_arm_degrees(2.0, 30.0).unwrap(), 1.0, 1.0e-12);
    }

    #[test]
    fn net_torque_helpers_sum_systems_and_pairs() {
        assert_eq!(net_torque(&[10.0, -4.0, 2.0]), Some(8.0));
        assert_eq!(net_torque(&[]), Some(0.0));
        assert_eq!(net_torque(&[10.0, f64::NAN]), None);
        assert_eq!(
            torques_from_force_lever_pairs(&[(10.0, 2.0), (-3.0, 4.0)]),
            Some(vec![20.0, -12.0])
        );
        assert_eq!(
            net_torque_from_force_lever_pairs(&[(10.0, 2.0), (-3.0, 4.0)]),
            Some(8.0)
        );
    }

    #[test]
    fn rotational_equilibrium_helpers_balance_known_torques() {
        assert_eq!(
            is_rotational_equilibrium(&[10.0, -10.0], 0.000_001),
            Some(true)
        );
        assert_eq!(
            is_rotational_equilibrium(&[10.0, -9.0], 0.000_001),
            Some(false)
        );
        assert_eq!(is_rotational_equilibrium(&[10.0, -10.0], -1.0), None);
        assert_eq!(balancing_force(20.0, 2.0), Some(-10.0));
        assert_eq!(balancing_force(20.0, 0.0), None);
        assert_eq!(balancing_lever_arm(20.0, 10.0), Some(-2.0));
        assert_eq!(balancing_lever_arm(20.0, 0.0), None);
    }

    #[test]
    fn angular_acceleration_validates_positive_inertia() {
        assert_eq!(angular_acceleration_from_torque(20.0, 4.0), Some(5.0));
        assert_eq!(angular_acceleration_from_torque(20.0, 0.0), None);
    }

    #[test]
    fn moment_of_inertia_helpers_cover_basic_shapes() {
        assert_eq!(point_mass_moment_of_inertia(2.0, 3.0), Some(18.0));
        assert_eq!(point_mass_moment_of_inertia(-2.0, 3.0), None);
        assert_eq!(point_mass_moment_of_inertia(2.0, -3.0), None);
        assert_eq!(rod_moment_of_inertia_about_center(12.0, 2.0), Some(4.0));
        assert_eq!(rod_moment_of_inertia_about_end(3.0, 2.0), Some(4.0));
    }

    #[test]
    fn lever_force_validates_and_delegates() {
        let lever = LeverForce::new(10.0, 2.0).unwrap();

        assert_eq!(lever.torque(), Some(20.0));
        approx_eq(lever.torque_at_angle(FRAC_PI_2).unwrap(), 20.0, 1.0e-12);
        assert_eq!(LeverForce::new(f64::NAN, 2.0), None);
    }

    #[test]
    fn torque_system_validates_and_delegates() {
        let system = TorqueSystem::new(vec![10.0, -4.0, 2.0]).unwrap();

        assert_eq!(system.net_torque(), Some(8.0));
        assert_eq!(TorqueSystem::new(vec![10.0, f64::NAN]), None);
        assert_eq!(
            TorqueSystem::new(vec![10.0, -10.0])
                .unwrap()
                .is_equilibrium(0.000_001),
            Some(true)
        );
    }
}
