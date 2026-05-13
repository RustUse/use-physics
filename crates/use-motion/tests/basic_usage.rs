#![allow(clippy::float_cmp)]

use use_motion::{average_speed, distance};

#[test]
fn motion_helpers_cover_distance_and_speed() -> Result<(), use_motion::MotionError> {
    assert_eq!(distance(5.0, 4.0), 20.0);
    assert_eq!(average_speed(100.0, 10.0)?, 10.0);
    Ok(())
}
