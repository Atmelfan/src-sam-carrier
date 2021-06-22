use nalgebra as na;
use serde::{Deserialize, Serialize};

use crate::servo::Servo;

#[derive(Serialize, Deserialize, Debug)]
pub struct LegIk {
    lengths: [f32; 3],
    mirror: bool,
}

impl LegIk {
    /// Calculate an IK solution in leg-local space
    /// TODO: Boundry check
    pub fn calculate(&self, t: na::Point3<f32>) -> [f32; 3] {
        let (x, y, z) = (t[0], t[1], t[2]);

        let d_sq = x.powi(2) + y.powi(2);
        let d = d_sq.sqrt() - self.lengths[0];
        let v = (d.powi(2) + z.powi(2)).sqrt();
        let s1 = (v.powi(2) + self.lengths[1].powi(2) - self.lengths[2].powi(2))
            / (2.0 * v * self.lengths[1]);
        let s2 = (self.lengths[1].powi(2) + self.lengths[2].powi(2) - v.powi(2))
            / (2.0 * self.lengths[1] * self.lengths[2]);

        // Invert if mirrored
        let inv = if self.mirror { -1.0 } else { 1.0 };
        [
            inv * f32::atan2(y, x),
            -inv * (f32::acos(s1) + f32::atan2(z, d)),
            inv * f32::acos(s2),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegTransform {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {
    pub name: String,
    pub servos: [usize; 3],
    pub offset: [f32; 3],
    pub ik: LegIk,
}

impl Leg {
    /// Calculate ik solution with servo offsets
    fn calculate_ik(&self, t: na::Point3<f32>) -> [f32; 3] {
        let mut c = self.ik.calculate(t);
        c.iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x += self.offset[i]);
        c
    }
}

fn default_transform() -> na::Transform3<f32> {
    na::Transform3::identity()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cephalothorax {
    pub leg_servos: Servo,
    pub eye_servos: Servo,
    pub legs: [Leg; 8],
    #[serde(skip, default = "default_transform")]
    body_transform: na::Transform3<f32>,
}
