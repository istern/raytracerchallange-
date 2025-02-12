use std::ops::{Add, Neg, Mul,Div};

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(& self)->bool{
        self.w == 1.0
    }

    pub fn is_vector(& self)->bool{
        self.w == 0.0
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self  {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, other: f64) -> Self  {
        Self  {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, other: f64) -> Self  {
        Self  {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}