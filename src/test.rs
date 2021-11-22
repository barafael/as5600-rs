use crate::{
    error,
    status::{self, Status},
    As5600,
};

use embedded_hal_mock::i2c::{Mock, Transaction};

#[test]
fn construct_then_release_is_noop() {
    let i2c = Mock::new(&[]);
    let delay = embedded_hal_mock::delay::MockNoop;
    let as5600 = As5600::new(i2c, 12, delay);
    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn detects_magnet() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x0b], vec![0x10]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x8]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x40]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
    ]);
    let expected_status = [
        Ok(Status::MagnetLow),
        Ok(Status::MagnetHigh),
        Ok(Status::MagnetDetected),
        Err(error::Error::Status(status::Error::InvalidBitPattern(0x40))),
        Ok(Status::MagnetDetected),
    ];
    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    assert!((0..5)
        .into_iter()
        .map(|_| as5600.magnet_status())
        .zip(expected_status)
        .all(|(a, b)| a == b));

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}
