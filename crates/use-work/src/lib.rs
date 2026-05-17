#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Mechanical work helpers.

pub mod prelude;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// Computes mechanical work from a constant force and displacement.
///
/// Formula: `W = F * d`
///
/// Returns `None` when either input is not finite or when the computed result is not finite.
/// Negative force and displacement values are allowed.
///
/// # Examples
///
/// ```rust
/// use use_work::work;
///
/// assert_eq!(work(10.0, 2.0), Some(20.0));
/// assert_eq!(work(-10.0, 2.0), Some(-20.0));
/// ```
#[must_use]
pub fn work(force: f64, displacement: f64) -> Option<f64> {
    if !force.is_finite() || !displacement.is_finite() {
        return None;
    }

    finite(force * displacement)
}

/// Computes mechanical work when the force is applied at an angle to the displacement.
///
/// Formula: `W = F * d * cos(theta)`
///
/// Returns `None` when any input is not finite or when the computed result is not finite.
/// The `angle_radians` input is interpreted in radians.
///
/// # Examples
///
/// ```rust
/// use use_work::work_at_angle;
///
/// let result = work_at_angle(10.0, 2.0, 0.0).unwrap();
///
/// assert_eq!(result, 20.0);
/// ```
#[must_use]
pub fn work_at_angle(force: f64, displacement: f64, angle_radians: f64) -> Option<f64> {
    if !force.is_finite() || !displacement.is_finite() || !angle_radians.is_finite() {
        return None;
    }

    finite(force * displacement * angle_radians.cos())
}

/// Computes mechanical work when the applied-force angle is given in degrees.
///
/// This function converts `angle_degrees` to radians internally and then delegates to
/// [`work_at_angle`].
#[must_use]
pub fn work_at_angle_degrees(force: f64, displacement: f64, angle_degrees: f64) -> Option<f64> {
    work_at_angle(force, displacement, angle_degrees.to_radians())
}

/// Computes the force required to perform a given amount of work over a displacement.
///
/// Formula: `F = W / d`
///
/// Returns `None` when `displacement` is zero, when either input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn force_from_work(work: f64, displacement: f64) -> Option<f64> {
    if !work.is_finite() || !displacement.is_finite() || displacement == 0.0 {
        return None;
    }

    finite(work / displacement)
}

/// Computes the displacement implied by a work value and a constant force.
///
/// Formula: `d = W / F`
///
/// Returns `None` when `force` is zero, when either input is not finite, or when the computed
/// result is not finite.
#[must_use]
pub fn displacement_from_work(work: f64, force: f64) -> Option<f64> {
    if !work.is_finite() || !force.is_finite() || force == 0.0 {
        return None;
    }

    finite(work / force)
}

/// Computes the net work from a slice of work contributions.
///
/// Returns `Some(0.0)` for an empty slice. Returns `None` when any input is not finite or when
/// the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_work::net_work;
///
/// assert_eq!(net_work(&[10.0, -2.0, 5.0]), Some(13.0));
/// assert_eq!(net_work(&[]), Some(0.0));
/// ```
#[must_use]
pub fn net_work(works: &[f64]) -> Option<f64> {
    let mut total = 0.0;

    for &value in works {
        if !value.is_finite() {
            return None;
        }

        total += value;

        if !total.is_finite() {
            return None;
        }
    }

    Some(total)
}

/// Approximates work from aligned force and displacement samples.
///
/// Formula: `W = Σ(F_i * d_i)`
///
/// Returns `None` when the slice lengths differ, when any value is not finite, or when the
/// computed result is not finite. Returns `Some(0.0)` for two empty slices.
#[must_use]
pub fn work_from_force_samples(displacements: &[f64], forces: &[f64]) -> Option<f64> {
    if displacements.len() != forces.len() {
        return None;
    }

    let mut total = 0.0;

    for (&displacement, &force) in displacements.iter().zip(forces.iter()) {
        if !displacement.is_finite() || !force.is_finite() {
            return None;
        }

        total += force * displacement;

        if !total.is_finite() {
            return None;
        }
    }

    Some(total)
}

/// Computes net work from the change in kinetic energy.
///
/// Formula: `W_net = KE_final - KE_initial`
///
/// Returns `None` when either kinetic energy is negative, when any input is not finite, or when
/// the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_work::work_from_kinetic_energy_change;
///
/// assert_eq!(work_from_kinetic_energy_change(5.0, 12.0), Some(7.0));
/// assert_eq!(work_from_kinetic_energy_change(12.0, 5.0), Some(-7.0));
/// ```
#[must_use]
pub fn work_from_kinetic_energy_change(
    initial_kinetic_energy: f64,
    final_kinetic_energy: f64,
) -> Option<f64> {
    if !initial_kinetic_energy.is_finite()
        || !final_kinetic_energy.is_finite()
        || initial_kinetic_energy < 0.0
        || final_kinetic_energy < 0.0
    {
        return None;
    }

    finite(final_kinetic_energy - initial_kinetic_energy)
}

