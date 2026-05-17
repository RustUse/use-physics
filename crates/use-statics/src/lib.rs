#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small statics helpers.

/// Re-exports for ergonomic glob imports.
pub mod prelude;

fn finite(value: f64) -> Option<f64> {
    if value.is_finite() {
        Some(if value == 0.0 { 0.0 } else { value })
    } else {
        None
    }
}

fn finite_pair(x: f64, y: f64) -> Option<(f64, f64)> {
    Some((finite(x)?, finite(y)?))
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

/// Computes the net force from a list of scalar forces.
///
/// Returns `Some(0.0)` for an empty slice and `None` when any input or the computed result is not
/// finite.
///
/// # Examples
///
/// ```rust
/// use use_statics::net_force_1d;
///
/// assert_eq!(net_force_1d(&[10.0, -4.0, -6.0]), Some(0.0));
/// ```
#[must_use]
pub fn net_force_1d(forces: &[f64]) -> Option<f64> {
    if !all_finite(forces) {
        return None;
    }

    finite(forces.iter().copied().sum())
}

/// Computes the net force from a list of planar force components.
///
/// Each tuple is interpreted as `(force_x, force_y)`.
///
/// # Examples
///
/// ```rust
/// use use_statics::net_force_2d;
///
/// assert_eq!(net_force_2d(&[(3.0, 4.0), (-1.0, 2.0)]), Some((2.0, 6.0)));
/// ```
#[must_use]
pub fn net_force_2d(forces: &[(f64, f64)]) -> Option<(f64, f64)> {
    let (sum_x, sum_y) =
        forces
            .iter()
            .try_fold((0.0, 0.0), |(sum_x, sum_y), (force_x, force_y)| {
                if !force_x.is_finite() || !force_y.is_finite() {
                    return None;
                }

                Some((sum_x + force_x, sum_y + force_y))
            })?;

    finite_pair(sum_x, sum_y)
}

/// Computes the magnitude of a planar force vector.
///
/// Formula: `F = sqrt(Fx² + Fy²)`.
#[must_use]
pub fn force_magnitude(force_x: f64, force_y: f64) -> Option<f64> {
    if !force_x.is_finite() || !force_y.is_finite() {
        return None;
    }

    finite(force_x.hypot(force_y))
}

/// Computes the planar angle of a force vector in radians.
///
/// This helper uses `atan2(force_y, force_x)`.
#[must_use]
pub fn force_angle_radians(force_x: f64, force_y: f64) -> Option<f64> {
    if !force_x.is_finite() || !force_y.is_finite() {
        return None;
    }

    finite(force_y.atan2(force_x))
}

/// Checks whether a 1D force system is in translational equilibrium.
///
/// Returns `None` when `tolerance` is negative or not finite, or when the net force cannot be
/// computed as a finite value.
#[must_use]
pub fn is_translational_equilibrium_1d(forces: &[f64], tolerance: f64) -> Option<bool> {
    if !nonnegative_finite(tolerance) {
        return None;
    }

    Some(net_force_1d(forces)?.abs() <= tolerance)
}

/// Checks whether a 2D force system is in translational equilibrium.
///
/// Translational equilibrium requires the net force in both axes to be within the provided
/// tolerance.
#[must_use]
pub fn is_translational_equilibrium_2d(forces: &[(f64, f64)], tolerance: f64) -> Option<bool> {
    if !nonnegative_finite(tolerance) {
        return None;
    }

    let (net_x, net_y) = net_force_2d(forces)?;

    Some(net_x.abs() <= tolerance && net_y.abs() <= tolerance)
}

/// Computes the scalar `z`-moment of a planar force about a chosen point.
///
/// Formula: `M = x * Fy - y * Fx`.
/// Positive values are counterclockwise by convention.
///
/// # Examples
///
/// ```rust
/// use use_statics::moment_2d;
///
/// assert_eq!(moment_2d(2.0, 0.0, 0.0, 10.0), Some(20.0));
/// ```
#[must_use]
pub fn moment_2d(position_x: f64, position_y: f64, force_x: f64, force_y: f64) -> Option<f64> {
    if !position_x.is_finite()
        || !position_y.is_finite()
        || !force_x.is_finite()
        || !force_y.is_finite()
    {
        return None;
    }

    finite(position_x.mul_add(force_y, -(position_y * force_x)))
}

/// Computes a moment from a signed force and a signed moment arm.
///
/// Formula: `M = F * d`.
#[must_use]
pub fn moment_from_force_and_arm(force: f64, moment_arm: f64) -> Option<f64> {
    if !force.is_finite() || !moment_arm.is_finite() {
        return None;
    }

    finite(force * moment_arm)
}

/// Computes the net moment from a slice of scalar moment values.
#[must_use]
pub fn net_moment(moments: &[f64]) -> Option<f64> {
    if !all_finite(moments) {
        return None;
    }

    finite(moments.iter().copied().sum())
}

/// Computes the net moment from planar point forces.
///
/// Each tuple is interpreted as `(position_x, position_y, force_x, force_y)`.
#[must_use]
pub fn net_moment_2d(force_positions: &[(f64, f64, f64, f64)]) -> Option<f64> {
    let total = force_positions.iter().try_fold(
        0.0,
        |total, (position_x, position_y, force_x, force_y)| {
            Some(total + moment_2d(*position_x, *position_y, *force_x, *force_y)?)
        },
    )?;

    finite(total)
}

/// Checks whether a moment system is in rotational equilibrium.
#[must_use]
pub fn is_rotational_equilibrium(moments: &[f64], tolerance: f64) -> Option<bool> {
    if !nonnegative_finite(tolerance) {
        return None;
    }

    Some(net_moment(moments)?.abs() <= tolerance)
}

/// Checks whether a planar static system satisfies both translational and rotational equilibrium.
///
/// # Examples
///
/// ```rust
/// use use_statics::is_static_equilibrium_2d;
///
/// assert_eq!(
///     is_static_equilibrium_2d(&[(1.0, 2.0), (-1.0, -2.0)], &[10.0, -10.0], 0.0),
///     Some(true)
/// );
/// ```
#[must_use]
pub fn is_static_equilibrium_2d(
    forces: &[(f64, f64)],
    moments: &[f64],
    tolerance: f64,
) -> Option<bool> {
    Some(
        is_translational_equilibrium_2d(forces, tolerance)?
            && is_rotational_equilibrium(moments, tolerance)?,
    )
}

/// Computes the maximum available static friction.
///
/// Formula: `f_s,max = μ_s * N`.
///
/// # Examples
///
/// ```rust
/// use use_statics::maximum_static_friction;
///
/// assert_eq!(maximum_static_friction(0.5, 100.0), Some(50.0));
/// ```
#[must_use]
pub fn maximum_static_friction(coefficient_static_friction: f64, normal_force: f64) -> Option<f64> {
    if !nonnegative_finite(coefficient_static_friction) || !nonnegative_finite(normal_force) {
        return None;
    }

    finite(coefficient_static_friction * normal_force)
}

/// Computes the static friction magnitude required to hold horizontal equilibrium.
#[must_use]
pub fn required_static_friction(applied_horizontal_force: f64) -> Option<f64> {
    if !applied_horizontal_force.is_finite() {
        return None;
    }

    finite(applied_horizontal_force.abs())
}

/// Checks whether static friction can hold a body at rest.
#[must_use]
pub fn can_static_friction_hold(
    applied_horizontal_force: f64,
    coefficient_static_friction: f64,
    normal_force: f64,
) -> Option<bool> {
    Some(
        required_static_friction(applied_horizontal_force)?
            <= maximum_static_friction(coefficient_static_friction, normal_force)?,
    )
}

/// Computes the normal force on a horizontal surface.
///
/// Formula: `N = m * g`.
#[must_use]
pub fn normal_force_horizontal_surface(mass: f64, gravitational_acceleration: f64) -> Option<f64> {
    if !nonnegative_finite(mass) || !nonnegative_finite(gravitational_acceleration) {
        return None;
    }

    finite(mass * gravitational_acceleration)
}

/// Computes the normal force on an incline.
///
/// Formula: `N = m * g * cos(theta)`.
///
/// # Examples
///
/// ```rust
/// use use_statics::normal_force_incline;
///
/// let Some(normal_force) = normal_force_incline(10.0, 9.80665, 0.0) else {
///     panic!("valid incline inputs should produce a normal force");
/// };
///
/// assert!((normal_force - 98.0665).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn normal_force_incline(
    mass: f64,
    gravitational_acceleration: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !nonnegative_finite(mass)
        || !nonnegative_finite(gravitational_acceleration)
        || !angle_radians.is_finite()
    {
        return None;
    }

    let normal_force = mass * gravitational_acceleration * angle_radians.cos();

    if normal_force < 0.0 {
        return None;
    }

    finite(normal_force)
}

