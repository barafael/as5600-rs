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

    expected_status
        .iter()
        .map(|s| (as5600.magnet_status(), s))
        .all(|(a, b)| a == *b);

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn get_zmco() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0000]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0001]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0010]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0011]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0100]),
    ]);

    let expected_status = [0, 1, 2, 3, 0];
    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    expected_status
        .iter()
        .map(|s| (as5600.get_zmco(), *s))
        .all(|(a, b)| a == Ok(b));

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn get_zero_position() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x01],
        vec![0b1001_1010, 0b1010_1111],
    )]);

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    assert_eq!(0b0000_1010_1010_1111, as5600.get_zero_position().unwrap());

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn get_maximum_position() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x03],
        vec![0b1101_0010, 0b0010_1010],
    )]);

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    assert_eq!(
        0b0000_0010_0010_1010,
        as5600.get_maximum_position().unwrap()
    );

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}