/// Computes final kinetic energy from an initial kinetic energy and applied work.
///
/// Formula: `KE_final = KE_initial + W`
///
/// Returns `None` when `initial_kinetic_energy` is negative, when any input is not finite, when
/// the computed result is negative, or when the computed result is not finite.
#[must_use]
pub fn final_kinetic_energy_from_work(initial_kinetic_energy: f64, work: f64) -> Option<f64> {
    if !initial_kinetic_energy.is_finite() || !work.is_finite() || initial_kinetic_energy < 0.0 {
        return None;
    }

    let result = initial_kinetic_energy + work;

    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes initial kinetic energy from a final kinetic energy and applied work.
///
/// Formula: `KE_initial = KE_final - W`
///
/// Returns `None` when `final_kinetic_energy` is negative, when any input is not finite, when
/// the computed result is negative, or when the computed result is not finite.
#[must_use]
pub fn initial_kinetic_energy_from_work(final_kinetic_energy: f64, work: f64) -> Option<f64> {
    if !final_kinetic_energy.is_finite() || !work.is_finite() || final_kinetic_energy < 0.0 {
        return None;
    }

    let result = final_kinetic_energy - work;

    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes work done by an ideal spring force over a displacement interval.
///
/// Formula: `W = 0.5 * k * (x_initial^2 - x_final^2)`
///
/// Returns `None` when `spring_constant` is negative, when any input is not finite, or when the
/// computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_work::spring_work;
///
/// assert_eq!(spring_work(100.0, 0.5, 0.0), Some(12.5));
/// assert_eq!(spring_work(100.0, 0.0, 0.5), Some(-12.5));
/// ```
#[must_use]
pub fn spring_work(
    spring_constant: f64,
    initial_displacement: f64,
    final_displacement: f64,
) -> Option<f64> {
    if !spring_constant.is_finite()
        || !initial_displacement.is_finite()
        || !final_displacement.is_finite()
        || spring_constant < 0.0
    {
        return None;
    }

    let initial_squared = initial_displacement * initial_displacement;
    let final_squared = final_displacement * final_displacement;

    finite(0.5 * spring_constant * (initial_squared - final_squared))
}

/// Computes the spring potential energy stored at a displacement.
///
/// Formula: `U = 0.5 * k * x^2`
///
/// Returns `None` when `spring_constant` is negative, when any input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn spring_potential_energy(spring_constant: f64, displacement: f64) -> Option<f64> {
    if !spring_constant.is_finite() || !displacement.is_finite() || spring_constant < 0.0 {
        return None;
    }

    finite(0.5 * spring_constant * displacement.powi(2))
}

/// Computes work done against gravity near a surface.
///
/// Formula: `W = m * g * h`
///
/// Returns `None` when `mass` is negative, when any input is not finite, or when the computed
/// result is not finite. Negative heights are allowed.
///
/// # Examples
///
/// ```rust
/// use use_work::work_against_gravity;
///
/// let work = work_against_gravity(2.0, 9.806_65, 10.0).unwrap();
///
/// assert!((work - 196.133).abs() < 1e-12);
/// ```
#[must_use]
pub fn work_against_gravity(
    mass: f64,
    gravitational_acceleration: f64,
    height: f64,
) -> Option<f64> {
    if !mass.is_finite()
        || !gravitational_acceleration.is_finite()
        || !height.is_finite()
        || mass < 0.0
    {
        return None;
    }

    finite(mass * gravitational_acceleration * height)
}

/// Computes work done by gravity near a surface.
///
/// Formula: `W = -m * g * Δh`
///
/// Returns `None` when `mass` is negative, when any input is not finite, or when the computed
/// result is not finite. Negative height changes are allowed.
#[must_use]
pub fn work_by_gravity(
    mass: f64,
    gravitational_acceleration: f64,
    height_change: f64,
) -> Option<f64> {
    if !mass.is_finite()
        || !gravitational_acceleration.is_finite()
        || !height_change.is_finite()
        || mass < 0.0
    {
        return None;
    }

    finite(-mass * gravitational_acceleration * height_change)
}

/// Computes work done by kinetic friction.
///
/// Formula: `W = -f_k * abs(d)`
///
/// Returns `None` when `friction_force_magnitude` is negative, when any input is not finite, or
/// when the computed result is not finite.
#[must_use]
pub fn work_by_friction(friction_force_magnitude: f64, displacement: f64) -> Option<f64> {
    if !friction_force_magnitude.is_finite()
        || !displacement.is_finite()
        || friction_force_magnitude < 0.0
    {
        return None;
    }

    finite(-friction_force_magnitude * displacement.abs())
}

/// Constant-force work inputs for repeated calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantForceWork {
    pub force: f64,
    pub displacement: f64,
}

