#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Electricity-specific scalar helpers.

pub mod prelude;

/// Coulomb's constant for electrostatic force calculations.
///
/// More general physical constants belong in the top-level `use-constants` set.
pub const COULOMB_CONSTANT: f64 = 8.987_551_792_3e9;

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// Computes electric charge from current and elapsed time.
#[must_use]
pub fn charge_from_current_time(current: f64, time: f64) -> Option<f64> {
    if !all_finite(&[current, time]) || time < 0.0 {
        return None;
    }

    finite_result(current * time)
}

/// Computes current from electric charge and elapsed time.
#[must_use]
pub fn current_from_charge_time(charge: f64, time: f64) -> Option<f64> {
    if !all_finite(&[charge, time]) || time <= 0.0 {
        return None;
    }

    finite_result(charge / time)
}

/// Computes voltage from current and resistance using Ohm's law.
///
/// # Examples
///
/// ```
/// use use_electricity::voltage;
///
/// assert_eq!(voltage(2.0, 5.0), Some(10.0));
/// assert_eq!(voltage(2.0, -5.0), None);
/// ```
#[must_use]
pub fn voltage(current: f64, resistance: f64) -> Option<f64> {
    if !all_finite(&[current, resistance]) || resistance < 0.0 {
        return None;
    }

    finite_result(current * resistance)
}

/// Computes current from voltage and resistance using Ohm's law.
///
/// # Examples
///
/// ```
/// use use_electricity::current;
///
/// assert_eq!(current(10.0, 5.0), Some(2.0));
/// assert_eq!(current(10.0, 0.0), None);
/// ```
#[must_use]
pub fn current(voltage: f64, resistance: f64) -> Option<f64> {
    if !all_finite(&[voltage, resistance]) || resistance <= 0.0 {
        return None;
    }

    finite_result(voltage / resistance)
}

/// Computes resistance from voltage and current using Ohm's law.
///
/// # Examples
///
/// ```
/// use use_electricity::resistance;
///
/// assert_eq!(resistance(10.0, 2.0), Some(5.0));
/// assert_eq!(resistance(-10.0, 2.0), None);
/// ```
#[must_use]
pub fn resistance(voltage: f64, current: f64) -> Option<f64> {
    if !all_finite(&[voltage, current]) || current == 0.0 {
        return None;
    }

    let resistance = voltage / current;
    if !resistance.is_finite() || resistance < 0.0 {
        None
    } else {
        Some(resistance)
    }
}

/// Computes conductance from resistance.
#[must_use]
pub fn conductance(resistance: f64) -> Option<f64> {
    if !resistance.is_finite() || resistance <= 0.0 {
        return None;
    }

    finite_result(1.0 / resistance)
}

/// Computes resistance from conductance.
#[must_use]
pub fn resistance_from_conductance(conductance: f64) -> Option<f64> {
    if !conductance.is_finite() || conductance <= 0.0 {
        return None;
    }

    finite_result(1.0 / conductance)
}

/// Computes electrical power from voltage and current.
///
/// # Examples
///
/// ```
/// use use_electricity::power_from_voltage_current;
///
/// assert_eq!(power_from_voltage_current(10.0, 2.0), Some(20.0));
/// assert_eq!(power_from_voltage_current(-10.0, 2.0), Some(-20.0));
/// ```
#[must_use]
pub fn power_from_voltage_current(voltage: f64, current: f64) -> Option<f64> {
    if !all_finite(&[voltage, current]) {
        return None;
    }

    finite_result(voltage * current)
}

/// Computes electrical power from current and resistance.
#[must_use]
pub fn power_from_current_resistance(current: f64, resistance: f64) -> Option<f64> {
    if !all_finite(&[current, resistance]) || resistance < 0.0 {
        return None;
    }

    finite_result(current * current * resistance)
}

/// Computes electrical power from voltage and resistance.
#[must_use]
pub fn power_from_voltage_resistance(voltage: f64, resistance: f64) -> Option<f64> {
    if !all_finite(&[voltage, resistance]) || resistance <= 0.0 {
        return None;
    }

    finite_result((voltage * voltage) / resistance)
}

