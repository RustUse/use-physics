#![allow(clippy::float_cmp)]

use use_motion::{average_speed, final_velocity};

fn main() -> Result<(), use_motion::MotionError> {
    let speed = average_speed(100.0, 10.0)?;

    assert_eq!(speed, 10.0);
    assert_eq!(final_velocity(2.0, 3.0, 4.0), 14.0);

    Ok(())
}