impl ConstantForceWork {
    /// Creates a constant-force work helper from finite inputs.
    ///
    /// Returns `None` when either input is not finite.
    #[must_use]
    pub const fn new(force: f64, displacement: f64) -> Option<Self> {
        if !force.is_finite() || !displacement.is_finite() {
            return None;
        }

        Some(Self {
            force,
            displacement,
        })
    }

    /// Computes the work represented by this constant-force relationship.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_work::ConstantForceWork;
    ///
    /// let helper = ConstantForceWork::new(10.0, 2.0).unwrap();
    ///
    /// assert_eq!(helper.work(), Some(20.0));
    /// ```
    #[must_use]
    pub fn work(&self) -> Option<f64> {
        work(self.force, self.displacement)
    }

    /// Computes the work represented by this constant-force relationship at an angle.
    #[must_use]
    pub fn work_at_angle(&self, angle_radians: f64) -> Option<f64> {
        work_at_angle(self.force, self.displacement, angle_radians)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        ConstantForceWork, displacement_from_work, final_kinetic_energy_from_work, force_from_work,
        initial_kinetic_energy_from_work, net_work, spring_potential_energy, spring_work, work,
        work_against_gravity, work_at_angle, work_at_angle_degrees, work_by_friction,
        work_by_gravity, work_from_force_samples, work_from_kinetic_energy_change,
    };

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        let delta = (left - right).abs();

        assert!(
            delta <= tolerance,
            "left={left} right={right} delta={delta} tolerance={tolerance}"
        );
    }

    #[test]
    fn work_handles_basic_cases() {
        assert_eq!(work(10.0, 2.0), Some(20.0));
        assert_eq!(work(-10.0, 2.0), Some(-20.0));
        assert_eq!(work(10.0, -2.0), Some(-20.0));
    }

    #[test]
    fn angled_work_handles_radians_and_degrees() {
        assert_eq!(work_at_angle(10.0, 2.0, 0.0), Some(20.0));
        approx_eq(work_at_angle_degrees(10.0, 2.0, 60.0).unwrap(), 10.0, 1e-12);
        approx_eq(work_at_angle_degrees(10.0, 2.0, 90.0).unwrap(), 0.0, 1e-10);
    }

    #[test]
    fn inverse_helpers_require_non_zero_divisors() {
        assert_eq!(force_from_work(20.0, 2.0), Some(10.0));
        assert_eq!(force_from_work(20.0, 0.0), None);
        assert_eq!(displacement_from_work(20.0, 10.0), Some(2.0));
        assert_eq!(displacement_from_work(20.0, 0.0), None);
    }

    #[test]
    fn net_work_and_force_samples_cover_common_cases() {
        assert_eq!(net_work(&[10.0, -2.0, 5.0]), Some(13.0));
        assert_eq!(net_work(&[]), Some(0.0));
        assert_eq!(
            work_from_force_samples(&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]),
            Some(140.0)
        );
        assert_eq!(work_from_force_samples(&[1.0], &[10.0, 20.0]), None);
    }

    #[test]
    fn work_energy_relationships_cover_forward_and_inverse_paths() {
        assert_eq!(work_from_kinetic_energy_change(5.0, 12.0), Some(7.0));
        assert_eq!(work_from_kinetic_energy_change(12.0, 5.0), Some(-7.0));
        assert_eq!(work_from_kinetic_energy_change(-1.0, 5.0), None);

        assert_eq!(final_kinetic_energy_from_work(5.0, 7.0), Some(12.0));
        assert_eq!(final_kinetic_energy_from_work(5.0, -10.0), None);

        assert_eq!(initial_kinetic_energy_from_work(12.0, 7.0), Some(5.0));
        assert_eq!(initial_kinetic_energy_from_work(5.0, 10.0), None);
    }

    #[test]
    fn spring_helpers_cover_energy_and_work() {
        assert_eq!(spring_potential_energy(100.0, 0.5), Some(12.5));
        assert_eq!(spring_work(100.0, 0.5, 0.0), Some(12.5));
        assert_eq!(spring_work(100.0, 0.0, 0.5), Some(-12.5));
        assert_eq!(spring_work(-100.0, 0.5, 0.0), None);
    }

    #[test]
    fn gravity_and_friction_helpers_cover_common_cases() {
        approx_eq(
            work_against_gravity(2.0, 9.806_65, 10.0).unwrap(),
            196.133,
            1e-12,
        );
        approx_eq(
            work_by_gravity(2.0, 9.806_65, 10.0).unwrap(),
            -196.133,
            1e-12,
        );

        assert_eq!(work_by_friction(5.0, 10.0), Some(-50.0));
        assert_eq!(work_by_friction(5.0, -10.0), Some(-50.0));
        assert_eq!(work_by_friction(-5.0, 10.0), None);
    }

    #[test]
    fn constant_force_work_requires_finite_inputs() {
        assert_eq!(
            ConstantForceWork::new(10.0, 2.0).unwrap().work(),
            Some(20.0)
        );
        assert_eq!(ConstantForceWork::new(f64::NAN, 2.0), None);
    }
}
