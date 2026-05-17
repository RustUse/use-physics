#![allow(clippy::float_cmp)]

use use_fluid::{Fluid, PipeFlow, drag_force, dynamic_pressure};

fn approx_eq(left: f64, right: f64, tolerance: f64) {
    let delta = (left - right).abs();

    assert!(
        delta <= tolerance,
        "left={left} right={right} delta={delta} tolerance={tolerance}"
    );
}

#[test]
fn fluid_and_pipe_flow_cover_common_workflow() {
    let water = Fluid::with_dynamic_viscosity(1000.0, 0.001).unwrap();
    let flow = PipeFlow::new(2.0, 3.0).unwrap();

    assert_eq!(flow.volumetric_flow_rate(), Some(6.0));
    assert_eq!(flow.mass_flow_rate(water.density), Some(6000.0));
    assert_eq!(dynamic_pressure(water.density, 3.0), Some(4500.0));

    approx_eq(water.reynolds_number(2.0, 0.1).unwrap(), 200_000.0, 1.0e-9);
    approx_eq(drag_force(1.225, 10.0, 0.47, 1.0).unwrap(), 28.7875, 1.0e-9);
}
