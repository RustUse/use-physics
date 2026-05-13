#![allow(clippy::float_cmp)]

use use_force::{force, weight};

#[test]
fn force_helpers_cover_force_and_weight() {
    assert_eq!(force(10.0, 2.0), 20.0);
    assert_eq!(weight(2.0, 10.0), 20.0);
}
