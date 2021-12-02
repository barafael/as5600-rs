use core::time::Duration;

/// Default i2c address of AS5600.
pub const DEFAULT_I2C_ADDRESS: u8 = 0x36;

/// Watchdog timeout duration (before it changes power modes).
pub const WATCHDOG_TIMEOUT: Duration = Duration::from_secs(60);
/// Time to power up AS5600.
pub const POWER_UP_TIME: Duration = Duration::from_millis(10);

/// Sampling rate in normal power mode.
pub const SAMPLE_RATE: Duration = Duration::from_micros(150);

/// Settling time 1.
pub const SETTLING_TIME_1: Duration = Duration::from_micros(2200);
/// Settling time 2.
pub const SETTLING_TIME_2: Duration = Duration::from_micros(1100);
/// Settling time 3.
pub const SETTLING_TIME_3: Duration = Duration::from_micros(550);
/// Settling time 4.
pub const SETTLING_TIME_4: Duration = Duration::from_micros(286);
