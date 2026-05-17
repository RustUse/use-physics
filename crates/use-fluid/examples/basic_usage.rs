#![allow(clippy::float_cmp)]

use use_fluid::{Fluid, PipeFlow, drag_force};

fn main() {
    let Some(water) = Fluid::with_dynamic_viscosity(1000.0, 0.001) else {
        unreachable!("valid fluid inputs")
    };
    let Some(flow) = PipeFlow::new(2.0, 3.0) else {
        unreachable!("valid pipe-flow inputs")
    };

    assert_eq!(flow.volumetric_flow_rate(), Some(6.0));
    assert_eq!(flow.mass_flow_rate(water.density), Some(6000.0));
    assert!(
        water
            .reynolds_number(2.0, 0.1)
            .is_some_and(|reynolds| (reynolds - 200_000.0).abs() < 1.0e-9)
    );
    assert!(
        drag_force(1.225, 10.0, 0.47, 1.0).is_some_and(|force| (force - 28.7875).abs() < 1.0e-9)
    );
}
