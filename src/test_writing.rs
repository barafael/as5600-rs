use crate::{
    configuration::{
        Configuration, FastFilterThreshold, Hysteresis, OutputStage, PowerMode, PwmFreq,
        SlowFilterMode, WatchdogState,
    },
    error::Error,
    As5600,
};
use embedded_hal_mock::eh1::{
    delay::NoopDelay,
    i2c::{Mock, Transaction},
};

#[test]
fn set_zero_position() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x01, 0x0A, 0xAF]),
        Transaction::write(0x36, vec![0x01, 0x01, 0x10]),
        Transaction::write(0x36, vec![0x01, 0x0A, 0xCA]),
        Transaction::write(0x36, vec![0x01, 0x01, 0x0B]),
    ]);
    let mut as5600 = As5600::new(i2c);
    for angle in [0x1AAF, 0x0110, 0x0ACA, 0x010B] {
        as5600.set_zero_position(angle).unwrap();
    }
    as5600.release().done();
}

#[test]
fn set_maximum_position() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x03, 0x0F, 0xAF]),
        Transaction::write(0x36, vec![0x03, 0x00, 0x10]),
        Transaction::write(0x36, vec![0x03, 0x0F, 0xAF]),
        Transaction::write(0x36, vec![0x03, 0x01, 0x00]),
    ]);
    let mut as5600 = As5600::new(i2c);
    for angle in [0xAFAF, 0x2010, 0x1FAF, 0x1100] {
        as5600.set_maximum_position(angle).unwrap();
    }
    as5600.release().done();
}

#[test]
fn set_maximum_angle() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x05, 0x0F, 0xFA]),
        Transaction::write(0x36, vec![0x05, 0x00, 0x01]),
        Transaction::write(0x36, vec![0x05, 0x0F, 0xFA]),
        Transaction::write(0x36, vec![0x05, 0x00, 0x00]),
    ]);
    let mut as5600 = As5600::new(i2c);
    for angle in [0x0FFA, 0x0001, 0xAFFA, 0x1000] {
        as5600.set_maximum_angle(angle).unwrap();
    }
    as5600.release().done();
}

#[test]
fn set_config() {
    let config = Configuration {
        power_mode: PowerMode::Lpm1,
        hysteresis: Hysteresis::Lsb2,
        output_stage: OutputStage::DigitalPwm,
        pwm_frequency: PwmFreq::PwmF2,
        slow_filter: SlowFilterMode::X2,
        fast_filter_threshold: FastFilterThreshold::Lsb21,
        watchdog_state: WatchdogState::On,
    };
    let config_bytes: [u8; 2] = u16::from(config).to_be_bytes();
    let top_most_set = config_bytes[0] | 0b1000_0000;

    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x07], vec![top_most_set, config_bytes[1]]),
        Transaction::write(0x36, vec![0x07, top_most_set, config_bytes[1]]),
    ]);
    let mut as5600 = As5600::new(i2c);
    as5600.set_config(config).unwrap();
    as5600.release().done();
}

#[test]
fn burn_angle_succeeds() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0001]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x20]),
        Transaction::write(0x36, vec![0xFF, 0x80]),
    ]);
    let mut delay = NoopDelay;
    let mut as5600 = As5600::new(i2c);
    as5600.persist_position_settings(&mut delay).unwrap();
    as5600.release().done();
}

#[test]
fn burn_angle_fails_due_to_zmco() {
    let i2c = Mock::new(&[Transaction::write_read(0x36, vec![0x00], vec![0b0000_0011])]);
    let mut delay = NoopDelay;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(
        as5600.persist_position_settings(&mut delay).unwrap_err(),
        Error::MaximumPositionPersistsReached
    );
    as5600.release().done();
}

#[test]
fn burn_angle_fails_due_to_magnet_detection() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0001]),
        Transaction::write_read(0x36, vec![0x0b], vec![0x10]),
    ]);
    let mut delay = NoopDelay;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(
        as5600.persist_position_settings(&mut delay).unwrap_err(),
        Error::MagnetRequired
    );
    as5600.release().done();
}

#[test]
fn burn_settings_succeeds() {
    let i2c = Mock::new(&[
        Transaction::write_read(0x36, vec![0x00], vec![0b0000_0000]),
        Transaction::write(0x36, vec![0xFF, 0x40]),
    ]);
    let mut delay = NoopDelay;
    let mut as5600 = As5600::new(i2c);
    as5600
        .persist_maximum_angle_and_config_settings(&mut delay)
        .unwrap();
    as5600.release().done();
}

#[test]
fn burn_settings_fails_when_zmco_is_not_zero() {
    let i2c = Mock::new(&[Transaction::write_read(0x36, vec![0x00], vec![0b0000_0001])]);
    let mut delay = NoopDelay;
    let mut as5600 = As5600::new(i2c);
    assert_eq!(
        as5600
            .persist_maximum_angle_and_config_settings(&mut delay)
            .unwrap_err(),
        Error::MangConfigPersistenceExhausted
    );
    as5600.release().done();
}
