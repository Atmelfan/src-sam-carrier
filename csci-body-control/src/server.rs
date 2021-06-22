use std::{fs::File, thread::sleep, time};

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use serde::{Serialize, Deserialize};
use nalgebra as na;



mod servo;
mod cephalothorax;
mod gait;
mod util;

use crate::cephalothorax::Leg;
use crate::servo::Servo;
use crate::cephalothorax::Cephalothorax;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub cephalothorax: Cephalothorax
}

fn fixed_slice(s: &[u16]) -> [u16; 16]  {
    if s.len() != 16 {
        panic!("Slice must have a length of 16")
    }
    let mut x = [0u16; 16];
    for (i, v) in s.iter().enumerate() {
        x[i] = *v;
    }
    x
}

/// Liste
fn body_model_subscriber() {
    let s = Socket::new(Protocol::Sub0)?;
    s.listen(url)?;

    println!("SUBSCRIBER: SUBSCRIBING TO ALL TOPICS");
    let all_topics = vec![];
    s.set_opt::<Subscribe>(all_topics)?;
}

/// Liste
fn gait_ctrl_subscriber() {

}

/// Liste
fn body_status_publisher() {

}

fn main() {
    // Load config
    let config_file = File::open("body-control/config.json").expect("Failed to open config");
    let config: Config = serde_json::from_reader(config_file).expect("Error reading json");
    println!("{:?}", &config);
    
    // Init PWM driver
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();
    
    pwm.set_prescale(30).unwrap();
    pwm.enable().unwrap();

    let on = [0u16; 32];
    let mut off = [0u16; 32];

    for leg in config.cephalothorax.legs.iter() {
        let angles = leg.ik.calculate(na::Point3::new(200.0, 0.0, 0.0));
        println!("{} -> {}, {}, {}", leg.name, (angles[0] + leg.offset[0]).to_degrees(), (angles[1] + leg.offset[1]).to_degrees(), (angles[2] + leg.offset[2]).to_degrees());
        for (x, angle) in angles.iter().enumerate() {
            let c = config.cephalothorax.leg_servos.to(*angle + leg.offset[x]);
            off[leg.servos[x]] = c;
            println!("s{} = {}", leg.servos[x], c);
        }
        
    }
    println!("{:?}", off);
    pwm.set_all_on_off(&fixed_slice(&on[..16]), &fixed_slice(&off[..16])).unwrap();


    let _dev = pwm.destroy(); // Get the I2C device back
}