/// Computes the downslope component of weight on an incline.
///
/// Formula: `F_parallel = m * g * sin(theta)`.
#[must_use]
pub fn downslope_force_incline(
    mass: f64,
    gravitational_acceleration: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !nonnegative_finite(mass)
        || !nonnegative_finite(gravitational_acceleration)
        || !angle_radians.is_finite()
    {
        return None;
    }

    finite(mass * gravitational_acceleration * angle_radians.sin())
}

/// Computes the minimum static friction coefficient needed to prevent sliding on an incline.
///
/// Formula: `μ_s,min = tan(theta)`.
#[must_use]
pub fn minimum_static_friction_coefficient_for_incline(angle_radians: f64) -> Option<f64> {
    if !nonnegative_finite(angle_radians) {
        return None;
    }

    let coefficient = finite(angle_radians.tan())?;

    (coefficient >= 0.0).then_some(coefficient)
}

/// Computes support reactions for a simply supported beam with one point load.
///
/// Formulae:
/// - `R_left = P * (L - a) / L`
/// - `R_right = P * a / L`
///
/// # Examples
///
/// ```rust
/// use use_statics::simply_supported_point_load_reactions;
///
/// assert_eq!(
///     simply_supported_point_load_reactions(10.0, 100.0, 5.0),
///     Some((50.0, 50.0))
/// );
/// ```
#[must_use]
pub fn simply_supported_point_load_reactions(
    span: f64,
    load: f64,
    load_position_from_left: f64,
) -> Option<(f64, f64)> {
    if !positive_finite(span)
        || !nonnegative_finite(load)
        || !load_position_from_left.is_finite()
        || !(0.0..=span).contains(&load_position_from_left)
    {
        return None;
    }

    let left_reaction = load * (span - load_position_from_left) / span;
    let right_reaction = load * load_position_from_left / span;

    finite_pair(left_reaction, right_reaction)
}

