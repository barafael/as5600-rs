#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

//! Platform-agnostic Rust driver for the AS5600 digital magnetic potentiometer.

use configuration::Configuration;
use embedded_hal;
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
mod test;

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

    /// Get value of register RAW_ANGLE.
    /// This register holds the unfiltered unprocessed value of the measured angle.
    pub fn get_raw_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::RawAngle.into())? & 0x0FFF)
    }

    /// Get value of register ANGLE.
    /// This register holds the filtered value of the measured angle.
    pub fn get_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Angle.into())? & 0x0FFF)
    }

    /// Get value of register ZMCO.
    /// This register holds the number of persistent burns to angle and config registers.
    pub fn get_zmco(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Zmco.into()], &mut buffer)?;
        Ok(buffer[0] & 0b0000_0011)
    }

    /// Get value of register STATUS.
    /// This register indicates whether a magnet is detected, or too low, or too high.
    pub fn magnet_status(&mut self) -> Result<status::Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Status.into()], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }

    /// Get value of register ZPOS.
    /// This register holds the value of the programmed zero-position.
    pub fn get_zero_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Zpos)? & 0x0FFF)
    }

    /// Get value of register MPOS.
    /// This register holds the value of the programmed maximum position.
    pub fn get_maximum_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mpos)? & 0x0FFF)
    }

    /// Get value of register MANG.
    /// This register holds the value of the programmed maximum angle.
    pub fn get_maximum_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mang)? & 0x0FFF)
    }

    /// Get value of register CONF and parse it.
    /// This register holds R/W/P bitfields of the current configuration.
    pub fn get_config(&mut self) -> Result<Configuration, Error<E>> {
        let bytes = self.read_u16(Register::Conf)?;
        configuration::Configuration::try_from(bytes).map_err(Error::Configuration)
    }

    /// Get value of register AGC.
    /// This register holds the current automatic gain control value.
    /// This value is different depending on the supply voltage (5V or 3v3), see datasheet.
    pub fn get_automatic_gain_control(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x1a], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Get value of register MAGNITUDE.
    /// This value holds the measured magnitude of the applied magnetic field.
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
}
