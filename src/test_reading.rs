use crate::{
    configuration::{
        Configuration, FastFilterThreshold, Hysteresis, OutputStage, PowerMode, PwmFreq,
        SlowFilterMode, WatchdogState,
    },
    error,
    status::{self, Status},
    As5600,
};
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

#[test]
fn detects_magnet() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x0b], vec![0x10]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x8]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x28]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x40]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x18]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x30]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x38]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
    ]);
    let expected_status = [
        Ok(Status::MagnetLow),
        Ok(Status::MagnetHigh),
        Ok(Status::MagnetDetectedHigh),
        Ok(Status::MagnetDetected),
        Err(error::Error::Status(status::Error::InvalidBitPattern(0x40))),
        Err(error::Error::Status(status::Error::InvalidBitPattern(0x18))),
        Ok(Status::MagnetDetected),
        Ok(Status::MagnetDetectedLow),
        Err(error::Error::Status(status::Error::InvalidBitPattern(0x38))),
        Ok(Status::MagnetDetected),
    ];
    let mut as5600 = As5600::new(i2c);
    expected_status
        .iter()
        .map(|s| (as5600.magnet_status(), s))
        .all(|(a, b)| a == *b);
    as5600.release().done();
}

#[test]
fn reads_zmco_register() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0000]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0001]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0010]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0011]),
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0100]),
    ]);
    let expected_status = [0, 1, 2, 3, 0];
    let mut as5600 = As5600::new(i2c);
    expected_status
        .iter()
        .map(|s| (as5600.zmco(), *s))
        .all(|(a, b)| a == Ok(b));
    as5600.release().done();
}

#[test]
fn reads_zero_position_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x01],
        vec![0b1001_1010, 0b1010_1111],
    )]);
    let mut as5600 = As5600::new(i2c);
    assert_eq!(0b0000_1010_1010_1111, as5600.zero_position().unwrap());
    as5600.release().done();
}

#[test]
fn reads_maximum_position_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x03],
        vec![0b1101_0010, 0b0010_1010],
    )]);
    let mut as5600 = As5600::new(i2c);
    assert_eq!(0b0000_0010_0010_1010, as5600.maximum_position().unwrap());
    as5600.release().done();
}

#[test]
fn reads_maximum_angle_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x05],
        vec![0b0001_1110, 0b1010_1011],
    )]);
    let mut as5600 = As5600::new(i2c);
    assert_eq!(0b0000_1110_1010_1011, as5600.maximum_angle().unwrap());
    as5600.release().done();
}

#[test]
fn reads_config_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x07],
        vec![0b1110_0011, 0b1010_1100],
    )]);
    let expected_config = Configuration {
        power_mode: PowerMode::Nom,
        hysteresis: Hysteresis::Lsb3,
        output_stage: OutputStage::DigitalPwm,
        pwm_frequency: PwmFreq::PwmF3,
        slow_filter: SlowFilterMode::X2,
        fast_filter_threshold: FastFilterThreshold::SlowFilterOnly,
        watchdog_state: WatchdogState::On,
    };
    let mut as5600 = As5600::new(i2c);
    assert_eq!(expected_config, as5600.config().unwrap());
    as5600.release().done();
}

#[test]
fn reads_raw_angle_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x0c],
        vec![0b1110_0001, 0b0010_0011],
    )]);
    let expected_angle = 0x0123;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(expected_angle, as5600.raw_angle().unwrap());
    as5600.release().done();
}

#[test]
fn reads_angle_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x0e],
        vec![0b1110_1000, 0b0100_0010],
    )]);
    let expected_angle = 0x0842;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(expected_angle, as5600.angle().unwrap());
    as5600.release().done();
}

#[test]
fn reads_automatic_gain_control_register() {
    let i2c = Mock::new(&[Transaction::write_read(0x36, vec![0x1a], vec![0b0101_1010])]);
    let expected_agc = 0b0101_1010;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(expected_agc, as5600.automatic_gain_control().unwrap());
    as5600.release().done();
}

#[test]
fn reads_magnitude_register() {
    let i2c = Mock::new(&[Transaction::write_read(
        0x36,
        vec![0x1b],
        vec![0b0101_1010, 0b1101_0101],
    )]);

    let expected_magnitude = 0b0000_1010_1101_0101;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(expected_magnitude, as5600.magnitude().unwrap());
    as5600.release().done();
}
