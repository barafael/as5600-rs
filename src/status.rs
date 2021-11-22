#[derive(Debug, PartialEq)]
pub enum Status {
    MagnetHigh,
    MagnetLow,
    MagnetDetected,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidBitPattern(u8),
}

impl TryFrom<u8> for Status {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let bits = value & 0b00111000;
        match bits {
            0x8 => Ok(Self::MagnetHigh),
            0x10 => Ok(Self::MagnetLow),
            0x20 => Ok(Self::MagnetDetected),
            _ => Err(Error::InvalidBitPattern(value)),
        }
    }
}

impl TryFrom<[u8; 1]> for Status {
    type Error = Error;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Self::try_from(value[0])
    }
}
