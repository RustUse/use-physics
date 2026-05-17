#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small fluid mechanics helpers.

pub mod prelude;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Computes buoyant force from fluid density, displaced volume, and gravitational acceleration.
///
/// Formula: `F_b = ρ * V * g`
///
/// Returns `None` when `fluid_density` or `displaced_volume` is negative, when
/// `gravitational_acceleration` is not finite, or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::buoyant_force;
///
/// let force = buoyant_force(1000.0, 0.01, 9.80665).unwrap();
///
/// assert!((force - 98.0665).abs() < 1.0e-10);
/// ```
#[must_use]
pub fn buoyant_force(
    fluid_density: f64,
    displaced_volume: f64,
    gravitational_acceleration: f64,
) -> Option<f64> {
    if !nonnegative(fluid_density)
        || !nonnegative(displaced_volume)
        || !gravitational_acceleration.is_finite()
    {
        return None;
    }

    finite(fluid_density * displaced_volume * gravitational_acceleration)
}

/// Computes displaced volume from buoyant force, fluid density, and gravitational acceleration.
///
/// Formula: `V = F_b / (ρ * g)`
///
/// Returns `None` when `buoyant_force` is negative, when `fluid_density` is less than or equal
/// to zero, when `gravitational_acceleration` is zero or not finite, when the computed volume is
/// negative, or when the computed volume is not finite.
#[must_use]
pub fn displaced_volume_from_buoyant_force(
    buoyant_force: f64,
    fluid_density: f64,
    gravitational_acceleration: f64,
) -> Option<f64> {
    if !nonnegative(buoyant_force)
        || !positive(fluid_density)
        || !gravitational_acceleration.is_finite()
        || gravitational_acceleration == 0.0
    {
        return None;
    }

    let volume = buoyant_force / (fluid_density * gravitational_acceleration);
    if volume < 0.0 {
        return None;
    }

    finite(volume)
}

/// Computes hydrostatic pressure from fluid density, gravitational acceleration, and depth.
///
/// Formula: `P = ρ * g * h`
///
/// Returns `None` when `fluid_density` or `depth` is negative, when
/// `gravitational_acceleration` is not finite, or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::hydrostatic_pressure;
///
/// let pressure = hydrostatic_pressure(1000.0, 9.80665, 10.0).unwrap();
///
/// assert!((pressure - 98_066.5).abs() < 1.0e-9);
/// ```
#[must_use]
pub fn hydrostatic_pressure(
    fluid_density: f64,
    gravitational_acceleration: f64,
    depth: f64,
) -> Option<f64> {
    if !nonnegative(fluid_density) || !nonnegative(depth) || !gravitational_acceleration.is_finite()
    {
        return None;
    }

    finite(fluid_density * gravitational_acceleration * depth)
}

/// Computes absolute pressure from surface pressure and the hydrostatic contribution.
///
/// Formula: `P_abs = P_surface + ρ * g * h`
///
/// Returns `None` when `surface_pressure`, `fluid_density`, or `depth` is negative, when
/// `gravitational_acceleration` is not finite, or when the computed result is not finite.
#[must_use]
pub fn absolute_pressure(
    surface_pressure: f64,
    fluid_density: f64,
    gravitational_acceleration: f64,
    depth: f64,
) -> Option<f64> {
    if !nonnegative(surface_pressure)
        || !nonnegative(fluid_density)
        || !nonnegative(depth)
        || !gravitational_acceleration.is_finite()
    {
        return None;
    }

    let hydrostatic_gradient = fluid_density * gravitational_acceleration;

    finite(hydrostatic_gradient.mul_add(depth, surface_pressure))
}

/// Computes volumetric flow rate from area and flow velocity.
///
/// Formula: `Q = A * v`
///
/// Returns `None` when `area` is negative, when either input is not finite, or when the
/// computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::volumetric_flow_rate;
///
/// assert_eq!(volumetric_flow_rate(2.0, 3.0), Some(6.0));
/// assert_eq!(volumetric_flow_rate(2.0, -3.0), Some(-6.0));
/// ```
#[must_use]
pub fn volumetric_flow_rate(area: f64, velocity: f64) -> Option<f64> {
    if !nonnegative(area) || !velocity.is_finite() {
        return None;
    }

    finite(area * velocity)
}

