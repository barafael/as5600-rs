use core::time::Duration;

pub enum PowerMode {
    Nom,
    Lpm1,
    Lpm2,
    Lpm3,
}

pub enum Hysteresis {
    Lsb1,
    Lsb2,
    Lsb3,
}

pub enum OutputStage {
    Analog,
    ReducedAnalog,
    DigitalPwm,
}

pub enum PwmFreq {
    PwmF1 = 115,
    PwmF2 = 230,
    PwmF3 = 460,
    PwmF4 = 920,
}

pub enum SlowFilter {
    X16,
    X8,
    X4,
    X2,
}

pub enum FastFilterThreshold {
    SlowFilterOnly,
    Lsb6,
    Lsb7,
    Lsb9,
    Lsb18,
    Lsb21,
    Lsb24,
    Lsb10,
}

pub mod watchdog {
    pub struct On;
    pub struct Off;
}
