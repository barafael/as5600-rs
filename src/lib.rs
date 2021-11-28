#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use configuration::Configuration;
use embedded_hal;
use embedded_hal::blocking::i2c;
use error::Error;
use register::Register;

pub mod configuration;
pub mod constants;
pub mod error;
pub(crate) mod register;
pub mod status;
#[cfg(test)]
mod test;

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
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    pub fn release(self) -> (I2C, D) {
        (self.i2c, self.delay)
    }

    pub fn get_raw_angle(&mut self) -> Result<u16, Error<E>> {
        Ok(self.read_u16(Register::RawAngle.into())? & 0x0FFF)
    }

    pub fn get_angle(&mut self) -> Result<u16, Error<E>> {
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

    pub fn magnet_status(&mut self) -> Result<status::Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Status.into()], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }

    pub fn get_zero_position(&mut self) -> Result<u16, Error<E>> {
        Ok(self.read_u16(Register::Zpos)? & 0x0FFF)
    }

    pub fn get_maximum_position(&mut self) -> Result<u16, Error<E>> {
        Ok(self.read_u16(Register::Mpos)? & 0x0FFF)
    }

    pub fn get_maximum_angle(&mut self) -> Result<u16, Error<E>> {
        Ok(self.read_u16(Register::Mang)? & 0x0FFF)
    }

    pub fn get_config(&mut self) -> Result<Configuration, Error<E>> {
        let bytes = self.read_u16(Register::Conf)?;
        configuration::Configuration::try_from(bytes).map_err(Error::Configuration)
    }

    pub fn get_automatic_gain_control(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x1a], &mut buffer)?;
        Ok(buffer[0])
    }

    pub fn get_magnitude(&mut self) -> Result<u16, Error<E>> {
        Ok(self.read_u16(Register::Magnitude)? & 0x0FFF)
    }

    fn read_u16(&mut self, command: Register) -> Result<u16, Error<E>> {
        let mut buffer = [0u8; 2];
        self.i2c
            .write_read(self.address, &[command.into()], &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }
}
