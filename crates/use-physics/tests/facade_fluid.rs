#![allow(clippy::float_cmp)]

#[cfg(feature = "fluid")]
#[test]
fn facade_reexports_fluid_workflow() {
    use use_physics::{Fluid, PipeFlow, dynamic_pressure, fluid};

    let water = Fluid::with_dynamic_viscosity(1000.0, 0.001).unwrap();
    let flow = PipeFlow::new(2.0, 3.0).unwrap();

    assert_eq!(flow.volumetric_flow_rate(), Some(6.0));
    assert_eq!(flow.mass_flow_rate(water.density), Some(6000.0));
    assert_eq!(dynamic_pressure(water.density, 3.0), Some(4500.0));
    assert_eq!(
        fluid::hydrostatic_pressure(1000.0, 10.0, 2.0),
        Some(20_000.0)
    );
    assert!((water.reynolds_number(2.0, 0.1).unwrap() - 200_000.0).abs() < 1.0e-9);
}

#[cfg(all(feature = "fluid", feature = "pressure"))]
#[test]
fn facade_aliases_fluid_hydrostatic_pressure_when_pressure_is_enabled() {
    use use_physics::{fluid_hydrostatic_pressure, hydrostatic_pressure};

    assert_eq!(hydrostatic_pressure(1000.0, 10.0, 2.0), 20_000.0);
    assert_eq!(
        fluid_hydrostatic_pressure(1000.0, 10.0, 2.0),
        Some(20_000.0)
    );
}