/// Computes support reactions for a simply supported beam with a uniform load.
#[must_use]
pub fn simply_supported_uniform_load_reactions(
    span: f64,
    load_per_length: f64,
) -> Option<(f64, f64)> {
    if !positive_finite(span) || !nonnegative_finite(load_per_length) {
        return None;
    }

    let total_load = load_per_length * span;
    let reaction = finite(total_load / 2.0)?;

    Some((reaction, reaction))
}

/// Fixed-end reaction data for a cantilever with a free-end point load.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CantileverReaction {
    /// The vertical reaction at the fixed support.
    pub vertical_reaction: f64,
    /// The fixed-end moment at the support.
    pub fixed_end_moment: f64,
}

/// Computes the fixed-end reaction for a cantilever with a downward load at the free end.
///
/// # Examples
///
/// ```rust
/// use use_statics::{CantileverReaction, cantilever_end_point_load_reaction};
///
/// assert_eq!(
///     cantilever_end_point_load_reaction(10.0, 100.0),
///     Some(CantileverReaction {
///         vertical_reaction: 100.0,
///         fixed_end_moment: 1000.0,
///     })
/// );
/// ```
#[must_use]
pub fn cantilever_end_point_load_reaction(span: f64, load: f64) -> Option<CantileverReaction> {
    if !positive_finite(span) || !nonnegative_finite(load) {
        return None;
    }

    Some(CantileverReaction {
        vertical_reaction: finite(load)?,
        fixed_end_moment: finite(load * span)?,
    })
}

