use std::ops::{Sub, Mul, Add};
use crate::point::Point;
use crate::tuple::Tuple;

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl PartialEq<Vector> for Tuple {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == 0.0
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self  {
        Self  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self  {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Vector {
    type Output = Point;
    fn sub(self, other: Point) -> Point  {
        Point  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

pub fn magnitude(vector: Vector) -> f64{
    (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2)).sqrt()
}

pub fn normalize(vector: Vector) -> Vector{
    let mag = magnitude(vector);
    Vector {
        x: (vector.x / mag), 
        y: (vector.y / mag),
        z: (vector.z / mag),
    }
}

pub fn dot(vector_a: Vector, vector_b: Vector) -> f64{
    vector_a.x*vector_b.x + vector_a.y*vector_b.y + vector_a.z*vector_b.z
}

pub fn cross(vector_a: Vector, vector_b: Vector) -> Vector{
    Vector{
        x: vector_a.y * vector_b.z - vector_a.z * vector_b.y,
        y: vector_a.z * vector_b.x - vector_a.x * vector_b.z,
        z: vector_a.x * vector_b.y - vector_a.y * vector_b.x,
    }
}

pub fn reflect(vector_a: Vector, vector_b: Vector) -> Vector{
    vector_a - (vector_b * (2.0 * dot(vector_a, vector_b)))
}