/// Computes electrical energy from power and elapsed time.
#[must_use]
pub fn energy_from_power_time(power: f64, time: f64) -> Option<f64> {
    if !all_finite(&[power, time]) || time < 0.0 {
        return None;
    }

    finite_result(power * time)
}

/// Computes electrical energy from voltage and charge.
#[must_use]
pub fn energy_from_voltage_charge(voltage: f64, charge: f64) -> Option<f64> {
    if !all_finite(&[voltage, charge]) {
        return None;
    }

    finite_result(voltage * charge)
}

/// Computes the total resistance for resistors in series.
///
/// # Examples
///
/// ```
/// use use_electricity::series_resistance;
///
/// assert_eq!(series_resistance(&[1.0, 2.0, 3.0]), Some(6.0));
/// assert_eq!(series_resistance(&[]), Some(0.0));
/// ```
#[must_use]
pub fn series_resistance(resistances: &[f64]) -> Option<f64> {
    let mut total = 0.0;

    for &resistance in resistances {
        if !resistance.is_finite() || resistance < 0.0 {
            return None;
        }

        total += resistance;
    }

    finite_result(total)
}

/// Computes the total resistance for resistors in parallel.
///
/// # Examples
///
/// ```
/// use use_electricity::parallel_resistance;
///
/// assert_eq!(parallel_resistance(&[2.0, 2.0]), Some(1.0));
/// assert_eq!(parallel_resistance(&[]), None);
/// ```
#[must_use]
pub fn parallel_resistance(resistances: &[f64]) -> Option<f64> {
    if resistances.is_empty() {
        return None;
    }

    let mut reciprocal_sum = 0.0;

    for &resistance in resistances {
        if !resistance.is_finite() || resistance <= 0.0 {
            return None;
        }

        reciprocal_sum += 1.0 / resistance;
    }

    finite_result(1.0 / reciprocal_sum)
}

/// Computes electrostatic force using Coulomb's law.
///
/// The sign is preserved so callers can distinguish attractive and repulsive
/// direction conventions.
///
/// # Examples
///
/// ```
/// use use_electricity::{COULOMB_CONSTANT, coulomb_force};
///
/// assert_eq!(coulomb_force(1.0, 1.0, 1.0), Some(COULOMB_CONSTANT));
/// assert_eq!(coulomb_force(1.0, -1.0, 1.0), Some(-COULOMB_CONSTANT));
/// ```
#[must_use]
pub fn coulomb_force(charge_a: f64, charge_b: f64, distance: f64) -> Option<f64> {
    if !all_finite(&[charge_a, charge_b, distance]) || distance <= 0.0 {
        return None;
    }

    finite_result(COULOMB_CONSTANT * charge_a * charge_b / (distance * distance))
}

/// A simple electrical load described by voltage and resistance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElectricalLoad {
    pub voltage: f64,
    pub resistance: f64,
}

impl ElectricalLoad {
    /// Creates a new electrical load from voltage and resistance.
    #[must_use]
    pub fn new(voltage: f64, resistance: f64) -> Option<Self> {
        if !voltage.is_finite() || !resistance.is_finite() || resistance <= 0.0 {
            return None;
        }

        Some(Self {
            voltage,
            resistance,
        })
    }

    /// Computes the current drawn by the load.
    #[must_use]
    pub fn current(&self) -> Option<f64> {
        current(self.voltage, self.resistance)
    }

    /// Computes the power consumed by the load.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_electricity::ElectricalLoad;
    ///
    /// let load = ElectricalLoad::new(10.0, 5.0).expect("valid load");
    /// assert_eq!(load.power(), Some(20.0));
    /// ```
    #[must_use]
    pub fn power(&self) -> Option<f64> {
        power_from_voltage_resistance(self.voltage, self.resistance)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        COULOMB_CONSTANT, ElectricalLoad, charge_from_current_time, conductance, coulomb_force,
        current, current_from_charge_time, energy_from_power_time, energy_from_voltage_charge,
        parallel_resistance, power_from_current_resistance, power_from_voltage_current,
        power_from_voltage_resistance, resistance, resistance_from_conductance, series_resistance,
        voltage,
    };

