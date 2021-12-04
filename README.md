# as5600-rs
Rust Embedded HAL driver for the AS5600 contactless 12-bit digital potentiometer

# Status

- [x] Reading/parsing all device registers
- [x] Writing configuration settings
- [x] Setting zero position, maximum position, maximum angle
- [x] Burn Settings and angle

# Example

Here's how using this driver looks on a raspberry pi:

```rust,no_run
use std::{thread, time::Duration};

use as5600::As5600;
use linux_embedded_hal::I2cdev;

fn main() {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let delay = linux_embedded_hal::Delay;
    let mut as5600 = As5600::new(as5600::constants::DEFAULT_I2C_ADDRESS, delay);
    let config = as5600.config(&mut i2c).unwrap();
    println!("{:?}", config);

    thread::sleep(Duration::from_secs(5));

    let status = as5600.magnet_status(&mut i2c).unwrap();
    let agc = as5600.automatic_gain_control(&mut i2c).unwrap();
    let mag = as5600.magnitude(&mut i2c).unwrap();
    let zmco = as5600.zmco(&mut i2c).unwrap();

    println!("{:?}", status);
    println!("{:?}", agc);
    println!("{:?}", mag);
    println!("{:?}", zmco);

    thread::sleep(Duration::from_secs(5));

    loop {
        let value = as5600.angle(&mut i2c).unwrap();
        println!("{:?}", value);
        thread::sleep(Duration::from_millis(100));
    }
}
```
