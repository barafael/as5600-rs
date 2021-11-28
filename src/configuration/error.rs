/// Errors of converting/parsing configuration bytes.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid bit pattern for the output stage configuration.
    OutputStageBitPattern(u8),
}
