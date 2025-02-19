use std::ops::{Sub,Add};

use crate::vector::Vector;
use crate::tuple::Tuple;
// Implement PartialEq so Point can be compared with Tuple
impl PartialEq<Tuple> for Point {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        other.w == 1.0
    }
}

impl PartialEq<Tuple> for Vector {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        other.w == 0.0
    }
}

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, other: Self) -> Vector  {
        Vector  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    // ✅ Compare with a small tolerance (epsilon)
    pub fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
        (self.x - other.x).abs() < epsilon &&
        (self.y - other.y).abs() < epsilon &&
        (self.z - other.z).abs() < epsilon
    }
}


impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Point  {
        Point  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point  {
        Point  {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PartialEq<Point> for Tuple {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == 1.0
    }
}

