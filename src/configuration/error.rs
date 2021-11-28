/// Errors of converting/parsing configuration bytes.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid power mode bit pattern.
    PowerModeBitPattern(u8),

    /// Invalid hysteresis bit pattern.
    HysteresisBitPattern(u8),

    /// Invalid pwm frequency bit pattern.
    PwmFreqBitPattern(u8),

    /// Invalid slow filter mode bit pattern.
    SlowFilterModeBitPattern(u8),

    /// Invalid fast filter threshold bit pattern.
    FastFilterThresholdBitPattern(u8),

    /// Invalid watchdog state bit pattern.
    WatchdogState(u8),

    /// Invalid output stage configuration bit pattern.
    OutputStageBitPattern(u8),
}