/// Computes velocity from volumetric flow rate and area.
///
/// Formula: `v = Q / A`
///
/// Returns `None` when `area` is less than or equal to zero, when `flow_rate` is not finite, or
/// when the computed result is not finite.
#[must_use]
pub fn velocity_from_flow_rate(flow_rate: f64, area: f64) -> Option<f64> {
    if !flow_rate.is_finite() || !positive(area) {
        return None;
    }

    finite(flow_rate / area)
}

/// Computes mass flow rate from density and volumetric flow rate.
///
/// Formula: `ṁ = ρ * Q`
///
/// Returns `None` when `density` is negative, when either input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn mass_flow_rate(density: f64, volumetric_flow_rate: f64) -> Option<f64> {
    if !nonnegative(density) || !volumetric_flow_rate.is_finite() {
        return None;
    }

    finite(density * volumetric_flow_rate)
}

/// Computes downstream velocity from continuity for incompressible flow.
///
/// Formula: `A1 * v1 = A2 * v2`, so `v2 = A1 * v1 / A2`
///
/// Returns `None` when `area_a` is negative, when `area_b` is less than or equal to zero, when
/// any input is not finite, or when the computed result is not finite.
#[must_use]
pub fn continuity_velocity(area_a: f64, velocity_a: f64, area_b: f64) -> Option<f64> {
    if !nonnegative(area_a) || !velocity_a.is_finite() || !positive(area_b) {
        return None;
    }

    finite(area_a * velocity_a / area_b)
}

/// Computes downstream area from continuity for incompressible flow.
///
/// Formula: `A2 = A1 * v1 / v2`
///
/// Returns `None` when `area_a` is negative, when `velocity_b` is zero, when any input is not
/// finite, when the computed area is negative, or when the computed area is not finite.
#[must_use]
pub fn continuity_area(area_a: f64, velocity_a: f64, velocity_b: f64) -> Option<f64> {
    if !nonnegative(area_a)
        || !velocity_a.is_finite()
        || !velocity_b.is_finite()
        || velocity_b == 0.0
    {
        return None;
    }

    let area = area_a * velocity_a / velocity_b;
    if area < 0.0 {
        return None;
    }

    finite(area)
}

/// Computes downstream pressure from the Bernoulli relation between two points.
///
/// Formula: `P2 = P1 + 0.5ρ(v1² - v2²) + ρg(h1 - h2)`
///
/// Returns `None` when `reference_pressure` or `density` is negative, when any input is not
/// finite, or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::bernoulli_pressure;
///
/// let pressure = bernoulli_pressure(100000.0, 1000.0, 4.0, 2.0, 9.80665, 10.0, 5.0).unwrap();
///
/// assert!(pressure.is_finite());
/// ```
#[must_use]
pub fn bernoulli_pressure(
    reference_pressure: f64,
    density: f64,
    reference_velocity: f64,
    velocity: f64,
    gravitational_acceleration: f64,
    reference_height: f64,
    height: f64,
) -> Option<f64> {
    if !nonnegative(reference_pressure)
        || !nonnegative(density)
        || !reference_velocity.is_finite()
        || !velocity.is_finite()
        || !gravitational_acceleration.is_finite()
        || !reference_height.is_finite()
        || !height.is_finite()
    {
        return None;
    }

    let velocity_delta = velocity.mul_add(-velocity, reference_velocity * reference_velocity);
    let pressure_without_height = (0.5 * density).mul_add(velocity_delta, reference_pressure);
    let hydrostatic_gradient = density * gravitational_acceleration;

    finite(hydrostatic_gradient.mul_add(reference_height - height, pressure_without_height))
}

/// Computes dynamic pressure from density and flow velocity.
///
/// Formula: `q = 0.5 * ρ * v²`
///
/// Returns `None` when `density` is negative, when any input is not finite, or when the
/// computed result is not finite.
#[must_use]
pub fn dynamic_pressure(density: f64, velocity: f64) -> Option<f64> {
    if !nonnegative(density) || !velocity.is_finite() {
        return None;
    }

    let pressure = 0.5 * density * velocity.powi(2);
    if pressure < 0.0 {
        return None;
    }

    finite(pressure)
}

