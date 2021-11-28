use crate::configuration::error::Error;
use num_traits::FromPrimitive;

use super::{
    Configuration, FastFilterThreshold, Hysteresis, OutputStage, PowerMode, PwmFreq,
    SlowFilterMode, WatchdogState,
};

impl TryFrom<u8> for PowerMode {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /* & 0b0000_0011*/).ok_or(Error::PowerModeBitPattern(byte))
    }
}

impl From<PowerMode> for u8 {
    fn from(mode: PowerMode) -> Self {
        mode as Self
    }
}

impl TryFrom<u8> for Hysteresis {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /* & 0b0000_0011*/).ok_or(Error::HysteresisBitPattern(byte))
    }
}

impl From<Hysteresis> for u8 {
    fn from(hyst: Hysteresis) -> Self {
        hyst as Self
    }
}

impl TryFrom<u8> for OutputStage {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte & 0b0000_0011 {
            0b00 => Ok(Self::Analog),
            0b01 => Ok(Self::ReducedAnalog),
            0b10 => Ok(Self::DigitalPwm),
            0b11 => Err(Error::OutputStageBitPattern(byte)),
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<OutputStage> for u8 {
    fn from(stage: OutputStage) -> Self {
        stage as Self
    }
}

impl TryFrom<u8> for PwmFreq {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /* & 0b0000_0011*/).ok_or(Error::PwmFreqBitPattern(byte))
    }
}

impl From<PwmFreq> for u8 {
    fn from(freq: PwmFreq) -> Self {
        freq as Self
    }
}

impl TryFrom<u8> for SlowFilterMode {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /*& 0b0000_0011*/).ok_or(Error::SlowFilterModeBitPattern(byte))
    }
}

impl From<SlowFilterMode> for u8 {
    fn from(filter: SlowFilterMode) -> Self {
        filter as Self
    }
}

impl TryFrom<u8> for FastFilterThreshold {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /*& 0b0000_0111*/)
            .ok_or(Error::FastFilterThresholdBitPattern(byte))
    }
}

impl From<FastFilterThreshold> for u8 {
    fn from(fth: FastFilterThreshold) -> Self {
        fth as Self
    }
}

impl TryFrom<u8> for WatchdogState {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(byte /*& 0b0000_0001*/).ok_or(Error::WatchdogState(byte))
    }
}

impl From<WatchdogState> for u8 {
    fn from(state: WatchdogState) -> Self {
        state as Self
    }
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
            power_mode: pm.try_into()?,
            hysteresis: hyst.try_into()?,
            output_stage: outs.try_into()?,
            pwm_frequency: pwmf.try_into()?,
            slow_filter: sf.try_into()?,
            fast_filter_threshold: fth.try_into()?,
            watchdog_state: wd.try_into()?,
            fields: bytes,
        })
    }
}

impl From<Configuration> for u16 {
    fn from(config: Configuration) -> Self {
        let mut fields = 0;
        let power_mode_bits = u8::from(config.power_mode) as Self;
        fields |= power_mode_bits;
        let hyst_bits = (u8::from(config.hysteresis) as Self) << 2;
        fields |= hyst_bits;
        let outs_bits = (u8::from(config.output_stage) as Self) << 4;
        fields |= outs_bits;
        let pwmf_bits = (u8::from(config.pwm_frequency) as Self) << 6;
        fields |= pwmf_bits;
        let sf_bits = (u8::from(config.slow_filter) as Self) << 8;
        fields |= sf_bits;
        let fth_bits = (u8::from(config.fast_filter_threshold) as Self) << 10;
        fields |= fth_bits;
        let wd_bits = (u8::from(config.watchdog_state) as Self) << 13;
        fields |= wd_bits;
        // Restore 2 top-most bits.
        fields |= config.fields & 0b1100_0000_0000_0000;
        fields
    }
}
