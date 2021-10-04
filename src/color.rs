use std::ops::{Add, Div, Mul};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn color_to_int(f: f64) -> i32 {
    (255.99f64 * f) as i32
}

impl Color {
    pub fn ir(&self) -> i32 {
        color_to_int(self.r)
    }
    pub fn ig(&self) -> i32 {
        color_to_int(self.g)
    }
    pub fn ib(&self) -> i32 {
        color_to_int(self.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