/// Computes Reynolds number from density, velocity, characteristic length, and dynamic viscosity.
///
/// Formula: `Re = ρ * v * L / μ`
///
/// Returns `None` when `density` is negative, when `characteristic_length` is negative, when
/// `dynamic_viscosity` is less than or equal to zero, when any input is not finite, or when the
/// computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::reynolds_number;
///
/// let reynolds = reynolds_number(1000.0, 2.0, 0.1, 0.001).unwrap();
///
/// assert!((reynolds - 200_000.0).abs() < 1.0e-9);
/// ```
#[must_use]
pub fn reynolds_number(
    density: f64,
    velocity: f64,
    characteristic_length: f64,
    dynamic_viscosity: f64,
) -> Option<f64> {
    if !nonnegative(density)
        || !velocity.is_finite()
        || !nonnegative(characteristic_length)
        || !positive(dynamic_viscosity)
    {
        return None;
    }

    finite(density * velocity.abs() * characteristic_length / dynamic_viscosity)
}

/// Computes kinematic viscosity from dynamic viscosity and density.
///
/// Formula: `ν = μ / ρ`
///
/// Returns `None` when `dynamic_viscosity` is negative, when `density` is less than or equal to
/// zero, when any input is not finite, or when the computed result is not finite.
#[must_use]
pub fn kinematic_viscosity(dynamic_viscosity: f64, density: f64) -> Option<f64> {
    if !nonnegative(dynamic_viscosity) || !positive(density) {
        return None;
    }

    finite(dynamic_viscosity / density)
}

/// Computes dynamic viscosity from kinematic viscosity and density.
///
/// Formula: `μ = ν * ρ`
///
/// Returns `None` when `kinematic_viscosity` or `density` is negative, when any input is not
/// finite, or when the computed result is not finite.
#[must_use]
pub fn dynamic_viscosity(kinematic_viscosity: f64, density: f64) -> Option<f64> {
    if !nonnegative(kinematic_viscosity) || !nonnegative(density) {
        return None;
    }

    finite(kinematic_viscosity * density)
}

/// Computes drag force from density, velocity, drag coefficient, and area.
///
/// Formula: `F_d = 0.5 * ρ * v² * C_d * A`
///
/// Returns `None` when `density`, `drag_coefficient`, or `area` is negative, when any input is
/// not finite, or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_fluid::drag_force;
///
/// let force = drag_force(1.225, 10.0, 0.47, 1.0).unwrap();
///
/// assert!((force - 28.7875).abs() < 1.0e-9);
/// ```
#[must_use]
pub fn drag_force(density: f64, velocity: f64, drag_coefficient: f64, area: f64) -> Option<f64> {
    if !nonnegative(density)
        || !velocity.is_finite()
        || !nonnegative(drag_coefficient)
        || !nonnegative(area)
    {
        return None;
    }

    let force = 0.5 * density * velocity.powi(2) * drag_coefficient * area;
    if force < 0.0 {
        return None;
    }

    finite(force)
}

/// A simple fluid model with density and optional dynamic viscosity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fluid {
    pub density: f64,
    pub dynamic_viscosity: Option<f64>,
}

impl Fluid {
    /// Creates a fluid from density when the density is non-negative and finite.
    #[must_use]
    pub fn new(density: f64) -> Option<Self> {
        if !nonnegative(density) {
            return None;
        }

        Some(Self {
            density,
            dynamic_viscosity: None,
        })
    }

    /// Creates a fluid from density and dynamic viscosity when both values are non-negative and finite.
    #[must_use]
    pub fn with_dynamic_viscosity(density: f64, dynamic_viscosity: f64) -> Option<Self> {
        if !nonnegative(density) || !nonnegative(dynamic_viscosity) {
            return None;
        }

        Some(Self {
            density,
            dynamic_viscosity: Some(dynamic_viscosity),
        })
    }

