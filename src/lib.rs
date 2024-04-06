#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

//! Platform-agnostic Rust driver for the AS5600 digital magnetic potentiometer.

#![doc = include_str!("../README.md")]

pub use as5600::*;

//#[cfg(feature = "async")]

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

mod as5600;

#[cfg(feature = "async")]
pub mod asynch;
