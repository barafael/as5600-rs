#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal as hal;
use states::Initial;

pub mod options;
pub mod status;
mod states;
pub mod constants;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct As5600<I2C, D, State> {
    i2c: I2C,
    address: u8,
    delay: D,
    state: PhantomData<State>,
}

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
