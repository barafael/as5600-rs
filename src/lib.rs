#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

//! Platform-agnostic Rust driver for the AS5600 digital magnetic potentiometer.

#![doc = include_str!("../README.md")]

use configuration::Configuration;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c;
use error::Error;
use register::Register;
use status::Status;

/// Configuration of As5600.
pub mod configuration;
/// Constants.
pub mod constants;
/// Errors.
pub mod error;
/// Registers.
pub(crate) mod register;
/// Magnet detection status.
pub mod status;
#[cfg(test)]
mod test_reading;
#[cfg(test)]
mod test_writing;

/// As5600 driver instance.
#[derive(Debug, PartialEq)]
pub struct As5600 {
    address: u8,
}

impl As5600 {
    /// Create a new As5600 driver instance.
    pub fn new(address: u8) -> Self {
        Self { address }
    }

    /// Get value of register `RAW_ANGLE`.
    pub fn raw_angle<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::RawAngle, i2c)? & 0x0FFF)
    }

    /// Get value of register `ANGLE`.
    pub fn angle<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::Angle, i2c)? & 0x0FFF)
    }

    /// Get value of register `ZMCO`.
    pub fn zmco<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u8, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let mut buffer = [0u8; 1];
        i2c.write_read(self.address, &[Register::Zmco.into()], &mut buffer)?;
        Ok(buffer[0] & 0b0000_0011)
    }

    /// Get value of register `STATUS`.
    pub fn magnet_status<I2c, E>(&mut self, i2c: &mut I2c) -> Result<status::Status, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let mut buffer = [0u8; 1];
        i2c.write_read(self.address, &[Register::Status.into()], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }

    /// Get value of register `ZPOS`.
    pub fn zero_position<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::Zpos, i2c)? & 0x0FFF)
    }

    /// Set value of register `ZPOS`.
    pub fn set_zero_position<I2c, E>(&mut self, bytes: u16, i2c: &mut I2c) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        self.write_u16(Register::Zpos, bytes & 0x0FFF, i2c)
    }

    /// Get value of register `MPOS`.
    pub fn maximum_position<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::Mpos, i2c)? & 0x0FFF)
    }

    /// Set value of register `MPOS`.
    pub fn set_maximum_position<I2c, E>(
        &mut self,
        bytes: u16,
        i2c: &mut I2c,
    ) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        self.write_u16(Register::Mpos, bytes & 0x0FFF, i2c)
    }

    /// Get value of register `MANG`.
    pub fn maximum_angle<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::Mang, i2c)? & 0x0FFF)
    }

    /// Set value of register `MANG`.
    pub fn set_maximum_angle<I2c, E>(&mut self, bytes: u16, i2c: &mut I2c) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        self.write_u16(Register::Mang, bytes & 0x0FFF, i2c)
    }

    /// Get value of register `CONF` and parse it.
    pub fn config<I2c, E>(&mut self, i2c: &mut I2c) -> Result<Configuration, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let bytes = self.read_u16(Register::Conf, i2c)?;
        configuration::Configuration::try_from(bytes).map_err(Error::Configuration)
    }

    /// Set value of register `CONF`.
    pub fn set_config<I2c, E>(
        &mut self,
        config: Configuration,
        i2c: &mut I2c,
    ) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // See note in datasheet about "blank fields may contain factory settings" on page 18.
        let current_config = self.read_u16(Register::Conf, i2c)?;
        let blank_fields = current_config & 0b1100_0000_0000_0000;
        let mut bytes = u16::from(config);
        bytes |= blank_fields;
        self.write_u16(Register::Conf, bytes, i2c)
    }

    /// Get value of register `AGC`.
    /// This value differs depending on the supply voltage (5V or 3v3), see datasheet.
    pub fn automatic_gain_control<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u8, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let mut buffer = [0u8; 1];
        i2c.write_read(self.address, &[0x1a], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Get value of register `MAGNITUDE`.
    pub fn magnitude<I2c, E>(&mut self, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        // 12-bit value.
        Ok(self.read_u16(Register::Magnitude, i2c)? & 0x0FFF)
    }

    /// Burn maximum angle and config register.
    /// Only proceeds if position settings (MPOS and ZPOS) have never been persisted before.
    /// See datasheet for constraints.
    pub fn persist_maximum_angle_and_config_settings<I2c, D, E>(
        &mut self,
        i2c: &mut I2c,
        delay: &mut D,
    ) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
        D: DelayMs<u32>,
    {
        let zmco = self.zmco(i2c)?;
        if zmco != 0 {
            return Err(Error::MangConfigPersistenceExhausted);
        }
        i2c.write(self.address, &[Register::Burn.into(), 0x40])?;
        delay.delay_ms(1);
        Ok(())
    }

    /// Burn zero position and maximum to As5600 memory, if ZMCO permits it and a magnet is detected.
    /// See datasheet for constraints.
    pub fn persist_position_settings<I2c, D, E>(
        &mut self,
        i2c: &mut I2c,
        delay: &mut D,
    ) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
        D: DelayMs<u32>,
    {
        let zmco = self.zmco(i2c)?;
        if zmco >= 3 {
            return Err(Error::MaximumPositionPersistsReached);
        }
        if self.magnet_status(i2c)? != Status::MagnetDetected {
            return Err(Error::MagnetRequired);
        }
        i2c.write(self.address, &[Register::Burn.into(), 0x80])
            .map_err(Error::Communication)?;
        delay.delay_ms(1);
        Ok(())
    }

    /// Helper function for write-reading 2 bytes from the given register.
    fn read_u16<I2c, E>(&mut self, command: Register, i2c: &mut I2c) -> Result<u16, Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let mut buffer = [0u8; 2];
        i2c.write_read(self.address, &[command.into()], &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Helper function for writing 2 bytes to the given register.
    fn write_u16<I2c, E>(
        &mut self,
        command: Register,
        bytes: u16,
        i2c: &mut I2c,
    ) -> Result<(), Error<E>>
    where
        I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    {
        let bytes: [u8; 2] = bytes.to_be_bytes();
        let buffer = [u8::from(command), bytes[0], bytes[1]];
        Ok(i2c.write(self.address, &buffer)?)
    }
}
