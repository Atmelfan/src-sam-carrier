use std::{thread::sleep, time};

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use crate::servo::Servo;

mod servo;

const S901D_MIN_PULSE: u16 = 403;
const S901D_MAX_PULSE: u16 = 2016;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    let servo1 = Servo::new_standard(S901D_MIN_PULSE, S901D_MAX_PULSE);

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(30).unwrap();
    pwm.enable().unwrap();

    let on = [0u16; 16];
    let mut off = [servo1.center(); 16];

    // Turn on channel 0 at 0.
    pwm.set_all_on_off(&on, &off).unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`
 


    let _dev = pwm.destroy(); // Get the I2C device back
}