#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

//! Platform-agnostic Rust driver for the AS5600 digital magnetic potentiometer.

use configuration::Configuration;
use embedded_hal::blocking::i2c;
use error::Error;
use register::Register;

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
#[derive(Debug)]
pub struct As5600<I2C, D> {
    i2c: I2C,
    address: u8,
    delay: D,
}

impl<I2C, D, E> As5600<I2C, D>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Create a new As5600 driver instance.
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    /// Release this drivers' resources, dropping it.
    pub fn release(self) -> (I2C, D) {
        (self.i2c, self.delay)
    }

    /// Get value of register `RAW_ANGLE`.
    pub fn get_raw_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::RawAngle)? & 0x0FFF)
    }

    /// Get value of register `ANGLE`.
    pub fn get_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Angle)? & 0x0FFF)
    }

    /// Get value of register `ZMCO`.
    pub fn get_zmco(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Zmco.into()], &mut buffer)?;
        Ok(buffer[0] & 0b0000_0011)
    }

    /// Get value of register `STATUS`.
    pub fn magnet_status(&mut self) -> Result<status::Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Status.into()], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }

    /// Get value of register `ZPOS`.
    pub fn get_zero_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Zpos)? & 0x0FFF)
    }

    /// Set value of register `ZPOS`.
    pub fn set_zero_position(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Zpos, bytes & 0x0FFF)
    }

    /// Get value of register `MPOS`.
    pub fn get_maximum_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mpos)? & 0x0FFF)
    }

    /// Set value of register `MPOS`.
    pub fn set_maximum_position(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Mpos, bytes & 0x0FFF)
    }

    /// Get value of register `MANG`.
    pub fn get_maximum_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mang)? & 0x0FFF)
    }

    /// Set value of register `MANG`.
    pub fn set_maximum_angle(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Mang, bytes & 0x0FFF)
    }

    /// Get value of register `CONF` and parse it.
    pub fn get_config(&mut self) -> Result<Configuration, Error<E>> {
        let bytes = self.read_u16(Register::Conf)?;
        configuration::Configuration::try_from(bytes).map_err(Error::Configuration)
    }

    /// Set value of register `CONF`.
    pub fn set_config(&mut self, config: Configuration) -> Result<(), Error<E>> {
        // TODO to preserve top-most 2 bits, the proper thing to do would be to read them here.
        let bytes = u16::from(config);
        self.write_u16(Register::Conf, bytes)
    }

    /// Get value of register `AGC`.
    /// This value differs depending on the supply voltage (5V or 3v3), see datasheet.
    pub fn get_automatic_gain_control(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x1a], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Get value of register `MAGNITUDE`.
    pub fn get_magnitude(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Magnitude)? & 0x0FFF)
    }

    /// Helper function for write-reading 2 bytes from the given register.
    fn read_u16(&mut self, command: Register) -> Result<u16, Error<E>> {
        let mut buffer = [0u8; 2];
        self.i2c
            .write_read(self.address, &[command.into()], &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Helper function for writing 2 bytes to the given register.
    fn write_u16(&mut self, command: Register, bytes: u16) -> Result<(), Error<E>> {
        let bytes: [u8; 2] = bytes.to_be_bytes();
        let buffer = [u8::from(command), bytes[0], bytes[1]];
        Ok(self.i2c.write(self.address, &buffer)?)
    }
}
