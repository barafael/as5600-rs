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
    let mut as5600 = As5600::new(i2c);
    let config = as5600.config().unwrap();
    println!("{:?}", config);

    thread::sleep(Duration::from_secs(2));

    let status = as5600.magnet_status().unwrap();
    let agc = as5600.automatic_gain_control().unwrap();
    let mag = as5600.magnitude().unwrap();
    let zmco = as5600.zmco().unwrap();

    println!("{:?}", status);
    println!("{:?}", agc);
    println!("{:?}", mag);
    println!("{:?}", zmco);

    thread::sleep(Duration::from_secs(2));

    loop {
        let value = as5600.angle().unwrap();
        println!("{:?}", value);
        thread::sleep(Duration::from_millis(100));
    }
}
```

# TODO for a more relaxed and civilized age

 [ ] make a wooden harness for the as5600 with a knob that turns a radial magnet at the right distance to the sensor
 [ ] make a CLI/GUI tool similar to [ebyte-e32-ui](https://github.com/barafael/ebyte-e32-ui)
 [ ] use klask progress bars or something comparable to show the current magnet angle

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
