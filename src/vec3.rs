use std::ops::Add;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
}
