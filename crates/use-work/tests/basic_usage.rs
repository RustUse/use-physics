#![allow(clippy::float_cmp)]

use use_work::{ConstantForceWork, net_work, work};

#[test]
fn work_helpers_cover_constant_force_and_net_work() {
    let constant = ConstantForceWork::new(10.0, 2.0).unwrap();

    assert_eq!(work(10.0, 2.0), Some(20.0));
    assert_eq!(constant.work(), Some(20.0));
    assert_eq!(net_work(&[20.0, -5.0, 3.0]), Some(18.0));
}
