#[cfg(test)]
use proptest_derive::Arbitrary;

pub mod error;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum Register {
    Zmco = 0x00,
    Zpos = 0x01,
    Mpos = 0x03,
    Mang = 0x05,
    Conf = 0x07,
    RawAngle = 0x0C,
    Angle = 0x0E,
    Status = 0x0B,
    Agc = 0x1A,
    Magnitude = 0x1B,
    Burn = 0xFF,
}

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg as u8
    }
}

impl TryFrom<u8> for Register {
    type Error = error::Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Register::Zmco),
            0x01 => Ok(Register::Zpos),
            0x03 => Ok(Register::Mpos),
            0x05 => Ok(Register::Mang),
            0x07 => Ok(Register::Conf),
            0x0C => Ok(Register::RawAngle),
            0x0E => Ok(Register::Angle),
            0x0B => Ok(Register::Status),
            0x1A => Ok(Register::Agc),
            0x1B => Ok(Register::Magnitude),
            0xFF => Ok(Register::Burn),
            _ => Err(error::Error::Register(byte)),
        }
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use crate::register::Register;

    proptest! {
        #[test]
        fn register_to_byte_roundtrip(register in any::<Register>()) {
            let byte = u8::from(register);
            let back = Register::try_from(byte).unwrap();
            assert_eq!(register, back);
        }

        #[test]
        fn byte_to_register_roundtrip(byte in any::<u8>()) {
            if let Ok(register) = Register::try_from(byte) {
                let back = u8::from(register);
                assert_eq!(byte, back);
            }
        }
    }
}
