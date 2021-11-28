#[cfg(test)]
use proptest_derive::Arbitrary;

mod conversion;
pub mod error;
#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum PowerMode {
    Nom,
    Lpm1,
    Lpm2,
    Lpm3,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Hysteresis {
    Off,
    Lsb1,
    Lsb2,
    Lsb3,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum OutputStage {
    Analog,
    ReducedAnalog,
    DigitalPwm,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum PwmFreq {
    PwmF1 = 115,
    PwmF2 = 230,
    PwmF3 = 460,
    PwmF4 = 920,
}

impl PwmFreq {
    pub const fn to_hz(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum SlowFilterMode {
    X16,
    X8,
    X4,
    X2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum FastFilterThreshold {
    SlowFilterOnly,
    Lsb6,
    Lsb7,
    Lsb9,
    Lsb18,
    Lsb21,
    Lsb24,
    Lsb10,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum WatchdogState {
    On,
    Off,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Configuration {
    pub power_mode: PowerMode,
    pub hysteresis: Hysteresis,
    pub output_stage: OutputStage,
    pub pwm_frequency: PwmFreq,
    pub slow_filter: SlowFilterMode,
    pub fast_filter_threshold: FastFilterThreshold,
    pub watchdog_state: WatchdogState,
    pub fields: u16, // See note in datasheet about "blank fields may contain factory settings" on page 18
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.power_mode == other.power_mode
            && self.hysteresis == other.hysteresis
            && self.output_stage == other.output_stage
            && self.pwm_frequency == other.pwm_frequency
            && self.slow_filter == other.slow_filter
            && self.fast_filter_threshold == other.fast_filter_threshold
            && self.watchdog_state == other.watchdog_state
            && (self.fields & 0b1100_0000_0000_0000) == (other.fields & 0b1100_0000_0000_0000)
    }
}
