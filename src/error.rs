use crate::{configuration, status};

/// Crate errors.
#[derive(Debug, PartialEq)]
pub enum Error<E> {
    /// `I2C` communication error.
    Communication(E),

    /// Magnet status parsing/conversion error.
    Status(status::Error),

    /// Configuration parsing/conversion error.
    Configuration(configuration::error::Error),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::Communication(e)
    }
}
