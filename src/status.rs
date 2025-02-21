use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[cfg(test)]
use proptest_derive::Arbitrary;

/// Magnet detection status parsing/conversion error.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Bit pattern not valid for magnet detection status.
    InvalidBitPattern(u8),
}

/// Magnet detection status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Status {
    /// Magnet too close.
    MagnetHigh = 0x8,
    /// Magnet too far.
    MagnetLow = 0x10,
    /// Magnet detected.
    MagnetDetected = 0x20,
    /// Magnet detected, but close.
    MagnetDetectedHigh = 0x28,
    /// Magnet detected, but low.
    MagnetDetectedLow = 0x30,
}

impl TryFrom<u8> for Status {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // See datasheet Fig. 21, section "Status Registers".
        FromPrimitive::from_u8(value & 0b0011_1000).ok_or(Error::InvalidBitPattern(value))
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
        status as Self
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
