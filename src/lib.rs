#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod test;

use core::marker::PhantomData;
use embedded_hal as hal;

pub const DEFAULT_I2C_ADDRESS: u8 = 0x36;

#[derive(Debug)]
pub struct As5600<I2C, D, State> {
    i2c: I2C,
    address: u8,
    delay: D,
    state: PhantomData<State>,
}

pub struct Initial;

impl<I2C, D> As5600<I2C, D, Initial> {
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
}