/// A validated planar force value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Force2D {
    /// Horizontal force component.
    pub x: f64,
    /// Vertical force component.
    pub y: f64,
}

impl Force2D {
    /// Creates a new planar force when both components are finite.
    #[must_use]
    pub fn new(x: f64, y: f64) -> Option<Self> {
        Some(Self {
            x: finite(x)?,
            y: finite(y)?,
        })
    }

    /// Returns the magnitude of this force.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_statics::Force2D;
    ///
    /// assert_eq!(Force2D::new(3.0, 4.0).and_then(|force| force.magnitude()), Some(5.0));
    /// ```
    #[must_use]
    pub fn magnitude(&self) -> Option<f64> {
        force_magnitude(self.x, self.y)
    }

    /// Returns the planar angle of this force in radians.
    #[must_use]
    pub fn angle_radians(&self) -> Option<f64> {
        force_angle_radians(self.x, self.y)
    }
}

/// A planar force applied at a position relative to a chosen moment point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointForce2D {
    /// Horizontal position from the chosen origin.
    pub position_x: f64,
    /// Vertical position from the chosen origin.
    pub position_y: f64,
    /// Horizontal force component.
    pub force_x: f64,
    /// Vertical force component.
    pub force_y: f64,
}

impl PointForce2D {
    /// Creates a new point force when all values are finite.
    #[must_use]
    pub fn new(position_x: f64, position_y: f64, force_x: f64, force_y: f64) -> Option<Self> {
        Some(Self {
            position_x: finite(position_x)?,
            position_y: finite(position_y)?,
            force_x: finite(force_x)?,
            force_y: finite(force_y)?,
        })
    }

    /// Returns the moment of this force about the origin.
    #[must_use]
    pub fn moment_about_origin(&self) -> Option<f64> {
        moment_2d(self.position_x, self.position_y, self.force_x, self.force_y)
    }
}

/// A simple planar static system made of force vectors and scalar moments.
#[derive(Debug, Clone, PartialEq)]
pub struct StaticSystem2D {
    /// The planar forces acting on the system.
    pub forces: Vec<Force2D>,
    /// Free moments acting on the system.
    pub moments: Vec<f64>,
}

impl StaticSystem2D {
    /// Creates a validated static system.
    #[must_use]
    pub fn new(forces: Vec<Force2D>, moments: Vec<f64>) -> Option<Self> {
        if !forces
            .iter()
            .all(|force| force.x.is_finite() && force.y.is_finite())
            || !all_finite(&moments)
        {
            return None;
        }

        Some(Self { forces, moments })
    }

    fn force_components(&self) -> Vec<(f64, f64)> {
        self.forces.iter().map(|force| (force.x, force.y)).collect()
    }

    /// Returns the net force in the system.
    #[must_use]
    pub fn net_force(&self) -> Option<(f64, f64)> {
        net_force_2d(&self.force_components())
    }

    /// Returns the net free moment in the system.
    #[must_use]
    pub fn net_moment(&self) -> Option<f64> {
        net_moment(&self.moments)
    }