    const EPSILON: f64 = 1.0e-12;

    fn assert_some_close(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("expected Some value");
        let tolerance = expected.abs().max(1.0) * EPSILON;
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn charge_helpers_cover_common_relationships() {
        assert_eq!(charge_from_current_time(2.0, 3.0), Some(6.0));
        assert_eq!(charge_from_current_time(2.0, -1.0), None);
        assert_eq!(current_from_charge_time(10.0, 2.0), Some(5.0));
        assert_eq!(current_from_charge_time(10.0, 0.0), None);
    }

    #[test]
    fn ohms_law_helpers_cover_common_relationships() {
        assert_eq!(voltage(2.0, 5.0), Some(10.0));
        assert_eq!(voltage(2.0, -5.0), None);
        assert_eq!(current(10.0, 5.0), Some(2.0));
        assert_eq!(current(10.0, 0.0), None);
        assert_eq!(resistance(10.0, 2.0), Some(5.0));
        assert_eq!(resistance(10.0, 0.0), None);
        assert_eq!(resistance(-10.0, 2.0), None);
    }

    #[test]
    fn conductance_helpers_cover_common_relationships() {
        assert_some_close(conductance(5.0), 0.2);
        assert_eq!(conductance(0.0), None);
        assert_some_close(resistance_from_conductance(0.2), 5.0);
        assert_eq!(resistance_from_conductance(0.0), None);
    }

    #[test]
    fn power_helpers_cover_common_relationships() {
        assert_eq!(power_from_voltage_current(10.0, 2.0), Some(20.0));
        assert_eq!(power_from_current_resistance(2.0, 5.0), Some(20.0));
        assert_eq!(power_from_voltage_resistance(10.0, 5.0), Some(20.0));
    }

    #[test]
    fn energy_helpers_cover_common_relationships() {
        assert_eq!(energy_from_power_time(20.0, 3.0), Some(60.0));
        assert_eq!(energy_from_power_time(20.0, -1.0), None);
        assert_eq!(energy_from_voltage_charge(10.0, 2.0), Some(20.0));
    }

    #[test]
    fn resistance_network_helpers_cover_common_relationships() {
        assert_eq!(series_resistance(&[1.0, 2.0, 3.0]), Some(6.0));
        assert_eq!(series_resistance(&[]), Some(0.0));
        assert_eq!(series_resistance(&[1.0, -2.0]), None);

        assert_eq!(parallel_resistance(&[2.0, 2.0]), Some(1.0));
        assert_eq!(parallel_resistance(&[]), None);
        assert_eq!(parallel_resistance(&[2.0, 0.0]), None);
    }

    #[test]
    fn coulomb_force_helpers_cover_common_relationships() {
        assert_some_close(coulomb_force(1.0, 1.0, 1.0), COULOMB_CONSTANT);
        assert_some_close(coulomb_force(1.0, -1.0, 1.0), -COULOMB_CONSTANT);
        assert_eq!(coulomb_force(1.0, 1.0, 0.0), None);
    }

    #[test]
    fn electrical_load_delegates_to_public_helpers() {
        let load = ElectricalLoad::new(10.0, 5.0).expect("valid load");

        assert_eq!(load.current(), Some(2.0));
        assert_eq!(load.power(), Some(20.0));
        assert_eq!(ElectricalLoad::new(10.0, 0.0), None);
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert_eq!(charge_from_current_time(f64::NAN, 1.0), None);
        assert_eq!(current_from_charge_time(1.0, f64::INFINITY), None);
        assert_eq!(power_from_voltage_current(f64::INFINITY, 1.0), None);
        assert_eq!(series_resistance(&[1.0, f64::NAN]), None);
    }
}
