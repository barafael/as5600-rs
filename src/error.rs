use crate::status;

#[derive(Debug, PartialEq)]
pub enum Error<E> {
    Communication(E),
    Status(status::Error),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::Communication(e)
    }
}
