#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal as hal;
use error::Error;
use hal::blocking::i2c;
use states::Initial;

pub mod constants;
pub mod error;
pub mod options;
mod states;
pub mod status;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct As5600<I2C, D, State> {
    i2c: I2C,
    address: u8,
    delay: D,
    state: PhantomData<State>,
}

impl<I2C, D, E> As5600<I2C, D, Initial>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
            state: PhantomData::<Initial>,
        }
    }

    pub fn release(self) -> (I2C, D) {
        (self.i2c, self.delay)
    }

    pub fn get_zmco(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x00], &mut buffer)?;
        Ok(buffer[0] & 0b0000_0011)
    }

    pub fn magnet_status(&mut self) -> Result<status::Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x0b], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }

    pub fn get_zero_position(&mut self) -> Result<u16, E> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[0x01], &mut buffer)?;
        let high_byte = buffer[0] & 0b0000_1111;
        let low_byte = buffer[1];
        Ok(u16::from(high_byte) << 8 | u16::from(low_byte))
    }

    pub fn get_maximum_position(&mut self) -> Result<u16, E> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[0x03], &mut buffer)?;
        let high_byte = buffer[0] & 0b0000_1111;
        let low_byte = buffer[1];
        Ok(u16::from(high_byte) << 8 | u16::from(low_byte))
    }
}
