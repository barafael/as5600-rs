#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    Register(u8),
}
