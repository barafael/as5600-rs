#[cfg(test)]
use proptest_derive::Arbitrary;

use self::error::Error;

mod conversion;
pub mod error;
#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum PowerMode {
    Nom,
    Lpm1,
    Lpm2,
    Lpm3,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Hysteresis {
    Off,
    Lsb1,
    Lsb2,
    Lsb3,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum OutputStage {
    Analog,
    ReducedAnalog,
    DigitalPwm,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum PwmFreq {
    PwmF1 = 115,
    PwmF2 = 230,
    PwmF3 = 460,
    PwmF4 = 920,
}

impl PwmFreq {
    pub fn to_hz(freq: PwmFreq) -> usize {
        freq as usize
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum SlowFilter {
    X16,
    X8,
    X4,
    X2,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum WatchdogState {
    On,
    Off,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Configuration {
    pub power_mode: PowerMode,
    pub hysteresis: Hysteresis,
    pub output_stage: OutputStage,
    pub pwm_frequency: PwmFreq,
    pub slow_filter: SlowFilter,
    pub fast_filter_threshold: FastFilterThreshold,
    pub watchdog_state: WatchdogState,
    pub fields: u16, // See note in datasheet about "blank fields may contain factory settings" on page 18
}

impl TryFrom<u16> for Configuration {
    type Error = Error;
    fn try_from(bytes: u16) -> Result<Self, Self::Error> {
        let pm = (bytes & 0b0000_0000_0000_0011) as u8;
        let hyst = ((bytes & 0b0000_0000_0000_1100) >> 2) as u8;
        let outs = ((bytes & 0b0000_0000_0011_0000) >> 4) as u8;
        let pwmf = ((bytes & 0b0000_0000_1100_0000) >> 6) as u8;
        let sf = ((bytes & 0b0000_0011_0000_0000) >> 8) as u8;
        let fth = ((bytes & 0b0001_1100_0000_0000) >> 10) as u8;
        let wd = ((bytes & 0b0010_0000_0000_0000) >> 13) as u8;
        Ok(Self {
            power_mode: pm.into(),
            hysteresis: hyst.into(),
            output_stage: outs.try_into()?,
            pwm_frequency: pwmf.into(),
            slow_filter: sf.into(),
            fast_filter_threshold: fth.into(),
            watchdog_state: wd.into(),
            fields: bytes,
        })
    }
}

impl From<Configuration> for u16 {
    fn from(config: Configuration) -> Self {
        let mut fields: u16 = 0;
        let power_mode_bits = u8::from(config.power_mode) as u16;
        fields |= power_mode_bits;
        let hyst_bits = (u8::from(config.hysteresis) as u16) << 2;
        fields |= hyst_bits;
        let outs_bits = (u8::from(config.output_stage) as u16) << 4;
        fields |= outs_bits;
        let pwmf_bits = (u8::from(config.pwm_frequency) as u16) << 6;
        fields |= pwmf_bits;
        let sf_bits = (u8::from(config.slow_filter) as u16) << 8;
        fields |= sf_bits;
        let fth_bits = (u8::from(config.fast_filter_threshold) as u16) << 10;
        fields |= fth_bits;
        let wd_bits = (u8::from(config.watchdog_state) as u16) << 13;
        fields |= wd_bits;
        // Restore 2 top-most bits.
        fields |= config.fields & 0b1100_0000_0000_0000;
        fields
    }
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
