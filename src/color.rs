use std::ops::{Add, Sub, Mul};

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Self) -> Self  {
        Self  {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self  {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self  {
        Self  {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, other: f64) -> Self  {
        Self  {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}