use crate::common::{initialize_test, TestParameters};

#[test]
fn device_initialization() {
    initialize_test(
        TestParameters::default(),
        |_| wgt::Limits::downlevel_webgl2_defaults(),
        |_ctx| {
            // intentionally empty
        },
    )
}
