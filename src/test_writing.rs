use crate::{
    configuration::{
        Configuration, FastFilterThreshold, Hysteresis, OutputStage, PowerMode, PwmFreq,
        SlowFilterMode, WatchdogState,
    },
    As5600,
};
use embedded_hal_mock::i2c::{Mock, Transaction};

#[test]
fn set_zero_position() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x01, 0x0A, 0xAF]),
        Transaction::write(0x36, vec![0x01, 0x01, 0x10]),
        Transaction::write(0x36, vec![0x01, 0x0A, 0xCA]),
        Transaction::write(0x36, vec![0x01, 0x01, 0x0B]),
    ]);

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    for angle in [0x1AAF, 0x0110, 0x0ACA, 0x010B] {
        as5600.set_zero_position(angle).unwrap();
    }

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn set_maximum_position() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x03, 0x0F, 0xAF]),
        Transaction::write(0x36, vec![0x03, 0x00, 0x10]),
        Transaction::write(0x36, vec![0x03, 0x0F, 0xAF]),
        Transaction::write(0x36, vec![0x03, 0x01, 0x00]),
    ]);

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    for angle in [0xAFAF, 0x2010, 0x1FAF, 0x1100] {
        as5600.set_maximum_position(angle).unwrap();
    }

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}

#[test]
fn set_maximum_angle() {
    let i2c = Mock::new(&[
        Transaction::write(0x36, vec![0x05, 0x0F, 0xFA]),
        Transaction::write(0x36, vec![0x05, 0x00, 0x01]),
        Transaction::write(0x36, vec![0x05, 0x0F, 0xFA]),
        Transaction::write(0x36, vec![0x05, 0x00, 0x00]),
    ]);

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    for angle in [0x0FFA, 0x0001, 0xAFFA, 0x1000] {
        as5600.set_maximum_angle(angle).unwrap();
    }

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
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

    let delay = embedded_hal_mock::delay::MockNoop;
    let mut as5600 = As5600::new(i2c, 0x36, delay);

    as5600.set_config(config).unwrap();

    let (mut i2c, _delay) = as5600.release();
    i2c.done();
}