    /// Computes buoyant force for a displaced volume in this fluid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_fluid::Fluid;
    ///
    /// let water = Fluid::new(1000.0).unwrap();
    /// let force = water.buoyant_force(0.01, 9.80665).unwrap();
    ///
    /// assert!((force - 98.0665).abs() < 1.0e-10);
    /// ```
    #[must_use]
    pub fn buoyant_force(
        &self,
        displaced_volume: f64,
        gravitational_acceleration: f64,
    ) -> Option<f64> {
        buoyant_force(self.density, displaced_volume, gravitational_acceleration)
    }

    /// Computes hydrostatic pressure at a depth in this fluid.
    #[must_use]
    pub fn hydrostatic_pressure(&self, gravitational_acceleration: f64, depth: f64) -> Option<f64> {
        hydrostatic_pressure(self.density, gravitational_acceleration, depth)
    }

    /// Computes dynamic pressure for this fluid at a given velocity.
    #[must_use]
    pub fn dynamic_pressure(&self, velocity: f64) -> Option<f64> {
        dynamic_pressure(self.density, velocity)
    }

    /// Computes Reynolds number for this fluid when dynamic viscosity is available.
    #[must_use]
    pub fn reynolds_number(&self, velocity: f64, characteristic_length: f64) -> Option<f64> {
        let dynamic_viscosity = self.dynamic_viscosity?;

        reynolds_number(
            self.density,
            velocity,
            characteristic_length,
            dynamic_viscosity,
        )
    }
}

/// A simple cross-sectional pipe flow with area and scalar velocity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipeFlow {
    pub area: f64,
    pub velocity: f64,
}

impl PipeFlow {
    /// Creates a pipe flow when `area` is non-negative and both values are finite.
    #[must_use]
    pub fn new(area: f64, velocity: f64) -> Option<Self> {
        if !nonnegative(area) || !velocity.is_finite() {
            return None;
        }

        Some(Self { area, velocity })
    }

    /// Computes volumetric flow rate for this pipe flow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_fluid::PipeFlow;
    ///
    /// let flow = PipeFlow::new(2.0, 3.0).unwrap();
    ///
    /// assert_eq!(flow.volumetric_flow_rate(), Some(6.0));
    /// ```
    #[must_use]
    pub fn volumetric_flow_rate(&self) -> Option<f64> {
        volumetric_flow_rate(self.area, self.velocity)
    }

    /// Computes mass flow rate for this pipe flow with a provided density.
    #[must_use]
    pub fn mass_flow_rate(&self, density: f64) -> Option<f64> {
        mass_flow_rate(density, self.volumetric_flow_rate()?)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        Fluid, PipeFlow, absolute_pressure, bernoulli_pressure, buoyant_force, continuity_area,
        continuity_velocity, displaced_volume_from_buoyant_force, drag_force, dynamic_pressure,
        dynamic_viscosity, hydrostatic_pressure, kinematic_viscosity, mass_flow_rate,
        reynolds_number, velocity_from_flow_rate, volumetric_flow_rate,
    };

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        let delta = (left - right).abs();

