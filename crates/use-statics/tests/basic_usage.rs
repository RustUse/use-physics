#![allow(clippy::float_cmp)]

use use_statics::{
    CantileverReaction, Force2D, StaticSystem2D, cantilever_end_point_load_reaction,
    simply_supported_point_load_reactions,
};

fn approx_eq(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "expected {left} to be within {tolerance} of {right}"
    );
}

#[test]
fn basic_usage_covers_equilibrium_and_support_reactions() {
    let Some(force_a) = Force2D::new(1.0, 2.0) else {
        panic!("valid force should construct");
    };
    let Some(force_b) = Force2D::new(-1.0, -2.0) else {
        panic!("valid force should construct");
    };

    let Some(system) = StaticSystem2D::new(vec![force_a, force_b], vec![10.0, -10.0]) else {
        panic!("valid system should construct");
    };

    assert_eq!(system.is_equilibrium(0.0), Some(true));

    let Some((left, right)) = simply_supported_point_load_reactions(10.0, 100.0, 5.0) else {
        panic!("valid point load should produce reactions");
    };

    approx_eq(left, 50.0, 1.0e-12);
    approx_eq(right, 50.0, 1.0e-12);

    assert_eq!(
        cantilever_end_point_load_reaction(10.0, 100.0),
        Some(CantileverReaction {
            vertical_reaction: 100.0,
            fixed_end_moment: 1000.0,
        })
    );
}
