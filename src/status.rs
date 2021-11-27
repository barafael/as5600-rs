#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
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
        // See datasheet Fig. 21, section "Status Registers".
        let bits = value & 0b0011_1000;
        match bits {
            0x8 => Ok(Self::MagnetHigh),
            0x10 => Ok(Self::MagnetLow),
            0x20 => Ok(Self::MagnetDetected),
            _ => Err(Error::InvalidBitPattern(bits)),
        }
    }
}

impl TryFrom<[u8; 1]> for Status {
    type Error = Error;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Self::try_from(value[0])
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::MagnetHigh => 0x8,
            Status::MagnetLow => 0x10,
            Status::MagnetDetected => 0x20,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::status::Status;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn status_to_byte_roundtrip(status in any::<Status>()) {
            let byte = u8::from(status);
            let roundtrip = Status::try_from(byte).unwrap();
            assert_eq!(status, roundtrip);
        }
    }
}