        assert!(
            delta <= tolerance,
            "left={left} right={right} delta={delta} tolerance={tolerance}"
        );
    }

    #[test]
    fn buoyancy_helpers_cover_valid_and_invalid_inputs() {
        approx_eq(
            buoyant_force(1000.0, 0.01, 9.80665).unwrap(),
            98.0665,
            1.0e-10,
        );
        assert_eq!(buoyant_force(-1000.0, 0.01, 9.80665), None);
        assert_eq!(buoyant_force(1000.0, -0.01, 9.80665), None);

        approx_eq(
            displaced_volume_from_buoyant_force(98.0665, 1000.0, 9.80665).unwrap(),
            0.01,
            1.0e-12,
        );
        assert_eq!(
            displaced_volume_from_buoyant_force(98.0665, 0.0, 9.80665),
            None
        );
    }

    #[test]
    fn hydrostatic_helpers_cover_valid_and_invalid_inputs() {
        approx_eq(
            hydrostatic_pressure(1000.0, 9.80665, 10.0).unwrap(),
            98_066.5,
            1.0e-9,
        );
        assert_eq!(hydrostatic_pressure(1000.0, 9.80665, -1.0), None);

        approx_eq(
            absolute_pressure(101_325.0, 1000.0, 9.80665, 10.0).unwrap(),
            199_391.5,
            1.0e-9,
        );
        assert_eq!(absolute_pressure(-1.0, 1000.0, 9.80665, 10.0), None);
    }

    #[test]
    fn flow_rate_helpers_cover_valid_and_invalid_inputs() {
        assert_eq!(volumetric_flow_rate(2.0, 3.0), Some(6.0));
        assert_eq!(volumetric_flow_rate(2.0, -3.0), Some(-6.0));
        assert_eq!(volumetric_flow_rate(-2.0, 3.0), None);

        assert_eq!(velocity_from_flow_rate(6.0, 2.0), Some(3.0));
        assert_eq!(velocity_from_flow_rate(6.0, 0.0), None);

        assert_eq!(mass_flow_rate(1000.0, 0.5), Some(500.0));
        assert_eq!(mass_flow_rate(-1000.0, 0.5), None);
    }

    #[test]
    fn continuity_helpers_cover_valid_and_invalid_inputs() {
        assert_eq!(continuity_velocity(2.0, 3.0, 1.0), Some(6.0));
        assert_eq!(continuity_velocity(2.0, 3.0, 0.0), None);

        assert_eq!(continuity_area(2.0, 3.0, 6.0), Some(1.0));
        assert_eq!(continuity_area(2.0, 3.0, 0.0), None);
    }

    #[test]
    fn bernoulli_and_dynamic_pressure_cover_common_cases() {
        assert_eq!(dynamic_pressure(1000.0, 3.0), Some(4500.0));
        assert_eq!(dynamic_pressure(-1000.0, 3.0), None);

        let pressure = bernoulli_pressure(100_000.0, 1000.0, 4.0, 2.0, 9.80665, 10.0, 5.0);
        assert!(pressure.is_some_and(f64::is_finite));
    }

    #[test]
    fn viscosity_helpers_cover_reynolds_and_conversions() {
        approx_eq(
            reynolds_number(1000.0, 2.0, 0.1, 0.001).unwrap(),
            200_000.0,
            1.0e-9,
        );
        assert_eq!(reynolds_number(1000.0, 2.0, 0.1, 0.0), None);

        approx_eq(
            kinematic_viscosity(0.001, 1000.0).unwrap(),
            0.000_001,
            1.0e-15,
        );
        assert_eq!(kinematic_viscosity(0.001, 0.0), None);

        approx_eq(
            dynamic_viscosity(0.000_001, 1000.0).unwrap(),
            0.001,
            1.0e-15,
        );
        assert_eq!(dynamic_viscosity(-0.000_001, 1000.0), None);
    }

    #[test]
    fn drag_force_handles_standard_cases() {
        approx_eq(drag_force(1.225, 10.0, 0.47, 1.0).unwrap(), 28.7875, 1.0e-9);
        approx_eq(
            drag_force(1.225, -10.0, 0.47, 1.0).unwrap(),
            28.7875,
            1.0e-9,
        );
        assert_eq!(drag_force(1.225, 10.0, -0.47, 1.0), None);
    }

    #[test]
    fn fluid_methods_delegate_to_public_functions() {
        approx_eq(
            Fluid::new(1000.0)
                .unwrap()
                .hydrostatic_pressure(9.80665, 10.0)
                .unwrap(),
            98_066.5,
            1.0e-9,
        );
        approx_eq(
            Fluid::with_dynamic_viscosity(1000.0, 0.001)
                .unwrap()
                .reynolds_number(2.0, 0.1)
                .unwrap(),
            200_000.0,
            1.0e-9,
        );
        assert_eq!(Fluid::new(1000.0).unwrap().reynolds_number(2.0, 0.1), None);
        assert_eq!(Fluid::new(-1000.0), None);
    }

    #[test]
    fn pipe_flow_methods_delegate_to_public_functions() {
        assert_eq!(
            PipeFlow::new(2.0, 3.0).unwrap().volumetric_flow_rate(),
            Some(6.0)
        );
        assert_eq!(
            PipeFlow::new(2.0, 3.0).unwrap().mass_flow_rate(1000.0),
            Some(6000.0)
        );
        assert_eq!(PipeFlow::new(-2.0, 3.0), None);
    }
}
