use crate::{configuration, status};

/// All possible errors in this crate.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]

pub enum Error<E> {
    /// `I2C` communication error.
    Communication(E),

    /// Magnet status parsing/conversion error.
    Status(status::Error),

    /// Configuration parsing/conversion error.
    Configuration(configuration::error::Error),

    /// Persistence error: configuration can only be persisted 3 times.
    MaximumPositionPersistsReached,

    /// Magnet is required for persistence action.
    MagnetRequired,

    /// No more persistent writes available for maximum angle and configuration registers.
    MangConfigPersistenceExhausted,

    /// Invalid I2C address for AS5600L. Address must be between 8 and 119.
    #[cfg(feature = "as5600l")]
    InvalidAddress,
}
