#[derive(Debug, PartialEq)]
pub enum PowerMode {
    Nom,
    Lpm1,
    Lpm2,
    Lpm3,
}

#[derive(Debug, PartialEq)]
pub enum Hysteresis {
    Off,
    Lsb1,
    Lsb2,
    Lsb3,
}

#[derive(Debug, PartialEq)]
pub enum OutputStage {
    Analog,
    ReducedAnalog,
    DigitalPwm,
}

#[derive(Debug, PartialEq)]
pub enum PwmFreq {
    PwmF1 = 115,
    PwmF2 = 230,
    PwmF3 = 460,
    PwmF4 = 920,
}

#[derive(Debug, PartialEq)]
pub enum SlowFilter {
    X16,
    X8,
    X4,
    X2,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

impl From<u8> for OutputStage {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0011 {
            0b00 => Self::Analog,
            0b01 => Self::ReducedAnalog,
            0b10 => Self::DigitalPwm,
            0b11 => panic!("Invalid bit pattern for output stage"), // TODO make it TryFrom, then
            _ => unreachable!("Bit pattern above eliminates all other bits"),
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

impl From<u8> for WatchdogState {
    fn from(byte: u8) -> Self {
        match byte & 0b0000_0001 {
            0 => WatchdogState::Off,
            1 => WatchdogState::On,
            _ => unreachable!("Bit pattern above eliminates all other bits"),
        }
    }
}

#[derive(Debug, PartialEq)]
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

impl From<u16> for Configuration {
    fn from(bytes: u16) -> Self {
        let pm = (bytes & 0b0000_0000_0000_0011) as u8;
        let hyst = ((bytes & 0b0000_0000_0000_1100) >> 2) as u8;
        let outs = ((bytes & 0b0000_0000_0011_0000) >> 4) as u8;
        let pwmf = ((bytes & 0b0000_0000_1100_0000) >> 6) as u8;
        let sf = ((bytes & 0b0000_0011_0000_0000) >> 8) as u8;
        let fth = ((bytes & 0b0001_1100_0000_0000) >> 10) as u8;
        let wd = ((bytes & 0b0010_0000_0000_0000) >> 13) as u8;
        Configuration {
            power_mode: pm.into(),
            hysteresis: hyst.into(),
            output_stage: outs.into(),
            pwm_frequency: pwmf.into(),
            slow_filter: sf.into(),
            fast_filter_threshold: fth.into(),
            watchdog_state: wd.into(),
            fields: bytes,
        }
    }
}

impl From<Configuration> for u16 {
    fn from(_config: Configuration) -> Self {
        todo!("implement reverse conversion")
    }
}
