use crate::configuration::error::Error;

use super::{
    FastFilterThreshold, Hysteresis, OutputStage, PowerMode, PwmFreq, SlowFilter, WatchdogState,
};

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