    /// Checks whether the system is in static equilibrium.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_statics::{Force2D, StaticSystem2D};
    ///
    /// let Some(system) = StaticSystem2D::new(
    ///     vec![
    ///         Force2D::new(1.0, 2.0).unwrap(),
    ///         Force2D::new(-1.0, -2.0).unwrap(),
    ///     ],
    ///     vec![10.0, -10.0],
    /// ) else {
    ///     panic!("valid system should construct");
    /// };
    ///
    /// assert_eq!(system.is_equilibrium(0.0), Some(true));
    /// ```
    #[must_use]
    pub fn is_equilibrium(&self, tolerance: f64) -> Option<bool> {
        is_static_equilibrium_2d(&self.force_components(), &self.moments, tolerance)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use core::f64;
    use core::f64::consts::FRAC_PI_4;

    use super::{
        CantileverReaction, Force2D, PointForce2D, StaticSystem2D, can_static_friction_hold,
        cantilever_end_point_load_reaction, downslope_force_incline, force_angle_radians,
        force_magnitude, is_rotational_equilibrium, is_static_equilibrium_2d,
        is_translational_equilibrium_1d, is_translational_equilibrium_2d, maximum_static_friction,
        minimum_static_friction_coefficient_for_incline, moment_2d, moment_from_force_and_arm,
        net_force_1d, net_force_2d, net_moment, net_moment_2d, normal_force_horizontal_surface,
        normal_force_incline, required_static_friction, simply_supported_point_load_reactions,
        simply_supported_uniform_load_reactions,
    };

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn force_helpers_cover_required_cases() {
        assert_eq!(net_force_1d(&[10.0, -4.0, -6.0]), Some(0.0));
        assert_eq!(net_force_1d(&[]), Some(0.0));
        assert_eq!(net_force_1d(&[10.0, f64::NAN]), None);

        assert_eq!(net_force_2d(&[(1.0, 2.0), (-1.0, -2.0)]), Some((0.0, 0.0)));
        assert_eq!(net_force_2d(&[]), Some((0.0, 0.0)));

        assert_eq!(force_magnitude(3.0, 4.0), Some(5.0));
        assert_eq!(force_angle_radians(1.0, 0.0), Some(0.0));
    }

    #[test]
    fn translational_equilibrium_helpers_cover_required_cases() {
        assert_eq!(
            is_translational_equilibrium_1d(&[10.0, -10.0], 0.0),
            Some(true)
        );
        assert_eq!(
            is_translational_equilibrium_1d(&[10.0, -9.0], 0.0),
            Some(false)
        );
        assert_eq!(is_translational_equilibrium_1d(&[10.0, -10.0], -1.0), None);

        assert_eq!(
            is_translational_equilibrium_2d(&[(1.0, 2.0), (-1.0, -2.0)], 0.0),
            Some(true)
        );
    }

    #[test]
    fn moment_helpers_cover_required_cases() {
        assert_eq!(moment_2d(2.0, 0.0, 0.0, 10.0), Some(20.0));
        assert_eq!(moment_2d(0.0, 2.0, 10.0, 0.0), Some(-20.0));

        assert_eq!(moment_from_force_and_arm(10.0, 2.0), Some(20.0));
        assert_eq!(moment_from_force_and_arm(-10.0, 2.0), Some(-20.0));

        assert_eq!(net_moment(&[10.0, -4.0, -6.0]), Some(0.0));
        assert_eq!(net_moment(&[]), Some(0.0));

        assert_eq!(
            net_moment_2d(&[(2.0, 0.0, 0.0, 10.0), (0.0, 2.0, 10.0, 0.0)]),
            Some(0.0)
        );
    }

    #[test]
    fn rotational_equilibrium_helpers_cover_required_cases() {
        assert_eq!(is_rotational_equilibrium(&[10.0, -10.0], 0.0), Some(true));
        assert_eq!(is_rotational_equilibrium(&[10.0, -9.0], 0.0), Some(false));
        assert_eq!(
            is_static_equilibrium_2d(&[(1.0, 2.0), (-1.0, -2.0)], &[10.0, -10.0], 0.0),
            Some(true)
        );
    }

    #[test]
    fn static_friction_helpers_cover_required_cases() {
        assert_eq!(maximum_static_friction(0.5, 100.0), Some(50.0));
        assert_eq!(maximum_static_friction(-0.5, 100.0), None);

        assert_eq!(required_static_friction(-20.0), Some(20.0));
        assert_eq!(can_static_friction_hold(20.0, 0.5, 100.0), Some(true));
        assert_eq!(can_static_friction_hold(60.0, 0.5, 100.0), Some(false));

        let Some(normal_force) = normal_force_horizontal_surface(10.0, 9.80665) else {
            panic!("valid horizontal surface inputs should produce a normal force");
        };
        approx_eq(normal_force, 98.0665, 1.0e-12);
        assert_eq!(normal_force_horizontal_surface(-10.0, 9.80665), None);
    }

    #[test]
    fn incline_helpers_cover_required_cases() {
        let Some(normal_force) = normal_force_incline(10.0, 9.80665, 0.0) else {
            panic!("valid incline inputs should produce a normal force");
        };
        approx_eq(normal_force, 98.0665, 1.0e-12);

        let Some(downslope_force) = downslope_force_incline(10.0, 9.80665, 0.0) else {
            panic!("valid incline inputs should produce a downslope force");
        };
        approx_eq(downslope_force, 0.0, 1.0e-12);

        let Some(coefficient) = minimum_static_friction_coefficient_for_incline(FRAC_PI_4) else {
            panic!("forty-five degrees should produce a finite coefficient");
        };
        approx_eq(coefficient, 1.0, 1.0e-12);
        assert_eq!(minimum_static_friction_coefficient_for_incline(-1.0), None);
    }

    #[test]
    fn support_reaction_helpers_cover_required_cases() {
        assert_eq!(
            simply_supported_point_load_reactions(10.0, 100.0, 5.0),
            Some((50.0, 50.0))
        );
        assert_eq!(
            simply_supported_point_load_reactions(10.0, 100.0, 0.0),
            Some((100.0, 0.0))
        );
        assert_eq!(
            simply_supported_point_load_reactions(10.0, 100.0, 10.0),
            Some((0.0, 100.0))
        );
        assert_eq!(
            simply_supported_point_load_reactions(10.0, 100.0, 11.0),
            None
        );
        assert_eq!(simply_supported_point_load_reactions(0.0, 100.0, 0.0), None);

        assert_eq!(
            simply_supported_uniform_load_reactions(10.0, 20.0),
            Some((100.0, 100.0))
        );
        assert_eq!(simply_supported_uniform_load_reactions(10.0, -20.0), None);

        assert_eq!(
            cantilever_end_point_load_reaction(10.0, 100.0),
            Some(CantileverReaction {
                vertical_reaction: 100.0,
                fixed_end_moment: 1000.0,
            })
        );
        assert_eq!(cantilever_end_point_load_reaction(0.0, 100.0), None);
    }

    #[test]
    fn structs_cover_required_cases() {
        let Some(force) = Force2D::new(3.0, 4.0) else {
            panic!("valid force should construct");
        };
        assert_eq!(force.magnitude(), Some(5.0));
        assert_eq!(Force2D::new(f64::NAN, 4.0), None);

        let Some(point_force) = PointForce2D::new(2.0, 0.0, 0.0, 10.0) else {
            panic!("valid point force should construct");
        };
        assert_eq!(point_force.moment_about_origin(), Some(20.0));
        assert_eq!(PointForce2D::new(f64::NAN, 0.0, 0.0, 10.0), None);
    }

    #[test]
    fn static_system_covers_required_cases() {
        let Some(force_a) = Force2D::new(1.0, 2.0) else {
            panic!("valid force should construct");
        };
        let Some(force_b) = Force2D::new(-1.0, -2.0) else {
            panic!("valid force should construct");
        };
        let Some(system) = StaticSystem2D::new(vec![force_a, force_b], vec![10.0, -10.0]) else {
            panic!("valid system should construct");
        };

        assert_eq!(system.net_force(), Some((0.0, 0.0)));
        assert_eq!(system.net_moment(), Some(0.0));
        assert_eq!(system.is_equilibrium(0.0), Some(true));

        let Some(valid_force) = Force2D::new(1.0, 2.0) else {
            panic!("valid force should construct");
        };
        assert_eq!(StaticSystem2D::new(vec![valid_force], vec![f64::NAN]), None);
    }
}
