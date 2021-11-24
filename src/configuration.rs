#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Debug, PartialEq)]
pub enum Error {
    OutputStageBitPattern(u8),
}

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

impl From<u8> for PowerMode {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0011 {
            0b00 => Self::Nom,
            0b01 => Self::Lpm1,
            0b10 => Self::Lpm2,
            0b11 => Self::Lpm3,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<PowerMode> for u8 {
    fn from(mode: PowerMode) -> Self {
        match mode {
            PowerMode::Nom => 0b00,
            PowerMode::Lpm1 => 0b01,
            PowerMode::Lpm2 => 0b10,
            PowerMode::Lpm3 => 0b11,
        }
    }
}

impl From<u8> for Hysteresis {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0011 {
            0b00 => Self::Off,
            0b01 => Self::Lsb1,
            0b10 => Self::Lsb2,
            0b11 => Self::Lsb3,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<Hysteresis> for u8 {
    fn from(hyst: Hysteresis) -> Self {
        match hyst {
            Hysteresis::Off => 0b00,
            Hysteresis::Lsb1 => 0b01,
            Hysteresis::Lsb2 => 0b10,
            Hysteresis::Lsb3 => 0b11,
        }
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
        match stage {
            OutputStage::Analog => 0b00,
            OutputStage::ReducedAnalog => 0b01,
            OutputStage::DigitalPwm => 0b10,
        }
    }
}

impl From<u8> for PwmFreq {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0011 {
            0b00 => Self::PwmF1,
            0b01 => Self::PwmF2,
            0b10 => Self::PwmF3,
            0b11 => Self::PwmF4,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

// impl From<PwmFreq> for embedded_time::rate::Hz // TODO

impl From<PwmFreq> for u8 {
    fn from(freq: PwmFreq) -> Self {
        match freq {
            PwmFreq::PwmF1 => 0b00,
            PwmFreq::PwmF2 => 0b01,
            PwmFreq::PwmF3 => 0b10,
            PwmFreq::PwmF4 => 0b11,
        }
    }
}

impl From<u8> for SlowFilter {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0011 {
            0b00 => Self::X16,
            0b01 => Self::X8,
            0b10 => Self::X4,
            0b11 => Self::X2,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<SlowFilter> for u8 {
    fn from(filter: SlowFilter) -> Self {
        match filter {
            SlowFilter::X16 => 0b00,
            SlowFilter::X8 => 0b01,
            SlowFilter::X4 => 0b10,
            SlowFilter::X2 => 0b11,
        }
    }
}

impl From<u8> for FastFilterThreshold {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0111 {
            0b000 => Self::SlowFilterOnly,
            0b001 => Self::Lsb6,
            0b010 => Self::Lsb7,
            0b011 => Self::Lsb9,
            0b100 => Self::Lsb18,
            0b101 => Self::Lsb21,
            0b110 => Self::Lsb24,
            0b111 => Self::Lsb10,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<FastFilterThreshold> for u8 {
    fn from(fth: FastFilterThreshold) -> Self {
        match fth {
            FastFilterThreshold::SlowFilterOnly => 0b000,
            FastFilterThreshold::Lsb6 => 0b001,
            FastFilterThreshold::Lsb7 => 0b010,
            FastFilterThreshold::Lsb9 => 0b011,
            FastFilterThreshold::Lsb18 => 0b100,
            FastFilterThreshold::Lsb21 => 0b101,
            FastFilterThreshold::Lsb24 => 0b110,
            FastFilterThreshold::Lsb10 => 0b111,
        }
    }
}

impl From<u8> for WatchdogState {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0001 {
            0 => Self::Off,
            1 => Self::On,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

impl From<WatchdogState> for u8 {
    fn from(state: WatchdogState) -> Self {
        match state {
            WatchdogState::Off => 0,
            WatchdogState::On => 1,
        }
    }
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
