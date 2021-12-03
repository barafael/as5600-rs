# as5600-rs
Rust Embedded HAL driver for the AS5600 contactless 12-bit digital potentiometer

# Status

- [x] Reading/parsing all device registers
- [x] Writing configuration settings
- [x] Setting zero position, maximum position, maximum angle
- [x] Burn Settings and angle

# Example

Here's how using this driver looks on a raspberry pi:

```rust
use std::{thread, time::Duration};

use as5600::As5600;
use linux_embedded_hal::I2cdev;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let delay = linux_embedded_hal::Delay;
    let mut as5600 = As5600::new(i2c, as5600::constants::DEFAULT_I2C_ADDRESS, delay);
    let config = as5600.config().unwrap();
    dbg!(config);

    thread::sleep(Duration::from_secs(5));

    let status = as5600.magnet_status().unwrap();
    let agc = as5600.automatic_gain_control().unwrap();
    let mag = as5600.magnitude().unwrap();
    let zmco = as5600.zmco().unwrap();

    dbg!(status);
    dbg!(agc);
    dbg!(mag);
    dbg!(zmco);

    thread::sleep(Duration::from_secs(5));

    loop {
        let value = as5600.angle().unwrap();
        dbg!(value);
        thread::sleep(Duration::from_millis(100));
    }
}
```