use core::time::Duration;

pub const DEFAULT_I2C_ADDRESS: u8 = 0x36;

pub const WATCHDOG_TIMEOUT: Duration = Duration::from_secs(60);
pub const POWER_UP_TIME: Duration = Duration::from_millis(10);

pub const SAMPLE_RATE: Duration = Duration::from_micros(150);

pub const SETTLING_TIME_1: Duration = Duration::from_micros(2200);
pub const SETTLING_TIME_2: Duration = Duration::from_micros(1100);
pub const SETTLING_TIME_3: Duration = Duration::from_micros(550);
pub const SETTLING_TIME_4: Duration = Duration::from_micros(286);

pub const APPLY_SETTINGS: Duration = Duration::from_millis(1);
