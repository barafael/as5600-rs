use crate::As5600;

use embedded_hal_mock::i2c::Mock;

#[test]
fn construct_then_release_is_noop() {
    let i2c = Mock::new(&[]);
    let delay = embedded_hal_mock::delay::MockNoop;
    let as5600 = As5600::new(i2c, 12, delay);
    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}
