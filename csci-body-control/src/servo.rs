use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Servo {
    max_pulse: u16,
    min_pulse: u16,
    factor: f32,
}

impl Servo {
    /// Create a new servo
    /// Parameters:
    /// * min_pulse: Minimum servo pulse length
    /// * max_pulse: Maximum servo pulse length
    /// * factor: Servo pulses per redian of rotation
    pub fn new(min_pulse: u16, max_pulse: u16, factor: f32) -> Servo {
        Servo {
            max_pulse,
            min_pulse,
            factor,
        }
    }

    pub fn new_standard(min_pulse: u16, max_pulse: u16) -> Servo {
        let range = max_pulse - min_pulse;
        Self::new(min_pulse, max_pulse, (range as f32)/std::f32::consts::PI)
    }

    pub fn center(&self) -> u16 {
        (self.max_pulse + self.min_pulse) / 2
    }

    pub fn to(&self, angle: f32) -> u16 {
        if !angle.is_finite() {
            self.center()
        } else {
            let pulse_float = self.center() as f32 + angle * self.factor;
            pulse_float.round().clamp(self.min_pulse as f32, self.max_pulse as f32) as u16
        }
        
    }

    pub fn to_deg(&self, angle: f32) -> u16 {
        self.to(f32::to_radians(angle))
    }
}
