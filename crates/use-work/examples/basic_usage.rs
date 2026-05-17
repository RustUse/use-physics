#![allow(clippy::float_cmp)]

use use_work::{ConstantForceWork, work, work_at_angle_degrees};

fn main() {
    assert_eq!(work(10.0, 2.0), Some(20.0));
    assert_eq!(
        ConstantForceWork::new(10.0, 2.0).and_then(|constant| constant.work()),
        Some(20.0)
    );
    assert!(matches!(
        work_at_angle_degrees(10.0, 2.0, 60.0),
        Some(value) if (value - 10.0).abs() < 1e-12
    ));
}
